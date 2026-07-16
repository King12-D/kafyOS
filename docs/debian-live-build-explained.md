# Debian And Live-Build Explained

This is the mental model for Kafy OS.

## What Debian Is

Debian is a Linux distribution. A distribution is the complete operating system packaging layer around the Linux kernel:

- the Linux kernel
- bootloader pieces
- system services
- hardware firmware
- drivers
- desktop environment
- apps
- package repositories
- installer
- update system
- default configuration

Debian's software is shipped as `.deb` packages. `dpkg` installs `.deb` files. `apt` talks to package repositories, resolves dependencies, downloads packages, and asks `dpkg` to install them.

So when we say Kafy is Debian-based, we mean:

- Kafy uses Debian packages as the foundation.
- Kafy can use Debian security updates.
- Kafy can add its own packages and defaults.
- Kafy eventually has its own repository for Kafy-specific software.

## What Makes Kafy Different From Debian

At first, Kafy is not a new kernel or a new package manager. Kafy is Debian plus:

- selected packages
- gaming defaults
- firmware and driver defaults
- Flatpak/Flathub setup
- desktop layout
- branding
- first-boot setup
- installer defaults
- eventually Kafy desktop components and compositor

That is how many practical distros start. The value is in the product decisions and integration, not in rewriting the whole operating system.

## What Live-Build Is

`live-build` is Debian's tool for creating live/installable images.

The command name is `lb`. It is installed by the `live-build` package. If the terminal says `lb: not found`, the build machine does not have live-build installed.

Install it with:

```sh
sudo apt update
sudo apt install live-build live-config live-boot debootstrap xorriso squashfs-tools
```

## What Happens During A Build

From `distro/kafy`, the flow is:

```sh
sudo ./auto/config
sudo lb build
```

`./auto/config` calls `lb config`. That creates live-build's generated configuration from our choices:

- Debian release: `bookworm`
- archive areas: `main contrib non-free non-free-firmware`
- image type: hybrid ISO
- bootloader options: GRUB where the installed live-build version supports explicit bootloader selection
- installer: Debian live installer
- firmware: included
- image name: `kafy-os`

Some live-build versions do not support every option name. `auto/config` checks the installed `lb config --help` output before using optional flags such as `--bootloaders` and `--image-name`.

Some older live-build versions also generate the obsolete security suite `bookworm/updates`, which fails for Debian Bookworm. Kafy disables that generated security entry and adds the correct Bookworm security repository through `config/archives/debian-security.list.chroot` and `config/archives/debian-security.list.binary`.

The same older live-build line also tries to auto-discover firmware by downloading `dists/bookworm/Contents-amd64.gz`, but Debian Bookworm exposes Contents files per archive component. Kafy disables live-build's firmware auto-discovery and installs the firmware packages explicitly through `config/package-lists/00-base.list.chroot`.

`sudo lb build` then:

1. Creates a temporary Debian filesystem.
2. Installs base Debian packages into it.
3. Installs every package listed in `config/package-lists/*.list.chroot`.
4. Copies `config/includes.chroot/` into the target filesystem.
5. Runs `config/hooks/normal/*.hook.chroot` inside the target filesystem.
6. Compresses the target filesystem with SquashFS.
7. Builds a bootable ISO using GRUB and xorriso.

## The Important Kafy Files

`distro/kafy/auto/config`

Defines the image shape. This is where we say "build Debian Bookworm as an ISO with firmware and installer support."

`distro/kafy/config/package-lists/00-base.list.chroot`

Core system packages: networking, audio, Bluetooth, printing, firmware, power management, certificates, and system basics.

`distro/kafy/config/package-lists/10-desktop.list.chroot`

Temporary production desktop. We use KDE Plasma Wayland first so Kafy becomes a real bootable OS before we build a custom compositor.

`distro/kafy/config/package-lists/20-gaming.list.chroot`

Gaming packages: Steam installer, GameMode, MangoHud, Vulkan tools, Mesa Vulkan drivers, Wine, Lutris, vkBasalt, and controller tools.

`distro/kafy/config/package-lists/30-apps.list.chroot`

Default user apps and tools.

`distro/kafy/config/includes.chroot/`

Files copied directly into Kafy OS. If a file is here:

```text
config/includes.chroot/usr/share/kafy/product.json
```

then it appears inside the built OS here:

```text
/usr/share/kafy/product.json
```

`distro/kafy/config/hooks/normal/`

Scripts that run while the OS image is being built. These are for setup tasks like enabling services, adding Flathub, setting gaming sysctl values, and applying branding.

## Why We Start With KDE

Building a desktop shell and compositor first would delay having a bootable OS. KDE Plasma Wayland gives us:

- a real Wayland desktop now
- login/session integration
- display settings
- panels/dock-like behavior
- portals
- app launching
- tested multi-monitor behavior
- a usable baseline for gaming tests

Kafy-specific desktop pieces can replace parts over time. The custom Smithay compositor comes later, after the distro is already real.

## What To Do When Builds Fail

If `lb` is missing:

```sh
sudo apt install live-build
```

If package installation fails, one of the package names may not exist in the selected Debian release. Check with:

```sh
apt-cache policy package-name
```

Package names differ between Debian releases and other distributions. For example, Debian Bookworm uses `kde-spectacle` for KDE's screenshot tool, and `fastfetch` is not part of the first Bookworm package set used by Kafy.

If the build fails after a partial attempt, clean generated build state:

```sh
sudo lb clean
```

Then configure and build again:

```sh
sudo ./auto/config
sudo lb build
```
