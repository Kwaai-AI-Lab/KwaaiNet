# metro-linux — hardware inventory

Captured on the machine 2026-08-25, from `df -h` and `sudo lshw -short`.
Recorded verbatim below; the summary and the notes are the interpretation.

There is no SSH to metro (Tailscale trial ended 2026-07-25), so re-capturing
this means physically sitting at the box. Update it when hardware changes
rather than assuming it is current.

## Summary

| | |
|---|---|
| System | HP Z8 G4 Workstation (4XQ36U) |
| CPU | 2 × Intel Xeon Gold 6154 — **36 cores / 72 threads** total |
| RAM | **96 GiB** DDR4 — 6 × 16 GiB, **18 of 24 DIMM slots empty** |
| GPU | NVIDIA **RTX A5000** (GA102GL, 24 GB) — swapped in for an A6000 48 GB |
| NVMe | Samsung MZVLB1T0HBLR 1 TB — EFI + `/boot` + 952 GiB LVM PV |
| SATA SSD | Crucial CT1000MX500 1 TB — EFI + 929 GiB LVM PV |
| Root | `/dev/mapper/rl-root`, 850 GB, 396 GB used, **454 GB free** |
| Optical | DVDRW GUD1N (`/dev/cdrom`) |
| NICs | Intel X722 (`enp9s0f2`), Intel I219 (`eno1`) |

## Notes that have bitten us

**72 threads against only 96 GB of RAM.** This ratio breaks any tool that sizes
parallelism from core count. An uncapped `cargo build --features cuda` spawns
one `cicc` per job at ~3 GB each — ~216 GB requested on a 96 GB machine. It
OOM-killed the box on 2026-08-25 and wedged it for roughly six hours. Always
pass `-j 8`. See `build-on-metro-linux.md`.

**Adding RAM is cheap and would help twice.** 18 slots are free, and only about
half the memory channels on this dual-socket board are populated, so the machine
is short on bandwidth as well as capacity. A Z8 G4 with 72 threads would normally
carry 256–512 GB.

**Two bootable disks, and only one of them is yours.** Both the NVMe and the
SATA SSD carry an EFI partition and an LVM PV. The live system boots from the
**NVMe** (`df` puts `/boot` on `nvme0n1p2`, `/boot/efi` on `nvme0n1p1`). The
Crucial SATA SSD holds an abandoned **CentOS 7.2** install — its GRUB still looks
for `vmlinuz-3.10.0-327.el7.x86_64` and a filesystem UUID that no longer exists.
If firmware boot order shifts, the machine lands on that stale GRUB and fails to
boot with `no such device` / `you need to load the kernel first`. Recover with
**F9** at the HP splash and pick the Samsung NVMe entry. Note that `lshw`'s
volume labels here are misleading: it calls `nvme0n1p1` a "Windows FAT volume"
and `nvme0n1p2` an "EFI partition", while `df` shows the opposite roles.

**`/dev/mapper/rl-root` holds `~/.kwaainet`** — the RAG graphs, `config.yaml`,
and the Ed25519 identity keypair that *is* this node's peer ID
(`12D3KooWCzuhpXrZ…`). Never let a repair or install path repartition. That
keypair is worth backing up off-machine.

**The journal is volatile.** `journalctl -b -1` reports no persistent journal, so
a reboot discards the evidence. The setup to fix that is in
`build-on-metro-linux.md`.

## Raw capture

### `df -h`

```
Filesystem           Size  Used Avail Use% Mounted on
devtmpfs              47G     0   47G   0% /dev
tmpfs                 47G  1.3M   47G   1% /dev/shm
tmpfs                 47G  148M   47G   1% /run
tmpfs                 47G     0   47G   0% /sys/fs/cgroup
/dev/mapper/rl-root  850G  396G  454G  47% /
/dev/nvme0n1p2      1014M  470M  545M  47% /boot
/dev/nvme0n1p1       599M   14M  585M   3% /boot/efi
tmpfs                9.4G   36K  9.4G   1% /run/user/1000
```

### `sudo lshw -short`

