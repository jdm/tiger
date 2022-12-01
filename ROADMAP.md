# Roadmap

## Tiger 0.6

- [x] Template syntax now uses Handlebars instead of Liquid
- [x] Draw hitbox names in workbench
- [x] Upgrade from Heroicons 1.0 to 2.0
- [x] Can snap to nearby frames
- [x] Can snap to frame duration multiples of n milliseconds
- [x] Add button to open sheet folder in explorer
- [x] Add context menu entry to open frame in explorer
- [x] Add context menu entries to paste animations/keyframes/hitboxes
- [x] Can press Home/End to snap to first/last keyframe
- [ ] Evaluate https://github.com/ChevyRay/crunch-rs
- [x] After deleting keyframes, another keyframe is automatically selected
- [x] Add shadow around application window
- [x] Can clear selection by clicking in the dead zone of animations, hitboxes
- [x] Can change active direction by clicking in deadzone of timeline
- [x] Can jump to end of animation by double clicking in deadzone of timeline
- [x] Performance improvements to animation playback and drag operations
- [x] Cut, Copy and Paste menu entries are now disabled when there is no applicable selection
- [x] List of recent documents is now limited to ten documents
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
- [ ] Frame, hitbox and animation panes auto-scroll when browsing with keyboard controls
- [ ] Frame, hitbox and animation panes have their scroll position in undo stack
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
- [ ] Can preview export image and metadata output
- [ ] Export complete notification
- [ ] Auto-updates
- [ ] Splash screen
- [ ] Add a a help menu with links to issues/discussions/documentation
- [ ] Add About dialog with version number / license info
- [ ] Can open log from help menu
- [ ] .tiger file association

## Tiger 1.0

- [ ] Review all TODO
- [ ] Re-evaluate https://github.com/1Password/typeshare to auto-generate typescript DTOs
- [ ] Tiger file format uses semver
- [ ] Remove support for pre-1.0 versions of Tiger file format
- [ ] Readme logo
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
- [ ] Interop with specific game engines (TBD)
- [ ] Import frames from single sheet image
- [ ] Import animation data from other software (Asesprite, TBD)
- [ ] Sockets (like hitbox but point)
- [ ] Events (arbitrary markers on timeline)
- [ ] Drag and drop frames from OS to content panel
- [ ] Drag and drop frames from OS to timeline panel
- [ ] Project files (.tiger files within a project can be moved without breaking)
