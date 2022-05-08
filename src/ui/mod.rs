use euclid::default::*;
use euclid::{rect, vec2};
use glium::glutin::event::VirtualKeyCode;
use imgui::StyleVar::*;
use imgui::*;

use crate::sheet::ExportFormat;
use crate::state::*;
use crate::streamer::{TextureCache, TextureCacheResult};
use crate::utils;

mod content_window;
mod hitboxes_window;
mod selection_window;
mod spinner;
mod timeline_window;
mod workbench_window;

pub fn run<'a>(ui: &Ui<'a>, app_state: &AppState, texture_cache: &TextureCache) -> CommandBuffer {
    let mut commands = CommandBuffer::new();

    let window_size = ui.io().display_size;
    let (window_width, window_height) = (window_size[0], window_size[1]);
    let window_size = (window_width, window_height);

    let content_width = 0.12 * window_width;
    let hitboxes_width = 0.12 * window_width;

    let [_, menu_height] = draw_main_menu(ui, app_state, &mut commands);

    {
        let workbench_width = window_width - content_width - hitboxes_width;
        let workbench_rect = rect(
            content_width,
            menu_height,
            workbench_width,
            window_height - menu_height,
        );
        workbench_window::draw(ui, &workbench_rect, app_state, &mut commands, texture_cache);
    }

    {
        let documents_rect = rect(content_width, menu_height, window_width, 0.0);
        draw_documents_window(ui, &documents_rect, app_state, &mut commands);
    }

    let panels_height = window_height - menu_height;
    let content_height = 0.80 * panels_height;

    {
        let content_rect = rect(0.0, menu_height, content_width, content_height);
        content_window::draw(ui, &content_rect, app_state, &mut commands);
    }

    {
        let selection_width = content_width;
        let selection_height = panels_height - content_height;

        let selection_rect = rect(
            0.0,
            window_height - selection_height,
            selection_width,
            selection_height,
        );
        selection_window::draw(ui, &selection_rect, app_state, texture_cache);
    }

    {
        let timeline_width = window_width - content_width;
        let timeline_height = panels_height - content_height;
        let timeline_rect = rect(
            content_width,
            window_height - timeline_height,
            timeline_width,
            timeline_height,
        );
        timeline_window::draw(ui, &timeline_rect, app_state, &mut commands);
    }

    {
        let hitboxes_height = content_height;
        let hitboxes_rect = rect(
            window_width - hitboxes_width,
            menu_height,
            hitboxes_width,
            hitboxes_height,
        );
        hitboxes_window::draw(ui, &hitboxes_rect, app_state, &mut commands);
    }

    draw_export_popup(ui, app_state, &mut commands);
    draw_rename_popup(ui, app_state, &mut commands);
    draw_unsaved_changes_popup(ui, app_state, window_size, &mut commands);
    draw_saving_popup(ui, app_state, window_size);
    draw_error_popup(ui, app_state, window_size, &mut commands);

    update_drag_and_drop(ui, app_state, &mut commands);
    draw_drag_and_drop(ui, app_state, texture_cache);
    process_shortcuts(ui, app_state, &mut commands);

    commands
}

fn save_all(app_state: &AppState, commands: &mut CommandBuffer) {
    for document in app_state.documents_iter() {
        commands.save(&document.source, &document.sheet, document.get_version());
    }
}

