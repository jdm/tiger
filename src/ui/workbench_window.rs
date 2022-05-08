use euclid::default::*;
use euclid::vec2;
use imgui::StyleVar::*;
use imgui::*;

use crate::sheet::{Animation, Frame, Hitbox, Keyframe, Shape};
use crate::state::*;
use crate::streamer::{TextureCache, TextureCacheResult};
use crate::ui::spinner::*;

const RESIZE_CONTROL_INTERACTIVE_SIZE: f32 = 16.0; // TODO.dpi

#[derive(Eq, PartialEq)]
enum WorkbenchHoverItem {
    Frame,
    Hitbox(String),
}

// imgui isn't very well equipped to deal with overlapping buttons (our hitboxes and animation frame)
// This function has logic to decide what we want the mouse cursor to interact with in the workbench
// while editing an animation keyframe.
fn compute_hovered_item<'a>(
    ui: &Ui<'a>,
    texture_cache: &TextureCache,
    document: &Document,
    keyframe: &Keyframe,
) -> Option<WorkbenchHoverItem> {
    let is_window_hovered =
        ui.is_window_hovered_with_flags(WindowHoveredFlags::ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM);
    if document.transient.is_some() || !is_window_hovered {
        return None;
    }

    struct WorkbenchHoverItemCandidate {
        item: WorkbenchHoverItem,
        priority: f32, // lower is better
    }
    let mut candidates: Vec<WorkbenchHoverItemCandidate> = Vec::new();
    let zoom = document.view.get_workbench_zoom_factor();
    let workbench_size: Vector2D<f32> = ui.window_size().into();

    // Animation frame
    if let Some(TextureCacheResult::Loaded(texture)) = texture_cache.get(&keyframe.get_frame()) {
        let draw_size = texture.size * zoom;
        let frame_top_left = Vector2D::from(ui.window_pos())
            + (workbench_size / 2.0).floor()
            + document.view.workbench_offset
            + keyframe.get_offset().to_f32() * zoom
            - (draw_size / zoom / 2.0).floor() * zoom;
        let frame_bottom_right = frame_top_left + draw_size;
        if ui.is_mouse_hovering_rect(frame_top_left.to_array(), frame_bottom_right.to_array()) {
            candidates.push(WorkbenchHoverItemCandidate {
                item: WorkbenchHoverItem::Frame,
                priority: draw_size.x * draw_size.y,
            });
        }
    }

    // Hitboxes
    for hitbox in keyframe.hitboxes_iter() {
        let Shape::Rectangle(ref rectangle) = hitbox.geometry;
        let draw_size = Vector2D::from(rectangle.size).to_f32() * zoom;
        let hitbox_top_left = Vector2D::from(ui.window_pos())
            + (workbench_size / 2.0).floor()
            + document.view.workbench_offset
            + Vector2D::from(rectangle.top_left).to_f32() * zoom;
        let hitbox_bottom_right = hitbox_top_left + draw_size;

        if ui.is_mouse_hovering_rect(hitbox_top_left.to_array(), hitbox_bottom_right.to_array()) {
            candidates.push(WorkbenchHoverItemCandidate {
                item: WorkbenchHoverItem::Hitbox(hitbox.get_name().to_owned()),
                priority: draw_size.x * draw_size.y,
            });
        }

        if document.is_hitbox_selected(hitbox) {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let knob_center: Vector2D<f32> = (hitbox_top_left
                        + hitbox_bottom_right
                        + Vector2D::new(dx as f32, dy as f32).component_mul(draw_size))
                        / 2.0;
                    let knob_top_left =
                        knob_center - Vector2D::splat(RESIZE_CONTROL_INTERACTIVE_SIZE) / 2.0;
                    let knob_bottom_right =
                        knob_center + Vector2D::splat(RESIZE_CONTROL_INTERACTIVE_SIZE) / 2.0;
                    if ui.is_mouse_hovering_rect(
                        knob_top_left.to_array(),
                        knob_bottom_right.to_array(),
                    ) {
                        candidates.push(WorkbenchHoverItemCandidate {
                            item: WorkbenchHoverItem::Hitbox(hitbox.get_name().to_owned()),
                            priority: -1.0,
                        });
                    }
                }
            }
        }
    }

    candidates
        .into_iter()
        .min_by_key(|c| c.priority.round() as i32)
        .map(|c| c.item)
}

