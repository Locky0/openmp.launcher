# open.mp launcher

Made with Tauri + React-Native ❤️

# Usage:

Use open.mp launcher to enjoy a live, reliable, and populated server list to find any server you want to play on!  
Just download it from [Releases](https://github.com/openmultiplayer/launcher/releases/latest) page and run it!

# Development

### For all OSes:

- Install [nightly version](https://rust-lang.github.io/rustup/concepts/channels.html) of rust toolchain
- Install [NodeJS](https://nodejs.org/en/download) and `npm` (or `yarn` or anything else)  
  **Note**: Please make sure you are not using node v20.6, anything else, lower or higher, should work fine.
- Clone repository:

```bash
git clone https://github.com/openmultiplayer/launcher
```

- Prepare for running:

```bash
cd launcher
yarn # or any other way you use to install dependecies using your installed package manager
yarn start
```

- For building a release version, you can use:

```bash
yarn tauri build
```

## Windows Defender notes

This launcher performs DLL injection into the GTA:SA process and may request elevated privileges when the game is installed in a protected directory. Those are common false-positive triggers for Windows antivirus heuristics.

To reduce the chance of a detection:

- Avoid packing the final executable with UPX.
- Prefer 64-bit Windows releases over legacy 32-bit builds.
- Prefer per-user installation over writing into system directories.
- Ship the `d3dx9_25.dll` runtime next to the game instead of copying it into `System32`.
- Code-sign the final executable and installer before distribution whenever possible.

The repository is now configured to build Windows releases as `x86_64-pc-windows-msvc`. GitHub Actions will also sign the executable and NSIS installer automatically when the `WINDOWS_CERT_BASE64` and `WINDOWS_CERT_PASSWORD` secrets are configured.

# Donations

While open.mp is a totally free project and everyone in the team dedicates their time on this project to provide the best for the community, we can still use whatever contribution by anyone, to cover our costs or motivate developers.  
Here is our donation link you can use to show your generosity!

- **https://opencollective.com/openmultiplayer**
