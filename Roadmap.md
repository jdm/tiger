# Roadmap

## Tiger 0.1

- [x] Don't store absolute paths in tiger files
- [x] Show frame names in timeline
- [x] Solution(s) to edit/order/insert frames in timeline
- [x] Add, position, tag, delete hitboxes
- [x] Full compat with crystal sheets:
  - [x] Hitboxes
  - [x] Sheet path in export
  - [x] Top left coords available in template
- [x] Tiger backwards compat
- [x] Release pipeline
- [x] No placeholder menu options
- [x] Export (using last known settings)
- [x] Draw frame being dragged even during animation
- [x] Draw hitboxes during animation
- [x] Animation renames
- [x] Allow user to choose what directory paths are relative to during export
- [x] Fix bug where export window shows weird absolute + relative concatenated paths
- [x] Fix bug where pressing delete while renaming an animation(/hitbox) deletes it
- [x] Fix bug where renaming an animation(/hitbox) unselects and unedits it
- [x] Fix bug where keyframe duration drag shows insert markers
- [x] Fix bug where animation frames can be reorderer by dragging timeline
- [x] Fix bugs when manipulating extremely short animation frames

## Tiger 0.2

- [x] Unsaved file marker
- [x] Offer to save on exit if needed
- [x] Undo/Redo
- [x] Allow interactions while file dialogs are open or I/O is in progress
- [x] Keyboard shortcuts for playback controls
- [x] Keyboard shortcuts menu entries
- [x] Keyboard shortcuts for moving hitbox / keyframe
- [x] Keyboard shortcuts for list navigation
- [x] Loading spinners
- [x] Begin editing animation after creating it
- [x] Automatically select hitbox after creating it
- [x] Select hitbox when clicking it
- [x] Select keyframe when clicking it
- [x] Selected hitbox should have handles for resizing instead of using invisible buttons along borders
- [x] When creating an animation, automatically select it
- [x] Grid
- [x] Drag and drop frames to workbench
- ~~[ ] Grid snapping?~~
- [x] Content of selection window when selecting keyframe
- [x] Content of selection window when selecting hitbox
- [x] In selection window, keep origin centered to preview turnarounds
- [x] When moving keyframe or hitbox, hold shift to move only on one axis
- [x] When resizing hitbox, hold shift to preserve aspect ratio
- [x] Workbench indicates what the current workbench item is
- [x] Sort content panel entries by name
- [x] Sort hitbox panel entries by name
- [x] Dont draw origin when editing frame
- [x] Use rect and point structs consistently instead of tuples everywhere
- [x] Fix bug where origin is not consistent within one animation in selection window (is ok in workbench)
- [x] Fix bug where frame name can go outside frame bound in timeline
- [x] Fix bug where reordering animation frames changes selected keyframe
- [x] Fix bug where a console window opens alongside Tiger on Windows
- [x] Workbench should illustrate selected hitbox or keyframe (w/ borders)
- [x] Clicking blank space within the workbench gets rid of the current selection
- [x] Ctrl+Space to center workbench
- [x] Fix issue where hitboxes are not created precisely where the mouse is clicked because we dont create until the mouse is dragging.
- [x] Pass in mouse drag deltas to drag/resize logic instead of mouse positions. See GetMouseDragDelta in imgui
- [x] Handle scenario when using "Save as" onto a file that is already open

## Tiger 0.3

- [x] Cap undo history at 100 entries
- [x] Offer to save when closing individual documents
- [x] Error dialogs
- [x] Handle save errors while performing a save on exit
- [x] Dependencies update
- [x] Remove placeholder app icon
- [x] Get rid of failure crate
- [x] Move hitbox data from frames to animation frames
- [x] Editing hitboxes while animation is in workbench
  - [x] Hitboxes can be created and moved while an animation is in the workbench
  - [x] When multiple hitboxes and the animation frame are overlapping, mouse hovers and clicks should only interact with one of them
  - [x] Hitboxes panel is renamed to Keyframe and allows selecting the animation frame
  - [x] Keyframe panel can lock individual keyframes, or the animation frame, to prevent mouse interactions
  - [x] Keyframe panel can link/unlink hitboxes to the animation frame so that moving the frame also moves them (default on)
