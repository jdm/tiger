[![Release](https://img.shields.io/github/v/release/agersant/tiger)](https://github.com/agersant/tiger/releases/latest) [![Build Status](https://img.shields.io/github/actions/workflow/status/agersant/tiger/ci.yml)](https://github.com/agersant/tiger/actions) [![Codecov](https://codecov.io/gh/agersant/tiger/branch/master/graph/badge.svg?token=Ekd9mm2Wii)](https://codecov.io/gh/agersant/tiger)

![Tiger Logo](res/readme/logo.svg?raw=true "Tiger")

# Overview

Tiger is a visual tool to author game spritesheets and their metadata.

This tool bridges the gap between the work done by an artist and data needed by a game engine. Artists draw animations as images, but game engines need lots of metadata about how to use them correctly. Tiger gives you a simple but versatile interface to organize frames into animations like "run", "idle" or "attack". Tiger can also adjust offsets, timings, and hitboxes for each frame.

After you organize everything in Tiger, it exports a spritesheet containing all the images, plus a metadata file. The exact format of this metadata file is defined by you, using a template system. This makes it easy to integrate Tiger Sheets with any game engine.

# Key Features

- ✅ Easy-to-use timeline to author animations.
- ✅ Supports perspectives for any 2D game (top-down, sidescroller, isometric, etc.).
- ✅ Automatically hot-reloads source images when they are changed.
- ✅ Packs animation frames into texture atlases.
- ✅ Can add and tag hitboxes.
- ✅ Flexible template system exports metadata in any text-based format.
- ✅ Free and open-source with a permissive license.

Note that Tiger is not:

- ❌ A drawing program.
- ❌ A skeletal animation program like Spine.
- ❌ A plug-and-play solution for any specific engine. You will need code to use the output of Tiger in your engine of choice.

# Screenshots & Videos

[tiger-demo-2.webm](https://user-images.githubusercontent.com/817256/223934634-59958844-6763-4e87-ad38-6aa69a6480d4.webm)

|                                                                                                             |                                                                                                           |
| :---------------------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------: |
|          [![](res/readme/screenshot-1.0.0.png?raw=true)](res/readme/screenshot-1.0.0.png?raw=true)          |  [![](res/readme/screenshot-1.0.0-export.png?raw=true)](res/readme/screenshot-1.0.0-export.png?raw=true)  |
| [![](res/readme/screenshot-1.0.0-relocate.png?raw=true)](res/readme/screenshot-1.0.0-relocate.png?raw=true) | [![](res/readme/screenshot-1.0.0-startup.png?raw=true)](res/readme/screenshot-1.0.0-startup.png?raw=true) |

# Getting Started

Tiger is only supported on Windows. To install it:

1. Go to the [latest release](https://github.com/agersant/tiger/releases/latest) page.
2. Download the `.msi` installer.
3. Run the installer. This will install Tiger and Microsoft Edge Webview (if necessary).
4. That's it, you're done!

To learn how to integrate Tiger Sheets into your game, please refer to the [Documentation](https://agersant.github.io/tiger/).

# Contributing

- 🗨 For help, feedback or suggesting new features, please use [Discussions](https://github.com/agersant/tiger/discussions).
- 🐛 For bug reports, please use the [Issues Tracker](https://github.com/agersant/tiger/issues).
- 🛠 Planned changes and features are listed in the project [Roadmap](ROADMAP.md).
- ❌ Please do not create pull requests adding new features.

# License

The MIT License in this repository applies to the Tiger _source code_. This does not include the Tiger logo and mascot illustrations. If you operate a public fork of this project, you cannot use the Tiger logo or mascot. For private forks (individual, or distribution limited to your team/company), you may use the Tiger logo or mascot.

Sprites in the example screenshots are from [OpenDuelyst](https://github.com/open-duelyst/duelyst) ([License](https://github.com/open-duelyst/duelyst/blob/main/LICENSE)).

&nbsp;

<p align="center">
	<img src="res/readme/happy_tiger.svg?raw=true" height="200" />
</p>
