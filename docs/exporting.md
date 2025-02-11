---
nav_order: 2
---

# Exporting Spritesheets

Spritesheet files with the `.tiger` extension being created and edited by Tiger are simple `json` files. You might be tempted to use them directly into your game engine of choice, but there are reasons not to:

- The exact format may change between Tiger releases and could break your engine integration.
- The `.tiger` files refer to animation frames as individual files. For performance reasons, it is often preferable to combine frames into atlas images.

The export process in Tiger is designed to solve these two issues. When exporting a spritesheet (from the `File` > `Export` menu), Tiger generates two files:

1. An atlas image file (`.png`) containing all the individual frames of animation packed together.
2. A metadata text file describing the position of individual frames in the atlas image, and additional metadata like hitboxes, animation names and timings.

The `Atlas Image File` and `Metadata File` options in the Export dialog tell Tiger where to save the corresponding files.

## Metadata Format

The exported metadata text file does not obey a predefined format. It is up to you to define the format by providing a **template file**. This template file is specified using the `Metadata Template File` option in the Export dialog. You most likely only need to make one template file for your entire game / engine.

Here is an example of a simple template file which can be used to generate XML metadata:

{% raw %}

```handlebars
<spritesheet>
	{{ #each frames as |frame| }}
	<frame id="{{ frame.index }}" x="{{ frame.x }}" y="{{ frame.y }}" width="{{ frame.width }}" height="{{ frame.height }}" />
	{{ /each }}
</spritesheet>
```

{% endraw %}

When used, this template would generate metadata files like the following:

```xml
<spritesheet>
	<frame id="0" x="0" y="0" width="33" height="27" />
	<frame id="1" x="33" y="0" width="25" height="44" />
	<frame id="2" x="58" y="0" width="35" height="34" />
	<frame id="3" x="93" y="0" width="35" height="31" />
	<frame id="4" x="128" y="0" width="25" height="29" />
</spritesheet>
```

More elaborate template examples can be found here:

