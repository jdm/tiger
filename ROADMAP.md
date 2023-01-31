# Roadmap

## Tiger 0.9.0

- [x] Document keyboard shortcuts
- [x] More template file examples in documentation
- [x] Documentation about use of file paths in .tiger files
- [x] Auto-updates
  - [x] Show banner when new version is available
  - [x] Can install new version
  - [x] Generate `update-manifest.json` during release process
  - [x] `tauri.conf.json` points to latest release's `update-manifest.json`
  - [x] Offer to save modified files before applying update
  - [x] Show notification with changelog link after update
- [x] Clean up visual jank during app startup
- [x] Add a a help menu with links to issues/discussions/documentation
- [ ] Add About dialog with version number / license info
- [ ] Add landing screen with links to new/open/recent/github/docs-home/keyboard-docs
- [ ] Clarify what MIT license applies to in readme
- [x] `.tiger` file association
- [x] Opening files from Windows Explorer re-uses existing instance
- [x] Webview cache is now stored alongside application files under `%LocalAppData%/Tiger`
- [x] Application files are now stored under `%LocalAppData%/Tiger` instead of `%LocalAppData%/Permafrost/Tiger` (this directory can safely be deleted manually)
- [x] Registry keys are now written under `HKCU\Software\Tiger` instead of `HKCU\Software\com.agersant.tiger`

## Tiger 1.0

- [ ] Review all TODO
- [ ] Re-evaluate https://github.com/1Password/typeshare to auto-generate typescript DTOs
- [ ] Tiger file format uses semver
- [ ] Remove support for pre-1.0 versions of Tiger file format
- [ ] Logo in readme
- [ ] Gif/video in readme
- [ ] Remove under-development warning from readme
- [ ] Logo in documentation
- [ ] Favicon in documentation
- [ ] Branding in installer and installer icon
- [ ] App icon (file explorer, taskbar, add/remove programs, title bar)
- [ ] Github social media preview image (in repo settings)
- [ ] Itch.io or other distribution method

## Post 1.0

- [ ] Tiger CLI
- [ ] Root motion
- [ ] Onion skin?
- [ ] Hitbox colors
- [ ] Can preview export image and metadata output
- [ ] Interop with specific game engines (TBD)
- [ ] Import frames from single sheet image
- [ ] Import animation data from other software (Asesprite, TBD)
- [ ] Sockets (like hitbox but point)
- [ ] Events (arbitrary markers on timeline)
- [ ] Drag and drop frames from OS to content panel
- [ ] Drag and drop frames from OS to timeline panel
- [ ] Project files (.tiger files within a project can be moved without breaking)
- [ ] Cleanup application files and webview cache on uninstall (blocked by https://github.com/tauri-apps/tauri/issues/6103 or https://github.com/tauri-apps/tauri/issues/6113)
