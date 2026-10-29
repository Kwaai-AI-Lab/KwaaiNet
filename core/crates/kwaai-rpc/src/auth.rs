//! Bearer-token auth for the daemon's gRPC surface.
//!
//! The daemon draws a fresh token at every start and writes it to
//! [`TOKEN_FILE`] in its `run/` dir, readable only by its own user. A client
//! that can read that file may drive the daemon. A client that reaches the
//! file by another path (a container's bind mount) names it in
//! [`TOKEN_FILE_ENV`]. Same scheme as Bitcoin Core's `.cookie` and Tor's
//! control-port cookie.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::{Request, Status};

/// File name under the daemon's `run/` dir.
pub const TOKEN_FILE: &str = "kwaainet.token";

/// Env var naming the token file, for a client whose own `run/` dir is not
/// the daemon's. A path rather than the value, so it survives the daemon
/// restarting and the secret stays out of the environment. Never read by the
/// daemon.
pub const TOKEN_FILE_ENV: &str = "KWAAINET_GRPC_TOKEN_FILE";

const SCHEME: &str = "Bearer ";

/// 32 bytes from the OS RNG, hex-encoded.
pub fn generate() -> io::Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).map_err(io::Error::other)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Write `token` to `path`, readable by the current user only, replacing any
/// previous file atomically so a client never reads a half-written token.
pub fn write_token_file(path: &Path, token: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("token.tmp");
    let _ = std::fs::remove_file(&tmp);
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut opts, 0o600);
    {
        use std::io::Write as _;
        let mut f = opts.open(&tmp)?;
        // Before the secret goes in, so the file is never readable by others.
        #[cfg(windows)]
        windows_acl::restrict_to_current_user(&tmp)?;
        f.write_all(token.as_bytes())?;
        f.sync_all()?;
    }
    replace(&tmp, path)
}

/// Windows refuses to replace a file another process has open without
/// FILE_SHARE_DELETE, and a client may be mid-read; retry briefly.
fn replace(from: &Path, to: &Path) -> io::Result<()> {
    let mut attempt = 1u64;
    loop {
        match std::fs::rename(from, to) {
            Err(_) if cfg!(windows) && attempt < 5 => {
                std::thread::sleep(std::time::Duration::from_millis(20 * attempt));
                attempt += 1;
            }
            r => return r,
        }
    }
}

/// Read a token file, trimming the trailing newline an operator may add.
pub fn read_token_file(path: &Path) -> io::Result<String> {
    let token = std::fs::read_to_string(path)?.trim().to_owned();
    if token.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "token file is empty",
        ));
    }
    Ok(token)
}

/// The token a client should send: the file [`TOKEN_FILE_ENV`] names if set,
/// else the daemon's file under `run_dir`.
pub fn client_token(run_dir: &Path) -> io::Result<String> {
    match std::env::var_os(TOKEN_FILE_ENV) {
        Some(p) if !p.is_empty() => read_token_file(Path::new(&p)),
        _ => read_token_file(&token_path(run_dir)),
    }
}

/// `<run_dir>/kwaainet.token`.
pub fn token_path(run_dir: &Path) -> PathBuf {
    run_dir.join(TOKEN_FILE)
}

/// Server-side check: rejects any request whose `authorization` header is
/// not `Bearer <token>`. Runs before the handler, for streaming RPCs too.
#[derive(Clone)]
pub struct RequireBearer {
    token: Arc<str>,
}

impl RequireBearer {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl Interceptor for RequireBearer {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        let presented = req
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix(SCHEME));
        match presented {
            Some(t) if constant_time_eq(t.as_bytes(), self.token.as_bytes()) => Ok(req),
            _ => Err(Status::unauthenticated("missing or invalid gRPC token")),
        }
    }
}

/// Client-side: attaches `authorization: Bearer <token>` to every request.
#[derive(Clone)]
pub struct BearerAuth {
    header: MetadataValue<tonic::metadata::Ascii>,
}

impl BearerAuth {
    /// Fails only if `token` holds characters a header cannot carry.
    pub fn new(token: &str) -> io::Result<Self> {
        let header = format!("{SCHEME}{token}").parse().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "token is not a valid header value",
            )
        })?;
        Ok(Self { header })
    }
}

impl Interceptor for BearerAuth {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut()
            .insert("authorization", self.header.clone());
        Ok(req)
    }
}

