# Changelog

## Tiger 0.7

- [x] Tooltips to show full path in frames pane
- [x] Tooltips for workbench toolbar items
- [x] Tooltip for keyframe snapping toggle
- [x] Tooltip for animation looping toggle
- [x] Tooltip for lock hitboxes toggle
- [x] Can navigate modal dialog buttons with keyboard controls
- [x] Can tab between details fields
- [x] Filters in the Animations and Frames panes are case-insensitive
- [x] Can press F2 to rename animations or hitboxes
- [x] Workbench zoom keeps screen center invariant (not origin)
- [x] Can zoom workbench with mousewheel
- [x] Frames pane auto-scrolls when browsing with keyboard controls
- [x] Animations pane auto-scrolls when browsing with keyboard controls
- [x] Hitboxes pane auto-scrolls when browsing with keyboard controls
- [x] Autoscroll to newly created animations
- [x] Autoscroll to newly created hitboxes
- [x] Autoscroll to newly imported frames
- [x] Autoscroll to newly renamed animations
- [x] Autoscroll to newly renamed hitboxes
- [x] Frames pane saves scroll position in undo stack
- [x] Animations pane saves scroll position in undo stack
- [x] Hitboxes pane saves scroll position in undo stack
- [x] Can press Home/End to select first/last frame
- [x] Can press Home/End to select first/last animation
- [x] Can press Home/End to select first/last hitbox
- [x] Can press Ctrl+A to select all frames
- [x] Can press Ctrl+A to select all animations
- [x] Can press Ctrl+A to select all hitboxes
- [x] Can press Ctrl+A to select all keyframes
- [x] Timeline scroll position in undo stack
- [x] Timeline zoom keeps current position invariant
- [x] Can zoom timeline with mousewheel
- [x] Can scroll timeline with mousewheel or right-click drag
- [x] Removed timeline scrollbar
- [x] Timeline scrolling follows playback
- [x] Timeline scrolling follows frame selection
- [x] Frames are now ordered by path in the UI and in tiger files
- [x] Fixed a bug where keyframes could not be dropped in the timeline dead zone
- [x] Navigation while backtracked in undo stack no longer truncates the undo stack
- [x] Fixed a bug where play/pause actions could cause selection inaccuracies in undo history
- [x] Fixed a bug where up/down arrows selected animations in case-sensitive order
- [x] Fixed a bug where hitboxes were ordered in case-sensitive order
- [x] Fixed a bug where buttons would have a white outline when pressing arrow keys after clicking them

## Tiger 0.6.1

- [x] Fixed a bug where exported textures were incorrectly sized

## Tiger 0.6.0

- [x] Template syntax now uses Handlebars instead of Liquid
- [x] Exported texture dimensions are now power-of-2
- [x] Hitbox names appear in workbench when there is enough space
- [x] Upgraded from Heroicons 1.0 to 2.0
- [x] Keyframe durations can snap to nearby keyframes
- [x] Keyframe durations can snap to multiples of n milliseconds
- [x] Added button to open sheet folder in explorer
- [x] Added context menu entry to open frame in explorer
- [x] Added context menu entries to paste animations/keyframes/hitboxes
- [x] Can press Home/End to snap to first/last keyframe in sequence
- [x] After deleting keyframes, another keyframe is automatically selected
- [x] Added shadow around application window
- [x] Can clear selection by clicking in the dead zone of animations, hitboxes or frames panels
- [x] Can change active direction by clicking in deadzone of timeline
- [x] Can jump to end of sequence by double clicking in deadzone of timeline
- [x] Performance improvements to animation playback and drag operations
- [x] Performance improvements to Export operations
- [x] Cut, Copy and Paste menu entries are now disabled when there is no applicable selection
- [x] List of recent documents is now limited to ten documents
- [x] Fixed a bug where context menus could overflow offscreen
- [x] Fixed a bug where resize handles on hitboxes with odd-numbered dimensions were slightly misaligned
- [x] Fixed a bug where in the list of recently opened files, files with identical names would highlight in unison
- [x] Fixed a bug where disabled menu actions could be interacted with
- [x] Fixed a bug where app window could be restored/maximized by double clicking inside menus
- [x] Fixed a bug where hitboxes panel could grow when there are long hitbox names
- [x] Fixed a bug where animations keyframes or sheet frames could incorrectly appear as selected
- [x] Fixed minor bugs related to using the Undo command immediately after opening a spritesheet

## Tiger 0.5.0

- [x] Document template format
  - [x] Switch documentation Github Pages to `release` branch
- [x] Export dialog links to documentation
- [x] Export process creates intermediate directories if needed
- [x] Error dialog specifies problematic path when export fails due to IO problem
- [x] Nicer looking paths in export settings (no ../ concat)
- [x] Add menu entry to open recent files
- [x] Export dialog form validation
- [x] Unit test timeline commands
- [x] Unit test transient commands
- [x] Fixed a bug where Ctrl+Shift+E opened a Search With Bing panel
- [x] When opening an existing document, an animation is immediately opened for edit

## Tiger 0.4.1

- [x] Fixed a bug where changes caused by mouse clicks would not appear immediately
- [x] Major performance improvements to panning/dragging actions

## Tiger 0.4.0

- [x] UI rewrite (prettier)
- [x] In-place rename UX
- [x] Right click menu to rename item
- [x] Right click menu to delete item
- [x] Filter frames/animations in content panel by text search
- [x] Can display frames in thumbnail or list mode
- [x] Add option to hide origin
- [x] Add option to hide hitboxes
- [x] Add option to lock hitboxes
- [x] Edit hitbox offset and size from details panel
- [x] Edit keyframe duration and offset from details panel
- [x] Support multiple directions/angles for the same animation
  - [x] Sheet stores multiple sequences per animation
  - [x] Can preview and edit multiple sequences
  - [x] Shift selecting keyframes on timeline selects the keyframes you would expect
- [x] Add toggle to hide keyframes (replaces linking/locking)
- [x] Add sprite darkening readility mode
- [x] Workbench tabs
- [x] Copy/paste animations
- [x] Copy/paste keyframes
- [x] Copy/paste hitboxes
- [x] Replace powershell release script with Github action
- [x] Unit test sheet module

## Tiger 0.3.0

- [x] Cap undo history at 100 entries
- [x] Offer to save when closing individual documents
- [x] Error dialogs
- [x] Handle save errors while performing a save on exit
- [x] Dependencies update
- [x] Remove placeholder app icon
- [x] Get rid of failure crate
- [x] Compile on Rust Stable
- [x] Remove commands threads (keep long commands thread)
- [x] Move hitbox data from frames to animation frames
- [x] Frame and hitboxes can be moved using arrow keys
- [x] Fix issue where O key gets stuck after using Ctrl+O shortcut (https://github.com/Gekkio/imgui-rs/pull/215)
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

## Tiger 0.2.0

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

## Tiger 0.1.0

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
