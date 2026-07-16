# Contributing to Kafy OS

Thank you for your interest in contributing to Kafy OS! We want to make this the best Linux distribution for a polished, macOS-like experience and zero-setup gaming.

## Project Structure

Our repository is organized into several key areas:

- **`distro/`**: Contains the Debian live-build configurations to build the actual ISO.
  - `distro/kafy/config/`: Live-build config files (packages, hooks, etc.).
- **`desktop/`**: Source code for the Kafy desktop environment, Wayland compositor, and custom UI components (written in Rust).
- **`packages/`**: Custom Debian packages (.deb) and packaging recipes specific to Kafy OS.
- **`artwork/`**: Branding, wallpapers, icons, and theme assets.
- **`scripts/`**: Development, build, and CI/CD tools.
- **`docs/`**: Documentation and roadmaps.

## How to Contribute

### 1. Reporting Issues
If you encounter bugs or have feature requests, please open an issue with:
- Steps to reproduce the issue.
- Your hardware specifications (if relevant).
- The version of the ISO you are using.

### 2. Developing the OS Image
To contribute to the base OS image:
1. Make sure you understand the `live-build` process (see `docs/debian-live-build-explained.md`).
2. Modify package lists in `distro/kafy/config/package-lists/` or hooks in `distro/kafy/config/hooks/`.
3. Test your build locally by building an ISO and testing it in a VM (e.g., QEMU or VirtualBox) before submitting a PR.

### 3. Developing the Desktop (Rust)
If you're contributing to the Kafy shell/compositor:
1. Navigate to `desktop/`.
2. Follow standard Rust formatting (`cargo fmt`) and linting (`cargo clippy`).
3. Ensure your code compiles and passes tests before submitting.

## Pull Request Process
1. Fork the repository and create your branch from `main`.
2. Keep your commits atomic and write descriptive commit messages.
3. If you've added new features or changed behaviors, update the relevant documentation.
4. Open a Pull Request and ask for a review!

## Code of Conduct
Please note that this project is governed by our Code of Conduct. By participating, you are expected to uphold this code and be respectful to all community members.