fn draw_main_menu<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) -> [f32; 2] {
    let mut size = [0.0, 0.0];
    let has_document = app_state.get_current_document().is_some();

    let _style_rounding = ui.push_style_var(WindowRounding(0.0));
    let _style_border = ui.push_style_var(WindowBorderSize(0.0));
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if MenuItem::new("New Sheet…").shortcut("Ctrl+N").build(ui) {
                commands.begin_new_document();
            }
            if MenuItem::new("Open Sheet…").shortcut("Ctrl+O").build(ui) {
                commands.begin_open_document();
            }
            ui.separator();
            if MenuItem::new("Save")
                .shortcut("Ctrl+S")
                .enabled(has_document)
                .build(ui)
            {
                if let Some(document) = app_state.get_current_document() {
                    commands.save(&document.source, &document.sheet, document.get_version());
                }
            }
            if MenuItem::new("Save As…")
                .shortcut("Ctrl+Shift+S")
                .enabled(has_document)
                .build(ui)
            {
                if let Some(document) = app_state.get_current_document() {
                    commands.save_as(&document.source, &document.sheet, document.get_version());
                }
            }
            if MenuItem::new("Save All")
                .shortcut("Ctrl+Alt+S")
                .enabled(has_document)
                .build(ui)
            {
                save_all(app_state, commands);
            }
            if MenuItem::new("Export")
                .shortcut("Ctrl+E")
                .enabled(has_document)
                .build(ui)
            {
                if let Some(document) = app_state.get_current_document() {
                    commands.export(&document.sheet);
                }
            }
            if MenuItem::new("Export As…")
                .shortcut("Ctrl+Shift+E")
                .enabled(has_document)
                .build(ui)
            {
                commands.begin_export_as();
            }
            ui.separator();
            if MenuItem::new("Close")
                .shortcut("Ctrl+W")
                .enabled(has_document)
                .build(ui)
            {
                commands.close_current_document();
            }
            if MenuItem::new("Close All")
                .shortcut("Ctrl+Shift+W")
                .enabled(has_document)
                .build(ui)
            {
                commands.close_all_documents();
            }
        });

        ui.menu("Edit", || {
            let undo_command_name = app_state
                .get_current_document()
                .and_then(|d| d.get_undo_command())
                .and_then(|c| Some(format!("Undo {}", c)));
            if MenuItem::new(&ImString::new(
                undo_command_name.clone().unwrap_or("Undo".to_owned()),
            ))
            .shortcut("Ctrl+Z")
            .enabled(undo_command_name.is_some())
            .build(ui)
            {
                commands.undo();
            }

            let redo_command_name = app_state
                .get_current_document()
                .and_then(|d| d.get_redo_command())
                .and_then(|c| Some(format!("Redo {}", c)));
            if MenuItem::new(&ImString::new(
                redo_command_name.clone().unwrap_or("Redo".to_owned()),
            ))
            .shortcut("Ctrl+Shift+Z")
            .enabled(redo_command_name.is_some())
            .build(ui)
            {
                commands.redo();
            }
        });

        ui.menu("View", || {
            if MenuItem::new("Center Workbench")
                .shortcut("Ctrl+Space")
                .build(ui)
            {
                commands.workbench_center();
            }
            if MenuItem::new("Zoom In (Workbench)")
                .shortcut("Ctrl++")
                .build(ui)
            {
                commands.workbench_zoom_in();
            }
            if MenuItem::new("Zoom Out (Workbench)")
                .shortcut("Ctrl+-")
                .build(ui)
            {
                commands.workbench_zoom_out();
            }
            if MenuItem::new("Reset Zoom (Workbench)")
                .shortcut("Ctrl+0")
                .build(ui)
            {
                commands.workbench_reset_zoom();
            }
            ui.separator();
            if MenuItem::new("Zoom In (Timeline)")
                .shortcut("Ctrl+Alt++")
                .build(ui)
            {
                commands.timeline_zoom_in();
            }
            if MenuItem::new("Zoom Out (Timeline)")
                .shortcut("Ctrl+Alt+-")
                .build(ui)
            {
                commands.timeline_zoom_out();
            }
            if MenuItem::new("Reset Zoom (Timeline)")
                .shortcut("Ctrl+Alt+0")
                .build(ui)
            {
                commands.timeline_reset_zoom();
            }
        });

        size = ui.window_size();
    });

    size
}

