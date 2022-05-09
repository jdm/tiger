use imgui::StyleVar::*;
use imgui::*;
use material_icons::Icon;

use crate::sheet::{Animation, Hitbox, Keyframe};
use crate::state::*;
use crate::ui::Rect;

fn draw_frame<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    document: &Document,
    animation: &Animation,
    keyframe_index: usize,
    keyframe: &Keyframe,
) {
    let is_selected = document.is_keyframe_selected(keyframe_index);
    if let Some(file_name) = keyframe.get_frame().file_name() {
        let file_name = file_name.to_string_lossy();
        if Selectable::new(format!("{} {file_name}", Icon::Image))
            .selected(is_selected)
            .size([0.0, 0.0])
            .build(ui)
        {
            let new_selection = MultiSelection::process(
                keyframe_index,
                ui.io().key_shift,
                ui.io().key_ctrl,
                &(0..animation.get_num_keyframes()).collect(),
                match &document.view.selection {
                    Some(Selection::Keyframe(s)) => Some(s),
                    _ => None,
                },
            );
            commands.select_keyframes(&new_selection);
        }
    }
}

fn draw_hitboxes<'a>(
    ui: &Ui<'a>,
    commands: &mut CommandBuffer,
    document: &Document,
    keyframe: &Keyframe,
) {
    let mut hitboxes: Vec<&Hitbox> = keyframe.hitboxes_iter().collect();
    hitboxes.sort_unstable();
    for hitbox in hitboxes.iter() {
        let hitbox_name = hitbox.get_name();

        {
            let _token = ui.push_id(&format!("link {}", hitbox_name));
            // TODO.style
            let _color = ui.push_style_color(
                StyleColor::Button,
                if hitbox.is_linked() {
                    [249.0 / 255.0, 212.0 / 255.0, 35.0 / 255.0, 1.0]
                } else {
                    [200.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0, 1.0]
                },
            );
            let _text_color = ui.push_style_color(
                StyleColor::Text,
                if hitbox.is_linked() {
                    [25.0 / 255.0, 15.0 / 255.0, 0.0 / 255.0, 1.0]
                } else {
                    [255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0]
                },
            );
            if ui.small_button(format!("{}", Icon::Link)) {
                commands.set_hitbox_linked(hitbox, !hitbox.is_linked());
            }
        }

        ui.same_line();

        {
            let _token = ui.push_id(&format!("lock {}", hitbox_name));
            // TODO.style
            let _color = ui.push_style_color(
                StyleColor::Button,
                if hitbox.is_locked() {
                    [249.0 / 255.0, 212.0 / 255.0, 35.0 / 255.0, 1.0]
                } else {
                    [200.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0, 1.0]
                },
            );
            let _text_color = ui.push_style_color(
                StyleColor::Text,
                if hitbox.is_locked() {
                    [25.0 / 255.0, 15.0 / 255.0, 0.0 / 255.0, 1.0]
                } else {
                    [255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0]
                },
            );
            if ui.small_button(format!("{}", Icon::Lock)) {
                commands.set_hitbox_locked(hitbox, !hitbox.is_locked());
            }
        }

        ui.same_line();

        let is_selected = document.is_hitbox_selected(hitbox);
        if Selectable::new(format!("{} {hitbox_name}", Icon::Crop169))
            .selected(is_selected)
            .size([0.0, 0.0])
            .disabled(hitbox.is_locked())
            .build(ui)
        {
            let new_selection = MultiSelection::process(
                hitbox.get_name().to_owned(),
                ui.io().key_shift,
                ui.io().key_ctrl,
                &hitboxes.iter().map(|h| h.get_name().to_owned()).collect(),
                match &document.view.selection {
                    Some(Selection::Hitbox(s)) => Some(s),
                    _ => None,
                },
            );
            commands.select_hitboxes(&new_selection);
        }
    }
}

pub fn draw<'a>(ui: &Ui<'a>, rect: &Rect<f32>, app_state: &AppState, commands: &mut CommandBuffer) {
    let _style_rounding = ui.push_style_var(WindowRounding(0.0));
    let _style_border = ui.push_style_var(WindowBorderSize(0.0));
    Window::new("Keyframe Content")
        .position(rect.min().to_array(), Condition::Always)
        .size(rect.size.to_array(), Condition::Always)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .build(ui, || {
            if let Some(document) = app_state.get_current_document() {
                if let Ok(animation) = document.get_workbench_animation() {
                    if let Ok((keyframe_index, keyframe)) = document.get_workbench_keyframe() {
                        draw_frame(ui, commands, document, animation, keyframe_index, keyframe);
                        ui.separator();
                        draw_hitboxes(ui, commands, document, keyframe);
                    }
                }
            }
        });
}
