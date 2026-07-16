# Kafy OS Roadmap

This roadmap ignores the old prototype implementation and treats Kafy as a real Debian-based distribution first.

## Phase 1: Bootable ISO

- Build the Debian live-build profile.
- Boot it in QEMU and on one real test machine.
- Confirm graphics, Wi-Fi, Bluetooth, audio, suspend/resume, display scaling, Flatpak, Steam installer, Vulkan, and controllers.
- Keep KDE Plasma Wayland as the temporary production desktop.

## Phase 2: Kafy Defaults

- Add Kafy branding, wallpapers, SDDM theme, icon/theme choices, dock layout, top panel, gestures, and default apps.
- Add a first-boot app for locale, keyboard, user setup, privacy, online accounts, and optional gaming sign-ins.
- Add a driver manager focused on NVIDIA, AMD, Intel, gamepads, and firmware status.

## Phase 3: Installer

- Start with Debian Installer or Calamares.
- Preconfigure the install path so users choose disk, user, timezone, keyboard, and privacy only.
- Preserve the live session defaults in the installed system.

## Phase 4: Package And Update Infrastructure

- Create a Kafy apt repository.
- Package branding, defaults, first-boot, settings helpers, and future shell components as `.deb` packages.
- Add signed repository metadata and automated image builds.

## Phase 5: Kafy Desktop Components

- Build Kafy settings, launcher, dock, notification center, app store surface, and control center as replaceable components.
- Integrate with portals, systemd user services, NetworkManager, PipeWire, BlueZ, CUPS, Flatpak, and GameMode.

## Phase 6: Kafy Compositor

- Build the compositor in Rust on Smithay.
- Require Wayland-first behavior and XWayland compatibility.
- Prioritize gaming latency, VRR, fractional scaling, multi-monitor behavior, gestures, screen capture portals, and robust suspend/resume.

## Non-Negotiables

- No manual post-install setup for ordinary use.
- No terminal requirement for installing common apps, drivers, or gaming tools.
- Flatpak-only gaming apps such as Bottles and Heroic should be preloaded or exposed through a Kafy first-boot/install surface, not hidden as terminal steps.
- No custom compositor until the distro image, defaults, installer, and update path are already real.
