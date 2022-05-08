use imgui::StyleVar::*;
use imgui::*;

use crate::sheet::{Hitbox, Keyframe};
use crate::state::*;
use crate::ui::Rect;

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
    Window::new("Hitboxes")
        .position(rect.min().to_array(), Condition::Always)
        .size(rect.size.to_array(), Condition::Always)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .build(ui, || {
            if let Some(document) = app_state.get_current_document() {
                if let Ok((_, keyframe)) = document.get_workbench_keyframe() {
                    draw_hitboxes(ui, commands, document, keyframe);
                }
            }
        });
}
