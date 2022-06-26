# How to make a release

- On Github, go to **Actions**, select the **Make Release** workflow and click **Run workflow**
- Select the branch to deploy (usually `master`)
- Input a user-facing version name (eg: **0.3.0**)
- Click the **Run workflow** button
- After CI completes, find the release on Github and write the changelog
- Move the release from Draft to Published

Note that the Github web UI will separate the release from the corresponding tag until published.

# How to increment Tiger format version

1. Create a new module file under `src/sheet/compat/versionN.rs` (copy-paste the previous version as a starting point)
2. In your new module, update the THIS_VERSION constant and the `as previous_version` import
3. Declare your new module in `src/sheet/compat.rs`
4. Also in `src/sheet/compat.rs`, update the `Version` enum and the `CURRENT_VERSION` constant
5. Update the `pub use self::compat::versionN::*;` line in src/sheet/mod.rs
6. Update the sheet structures and From<> implementations in your new module as needed
