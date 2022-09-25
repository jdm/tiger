---
nav_order: 2
---

# Exporting spritesheets

The spritesheet files with the `.tiger` extension being created and edited by Tiger are simple `json` files. You might be tempted to use them directly into your game engine of choice, but there are reasons not to:

- The exact format may change between Tiger releases and could break your engine integration
- The `.tiger` files refer to animation frames as individual files. For performance reasons, it is often preferable to combine frames into atlas textures

The export process in Tiger is designed to solve these two issues. When exporting a spritesheet (from the `File` > `Export` menu), Tiger generates two files:

1. An atlas image file (`.png`) containing all the individual frames of animation packed together
2. A metadata text file describing the position of individual frames in the atlas image, and additional metadata like hitboxes, animation names and timings

The `Texture File` and `Metadata File` options in the Export dialog tell Tiger where to save the corresponding files.

## Metadata Format

The exported metadata text file does not obey a specific format. It is up to you to define the format by providing a template file. This template file is specified using the `Metadata Template File` option in the Export dialog. You most likely only need to make one template file for your entire project / game engine.

Here is an example of a simple template file which could be used to generate XML metadata:

```
<sprite>
	\{% for frame in frames %}
	<frame id="{{frame.index}}" x="{{frame.x}}" y="{{frame.y}}" width="{{frame.width}}" height="{{frame.height}}" />
	\{% endfor %}
</sprite>
```

When used, this template would generate metadata files like the following:

```xml
<sprite>
	<frame id="0" x="0" y="0" width="33" height="27" />
	<frame id="1" x="33" y="0" width="25" height="44" />
	<frame id="2" x="58" y="0" width="35" height="34" />
	<frame id="3" x="93" y="0" width="35" height="31" />
	<frame id="4" x="128" y="0" width="25" height="29" />
</sprite>
```

## Metadata Template Syntax

Tiger template files are based on the general-purpose `liquid` template format, which has its [own documentation](https://shopify.github.io/liquid). This documentation describes how to do loops, branches and basic arithmetic in your templates.

The spritesheet data that can be referenced in the template is described in the following tables:

### Global Variables

| Field        | Type                      | Description |
|:-------------|:--------------------------|:------------|
| sheet_image  | String                    | Path to the atlas image file containing all the fames in the spritesheet. This path is relative to the directory selected in the `Metadata Root Directory` option of the Export dialog. |
| frames       | [Frame](#frame)[]         | List of all the frames in the spritesheet. |
| animations   | [Animation](#animation)[] | List of all the animations in the spritesheet. |

### Frame

| Field        | Type         | Description |
|:-------------|:-------------|:------------|
| source       | String       | Path to the source image file of this individual frame. This path is relative to the directory selected in the `Metadata Root Directory` option of the Export dialog. |
| index        | Number       | Arbitrary frame identifier. |
| x            | Number       | Horizontal position of the frame in the atlas image file, measured from the left edge. |
| y            | Number       | Vertical position of the frame in the atlas image file, measured from the top edge. |
| width        | Number       | Frame width in pixels. |
| height       | Number       | Frame height in pixels. |

### Animation

| Field        | Type                    | Description |
|:-------------|:------------------------|:------------|
| name	       | String                  | Name of the animation. |
| is_looping   | Boolean                 | True if the animation is meant to repeat after it ends. |
| sequences    | [Sequence](#sequence)[] | List of sequences in this animation. There is one sequence per direction in the animation. |

### Sequence

| Field        | Type                    | Description |
|:-------------|:------------------------|:------------|
| direction    | Direction               | Direction the sequence. |
| keyframes    | [Keyframe](#keyframe)[] | Chronological list of keyframes in this sequence. |

### Keyframe

| Field        | Type                    | Description |
|:-------------|:------------------------|:------------|
| frame        | [Frame](#frame)         | Frame to display during this keyframe. |
| duration     | Number                  | Duration in milliseconds. |
| x            | Number                  | Horizontal position of this keyframe, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions to the right of the origin. |
| y            | Number                  | Vertical position of this keyframe, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions above the origin. |
| hitboxes     | [Hitbox](#hitbox)[]     | List of hitboxes in this keyframe. |

### Hitbox

| Field        | Type                    | Description |
|:-------------|:------------------------|:------------|
| name         | String                  | Name of the hitbox. |
| x            | Number                  | Horizontal position of this hitbox, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions to the right of the origin. |
| y            | Number                  | Vertical position of this hitbox, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions above the origin. |
| width        | Number                  | Hitbox width in pixels. |
| height       | Number                  | Hitbox height in pixels. |