fn screen_to_workbench<'a>(
    ui: &Ui<'a>,
    screen_coords: Vector2D<f32>,
    document: &Document,
) -> Vector2D<f32> {
    let window_position: Vector2D<f32> = ui.window_pos().into();
    let window_size: Vector2D<f32> = ui.window_size().into();
    let zoom = document.view.get_workbench_zoom_factor();
    let offset = document.view.workbench_offset;
    (screen_coords - offset - window_position - window_size / 2.0) / zoom
}

fn axis_to_cursor(axis: ResizeAxis) -> MouseCursor {
    match axis {
        ResizeAxis::N => MouseCursor::ResizeNS,
        ResizeAxis::S => MouseCursor::ResizeNS,
        ResizeAxis::E => MouseCursor::ResizeEW,
        ResizeAxis::W => MouseCursor::ResizeEW,
        ResizeAxis::NW => MouseCursor::ResizeNWSE,
        ResizeAxis::SE => MouseCursor::ResizeNWSE,
        ResizeAxis::NE => MouseCursor::ResizeNESW,
        ResizeAxis::SW => MouseCursor::ResizeNESW,
    }
}

fn draw_hitbox_resize_controls<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    document: &Document,
    hitbox: &Hitbox,
) -> bool {
    let space: Vector2D<f32> = ui.window_size().into();
    let zoom = document.view.get_workbench_zoom_factor();
    let offset = document.view.workbench_offset;
    let is_mouse_dragging = ui.is_mouse_dragging(MouseButton::Left);
    let mut active = false;

    let rectangle = hitbox
        .get_rectangle()
        .to_f32()
        .scale(zoom, zoom)
        .translate(offset + space / 2.0);

    let draw_list = ui.get_window_draw_list();
    let knob_size = 4.0; // TODO.dpi
    let button_size = RESIZE_CONTROL_INTERACTIVE_SIZE;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if dx == 0 && rectangle.size.width < button_size * 2.0 {
                continue;
            }

            if dy == 0 && rectangle.size.height < button_size * 2.0 {
                continue;
            }

            let axis = match (dx, dy) {
                (-1, -1) => ResizeAxis::NW,
                (-1, 0) => ResizeAxis::W,
                (-1, 1) => ResizeAxis::SW,
                (0, -1) => ResizeAxis::N,
                (0, 1) => ResizeAxis::S,
                (1, -1) => ResizeAxis::NE,
                (1, 0) => ResizeAxis::E,
                (1, 1) => ResizeAxis::SE,
                _ => unreachable!(),
            };

            let position = rectangle.center()
                + vec2(rectangle.size.width / 2.0, 0.0) * dx as f32
                + vec2(0.0, rectangle.size.height / 2.0) * dy as f32;

            ui.set_cursor_pos(position.to_array());
            let screen_position = ui.cursor_screen_pos();

            draw_list
                .add_circle(screen_position, knob_size, [1.0, 1.0, 1.0, 1.0])
                .filled(true)
                .build();

            draw_list
                .add_circle(screen_position, knob_size - 1.0, [0.0, 0.4, 0.9, 1.0]) // TODO.dpi TODO.style
                .filled(true)
                .build();

            let id = format!("drag_handle_{}_{}_{}", hitbox.get_name(), dx, dy);
            let button_pos = position - vec2(button_size, button_size) / 2.0;
            ui.set_cursor_pos(button_pos.to_array());
            ui.invisible_button(&ImString::new(id), [button_size, button_size]);
            if ui.is_item_hovered() {
                ui.set_mouse_cursor(Some(axis_to_cursor(axis)));
                active = true;
            }
            if !document.is_sizing_hitbox() {
                if ui.is_item_active() && is_mouse_dragging {
                    commands.begin_hitbox_scale(axis);
                    active = true;
                }
            }
        }
    }

    active
}

