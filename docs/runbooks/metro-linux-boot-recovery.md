# metro-linux won't boot — on-site checklist

Written 2026-08-25, for the site visit. Hardware details:
[`metro-linux-hardware.md`](metro-linux-hardware.md).

## What happened

1. A `cargo build --features cuda` with uncapped parallelism OOM-killed the box
   (72 threads × ~3 GB of `cicc` against 96 GB of RAM).
2. It stayed unreachable ~6 hours: pingable, `sshd` accepting connections but
   never sending a banner.
3. `sudo reboot` over AnyDesk. It came back on the **wrong disk** — the
   abandoned CentOS 7.2 install on the SATA SSD, whose GRUB fails with
   `no such device: d8f37ad4-…` / `vmlinuz-3.10.0-327.el7.x86_64 not found`.

**Leading theory: the NVMe dropped off the bus.** Both disks carry an ESP, so
firmware needed no CSM to reach the SATA GRUB — it just fell to the next working
UEFI entry, which means the NVMe's entry failed. A vanished root device also
explains step 2 better than memory pressure alone: kernel and network live in
RAM and keep answering, while `sshd` blocks in D-state the moment it needs to
read `/etc/passwd`. The OOM may be what knocked the drive off — I/O stalling
until the controller timed out.

Ruled out: CMOS battery / NVRAM loss. A clean `sudo reboot` never drops standby
power.

## Bring

- USB stick with a **Rocky live or install ISO**, and a second **empty** stick
  for data.
- A Torx driver for the M.2 heatsink.
- The `rl-root` LUKS passphrase if the volume is encrypted (unknown — `lsblk -f`
  will say).

## Step 1 — is the drive there at all?

**F10 → storage / device inventory.** Not the boot menu; the device list. Look
for `SAMSUNG MZVLB1T0HBLR`.

| Result | Meaning | Go to |
|---|---|---|
| Listed | Drive alive, bootloader problem | Step 3 |
| Absent | Hardware | Step 2 |

## Step 2 — reseat before condemning

Full power down, pull the cord, reseat the M.2. PCIe link-training failures and
thermal dropouts both present as a missing drive and both survive a reseat. This
is a Samsung PM981a — an OEM part with a mixed firmware record.

Re-check F10. Still absent after a reseat: treat the drive as failed and go to
Step 4 for what that costs.

## Step 3 — boot a live USB (UEFI mode) and get the data off first

Repair second. Data first, always.

```bash
vgchange -ay
pvs; vgs; lsblk -f            # CAPTURE THIS — see "the open question" below
mkdir -p /mnt/root && mount /dev/mapper/rl-root /mnt/root
```

Copy to the empty stick, in priority order:

```bash
cp -a /mnt/root/home/metro/.kwaainet/identity*   /path/to/usb/   # THE PEER ID
cp -a /mnt/root/home/metro/.kwaainet/config.yaml /path/to/usb/
cp -a /mnt/root/home/metro/.kwaainet/rag         /path/to/usb/   # graphs, larger
```

Then rebuild the boot entry:

```bash
mount /dev/nvme0n1p2 /mnt/root/boot
mount /dev/nvme0n1p1 /mnt/root/boot/efi
for d in dev proc sys run; do mount --bind /$d /mnt/root/$d; done
chroot /mnt/root

cat /etc/os-release           # confirm Rocky 8 vs 9 before proceeding
efibootmgr -v                 # what does NVRAM actually hold?
dmesg | grep -i nvme          # any controller resets or timeouts?
smartctl -a /dev/nvme0        # media errors, spare blocks, temperature

dnf reinstall -y grub2-efi-x64 shim-x64
grub2-mkconfig -o /boot/grub2/grub.cfg
efibootmgr -c -d /dev/nvme0n1 -p 1 -L "Rocky Linux" -l '\EFI\rocky\shimx64.efi'
```

**Decline every offer to partition, reclaim space, or repair the disk layout.**
Rescue mode only.

## Step 4 — if the NVMe is dead

`rl-root` is 850 GB and the NVMe PV is 952 GiB, so root fits on the NVMe alone.
If it lived there and the drive is gone, `~/.kwaainet` is gone with it: RAG
graphs, config, and the Ed25519 identity keypair.

**There is no known off-machine backup of that keypair.** Losing it is
survivable but not free — the node returns with a new peer ID, and
`12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` is hardcoded in the dream
sweep scripts, the per-project KB tables, and the DHT. Budget a day of cleanup,
not a restore.

## The open question — answer it while you are there

**Does the `rl` volume group span both disks, or live on the NVMe alone?**

```bash
sudo pvs
```

It decides two things: whether any data survives an NVMe failure, and whether
the old CentOS SATA SSD is a removable trap or a load-bearing part of root.

## Once it boots — do these before leaving

```bash
# 1. Persistent journal, so the next failure leaves evidence
sudo mkdir -p /var/log/journal
sudo systemd-tmpfiles --create --prefix /var/log/journal
sudo systemctl restart systemd-journald

# 2. Back the identity up off-machine, permanently
scp ~/.kwaainet/identity* <somewhere-not-this-machine>

# 3. Pin the boot order to the NVMe
sudo efibootmgr -v && sudo efibootmgr -o <nvme-entry>,<rest>
```

And once `pvs` confirms the SATA SSD is **not** part of the `rl` VG, clear its
stale bootloader so the fallthrough cannot happen again — this wipes 446 bytes
of MBR boot code and leaves the partition table intact:

```bash
sudo dd if=/dev/zero of=/dev/sda bs=446 count=1
```

Removing its UEFI entry with `efibootmgr -B -b <num>` closes the other path.
