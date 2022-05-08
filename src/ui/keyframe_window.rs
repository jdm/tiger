use imgui::StyleVar::*;
use imgui::*;

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
        if Selectable::new(&ImString::new(file_name))
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
        let is_selected = document.is_hitbox_selected(hitbox);

        if Selectable::new(&ImString::new(hitbox.get_name()))
            .selected(is_selected)
            .size([0.0, 0.0])
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
