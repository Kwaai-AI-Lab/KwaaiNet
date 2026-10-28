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

/// Write `token` to `path`, owner-only on Unix, replacing any previous file
/// atomically so a client never reads a half-written token.
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
        f.write_all(token.as_bytes())?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)
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

    #[test]
    fn empty_token_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(TOKEN_FILE);
        std::fs::write(&path, "\n").unwrap();
        assert!(read_token_file(&path).is_err());
    }
}
