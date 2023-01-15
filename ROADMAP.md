# Roadmap

## Tiger 0.8

- [x] Handle missing frame files (warning + offer to relocate)
- [ ] In-place tutorials instead of blank data
- [x] ⚠ The `sheet_image` field in template data was replaced by `atlas_image` and now includes atlas dimensions (issue #13)
- [x] Bumped `.tiger` format version to `Tiger5`
- [x] Merge redundant code between save(), save_as() and save_all()
- [x] Automatically add extension to exported textures
- [x] All file dialogs open with a sensible default file/directory selected
- [x] Reach satisfying test coverage
- [ ] Using `Export` before defining export settings behaves like `Export As…`
- [x] Replace tarpaulin with grcov for code coverage measurements
- [x] Hover & pressed states for Button controls
- [x] Hover & pressed states for Toggle controls
- [x] Hover & pressed states for playback controls
- [x] Hover state for FlatMultiSwitch controls
- [x] Hover state for InputPath controls
- [x] Can close directions dropdown menu by clicking it again
- [x] Can press escape to close modals / export overlay
- [ ] Export complete notification
- [ ] Fixed a bug where scrollable panels would sometimes jitter when using the mouse wheel
- [x] Fixed a bug where window could be resized narrow enough to break timeline UI
- [x] Fixed a bug where the `sheet_image` field would use `\` as path separator instead of `/` (issue #14)
- [x] Fixed export documentation referencing a non-existing source field on `Frame`
- [x] Fixed a bug where timeline would not fill entire panel after resizing the program window

## Tiger 0.9

- [ ] Document keyboard shortcuts
- [ ] More template file examples in documentation
- [ ] Documentation about use of file paths in .tiger files
- [ ] Auto-updates
- [ ] Splash screen
- [ ] Add a a help menu with links to issues/discussions/documentation
- [ ] Add About dialog with version number / license info
- [ ] Add landing screen with links to new/open/recent/github/docs-home/keyboard-docs
- [ ] Can open log from help menu
- [ ] .tiger file association

## Tiger 1.0

- [ ] Review all TODO
- [ ] Re-evaluate https://github.com/1Password/typeshare to auto-generate typescript DTOs
- [ ] Tiger file format uses semver
- [ ] Remove support for pre-1.0 versions of Tiger file format
- [ ] Readme logo
- [ ] Gif/video in readme
- [ ] Branding in installer
- [ ] Github social media preview image (in repo settings)
- [ ] About dialog
- [ ] App icon (file explorer, taskbar, add/remove programers, installer, title bar)
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