Truncated at the right margin by `lshw -short`'s fixed column width, exactly as
it printed. The long run of `Sky Lake-E CHA/PCU/M3KTI Registers` lines is normal
uncore enumeration on a dual-socket Skylake-SP board — noise, kept for fidelity.

```
H/W path             Device          Class          Description
===============================================================
                                     system         HP Z8 G4 Workstation (4XQ36U
/0                                   bus            81C7
/0/1                                 memory         1152KiB L1 cache
/0/3                                 memory         18MiB L2 cache
/0/4                                 memory         24MiB L3 cache
/0/5                                 processor      Intel(R) Xeon(R) Gold 6154 C
/0/8                                 memory         1152KiB L1 cache
/0/9                                 memory         18MiB L2 cache
/0/6                                 memory         24MiB L3 cache
/0/7                                 processor      Intel(R) Xeon(R) Gold 6154 C
/0/a                                 memory         64KiB BIOS
/0/b                                 memory         96GiB System Memory
/0/b/0                               memory         16GiB DIMM DDR4 Synchronous 
/0/b/1                               memory         DIMM [empty]
/0/b/2                               memory         16GiB DIMM DDR4 Synchronous 
/0/b/3                               memory         DIMM [empty]
/0/b/4                               memory         DIMM [empty]
/0/b/5                               memory         DIMM [empty]
/0/b/6                               memory         DIMM [empty]
/0/b/7                               memory         DIMM [empty]
/0/b/8                               memory         DIMM [empty]
/0/b/9                               memory         DIMM [empty]
/0/b/a                               memory         DIMM [empty]
/0/b/b                               memory         16GiB DIMM DDR4 Synchronous 
/0/b/c                               memory         16GiB DIMM DDR4 Synchronous 
/0/b/d                               memory         DIMM [empty]
/0/b/e                               memory         16GiB DIMM DDR4 Synchronous 
/0/b/f                               memory         DIMM [empty]
/0/b/10                              memory         DIMM [empty]
/0/b/11                              memory         DIMM [empty]
/0/b/12                              memory         DIMM [empty]
/0/b/13                              memory         DIMM [empty]
/0/b/14                              memory         DIMM [empty]
/0/b/15                              memory         DIMM [empty]
/0/b/16                              memory         DIMM [empty]
/0/b/17                              memory         16GiB DIMM DDR4 Synchronous 
/0/100                               bridge         Sky Lake-E DMI3 Registers
/0/100/4             /dev/fb0        generic        Sky Lake-E CBDMA Registers
/0/100/4.1                           generic        Sky Lake-E CBDMA Registers
/0/100/4.2                           generic        Sky Lake-E CBDMA Registers
/0/100/4.3                           generic        Sky Lake-E CBDMA Registers
/0/100/4.4                           generic        Sky Lake-E CBDMA Registers
/0/100/4.5                           generic        Sky Lake-E CBDMA Registers
/0/100/4.6                           generic        Sky Lake-E CBDMA Registers
/0/100/4.7                           generic        Sky Lake-E CBDMA Registers
/0/100/5                             generic        Sky Lake-E MM/Vt-d Configura
/0/100/5.2                           generic        Sky Lake-E RAS
/0/100/5.4                           generic        Sky Lake-E IOAPIC
/0/100/8                             generic        Sky Lake-E Ubox Registers
/0/100/8.1                           generic        Sky Lake-E Ubox Registers
/0/100/8.2                           generic        Sky Lake-E Ubox Registers
/0/100/11                            generic        C620 Series Chipset Family M
/0/100/11.5                          storage        sSATA Controller [RAID Mode]
/0/100/14                            bus            C620 Series Chipset Family U
/0/100/14/0          usb1            bus            xHCI Host Controller
/0/100/14/0/1                        bus            4-Port USB 2.0 Hub
/0/100/14/0/4        input8          input          PixArt HP USB Optical Mouse
/0/100/14/0/6                        bus            4-Port USB 2.0 Hub
/0/100/14/0/6/1      input10         input          Logitech USB Keyboard
/0/100/14/1          usb2            bus            xHCI Host Controller
/0/100/14/1/5                        bus            4-Port USB 3.0 Hub
/0/100/14/1/9                        bus            4-Port USB 3.0 Hub
/0/100/14.2                          generic        C620 Series Chipset Family T
/0/100/16                            communication  C620 Series Chipset Family M
/0/100/17            scsi5           storage        SATA Controller [RAID Mode]
/0/100/17/0          /dev/sda        disk           1TB CT1000MX500SSD1
/0/100/17/0/1        /dev/sda1       volume         599MiB Windows FAT volume
/0/100/17/0/2        /dev/sda2       volume         1023MiB EFI partition
/0/100/17/0/3        /dev/sda3       volume         929GiB LVM Physical Volume
/0/100/17/1          /dev/cdrom      disk           DVDRW  GUD1N
/0/100/1c                            bridge         C620 Series Chipset Family P
/0/100/1c.4                          bridge         C620 Series Chipset Family P
/0/100/1c.4/0                        bridge         Intel Corporation
/0/100/1c.4/0/3                      bridge         Intel Corporation
/0/100/1c.4/0/3/0                    network        Ethernet Connection X722
/0/100/1c.4/0/3/0.2  enp9s0f2        network        Ethernet Connection X722 for
/0/100/1f                            bridge         C622 Series Chipset LPC/eSPI
/0/100/1f/0                          system         PnP device PNP0b00
/0/100/1f/1                          system         PnP device PNP0c02
/0/100/1f/2                          communication  PnP device PNP0501
/0/100/1f/3                          input          PnP device PNP0303
/0/100/1f/4                          input          PnP device PNP0f03
/0/100/1f/5                          system         PnP device PNP0c02
/0/100/1f/6                          system         PnP device PNP0c02
/0/100/1f/7                          system         PnP device PNP0c02
/0/100/1f.2                          memory         Memory controller
/0/100/1f.3          card0           multimedia     Intel Corporation
/0/100/1f.3/0        input18         input          HDA Intel PCH Mic
/0/100/1f.3/1        input19         input          HDA Intel PCH Line
/0/100/1f.3/2        input20         input          HDA Intel PCH Line Out
/0/100/1f.3/3        input21         input          HDA Intel PCH Front Headphon
/0/100/1f.4                          bus            C620 Series Chipset Family S
/0/100/1f.5                          bus            C620 Series Chipset Family S
/0/100/1f.6          eno1            network        Ethernet Connection (3) I219
/0/101                               bridge         Sky Lake-E PCI Express Root 
/0/101/0             /dev/fb0        display        GA102GL [RTX A5000]
/0/101/0.1           card1           multimedia     GA102 High Definition Audio 
/0/101/0.1/0         input14         input          HDA NVidia HDMI/DP,pcm=3
/0/101/0.1/1         input15         input          HDA NVidia HDMI/DP,pcm=7
/0/101/0.1/2         input16         input          HDA NVidia HDMI/DP,pcm=8
/0/101/0.1/3         input17         input          HDA NVidia HDMI/DP,pcm=9
/0/c                                 generic        Sky Lake-E VT-d
/0/d                                 generic        Sky Lake-E RAS Configuration
/0/f                                 generic        Sky Lake-E IOxAPIC Configura
/0/10                                generic        Sky Lake-E CHA Registers
/0/11                                generic        Sky Lake-E CHA Registers
/0/12                                generic        Sky Lake-E CHA Registers
/0/13                                generic        Sky Lake-E CHA Registers
/0/14                                generic        Sky Lake-E CHA Registers
/0/15                                generic        Sky Lake-E CHA Registers
/0/16                                generic        Sky Lake-E CHA Registers
/0/17                                generic        Sky Lake-E CHA Registers
/0/18                                generic        Sky Lake-E CHA Registers
/0/19                                generic        Sky Lake-E CHA Registers
/0/1a                                generic        Sky Lake-E CHA Registers
/0/1b                                generic        Sky Lake-E CHA Registers
/0/1c                                generic        Sky Lake-E CHA Registers
/0/1d                                generic        Sky Lake-E CHA Registers
/0/1e                                generic        Sky Lake-E CHA Registers
/0/1f                                generic        Sky Lake-E CHA Registers
/0/20                                generic        Sky Lake-E CHA Registers
/0/21                                generic        Sky Lake-E CHA Registers
/0/22                                generic        Sky Lake-E CHA Registers
/0/23                                generic        Sky Lake-E CHA Registers
/0/24                                generic        Sky Lake-E CHA Registers
/0/25                                generic        Sky Lake-E CHA Registers
/0/26                                generic        Sky Lake-E CHA Registers
/0/27                                generic        Sky Lake-E CHA Registers
/0/28                                generic        Sky Lake-E CHA Registers
/0/29                                generic        Sky Lake-E CHA Registers
/0/2a                                generic        Sky Lake-E CHA Registers
/0/2b                                generic        Sky Lake-E CHA Registers
/0/2c                                generic        Sky Lake-E CHA Registers
/0/2d                                generic        Sky Lake-E CHA Registers
/0/2e                                generic        Sky Lake-E CHA Registers
/0/2f                                generic        Sky Lake-E CHA Registers
/0/30                                generic        Sky Lake-E CHA Registers
/0/31                                generic        Sky Lake-E CHA Registers
/0/32                                generic        Sky Lake-E CHA Registers
/0/33                                generic        Sky Lake-E CHA Registers
/0/34                                generic        Sky Lake-E CHA Registers
/0/35                                generic        Sky Lake-E CHA Registers
/0/36                                generic        Sky Lake-E CHA Registers
/0/37                                generic        Sky Lake-E CHA Registers
/0/38                                generic        Sky Lake-E CHA Registers
/0/39                                generic        Sky Lake-E CHA Registers
/0/3a                                generic        Sky Lake-E CHA Registers
/0/3b                                generic        Sky Lake-E CHA Registers
/0/3c                                generic        Sky Lake-E CHA Registers
/0/3d                                generic        Sky Lake-E CHA Registers
/0/3e                                generic        Sky Lake-E CHA Registers
/0/3f                                generic        Sky Lake-E CHA Registers
/0/40                                generic        Sky Lake-E CHA Registers
/0/41                                generic        Sky Lake-E CHA Registers
/0/42                                generic        Sky Lake-E CHA Registers
/0/43                                generic        Sky Lake-E CHA Registers
/0/44                                generic        Sky Lake-E CHA Registers
/0/45                                generic        Sky Lake-E CHA Registers
/0/46                                generic        Sky Lake-E CHA Registers
/0/47                                generic        Sky Lake-E CHA Registers
/0/48                                generic        Sky Lake-E CHA Registers
/0/49                                generic        Sky Lake-E CHA Registers
/0/4a                                generic        Sky Lake-E CHA Registers
/0/4b                                generic        Sky Lake-E CHA Registers
/0/4c                                generic        Sky Lake-E PCU Registers
/0/4d                                generic        Sky Lake-E PCU Registers
/0/4e                                generic        Sky Lake-E PCU Registers
/0/4f                                generic        Sky Lake-E PCU Registers
/0/50                                generic        Sky Lake-E PCU Registers
/0/51                                generic        Sky Lake-E PCU Registers
/0/52                                generic        Sky Lake-E PCU Registers
/0/102                               bridge         Sky Lake-E PCI Express Root 
/0/103                               bridge         Sky Lake-E PCI Express Root 
/0/104                               bridge         Sky Lake-E PCI Express Root 
/0/105                               bridge         Sky Lake-E PCI Express Root 
/0/105/0             /dev/nvme0      storage        SAMSUNG MZVLB1T0HBLR-000H1
/0/105/0/0           /dev/ng0n1      disk           NVMe disk
/0/105/0/1           /dev/nvme0n1    disk           1024GB NVMe disk
/0/105/0/1/1         /dev/nvme0n1p1  volume         599MiB Windows FAT volume
/0/105/0/1/2         /dev/nvme0n1p2  volume         1023MiB EFI partition
/0/105/0/1/3         /dev/nvme0n1p3  volume         952GiB LVM Physical Volume
/0/53                                generic        Sky Lake-E VT-d
/0/54                                generic        Sky Lake-E RAS Configuration
/0/55                                generic        Sky Lake-E IOxAPIC Configura
/0/56                                generic        Sky Lake-E Integrated Memory
/0/57                                generic        Sky Lake-E Integrated Memory
/0/58                                generic        Sky Lake-E Integrated Memory
/0/59                                generic        Sky Lake-E Integrated Memory
/0/5a                                generic        Sky Lake-E Integrated Memory
/0/5b                                generic        Sky Lake-E Integrated Memory
/0/5c                                generic        Sky Lake-E Integrated Memory
/0/5d                                generic        Sky Lake-E LM Channel 1
/0/5e                                generic        Sky Lake-E LMS Channel 1
/0/5f                                generic        Sky Lake-E LMDP Channel 1
/0/60                                generic        Sky Lake-E DECS Channel 2
/0/61                                generic        Sky Lake-E LM Channel 2
/0/62                                generic        Sky Lake-E LMS Channel 2
/0/63                                generic        Sky Lake-E LMDP Channel 2
/0/64                                generic        Sky Lake-E Integrated Memory
/0/65                                generic        Sky Lake-E Integrated Memory
/0/66                                generic        Sky Lake-E Integrated Memory
/0/67                                generic        Sky Lake-E Integrated Memory
/0/68                                generic        Sky Lake-E Integrated Memory
/0/69                                generic        Sky Lake-E LM Channel 1
/0/6a                                generic        Sky Lake-E LMS Channel 1
/0/6b                                generic        Sky Lake-E LMDP Channel 1
/0/6c                                generic        Sky Lake-E DECS Channel 2
/0/6d                                generic        Sky Lake-E LM Channel 2
/0/6e                                generic        Sky Lake-E LMS Channel 2
/0/6f                                generic        Sky Lake-E LMDP Channel 2
/0/106                               bridge         Sky Lake-E PCI Express Root 
/0/70                                generic        Sky Lake-E VT-d
/0/71                                generic        Sky Lake-E RAS Configuration
/0/72                                generic        Sky Lake-E IOxAPIC Configura
/0/73                                generic        Sky Lake-E KTI 0
/0/74                                generic        Sky Lake-E UPI Registers
/0/75                                generic        Sky Lake-E KTI 0
/0/76                                generic        Sky Lake-E UPI Registers
/0/77                                generic        Sky Lake-E KTI 0
/0/78                                generic        Sky Lake-E UPI Registers
/0/79                                generic        Sky Lake-E M3KTI Registers
/0/7a                                generic        Sky Lake-E M3KTI Registers
/0/7b                                generic        Sky Lake-E M3KTI Registers
/0/7c                                generic        Sky Lake-E M3KTI Registers
/0/7d                                generic        Sky Lake-E M3KTI Registers
/0/7e                                generic        Sky Lake-E M2PCI Registers
/0/7f                                generic        Sky Lake-E M2PCI Registers
/0/80                                generic        Sky Lake-E M2PCI Registers
/0/81                                generic        Sky Lake-E M2PCI Registers
/0/82                                generic        Sky Lake-E CBDMA Registers
/0/4.1                               generic        Sky Lake-E CBDMA Registers
/0/4.2                               generic        Sky Lake-E CBDMA Registers
/0/4.3                               generic        Sky Lake-E CBDMA Registers
/0/4.4                               generic        Sky Lake-E CBDMA Registers
/0/4.5                               generic        Sky Lake-E CBDMA Registers
/0/4.6                               generic        Sky Lake-E CBDMA Registers
/0/4.7                               generic        Sky Lake-E CBDMA Registers
/0/83                                generic        Sky Lake-E MM/Vt-d Configura
/0/84                                generic        Sky Lake-E RAS
/0/85                                generic        Sky Lake-E IOAPIC
/0/86                                generic        Sky Lake-E Ubox Registers
/0/87                                generic        Sky Lake-E Ubox Registers
/0/88                                generic        Sky Lake-E Ubox Registers
/0/107                               bridge         Sky Lake-E PCI Express Root 
/0/89                                generic        Sky Lake-E VT-d
/0/8a                                generic        Sky Lake-E RAS Configuration
/0/8b                                generic        Sky Lake-E IOxAPIC Configura
/0/8c                                generic        Sky Lake-E CHA Registers
/0/8.1                               generic        Sky Lake-E CHA Registers
/0/8.2                               generic        Sky Lake-E CHA Registers
/0/8d                                generic        Sky Lake-E CHA Registers
/0/8e                                generic        Sky Lake-E CHA Registers
/0/8f                                generic        Sky Lake-E CHA Registers
/0/90                                generic        Sky Lake-E CHA Registers
/0/91                                generic        Sky Lake-E CHA Registers
/0/92                                generic        Sky Lake-E CHA Registers
/0/93                                generic        Sky Lake-E CHA Registers
/0/94                                generic        Sky Lake-E CHA Registers
/0/95                                generic        Sky Lake-E CHA Registers
/0/96                                generic        Sky Lake-E CHA Registers
/0/97                                generic        Sky Lake-E CHA Registers
/0/98                                generic        Sky Lake-E CHA Registers
/0/99                                generic        Sky Lake-E CHA Registers
/0/9a                                generic        Sky Lake-E CHA Registers
/0/9b                                generic        Sky Lake-E CHA Registers
/0/9c                                generic        Sky Lake-E CHA Registers
/0/9d                                generic        Sky Lake-E CHA Registers
/0/9e                                generic        Sky Lake-E CHA Registers
/0/9f                                generic        Sky Lake-E CHA Registers
/0/a0                                generic        Sky Lake-E CHA Registers
/0/a1                                generic        Sky Lake-E CHA Registers
/0/a2                                generic        Sky Lake-E CHA Registers
/0/a3                                generic        Sky Lake-E CHA Registers
/0/a4                                generic        Sky Lake-E CHA Registers
/0/a5                                generic        Sky Lake-E CHA Registers
/0/a6                                generic        Sky Lake-E CHA Registers
/0/a7                                generic        Sky Lake-E CHA Registers
/0/a8                                generic        Sky Lake-E CHA Registers
/0/a9                                generic        Sky Lake-E CHA Registers
/0/aa                                generic        Sky Lake-E CHA Registers
/0/ab                                generic        Sky Lake-E CHA Registers
/0/ac                                generic        Sky Lake-E CHA Registers
/0/ad                                generic        Sky Lake-E CHA Registers
/0/ae                                generic        Sky Lake-E CHA Registers
/0/af                                generic        Sky Lake-E CHA Registers
/0/b0                                generic        Sky Lake-E CHA Registers
/0/b1                                generic        Sky Lake-E CHA Registers
/0/b2                                generic        Sky Lake-E CHA Registers
/0/b3                                generic        Sky Lake-E CHA Registers
/0/b4                                generic        Sky Lake-E CHA Registers
/0/b5                                generic        Sky Lake-E CHA Registers
/0/b6                                generic        Sky Lake-E CHA Registers
/0/b7                                generic        Sky Lake-E CHA Registers
/0/b8                                generic        Sky Lake-E CHA Registers
/0/b9                                generic        Sky Lake-E CHA Registers
/0/ba                                generic        Sky Lake-E CHA Registers
/0/bb                                generic        Sky Lake-E CHA Registers
/0/bc                                generic        Sky Lake-E CHA Registers
/0/bd                                generic        Sky Lake-E CHA Registers
/0/be                                generic        Sky Lake-E CHA Registers
/0/bf                                generic        Sky Lake-E CHA Registers
/0/c0                                generic        Sky Lake-E CHA Registers
/0/c1                                generic        Sky Lake-E CHA Registers
/0/c2                                generic        Sky Lake-E CHA Registers
/0/c3                                generic        Sky Lake-E CHA Registers
/0/c4                                generic        Sky Lake-E CHA Registers
/0/c5                                generic        Sky Lake-E CHA Registers
/0/c6                                generic        Sky Lake-E PCU Registers
/0/c7                                generic        Sky Lake-E PCU Registers
/0/c8                                generic        Sky Lake-E PCU Registers
/0/c9                                generic        Sky Lake-E PCU Registers
/0/ca                                generic        Sky Lake-E PCU Registers
/0/cb                                generic        Sky Lake-E PCU Registers
/0/cc                                generic        Sky Lake-E PCU Registers
/0/108                               bridge         Sky Lake-E PCI Express Root 
/0/cd                                generic        Sky Lake-E VT-d
/0/ce                                generic        Sky Lake-E RAS Configuration
/0/cf                                generic        Sky Lake-E IOxAPIC Configura
/0/d0                                generic        Sky Lake-E Integrated Memory
/0/d1                                generic        Sky Lake-E Integrated Memory
/0/d2                                generic        Sky Lake-E Integrated Memory
/0/d3                                generic        Sky Lake-E Integrated Memory
/0/d4                                generic        Sky Lake-E Integrated Memory
/0/d5                                generic        Sky Lake-E Integrated Memory
/0/d6                                generic        Sky Lake-E Integrated Memory
/0/d7                                generic        Sky Lake-E LM Channel 1
/0/d8                                generic        Sky Lake-E LMS Channel 1
/0/d9                                generic        Sky Lake-E LMDP Channel 1
/0/da                                generic        Sky Lake-E DECS Channel 2
/0/db                                generic        Sky Lake-E LM Channel 2
/0/dc                                generic        Sky Lake-E LMS Channel 2
/0/dd                                generic        Sky Lake-E LMDP Channel 2
/0/de                                generic        Sky Lake-E Integrated Memory
/0/df                                generic        Sky Lake-E Integrated Memory
/0/e0                                generic        Sky Lake-E Integrated Memory
/0/e1                                generic        Sky Lake-E Integrated Memory
/0/e2                                generic        Sky Lake-E Integrated Memory
/0/e3                                generic        Sky Lake-E LM Channel 1
/0/e4                                generic        Sky Lake-E LMS Channel 1
/0/e5                                generic        Sky Lake-E LMDP Channel 1
/0/e6                                generic        Sky Lake-E DECS Channel 2
/0/e7                                generic        Sky Lake-E LM Channel 2
/0/e8                                generic        Sky Lake-E LMS Channel 2
/0/e9                                generic        Sky Lake-E LMDP Channel 2
/0/0                                 bridge         Sky Lake-E PCI Express Root 
/0/2                                 bridge         Sky Lake-E PCI Express Root 
/0/ea                                generic        Sky Lake-E VT-d
/0/5.2                               generic        Sky Lake-E RAS Configuration
/0/5.4                               generic        Sky Lake-E IOxAPIC Configura
/0/e                                 generic        Sky Lake-E KTI 0
/0/eb                                generic        Sky Lake-E UPI Registers
/0/ec                                generic        Sky Lake-E KTI 0
/0/ed                                generic        Sky Lake-E UPI Registers
/0/ee                                generic        Sky Lake-E KTI 0
/0/ef                                generic        Sky Lake-E UPI Registers
/0/f0                                generic        Sky Lake-E M3KTI Registers
/0/f1                                generic        Sky Lake-E M3KTI Registers
/0/f2                                generic        Sky Lake-E M3KTI Registers
/0/f3                                generic        Sky Lake-E M3KTI Registers
/0/f4                                generic        Sky Lake-E M3KTI Registers
/0/f5                                generic        Sky Lake-E M2PCI Registers
/0/f6                                generic        Sky Lake-E M2PCI Registers
/0/f7                                generic        Sky Lake-E M2PCI Registers
/0/f8                                generic        Sky Lake-E M2PCI Registers
/1                   input0          input          Power Button
/2                   input1          input          Power Button
/3                   input12         input          PC Speaker
/4                   input13         input          HP WMI hotkeys
```

## Not captured — worth grabbing next time you are at the console

```bash
swapon --show                   # df does not show swap; unknown whether any exists
lsblk -f                        # filesystem types and UUIDs on both disks
sudo vgs; sudo pvs; sudo lvs    # is the `rl` VG on the NVMe only, or spanning both?
sudo efibootmgr -v              # UEFI boot order — the wrong-disk boot above
nvidia-smi                      # driver and CUDA runtime version
cat /etc/os-release             # distro and release
sudo dmidecode -t memory | grep -E 'Size|Speed|Locator'   # DIMM speed/rank for an upgrade
```

`pvs` matters most: if the `rl` volume group spans **both** disks, then the old
CentOS SSD is not a spare part to pull — root depends on it.