- [x] Fix bug where hitbox resize handles don't work when clicked where they overlap with the corresponding hitbox
- [x] Auto-select new keyframes when inserted to timeline
- [x] Multiple selections
- [x] Jump to next/previous frame
- [x] Auto reload images on frame edit
- ~~[ ] Visible handles for adjusting frame durations (instead of invisible buttons)~~

## Tiger 0.4

- [ ] UI rewrite (prettier)
  - [x] Export dialog
  - [x] Export error dialog
  - [x] Open error dialog
  - [x] Save error dialog
  - [ ] Save As
  - [ ] Save All
  - [ ] Keyboard controls
  - [ ] Auto reload images on frame edit
  - [ ] Floating toolbar in workbench
  - [ ] Better looking playback controls
  - [ ] Jump to next/previous frame
  - [ ] Workbench interactive elements sort priority
  - [ ] Make details panel not completely placeholder
- [x] In-place rename UX
- [x] Right click menu to delete item
- [ ] Filter frames/animations in content panel by text search
- [ ] Grid/list display modes in content tab
- [x] Add option to hide hitboxes while looking at animations in workbench
- [x] Add readability mode
- [ ] Support multiple directions/angles for the same animation
  - [x] Sheet stores multiple sequences per animation
  - [x] Can preview and edit multiple sequences
  - [ ] Shift selecting keyframes on timeline selects the keyframes you would expect
- [x] Add toggle to hide keyframes (replaces linking/locking)
- [x] Add sprite darkening readility mode
- [x] Workbench tabs
- [ ] Replace powershell release script with Github action
- [x] Unit test sheet module

## Tiger 0.5

- [ ] Angle in liquid template sequence data
- [ ] Nicer looking paths in export settings (no concat)
- [ ] Automatically add extensions to exported files
- [ ] Default paths for NFD dialogs
- [ ] Add menu entry to open recent files
- [ ] Export perf improvements
- [ ] Duplicate animation / keyframe (within same sheet)
- [ ] Copy/paste hitboxes
- [ ] Rework content browser: view animations and frames at the same time, possibly with previews?

## Tiger 0.6

- [ ] Add option or keyboard shortcut (held) to hide origin crosshair
- [ ] Add buttons to align frame edges or center to origin
- [x] Frame and hitboxes can be moved using arrow keys
- [ ] Draw hitbox names in workbench
- [ ] Playback speed controls
- [ ] Time snapping of animation frames
- [ ] Fix bug where when zoomed in a lot, resize handles on hitboxes arent correctly centered

## Tiger 0.7

- [ ] Right click menu to rename item
- [ ] Edit keyframe duration and offset from details panel
- [ ] Edit hitbox offset and size from details panel
- [ ] Export dialog links to documentation
- [x] Fix issue where O key gets stuck after using Ctrl+O shortcut (https://github.com/Gekkio/imgui-rs/pull/215)

## Tiger 0.8

- [ ] Handle missing frame files (warning + offer to relocate)
- [ ] In-place tutorials instead of blank data
- [ ] Unit test state module

## Tiger 0.9

- [ ] Timeline scrolling follows playback
- [ ] Timeline scrolling follows frame selection (or double click?)
- [ ] Visual distinction between frame in workbench, editing animation or editing hitboxes (background color/pattern?)
- [ ] Fix jank spacing in exported metadata files
- [ ] Export complete notification
- [ ] Unit test UI

## Tiger 1.0

- [ ] Review all TODO
- [ ] Tiger file format uses semver
- [ ] Remove support for pre-1.0 versions of Tiger file format
- [ ] Provide export templates for some common formats (TBD)
- [x] Compile on Rust Stable
- [x] Remove commands threads (keep long commands thread)
- [ ] Document template format
- [ ] Readme Logo
- [ ] About dialog
- [ ] App icon
- [ ] Itch.io or other distribution method

## Post 1.0

- [ ] Tiger CLI
- [ ] Root motion
- [ ] Onion skin?
- [ ] Hitbox colors
- [ ] Import frames from single sheet image
- [ ] Import animation data from other software (Asesprite, TBD)
- [ ] Sockets (like hitbox but point)
- [ ] Events (arbitrary markers on timeline)
- [ ] Copy/paste animation or keyframe (between sheets)
- [ ] Drag and drop frames from OS to content panel
- [ ] Drag and drop frames from OS to timeline panel
- [ ] Projects?