/// Owner-only access on Windows: a protected DACL (nothing inherited from the
/// directory) with one ACE granting the current user full control.
#[cfg(windows)]
pub(crate) mod windows_acl {
    use std::ffi::OsStr;
    use std::io;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;
    use std::ptr::null_mut;

    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, ERROR_SUCCESS, HANDLE};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
        SetNamedSecurityInfoW, SDDL_REVISION_1, SE_FILE_OBJECT,
    };
    use windows_sys::Win32::Security::{
        GetSecurityDescriptorDacl, GetTokenInformation, TokenUser, ACL, DACL_SECURITY_INFORMATION,
        PROTECTED_DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, TOKEN_QUERY, TOKEN_USER,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    fn wide(s: &OsStr) -> Vec<u16> {
        s.encode_wide().chain(Some(0)).collect()
    }

    /// The current user's SID as an `S-1-5-…` string.
    pub(crate) fn current_user_sid() -> io::Result<String> {
        unsafe {
            let mut token: HANDLE = null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
                return Err(io::Error::last_os_error());
            }
            let mut len = 0u32;
            GetTokenInformation(token, TokenUser, null_mut(), 0, &mut len);
            // u64 storage keeps TOKEN_USER's pointer field aligned.
            let mut buf = vec![0u64; (len as usize).div_ceil(8)];
            let ok = GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len);
            CloseHandle(token);
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }
            let user = &*(buf.as_ptr() as *const TOKEN_USER);
            let mut sid_str: *mut u16 = null_mut();
            if ConvertSidToStringSidW(user.User.Sid, &mut sid_str) == 0 {
                return Err(io::Error::last_os_error());
            }
            let n = (0..).take_while(|&i| *sid_str.add(i) != 0).count();
            let sid = String::from_utf16_lossy(std::slice::from_raw_parts(sid_str, n));
            LocalFree(sid_str.cast());
            Ok(sid)
        }
    }

    pub(crate) fn restrict_to_current_user(path: &Path) -> io::Result<()> {
        let sddl = wide(OsStr::new(&format!("D:P(A;;FA;;;{})", current_user_sid()?)));
        let path = wide(path.as_os_str());
        unsafe {
            let mut sd: PSECURITY_DESCRIPTOR = null_mut();
            if ConvertStringSecurityDescriptorToSecurityDescriptorW(
                sddl.as_ptr(),
                SDDL_REVISION_1,
                &mut sd,
                null_mut(),
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
            let (mut present, mut defaulted) = (0, 0);
            let mut dacl: *mut ACL = null_mut();
            let got = GetSecurityDescriptorDacl(sd, &mut present, &mut dacl, &mut defaulted);
            let rc = if got == 0 {
                io::Error::last_os_error().raw_os_error().unwrap_or(1) as u32
            } else {
                SetNamedSecurityInfoW(
                    path.as_ptr(),
                    SE_FILE_OBJECT,
                    DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                    null_mut(),
                    null_mut(),
                    dacl,
                    null_mut(),
                )
            };
            LocalFree(sd);
            if rc != ERROR_SUCCESS {
                return Err(io::Error::from_raw_os_error(rc as i32));
            }
        }
        Ok(())
    }
}

/// Length leaks, content does not: every token is the same length anyway.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_header(v: Option<&str>) -> Request<()> {
        let mut req = Request::new(());
        if let Some(v) = v {
            req.metadata_mut()
                .insert("authorization", v.parse().unwrap());
        }
        req
    }

    #[test]
    fn generated_tokens_are_64_hex_and_distinct() {
        let a = generate().unwrap();
        let b = generate().unwrap();
        assert_eq!(a.len(), 64);
        assert!(a.bytes().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b);
    }

    #[test]
    fn require_bearer_accepts_only_the_exact_token() {
        let mut check = RequireBearer::new("abc123");
        assert!(check.call(with_header(Some("Bearer abc123"))).is_ok());
        for bad in [
            None,
            Some("Bearer abc124"),
            Some("Bearer abc12"),
            Some("abc123"),
            Some("bearer abc123"),
        ] {
            let err = check.call(with_header(bad)).unwrap_err();
            assert_eq!(err.code(), tonic::Code::Unauthenticated, "{bad:?}");
        }
    }

    #[test]
    fn client_header_satisfies_the_server_check() {
        let mut client = BearerAuth::new("tok").unwrap();
        let req = client.call(Request::new(())).unwrap();
        assert!(RequireBearer::new("tok").call(req).is_ok());
    }

    #[test]
    fn token_file_round_trips_owner_only() {
        let dir = tempfile::tempdir().unwrap();
        let path = token_path(&dir.path().join("run"));
        write_token_file(&path, "first").unwrap();
        write_token_file(&path, "second").unwrap();
        assert_eq!(read_token_file(&path).unwrap(), "second");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
    }

    /// The DACL is exactly "this user, full control", with nothing inherited.
    #[cfg(windows)]
    #[test]
    fn token_file_dacl_is_current_user_only() {
        use std::os::windows::ffi::OsStrExt;
        use std::ptr::null_mut;
        use windows_sys::Win32::Foundation::LocalFree;
        use windows_sys::Win32::Security::Authorization::{
            ConvertSecurityDescriptorToStringSecurityDescriptorW, GetNamedSecurityInfoW,
            SDDL_REVISION_1, SE_FILE_OBJECT,
        };
        use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR};

        let dir = tempfile::tempdir().unwrap();
        let path = token_path(dir.path());
        write_token_file(&path, "secret").unwrap();
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let sddl = unsafe {
            let mut sd: PSECURITY_DESCRIPTOR = null_mut();
            let rc = GetNamedSecurityInfoW(
                wide.as_ptr(),
                SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                &mut sd,
            );
            assert_eq!(rc, 0, "GetNamedSecurityInfoW");
            let mut out: *mut u16 = null_mut();
            let ok = ConvertSecurityDescriptorToStringSecurityDescriptorW(
                sd,
                SDDL_REVISION_1,
                DACL_SECURITY_INFORMATION,
                &mut out,
                null_mut(),
            );
            assert_ne!(
                ok, 0,
                "ConvertSecurityDescriptorToStringSecurityDescriptorW"
            );
            let n = (0..).take_while(|&i| *out.add(i) != 0).count();
            let s = String::from_utf16_lossy(std::slice::from_raw_parts(out, n));
            LocalFree(out.cast());
            LocalFree(sd);
            s
        };
        let sid = windows_acl::current_user_sid().unwrap();
        // Flags: P (protected) is what matters; Windows adds AI on every write.
        let (flags, aces) = sddl.split_at(sddl.find('(').expect("one ACE"));
        assert!(flags.starts_with("D:P"), "DACL not protected: {sddl}");
        assert_eq!(aces, format!("(A;;FA;;;{sid})"));
    }

    #[test]
    fn empty_token_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(TOKEN_FILE);
        std::fs::write(&path, "\n").unwrap();
        assert!(read_token_file(&path).is_err());
    }
}
