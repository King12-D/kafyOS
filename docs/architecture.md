# Kafy Architecture Direction

Kafy is a Debian-based Linux distribution first. The old Rust desktop prototype is no longer the active product path. The active path is a bootable Debian-derived OS image with Kafy defaults, then Kafy-specific desktop components, and finally a custom compositor.

## Base System

- Use Debian stable as the distro base for predictable packages, broad hardware support, and simple maintenance.
- Use Debian live-build for the first bootable ISO.
- Use KDE Plasma Wayland as the temporary production desktop while Kafy-specific components mature.
- Add a Kafy package repository for desktop components, gaming defaults, themes, and first-party tools.
- Prefer system defaults over post-install wizards: users should boot into a configured desktop, not a checklist.
- Keep the base conservative, then layer fast-moving gaming components through curated repositories, Flatpak, or vendor channels.

## Zero-Setup Install

- Install graphics, audio, networking, Bluetooth, printing, and power-management defaults during image creation.
- Include Flatpak and Flathub out of the box for consumer apps.
- Include Steam, Proton support, GameMode, MangoHud, Vulkan tools, controller support, and sane Wine/Lutris/Bottles paths.
- Detect NVIDIA, AMD, and Intel paths during install or first boot, then select the right driver/tooling profile.
- Ship a first-boot account flow only for identity, locale, keyboard, network, privacy, and optional cloud/game accounts.

## Desktop UX

- Build the desktop around a macOS-like structure: top menubar, centered dock, launcher/search, settings, notification center, window overview, and polished defaults.
- Keep app launching, pinned dock apps, recent apps, and app search as the first usability milestone.
- Use a consistent Kafy design language across shell, greeter, settings, installer, and system dialogs.
- Avoid requiring users to understand Linux concepts for basic tasks like installing apps, changing drivers, connecting controllers, or updating the system.

## Compositor Direction

- Build the compositor in Rust on Smithay when Kafy is ready for a real Wayland compositor.
- Keep the early shell decoupled from the compositor so the dock, launcher, settings, and app registry can mature before compositor work dominates the project.
- Target Wayland-first behavior with XWayland support for compatibility.
- Prioritize correctness, input handling, multi-monitor behavior, fractional scaling, screen capture portals, gaming latency, VRR, HDR readiness, and stable suspend/resume.

## Repository Milestones

1. Build and boot the Debian live-build ISO.
2. Add Kafy branding, desktop defaults, Flatpak, gaming tools, firmware, and services.
3. Add installer and first-boot flow.
4. Create signed Kafy package repository and package Kafy defaults as `.deb` packages.
5. Build Kafy desktop components on top of the working distro.
6. Start the Smithay-based compositor after the OS image and session defaults are real.
