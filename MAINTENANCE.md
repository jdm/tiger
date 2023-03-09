# Development environment

- Install Rust stable compiler
- Install Node.js
- Install the tauri CLI via `cargo install tauri-cli`
- Run `npm install`
- Run `cargo tauri dev`

# How to make a release

- Move relevant section of ROADMAP.md into CHANGELOG.md, commit and push
- On Github, go to **Actions**, select the **Make Release** workflow and click **Run workflow**
- Select the branch to deploy (usually `master`)
- Input a user-facing version name (eg: **0.3.0**)
- Click the **Run workflow** button
- After CI completes, move the release from Draft to Published on Github

# How to increment Tiger format version

1. Create a new module file under `src/sheet/version_x_y_z_.rs` (copy-paste the previous version as a starting point)
2. In your new module, update the `THIS_VERSION` constant and the `as previous_version` import
3. Declare your new module in `src/sheet.rs`
4. Also in `src/sheet.rs`, update the `Version` enum and the `CURRENT_VERSION` constant
5. Update the `pub use self::versionN::*;` line in `src/sheet.rs`
6. Update the sheet structures and `From<>` implementations in your new module as needed
7. Add `#[allow(dead_code)]` to `key` fields causing warnings in the old version structs