fn draw_documents_window<'a>(
    ui: &Ui<'a>,
    rect: &Rect<f32>,
    app_state: &AppState,
    commands: &mut CommandBuffer,
) -> [f32; 2] {
    let mut size = [0.0, 0.0];
    let _style_rounding = ui.push_style_var(WindowRounding(0.0));
    let _style_border = ui.push_style_var(WindowBorderSize(0.0));

    Window::new("Documents")
        .position(rect.origin.to_array(), Condition::Always)
        .always_auto_resize(true)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .menu_bar(false)
        .movable(false)
        .build(ui, || {
            for document in app_state.documents_iter() {
                let mut document_name = document.get_display_name();
                if !document.is_saved() {
                    document_name += " [Modified]";
                }
                if ui.small_button(&ImString::new(document_name)) {
                    commands.focus_document(document);
                }
                ui.same_line();
            }
            size = ui.window_size();
        });

    size
}

fn update_drag_and_drop<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) {
    if let Some(document) = app_state.get_current_document() {
        if !ui.is_mouse_down(MouseButton::Left) {
            match document.transient {
                Some(Transient::ContentFramesDrag) => commands.end_frames_drag(),
                Some(Transient::KeyframeDuration(_)) => commands.end_keyframe_duration_drag(),
                Some(Transient::KeyframePosition(_)) => commands.end_keyframe_offset_drag(),
                Some(Transient::TimelineFrameDrag) => commands.end_keyframe_drag(),
                Some(Transient::HitboxPosition(_)) => commands.end_hitbox_drag(),
                Some(Transient::HitboxSize(_)) => commands.end_hitbox_scale(),
                Some(Transient::TimelineScrub) => commands.end_scrub(),
                Some(Transient::Rename(_)) | None => (),
            }
        }
    }
}

fn draw_drag_and_drop<'a>(ui: &Ui<'a>, app_state: &AppState, texture_cache: &TextureCache) {
    if let Some(document) = app_state.get_current_document() {
        if document.transient == Some(Transient::ContentFramesDrag) {
            if let Some(Selection::Frame(paths)) = &document.view.selection {
                ui.tooltip(|| {
                    let tooltip_size = vec2(128.0, 128.0); // TODO hidpi?
                    let path = &paths.last_touched_in_range;
                    match texture_cache.get(path) {
                        Some(TextureCacheResult::Loaded(texture)) => {
                            if let Some(fill) = utils::fill(tooltip_size, texture.size) {
                                Image::new(texture.id, fill.rect.size.to_array()).build(ui);
                            }
                        }
                        Some(TextureCacheResult::Loading) => {
                            // TODO this doesn't work. Prob need to pad with ui.dummy()
                            spinner::draw_spinner(ui, &ui.get_window_draw_list(), tooltip_size);
                        }
                        _ => {
                            // TODO
                        }
                    }
                    // TODO Draw number of items selected
                });
            }
        }
    }
}

fn draw_export_popup<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) {
    if let Some(document) = app_state.get_current_document() {
        if let Some(settings) = &document.persistent.export_settings_edit {
            let relative_settings = settings.with_relative_paths(&document.source);
            let relative_settings = relative_settings.as_ref().unwrap_or(settings);
            let popup_id = "Export Options";
            Window::new(&popup_id)
                .collapsible(false)
                .resizable(true)
                .always_auto_resize(true)
                .build(ui, || {
                    {
                        let _token = ui.push_id(0);
                        ui.label_text(
                            &ImString::new(relative_settings.texture_destination.to_string_lossy()),
                            "Texture atlas destination:",
                        );
                        ui.same_line();

                        if ui.small_button("Browse…") {
                            commands.begin_set_export_texture_destination(document);
                        }
                    }

                    {
                        let _token = ui.push_id(1);
                        ui.label_text(
                            &ImString::new(
                                relative_settings.metadata_destination.to_string_lossy(),
                            ),
                            "Metadata destination:",
                        );
                        ui.same_line();
                        if ui.small_button("Browse…") {
                            commands.begin_set_export_metadata_destination(document);
                        }
                    }

                    {
                        let _token = ui.push_id(2);
                        ui.label_text(
                            &ImString::new(relative_settings.metadata_paths_root.to_string_lossy()),
                            "Store paths relative to:",
                        );
                        ui.same_line();
                        if ui.small_button("Browse…") {
                            commands.begin_set_export_metadata_paths_root(document);
                        }
                    }

                    {
                        let _token = ui.push_id(3);
                        match &relative_settings.format {
                            ExportFormat::Template(p) => {
                                ui.label_text(&ImString::new(p.to_string_lossy()), "Data Format:");
                                ui.same_line();
                                if ui.small_button("Browse…") {
                                    commands.begin_set_export_format(document);
                                }
                            }
                        };
                    }

                    // TODO grey out and disable if bad settings
                    if ui.small_button("Ok") {
                        commands.end_export_as(&document.sheet, settings.clone());
                    }
                    ui.same_line();
                    if ui.small_button("Cancel") {
                        commands.cancel_export_as();
                    }
                });
        }
    }
}