fn draw_hitbox<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    document: &Document,
    hitbox: &Hitbox,
    is_hovered: bool,
) {
    let zoom = document.view.get_workbench_zoom_factor();
    let workbench_offset = document.view.workbench_offset;
    let space: Vector2D<f32> = ui.window_size().into();
    let rectangle = hitbox.get_rectangle();
    let is_mouse_dragging = ui.is_mouse_dragging(MouseButton::Left);

    let cursor_pos =
        workbench_offset + (space / 2.0).floor() + (rectangle.origin.to_f32().to_vector()) * zoom;
    ui.set_cursor_pos(cursor_pos.to_array());

    let top_left: Vector2D<f32> = ui.cursor_screen_pos().into();
    let bottom_right = top_left + rectangle.size.to_f32().to_vector() * zoom;
    let is_selected = document.is_hitbox_selected(hitbox);

    // Draw hitbox outline
    {
        let outline_color = if is_selected {
            [1.0, 0.1, 0.6, 1.0] // TODO.style
        } else if is_hovered {
            [0.0, 0.9, 0.9, 1.0] // TODO.style
        } else {
            [1.0, 1.0, 1.0, 1.0] // TODO.style
        };
        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_rect(top_left.to_array(), bottom_right.to_array(), outline_color)
            .thickness(1.0) // TODO.dpi
            .build();
    }

    // Initiate resize
    if is_selected {
        if draw_hitbox_resize_controls(ui, commands, document, hitbox) {
            return;
        }
    }

    if rectangle.size.is_empty() {
        return;
    }

    if !is_hovered || document.transient.is_some() {
        return;
    }

    ui.set_mouse_cursor(Some(MouseCursor::ResizeAll));

    // Select hitbox
    let hitbox_id = ImString::new(format!("hitbox_button_{}", hitbox.get_name()));
    ui.set_cursor_pos(cursor_pos.to_array());
    if ui.invisible_button(
        &hitbox_id,
        (rectangle.size.to_f32().to_vector() * zoom).to_array(),
    ) {
        let (mut selection, was_blank) = match &document.view.selection {
            Some(Selection::Hitbox(s)) => (s.clone(), false),
            _ => (
                MultiSelection::new(vec![hitbox.get_name().to_owned()]),
                true,
            ),
        };
        if ui.io().key_ctrl {
            if !was_blank {
                selection.toggle(&vec![hitbox.get_name().to_owned()]);
            }
        } else {
            selection = MultiSelection::new(vec![hitbox.get_name().to_owned()]);
        }
        commands.select_hitboxes(&selection);
    }
    let is_active = ui.is_item_active();

    // Begin hitbox drag
    if is_active && is_mouse_dragging {
        let (mut selection, was_blank) = match &document.view.selection {
            Some(Selection::Hitbox(s)) => (s.clone(), false),
            _ => (
                MultiSelection::new(vec![hitbox.get_name().to_owned()]),
                true,
            ),
        };
        if !is_selected {
            if ui.io().key_ctrl {
                if !was_blank {
                    selection.toggle(&vec![hitbox.get_name().to_owned()]);
                }
            } else {
                selection = MultiSelection::new(vec![hitbox.get_name().to_owned()]);
            }
        }
        commands.select_hitboxes(&selection);
        commands.begin_hitbox_drag();
    }
}

fn draw_frame<'a>(ui: &Ui<'a>, texture_cache: &TextureCache, document: &Document, frame: &Frame) {
    let zoom = document.view.get_workbench_zoom_factor();
    let offset = document.view.workbench_offset;
    let workbench_size: Vector2D<f32> = ui.window_size().into();
    match texture_cache.get(&frame.get_source()) {
        Some(TextureCacheResult::Loaded(texture)) => {
            let draw_size = texture.size * zoom;
            let cursor_pos =
                offset + (workbench_size / 2.0).floor() - (draw_size / zoom / 2.0).floor() * zoom;
            ui.set_cursor_pos(cursor_pos.to_array());
            Image::new(texture.id, draw_size.to_array()).build(ui);
        }
        Some(TextureCacheResult::Loading) => {
            ui.set_cursor_pos(offset.to_array());
            draw_spinner(ui, &ui.get_window_draw_list(), workbench_size);
        }
        _ => {
            // TODO
        }
    }
}

