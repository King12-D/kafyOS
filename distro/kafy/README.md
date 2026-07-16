# Kafy Live Profile

This directory is the first real Kafy OS build profile. It uses Debian live-build to produce a bootable Debian-based ISO with Kafy defaults.

## Files

- `auto/config`: live-build image configuration.
- `config/package-lists/*.list.chroot`: packages installed into the OS.
- `config/includes.chroot/`: files copied into the target filesystem.
- `config/hooks/normal/`: build hooks run inside the target chroot.
- `config/archives/`: extra apt source definitions.

## Build

```sh
sudo apt install live-build live-config live-boot debootstrap xorriso squashfs-tools
./check-host
sudo lb clean
sudo ./auto/config
sudo lb build
```

`lb` is the live-build command. If you see `lb: not found`, the build host is missing `live-build`.

If `lb config` reports an unrecognized option, the installed live-build version is different from the one documented upstream. `auto/config` keeps optional flags version-aware, but run `lb config --help` to inspect what your host supports.

If apt fails on `http://security.debian.org bookworm/updates`, clean and rebuild. Older live-build versions generate that obsolete suite for Bookworm, so this profile disables live-build's generated security entry and adds the correct `bookworm-security` archive manually.

If the build fails while downloading `dists/bookworm/Contents-amd64.gz`, clean and rebuild. Older live-build versions use that obsolete path for firmware auto-discovery; Kafy disables that behavior and installs firmware packages explicitly.

## How This Directory Becomes An OS

`auto/config` tells live-build what kind of Debian image to create.

`config/package-lists/*.list.chroot` are installed into the target OS.

`config/includes.chroot/` is copied directly into the target OS filesystem. For example, a file at `config/includes.chroot/usr/share/kafy/product.json` becomes `/usr/share/kafy/product.json` inside Kafy OS.

`config/hooks/normal/*.hook.chroot` run inside the target OS during image creation. We use hooks to enable services, add Flathub, set gaming defaults, and apply branding.

When `sudo lb build` finishes, the output is a bootable ISO.

## First Image Strategy

The first Kafy ISO uses Debian stable plus KDE Plasma Wayland. That gives us a real bootable OS, a strong Wayland desktop, and enough polish to test the distro experience while Kafy-specific shell, greeter, settings, and compositor pieces are designed separately.

Smithay remains the long-term compositor direction. It should start after the distro image and session defaults are working, because compositor work is deep infrastructure and will slow the project down if it comes first.
