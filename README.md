# Kafy OS

Kafy is a Debian-based desktop Linux distribution focused on a polished macOS-like experience, zero-setup defaults, and gaming readiness.

The current source of truth is the Debian live-build profile in `distro/kafy`. Older Rust desktop experiments in `crates/` are kept for reference only and are not part of the first OS image path.

## Build Direction

- Base: Debian stable
- Image system: Debian live-build
- Desktop baseline: KDE Plasma Wayland while Kafy desktop components mature
- Gaming baseline: Steam installer, Proton support, GameMode, MangoHud, Vulkan tooling, controller support, Wine/Lutris, and Flatpak-based gaming apps
- App model: Flatpak and Flathub enabled by default
- Future compositor: Smithay-based Wayland compositor when the Kafy shell is ready

## Build An ISO

Install build dependencies on a Debian host:

```sh
sudo apt install live-build live-config live-boot debootstrap xorriso squashfs-tools
```

Build:

```sh
cd distro/kafy
./check-host
sudo lb clean
sudo ./auto/config
sudo lb build
```

The generated ISO appears in `distro/kafy` when the build completes.

If `./auto/config` says `lb: not found`, install `live-build`. `lb` is not built into Debian or Ubuntu; it is the command installed by the `live-build` package.

## How This Works

Debian is made of `.deb` packages managed by `apt` and `dpkg`. A Debian-based distro like Kafy starts from Debian repositories, chooses packages, adds defaults, adds branding, and builds an installable/live image.

`live-build` is Debian's image builder. It reads the files in `distro/kafy`, creates a temporary Debian filesystem, installs the package lists, copies our included files into that filesystem, runs our hooks, then compresses everything into a bootable ISO.

Read [docs/debian-live-build-explained.md](docs/debian-live-build-explained.md) for the full plain-English version.

## Product Rule

Kafy should boot into a usable desktop without making users learn Linux setup chores. Driver handling, app sources, gaming tooling, multimedia, power management, and common hardware support belong in the image defaults.