fn draw_hitboxes<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    document: &Document,
    keyframe: &Keyframe,
    hovered_item: &Option<WorkbenchHoverItem>,
) {
    let zoom = document.view.get_workbench_zoom_factor();
    let is_mouse_dragging = ui.is_mouse_dragging(MouseButton::Left);
    let mouse_pos = ui.io().mouse_pos.into();
    let mouse_position_in_workbench = screen_to_workbench(ui, mouse_pos, document);
    let drag_delta: Vector2D<f32> = ui.mouse_drag_delta().into();
    let is_shift_down = ui.io().key_shift;

    for hitbox in keyframe.hitboxes_iter() {
        let is_hovered = matches!(
            hovered_item,
            Some(WorkbenchHoverItem::Hitbox(n)) if n == &hitbox.name,
        );
        draw_hitbox(ui, commands, document, hitbox, is_hovered);
    }

    if document.is_sizing_hitbox() {
        if let Some(Transient::HitboxSize(s)) = &document.transient {
            ui.set_mouse_cursor(Some(axis_to_cursor(s.axis)));
            commands.update_hitbox_scale(drag_delta, is_shift_down);
        }
    } else if document.is_positioning_hitbox() {
        ui.set_mouse_cursor(Some(MouseCursor::ResizeAll));
        commands.update_hitbox_drag(drag_delta, !is_shift_down);
    } else if ui.is_window_hovered()
        && is_mouse_dragging
        && document.transient.is_none()
        && hovered_item.is_none()
    {
        // TODO prevent this and other interactions while playback is active
        let drag_delta: Vector2D<f32> = ui.mouse_drag_delta().into();
        commands.create_hitbox(mouse_position_in_workbench - drag_delta / zoom);
    }
}

fn draw_keyframe<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    texture_cache: &TextureCache,
    document: &Document,
    keyframe: &Keyframe,
    keyframe_index: usize,
    is_hovered: bool,
    is_selected: bool,
) -> bool {
    let zoom = document.view.get_workbench_zoom_factor();
    let offset = document.view.workbench_offset;
    let space: Vector2D<f32> = ui.window_size().into();
    match texture_cache.get(&keyframe.get_frame()) {
        Some(TextureCacheResult::Loaded(texture)) => {
            let frame_offset = keyframe.get_offset().to_f32();
            let draw_size = texture.size * zoom;
            let cursor_pos = offset + frame_offset * zoom + (space / 2.0).floor()
                - ((draw_size / zoom / 2.0).floor() * zoom);

            ui.set_cursor_pos(cursor_pos.to_array());
            let cursor_screen_pos: Vector2D<f32> = ui.cursor_screen_pos().into();
            Image::new(texture.id, draw_size.to_array()).build(ui);

            if is_hovered {
                ui.set_cursor_pos(cursor_pos.to_array());
                if ui.invisible_button("current_keyframe", draw_size.to_array()) {
                    if document.transient.is_none() {
                        commands.select_keyframes(&MultiSelection::new(vec![keyframe_index]));
                    }
                }
            }

            if is_selected || is_hovered {
                let outline_color = if is_selected {
                    [1.0, 0.1, 0.6, 1.0] // TODO.style
                } else {
                    [0.0, 0.9, 0.9, 1.0] // TODO.style
                };
                let draw_list = ui.get_window_draw_list();
                draw_list
                    .add_rect(
                        cursor_screen_pos.to_array(),
                        (cursor_screen_pos + draw_size).to_array(),
                        outline_color,
                    )
                    .thickness(1.0) // TODO.dpi
                    .build();
            };
            true
        }
        Some(TextureCacheResult::Loading) => {
            ui.set_cursor_pos(offset.to_array());
            draw_spinner(ui, &ui.get_window_draw_list(), space);
            false
        }
        _ => {
            // TODO
            false
        }
    }
}