fn draw_rename_popup<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) {
    if let Some(document) = app_state.get_current_document() {
        if let Some(Transient::Rename(rename)) = &document.transient {
            let popup_id = "Rename";
            // TODO position modal where selectable is
            ui.popup_modal(&popup_id)
                .title_bar(false)
                .resizable(false)
                .always_auto_resize(true)
                .build(ui, || {
                    let mut new_name = rename.new_name.clone();
                    let end_rename = ui
                        .input_text("", &mut new_name)
                        .enter_returns_true(true)
                        .build();
                    commands.update_rename_selection(&new_name);
                    if end_rename {
                        commands.end_rename_selection();
                    }
                });
            ui.open_popup(&popup_id);
        }
    }
}

fn draw_unsaved_changes_popup<'a>(
    ui: &Ui<'a>,
    app_state: &AppState,
    window_size: (f32, f32),
    commands: &mut CommandBuffer,
) {
    if let Some(document) = app_state.get_current_document() {
        match document.persistent.close_state {
            Some(CloseState::Saving) | Some(CloseState::Allowed) | None => (),
            Some(CloseState::Requested) => {
                let popup_id = "Unsaved Changes";
                Window::new(&popup_id)
                    .title_bar(true)
                    .collapsible(false)
                    .resizable(false)
                    .movable(true)
                    .always_auto_resize(true)
                    .position(
                        [window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0],
                        Condition::Always,
                    )
                    .position_pivot([0.5, 0.5])
                    .build(ui, || {
                        let popup_text = format!(
                            "{} has been modified. Would you like to save changes?",
                            document.get_display_name()
                        );
                        ui.text(&ImString::new(popup_text));
                        if ui.small_button("Save") {
                            commands.save(
                                &document.source,
                                &document.sheet,
                                document.get_version(),
                            );
                            commands.close_after_saving();
                        }
                        ui.same_line();
                        if ui.small_button("Don't Save") {
                            commands.close_without_saving();
                        }
                        ui.same_line();
                        if ui.small_button("Cancel") {
                            commands.cancel_close();
                            commands.cancel_exit();
                        }
                    });
            }
        }
    }
}

fn draw_saving_popup<'a>(ui: &Ui<'a>, app_state: &AppState, window_size: (f32, f32)) {
    if let Some(document) = app_state.get_current_document() {
        match document.persistent.close_state {
            Some(CloseState::Requested) | None => (),
            Some(CloseState::Saving) | Some(CloseState::Allowed) => {
                let popup_id = "Saving";
                Window::new(&popup_id)
                    .title_bar(false)
                    .resizable(false)
                    .movable(false)
                    .position(
                        [window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0],
                        Condition::Always,
                    )
                    .position_pivot([0.5, 0.5])
                    .size([80.0, 40.0], Condition::Always)
                    .build(ui, || {
                        ui.set_cursor_pos([0.0, 0.0]);
                        spinner::draw_spinner(
                            ui,
                            &ui.get_window_draw_list(),
                            ui.window_size().into(),
                        );
                    });
            }
        }
    }
}