- [JSON](https://github.com/agersant/tiger/blob/master/docs/examples/json.template)
- [XML](https://github.com/agersant/tiger/blob/master/docs/examples/xml.template)

## Metadata Template Syntax

Tiger template files are based on the general-purpose `handlebars` template format, which has its [own documentation](https://handlebarsjs.com/guide/).

In addition to standard Handlebars, additional helpers are available:

| Name     | Example                                                                    | Description                                                          |
| :------- | :------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| add      | `{%raw%}{{ add keyframe.x 5 }}{%endraw%}`                                  | Addition of two numbers.                                             |
| divide   | `{%raw%}{{ divide keyframe.duration 5 }}{%endraw%}`                        | Division of two numbers. Attempting to divide by zero will return 0. |
| multiply | `{%raw%}{{ multiply keyframe.x -1 }}{%endraw%}`                            | Multiplication of two numbers.                                       |
| subtract | `{%raw%}{{ subtract keyframe.x 5 }}{%endraw%}`                             | Subtraction of two numbers.                                          |
| eq       | `{%raw%}{{ #if eq hitbox.name "damage" }}{%endraw%}`                       | Equals operator.                                                     |
| ne       | `{%raw%}{{ #if ne hitbox.name "damage" }}{%endraw%}`                       | Not-equals operator.                                                 |
| gt       | `{%raw%}{{ #if gt frame.width 10 }}{%endraw%}`                             | Greater than operator.                                               |
| gte      | `{%raw%}{{ #if gte frame.width 10 }}{%endraw%}`                            | Greater than or equal operator.                                      |
| lt       | `{%raw%}{{ #if lt frame.width 10 }}{%endraw%}`                             | Less than operator.                                                  |
| lte      | `{%raw%}{{ #if lte frame.width 10 }}{%endraw%}`                            | Less than or equal operator.                                         |
| and      | `{%raw%}{{ #if and (lte frame.width 10) (lte frame.width 10) }}{%endraw%}` | Boolean `and` operator.                                              |
| or       | `{%raw%}{{ #if or (lte frame.width 10) (lte frame.width 10) }}{%endraw%}`  | Boolean `or` operator.                                               |
| not      | `{%raw%}{{ #if not (eq frame.x 10) }}{%endraw%}`                           | Boolean `not` operator.                                              |
| len      | `{%raw%}{{ len keyframe.hitboxes }}{%endraw%}`                             | Number of items in an array or object                                |

In the context of boolean operators, the following operands evaluate as `false`:

- `false` boolean
- Empty string
- Empty array
- Empty object

[String manipulation helpers](https://github.com/davidB/handlebars_misc_helpers#string-transformation) are also available.

## Using Spritesheet Data in Templates

Spritesheet data that can be referenced in template files is described in the following tables:

### Global Variables

| Field       | Type                      | Description                                              |
| :---------- | :------------------------ | :------------------------------------------------------- |
| atlas_image | [Image](#image)           | Image file containing all the frames in the spritesheet. |
| frames      | [Frame](#frame)[]         | List of frames in the spritesheet.                       |
| animations  | [Animation](#animation)[] | List of animations in the spritesheet.                   |

### Image

| Field  | Type   | Description                                                                                                                           |
| :----- | :----- | :------------------------------------------------------------------------------------------------------------------------------------ |
| path   | Number | Path to the image file. This path is relative to the directory selected in the `Metadata Root Directory` option of the Export dialog. |
| width  | Number | Image width in pixels.                                                                                                                |
| height | Number | Image height in pixels.                                                                                                               |

### Frame

| Field  | Type   | Description                                                                       |
| :----- | :----- | :-------------------------------------------------------------------------------- |
| index  | Number | Arbitrary frame identifier.                                                       |
| x      | Number | Horizontal position of the frame in the atlas image, measured from the left edge. |
| y      | Number | Vertical position of the frame in the atlas image, measured from the top edge.    |
| width  | Number | Frame width in pixels.                                                            |
| height | Number | Frame height in pixels.                                                           |

### Animation

| Field      | Type                    | Description                                                                                |
| :--------- | :---------------------- | :----------------------------------------------------------------------------------------- |
| name       | String                  | Name of the animation.                                                                     |
| is_looping | Boolean                 | True if the animation is meant to repeat after it ends.                                    |
| sequences  | [Sequence](#sequence)[] | List of sequences in this animation. There is one sequence per direction in the animation. |

### Sequence

| Field     | Type                    | Description                                       |
| :-------- | :---------------------- | :------------------------------------------------ |
| direction | [Direction](#direction) | Direction of the sequence.                        |
| keyframes | [Keyframe](#keyframe)[] | Chronological list of keyframes in this sequence. |

### Keyframe

| Field    | Type                | Description                                                                                                                                                         |
| :------- | :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| frame    | [Frame](#frame)     | Frame to display during this keyframe.                                                                                                                              |
| hitboxes | [Hitbox](#hitbox)[] | List of hitboxes in this keyframe.                                                                                                                                  |
| duration | Number              | Duration in milliseconds.                                                                                                                                           |
| x        | Number              | Position of this keyframe's left edge, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions to the right of the origin. |
| y        | Number              | Position of this keyframe's top edge, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions above the origin.            |

### Hitbox

| Field  | Type   | Description                                                                                                                                                       |
| :----- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| name   | String | Name of the hitbox.                                                                                                                                               |
| x      | Number | Position of this hitbox's left edge, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions to the right of the origin. |
| y      | Number | Position of this hitbox's top edge, relative to the origin of the animation (blue ➕ in the Tiger UI). Positive values for positions above the origin.            |
| width  | Number | Hitbox width in pixels.                                                                                                                                           |
| height | Number | Hitbox height in pixels.                                                                                                                                          |

### Direction

String with one of the following values:

- North
- East
- South
- West
- NorthEast
- NorthWest
- SouthEast
- SouthWest