fn draw_animation<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    texture_cache: &TextureCache,
    document: &Document,
    animation: &Animation,
) {
    let now = document.view.timeline_clock;
    if let Some((keyframe_index, keyframe)) = animation.get_keyframe_at(now) {
        let is_mouse_dragging = ui.is_mouse_dragging(MouseButton::Left);
        let hovered_item = compute_hovered_item(ui, texture_cache, document, keyframe);
        let is_hovered = hovered_item == Some(WorkbenchHoverItem::Frame);
        let is_selected = document.is_keyframe_selected(keyframe_index);

        let drew = draw_keyframe(
            ui,
            commands,
            texture_cache,
            document,
            keyframe,
            keyframe_index,
            is_hovered,
            is_selected,
        );

        if drew && document.transient.is_none() {
            if is_hovered {
                ui.set_mouse_cursor(Some(MouseCursor::ResizeAll));
            }
            if ui.is_item_active() && is_mouse_dragging {
                if !is_selected {
                    commands.select_keyframes(&MultiSelection::new(vec![keyframe_index]));
                }
                commands.begin_keyframe_offset_drag();
            }
        }

        if let Some(Selection::Keyframe(selected_frame_indexes)) = &document.view.selection {
            // Draw semi-transparent version of selected keyframes that are not at current clock time
            for selected_frame_index in &selected_frame_indexes.items {
                if *selected_frame_index != keyframe_index {
                    if let Some(keyframe) = animation.get_keyframe(*selected_frame_index) {
                        let _style_alpha = ui.push_style_var(StyleVar::Alpha(0.05));
                        draw_keyframe(
                            ui,
                            commands,
                            texture_cache,
                            document,
                            keyframe,
                            *selected_frame_index,
                            false,
                            true,
                        );
                    }
                }
            }

            // Move key frames
            let is_shift_down = ui.io().key_shift;
            if document.is_moving_keyframe() && is_mouse_dragging {
                ui.set_mouse_cursor(Some(MouseCursor::ResizeAll));
                let delta = ui.mouse_drag_delta().into();
                commands.update_keyframe_offset_drag(delta, !is_shift_down);
            }
        }

        draw_hitboxes(ui, commands, document, keyframe, &hovered_item);
    }
}

fn draw_grid<'a>(ui: &Ui<'a>, app_state: &AppState) {
    let draw_list = ui.get_window_draw_list();
    let thickness = 0.5; // TODO.dpi?
    let spacing = 16; // TODO.dpi?
    let grain = 4;

    ui.set_cursor_pos([0.0, 0.0]);

    let top_left: Vector2D<f32> = ui.cursor_screen_pos().into();
    let offset = app_state
        .get_current_document()
        .map(|t| t.view.workbench_offset)
        .unwrap_or_else(Vector2D::<f32>::zero);
    let space: Vector2D<f32> = ui.window_size().into();

    let line_color_main = [1.0, 1.0, 1.0, 0.02]; // TODO.style
    let line_color_dim = [1.0, 1.0, 1.0, 0.004]; // TODO.style

    let origin = top_left + offset + (space / 2.0).floor();
    let grid_start = origin - ((origin - top_left) / spacing as f32).floor() * spacing as f32;
    let num_lines = space.to_i32() / spacing + vec2(1, 1);

    for n in 0..num_lines.x {
        let x = grid_start.x + n as f32 * spacing as f32;
        let color = if (x - origin.x) as i32 % (grain * spacing) == 0 {
            line_color_main
        } else {
            line_color_dim
        };

        // TODO why isn't this using add_line?
        draw_list.add_rect_filled_multicolor(
            [x as f32 - thickness, top_left.y],
            [x as f32 + thickness, top_left.y + space.y],
            color,
            color,
            color,
            color,
        );
    }

    for n in 0..num_lines.y {
        let y = grid_start.y + n as f32 * spacing as f32;
        let color = if (y - origin.y) as i32 % (grain * spacing) == 0 {
            line_color_main
        } else {
            line_color_dim
        };
        // TODO why isn't this using add_line?
        draw_list.add_rect_filled_multicolor(
            [top_left.x, y as f32 - thickness],
            [top_left.x + space.x, y as f32 + thickness],
            color,
            color,
            color,
            color,
        );
    }
}

