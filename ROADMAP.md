# Roadmap

## Tiger 0.6

- [x] Draw hitbox names in workbench
- [x] Upgrade from Heroicons 1.0 to 2.0
- [ ] Playback speed controls?
- [ ] Time snapping / quantizing of animation frames
- [x] Add button to open sheet folder in explorer
- [x] Add context menu entry to open frame in explorer
- [x] Add context menu entries to paste animations/keyframes/hitboxes
- [x] Can press Home/End to snap to first/last keyframe
- [ ] Evaluate https://github.com/ChevyRay/crunch-rs
- [ ] Evaluate https://github.com/Keats/tera
- [ ] Can clear selection by clicking in the dead zone of animations, hitboxes or timeline panel
- [x] Performance improvements to animation playback and drag operations
- [x] Cut, Copy and Paste menu entries are now disabled when there is no applicable selection
- [x] Fixed a bug where context menus could overflow offscreen
- [x] Fixed a bug where resize handles on hitboxes with odd-numbered dimensions were slightly misaligned
- [x] Fixed a bug where in the list of recently opened files, files with identical names would highlight in unison
- [x] Fixed a bug where disabled menu actions could be interacted with
- [x] Fixed a bug where app window could be restored/maximized by double clicking inside menus
- [x] Fixed a bug where hitboxes panel could grow when there are long hitbox names
- [x] Fixed a bug where animations could or frames could incorrectly appear as selected
- [x] Fixed minor bugs related to using the Undo command immediately after opening a spritesheet

## Tiger 0.7

- [ ] Tooltips everywhere
- [ ] Export perf improvements
- [ ] Can press F2 to rename animations or hitboxes
- [ ] Workbench zoom keeps screen center invariant (not origin)
- [ ] Timeline zoom keeps current position invariant
- [ ] Can zoom workbench with mousewheel
- [ ] Can zoom timeline with mousewheel
- [ ] Can scroll timeline with mousewheel
- [ ] Frames pane scroll position in undo stack
- [ ] Animations pane scroll position in undo stack
- [ ] Timeline scroll position in undo stack

## Tiger 0.8

- [ ] Handle missing frame files (warning + offer to relocate)
- [ ] In-place tutorials instead of blank data
- [ ] Consider merging some code between save, save_as and save_all
- [ ] Automatically add extensions to exported files
- [ ] Default paths for NFD dialogs
- [ ] Reach satisfying test coverage

## Tiger 0.9

- [ ] Timeline scrolling follows playback
- [ ] Timeline scrolling follows frame selection (or double click?)
- [ ] Document keyboard shortcuts
- [ ] Fix jank spacing in exported metadata files
- [ ] Export complete notification
- [ ] Auto-updates
- [ ] Splash screen
- [ ] .tiger file association

## Tiger 1.0

- [ ] Review all TODO
- [ ] Tiger file format uses semver
- [ ] Remove support for pre-1.0 versions of Tiger file format
- [ ] Readme logo
- [ ] About dialog
- [ ] App icon (file explorer, taskbar, add/remove programers, installer, title bar)
- [ ] Itch.io or other distribution method

## Post 1.0

- [ ] Tiger CLI
- [ ] Root motion
- [ ] Onion skin?
- [ ] Hitbox colors
- [ ] Interop with specific game engines (TBD)
- [ ] Import frames from single sheet image
- [ ] Import animation data from other software (Asesprite, TBD)
- [ ] Sockets (like hitbox but point)
- [ ] Events (arbitrary markers on timeline)
- [ ] Drag and drop frames from OS to content panel
- [ ] Drag and drop frames from OS to timeline panel
- [ ] Project files (.tiger files within a project can be moved without breaking)