fn draw_error_popup<'a>(
    ui: &Ui<'a>,
    app_state: &AppState,
    window_size: (f32, f32),
    commands: &mut CommandBuffer,
) {
    match app_state.get_error() {
        None => (),
        Some(error) => {
            let popup_id = "Error";
            Window::new(&popup_id)
                .resizable(false)
                .collapsible(false)
                .always_auto_resize(true)
                .position(
                    [window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0],
                    Condition::Appearing,
                )
                .position_pivot([0.5, 0.5])
                .build(ui, || {
                    ui.text(&ImString::new(format!("{}", error)));
                    if ui.small_button("Ok") {
                        commands.clear_error();
                    }
                });
        }
    }
}

fn process_shortcuts<'a>(ui: &Ui<'a>, app_state: &AppState, commands: &mut CommandBuffer) {
    if ui.io().want_capture_keyboard {
        return;
    }

    // Global shortcuts
    if !ui.io().key_ctrl {
        if ui.is_key_pressed(Key::Delete) {
            commands.delete_selection();
        }
        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::F2 as _) {
            commands.begin_rename_selection();
        }
        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::Space as _) {
            commands.toggle_playback();
        }
    }

    // Arrow shortcuts
    if ui.io().key_ctrl {
        let large_nudge = ui.io().key_shift;
        if ui.is_key_pressed(Key::LeftArrow) {
            commands.nudge_selection_left(large_nudge);
        }
        if ui.is_key_pressed(Key::RightArrow) {
            commands.nudge_selection_right(large_nudge);
        }
        if ui.is_key_pressed(Key::UpArrow) {
            commands.nudge_selection_up(large_nudge);
        }
        if ui.is_key_pressed(Key::DownArrow) {
            commands.nudge_selection_down(large_nudge);
        }
    } else {
        if ui.is_key_pressed(Key::LeftArrow) {
            commands.snap_to_previous_frame();
        }
        if ui.is_key_pressed(Key::RightArrow) {
            commands.snap_to_next_frame();
        }
    }

    // Menu commands
    if ui.io().key_ctrl {
        if ui.is_key_pressed(Key::Z) {
            if ui.io().key_shift {
                commands.redo();
            } else {
                commands.undo();
            }
        }

        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::N as _) {
            commands.begin_new_document();
        }
        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::O as _) {
            commands.begin_open_document();
        }
        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::S as _) {
            if ui.io().key_shift {
                if let Some(document) = app_state.get_current_document() {
                    commands.save_as(&document.source, &document.sheet, document.get_version());
                }
            } else if ui.io().key_alt {
                save_all(app_state, commands);
            } else if let Some(document) = app_state.get_current_document() {
                commands.save(&document.source, &document.sheet, document.get_version());
            }
        }
        if ui.is_key_index_pressed_no_repeat(VirtualKeyCode::E as _) {
            if ui.io().key_shift {
                commands.begin_export_as();
            } else if let Some(document) = app_state.get_current_document() {
                commands.export(&document.sheet);
            }
        }
        if ui.is_key_index_pressed(VirtualKeyCode::W as _) {
            if ui.io().key_shift {
                commands.close_all_documents();
            } else {
                commands.close_current_document();
            }
        }
        if ui.is_key_index_pressed(VirtualKeyCode::NumpadAdd as _)
            || ui.is_key_index_pressed(VirtualKeyCode::Equals as _)
        {
            if ui.io().key_alt {
                commands.timeline_zoom_in();
            } else {
                commands.workbench_zoom_in();
            }
        }
        if ui.is_key_index_pressed(VirtualKeyCode::NumpadSubtract as _)
            || ui.is_key_index_pressed(VirtualKeyCode::Minus as _)
        {
            if ui.io().key_alt {
                commands.timeline_zoom_out();
            } else {
                commands.workbench_zoom_out();
            }
        }
        if ui.is_key_index_pressed(VirtualKeyCode::Key0 as _)
            || ui.is_key_index_pressed(VirtualKeyCode::Numpad0 as _)
        {
            if ui.io().key_alt {
                commands.timeline_reset_zoom();
            } else {
                commands.workbench_reset_zoom();
            }
        }
        if ui.is_key_pressed(Key::Space) {
            commands.workbench_center();
        }
    }
}