fn draw_origin<'a>(ui: &Ui<'a>, document: &Document) {
    let offset = document.view.workbench_offset;
    let size = 10.0; // TODO.dpi?
    let thickness = 1.0; // TODO.dpi?

    let draw_list = ui.get_window_draw_list();

    let fill_color = [0.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0]; // TODO.style
    ui.set_cursor_pos([0.0, 0.0]);

    let top_left: Vector2D<f32> = ui.cursor_screen_pos().into();
    let space: Vector2D<f32> = ui.window_size().into();
    let center = top_left + offset + (space / 2.0).floor();
    draw_list.add_rect_filled_multicolor(
        [center.x - thickness, center.y - size],
        [center.x + thickness, center.y + size],
        fill_color,
        fill_color,
        fill_color,
        fill_color,
    );

    draw_list.add_rect_filled_multicolor(
        [center.x - size, center.y - thickness],
        [center.x + size, center.y + thickness],
        fill_color,
        fill_color,
        fill_color,
        fill_color,
    );
}

fn draw_item_name<'a, T: AsRef<str>>(ui: &Ui<'a>, name: T) {
    let color = [1.0, 1.0, 1.0, 1.0]; // TODO.style
    let text_position: Vector2D<f32> = vec2(10.0, 30.0);
    ui.set_cursor_pos(text_position.to_array());
    ui.text_colored(color, &ImString::new(name.as_ref()));
}

fn handle_drag_and_drop<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) {
    let is_window_hovered =
        ui.is_window_hovered_with_flags(WindowHoveredFlags::ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM);
    let is_mouse_down = ui.is_mouse_down(MouseButton::Left);

    if is_window_hovered && !is_mouse_down {
        if let Some(document) = app_state.get_current_document() {
            if let Some(WorkbenchItem::Animation(animation_name)) = &document.view.workbench_item {
                if document.is_dragging_content_frames() {
                    if let Some(animation) = document.sheet.get_animation(animation_name) {
                        if let Some(Selection::Frame(paths)) = &document.view.selection {
                            let index = animation.get_num_keyframes();
                            commands.insert_keyframes_before(
                                paths.items.clone().iter().collect(),
                                index,
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn draw<'a>(
    ui: &Ui<'a>,
    rect: &Rect<f32>,
    app_state: &AppState,
    commands: &mut CommandBuffer,
    texture_cache: &TextureCache,
) {
    let _style_rounding = ui.push_style_var(WindowRounding(0.0));
    let _style_border = ui.push_style_var(WindowBorderSize(0.0));
    Window::new("Workbench")
        .position(rect.origin.to_array(), Condition::Always)
        .size(rect.size.to_array(), Condition::Always)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .menu_bar(false)
        .movable(false)
        .scrollable(false)
        .scroll_bar(false)
        .bring_to_front_on_focus(false)
        .build(ui, || {
            draw_grid(ui, app_state);

            if let Some(document) = app_state.get_current_document() {
                ui.set_cursor_pos([0.0, 0.0]);

                if document.transient.is_none() {
                    if ui.invisible_button("workbench_dead_zone", rect.size.to_array()) {
                        commands.clear_selection();
                    }
                    ui.set_item_allow_overlap();
                }

                match &document.view.workbench_item {
                    Some(WorkbenchItem::Frame(path)) => {
                        if let Some(frame) = document.sheet.get_frame(path) {
                            draw_frame(ui, texture_cache, document, frame);
                            let name = frame
                                .get_source()
                                .file_name()
                                .map(|s| s.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "".to_string());
                            draw_item_name(ui, name);
                        }
                    }
                    Some(WorkbenchItem::Animation(name)) => {
                        if let Some(animation) = document.sheet.get_animation(name) {
                            draw_animation(ui, commands, texture_cache, document, animation);
                            draw_origin(ui, document);
                            draw_item_name(ui, animation.get_name());
                        }
                    }
                    None => (),
                }

                handle_drag_and_drop(ui, app_state, commands);

                if ui.is_window_hovered() {
                    if ui.io().key_ctrl {
                        let mouse_wheel = ui.io().mouse_wheel;
                        if mouse_wheel > 0.0 {
                            commands.workbench_zoom_in();
                        } else if mouse_wheel < 0.0 {
                            commands.workbench_zoom_out();
                        }
                    }
                    if ui.is_mouse_dragging(MouseButton::Right) {
                        commands.pan(ui.io().mouse_delta.into());
                    }
                    if ui.is_mouse_down(MouseButton::Right) {
                        ui.set_mouse_cursor(Some(MouseCursor::ResizeAll));
                    }
                }
            }
        });
}
