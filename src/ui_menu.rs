use crate::colors::*;
use crate::file_handlers::*;
use crate::image_processing::*;
use crate::ImageViewer;


impl ImageViewer {

    pub fn draw_main_menu(&mut self, ctx: &egui::Context, _change_magnify: &mut f32, _mouse_zoom: &mut bool) {
        // Menüsor kialakítása
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke::NONE;

            egui::MenuBar::new().ui(ui, |ui: &mut egui::Ui| {
                //ui.push_id("main_menu_scope", |ui| {
                ui.menu_button("Fájl", |ui| {
                    ui.set_min_width(250.0);

                    let open_button =
                        egui::Button::new("Open ...").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::O),
                        ));
                    if ui.add(open_button).clicked() {
                        self.open_image_dialog(ctx, &None);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let reopen_button =
                        egui::Button::new("Reopen").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::R),
                        ));
                    if ui.add(reopen_button).clicked() {
                        self.load_image(ctx, true);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let save_button =
                        egui::Button::new("Save as ...").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::S),
                        ));
                    if ui.add(save_button).clicked() {
                        self.save_original = true;
                        self.starting_save(&None);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let save_button =
                        egui::Button::new("Save view as ...").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::S),
                        ));
                    if ui.add(save_button).clicked() {
                        self.save_original = false;
                        self.starting_save(&None);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let recent_button =
                        egui::Button::new("Recent Paths ...").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::A),
                        ));
                    if ui.add(recent_button).clicked() {
                        if !self.config.recent_files.is_empty() {
                            self.show_recent_window = true;
                        }
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    ui.separator();

                    let copy_button = egui::Button::new("Copy").shortcut_text(ctx.format_shortcut(
                        &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::C),
                    ));
                    if ui.add(copy_button).clicked() {
                        self.save_original = true;
                        self.copy_to_clipboard();
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let copy_button = egui::Button::new("Copy view").shortcut_text(
                        ctx.format_shortcut(&egui::KeyboardShortcut::new(
                            egui::Modifiers::ALT | egui::Modifiers::SHIFT,
                            egui::Key::C,
                        )),
                    );
                    if ui.add(copy_button).clicked() {
                        self.save_original = false;
                        self.copy_to_clipboard();
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let paste_button =
                        egui::Button::new("Paste").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::V),
                        ));
                    if ui.add(paste_button).clicked() {
                        self.copy_from_clipboard(ctx);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let copy_button = egui::Button::new("Change").shortcut_text(ctx.format_shortcut(
                        &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::X),
                    ));
                    if ui.add(copy_button).clicked() {
                        self.save_original = false;
                        self.change_with_clipboard(ctx);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let copy_button = egui::Button::new("Change view").shortcut_text(ctx.format_shortcut(
                        &egui::KeyboardShortcut::new(egui::Modifiers::ALT | egui::Modifiers::SHIFT, egui::Key::X),
                    ));
                    if ui.add(copy_button).clicked() {
                        self.save_original = true;
                        self.change_with_clipboard(ctx);
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    ui.separator();

                    let exit_button = egui::Button::new("Exit").shortcut_text(ctx.format_shortcut(
                        &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Escape),
                    ));
                    if ui.add(exit_button).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    ui.menu_button("Help", |ui| {
                        if ui.button("About IView...").clicked() {
                            self.show_about_window = true;
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                });
                //});
                ui.menu_button("Options", |ui| {
                    ui.menu_button("Order of Images", |ui| {
                        let mut changed = false;
                        if ui
                            .selectable_value(&mut self.sort, SortDir::Name, "by name")
                            .clicked()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.sort, SortDir::Ext, "by  extension")
                            .clicked()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.sort, SortDir::Date, "by date")
                            .clicked()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.sort, SortDir::Size, "by syze")
                            .clicked()
                        {
                            changed = true;
                        }
                        if changed {
                            self.make_image_list(); // Újrarendezzük a listát az új szempont szerint
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });

                    ui.menu_button("Position", |ui| {
                        let mut changed = false;
                        if ui
                            .selectable_value(&mut self.center, false, "Left Up")
                            .clicked()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(&mut self.center, true, "Center")
                            .clicked()
                        {
                            changed = true;
                        }
                        if changed {
                            self.load_image(ctx, false);
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                    ui.menu_button("Channels hide/show", |ui| {
                        let red_button = egui::Button::new(format!(
                            "Red{}",
                            if self.color_settings.show_r {
                                "✔"
                            } else {
                                ""
                            }
                        ))
                        .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                            egui::Modifiers::COMMAND,
                            egui::Key::R,
                        )));
                        if ui.add(red_button).clicked() {
                            self.color_settings.show_r = !self.color_settings.show_r;
                            self.review(ctx, true, false);
                        }

                        let green_button = egui::Button::new(format!(
                            "Green{}",
                            if self.color_settings.show_g {
                                "✔"
                            } else {
                                ""
                            }
                        ))
                        .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                            egui::Modifiers::COMMAND,
                            egui::Key::G,
                        )));
                        if ui.add(green_button).clicked() {
                            self.color_settings.show_g = !self.color_settings.show_g;
                            self.review(ctx, true, false);
                        }

                        let blue_button = egui::Button::new(format!(
                            "Blue{}",
                            if self.color_settings.show_b {
                                "✔"
                            } else {
                                ""
                            }
                        ))
                        .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                            egui::Modifiers::COMMAND,
                            egui::Key::B,
                        )));
                        if ui.add(blue_button).clicked() {
                            self.color_settings.show_b = !self.color_settings.show_b;
                            self.review(ctx, true, false);
                        }

                        let invert_button = egui::Button::new(format!(
                            "Invert{}",
                            if self.color_settings.invert {
                                "✔"
                            } else {
                                ""
                            }
                        ))
                        .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                            egui::Modifiers::COMMAND,
                            egui::Key::I,
                        )));
                        if ui.add(invert_button).clicked() {
                            self.color_settings.invert = !self.color_settings.invert;
                            self.review(ctx, true, false);
                        }
                    });
                    ui.menu_button("Rotate", |ui| {
                        let up_button = egui::Button::new("Up").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::ArrowUp),
                        ));
                        if ui.add(up_button).clicked() {
                            self.color_settings.rotate =
                                self.color_settings.rotate.add(Rotate::Rotate180);
                            self.review(ctx, true, false);
                            ui.close_kind(egui::UiKind::Menu);
                        }

                        let right_button = egui::Button::new("Right").shortcut_text(
                            ctx.format_shortcut(&egui::KeyboardShortcut::new(
                                egui::Modifiers::NONE,
                                egui::Key::ArrowRight,
                            )),
                        );
                        if ui.add(right_button).clicked() {
                            self.color_settings.rotate =
                                self.color_settings.rotate.add(Rotate::Rotate90);
                            self.review(ctx, true, true);
                            ui.close_kind(egui::UiKind::Menu);
                        }

                        let left_button = egui::Button::new("Left").shortcut_text(
                            ctx.format_shortcut(&egui::KeyboardShortcut::new(
                                egui::Modifiers::NONE,
                                egui::Key::ArrowLeft,
                            )),
                        );
                        if ui.add(left_button).clicked() {
                            self.color_settings.rotate =
                                self.color_settings.rotate.add(Rotate::Rotate270);
                            self.review(ctx, true, true);
                            ui.close_kind(egui::UiKind::Menu);
                        }

                        let down_button = egui::Button::new("Stand").shortcut_text(
                            ctx.format_shortcut(&egui::KeyboardShortcut::new(
                                egui::Modifiers::NONE,
                                egui::Key::ArrowDown,
                            )),
                        );
                        if ui.add(down_button).clicked() {
                            let r = self.color_settings.rotate == Rotate::Rotate90
                                || self.color_settings.rotate == Rotate::Rotate270;
                            self.color_settings.rotate = Rotate::Rotate0;
                            self.review(ctx, true, r);
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                    let col_button =
                        egui::Button::new("Color correction").shortcut_text(ctx.format_shortcut(
                            &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::C),
                        ));
                    if ui.add(col_button).clicked() {
                        self.color_correction_dialog = true;
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    if ui
                        .selectable_label(self.refit_reopen, "Refit at Reopen")
                        .clicked()
                    {
                        self.refit_reopen = !self.refit_reopen;
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    if ui
                        .selectable_label(self.use_gpu, "Use Gpu (off at restart)")
                        .clicked()
                    {
                        self.use_gpu = !self.use_gpu;
                        if !self.use_gpu {
                            self.gpu_interface = None;
                        } else {
                            self.gpu_tried_init = false;
                            ctx.request_repaint();
                        }
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    if ui.selectable_label(self.fit_open, "Fit at Open").clicked() {
                        self.fit_open = !self.fit_open;
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    if ui.selectable_label(self.same_correction_open, "No Correction at Open").clicked() {
                        self.same_correction_open = !self.same_correction_open;
                        ui.close_kind(egui::UiKind::Menu);
                    }

                    let info_button = egui::Button::new("Info").shortcut_text(ctx.format_shortcut(
                        &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::I),
                    ));
                    if ui.add(info_button).clicked() {
                        self.show_info = true;
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    ui.menu_button("Background \tD", |ui| {
                        if ui
                            .radio_value(&mut self.bg_style, BackgroundStyle::Black, "Black")
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui
                            .radio_value(&mut self.bg_style, BackgroundStyle::Gray, "Gray")
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui
                            .radio_value(&mut self.bg_style, BackgroundStyle::White, "White")
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui
                            .radio_value(&mut self.bg_style, BackgroundStyle::Green, "Green")
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        ui.separator();
                        if ui
                            .radio_value(
                                &mut self.bg_style,
                                BackgroundStyle::DarkBright,
                                "🏁 DarkBright",
                            )
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui
                            .radio_value(
                                &mut self.bg_style,
                                BackgroundStyle::GreenMagenta,
                                "🏁 GreenMagenta",
                            )
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        if ui
                            .radio_value(
                                &mut self.bg_style,
                                BackgroundStyle::BlackBrown,
                                "🏁 BlackBrown",
                            )
                            .clicked()
                        {
                            ui.close_kind(egui::UiKind::Menu);
                        }
                    });
                    if ui
                        .selectable_label(self.anim_loop, "Animation Loop")
                        .clicked()
                    {
                        self.anim_loop = !self.anim_loop;
                        ui.close_kind(egui::UiKind::Menu);
                    }
                });

                let prev_button = egui::Button::new("<<").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::B),
                ));
                if ui.add(prev_button).clicked() {
                    self.navigation(ctx, -1);
                    ui.close_kind(egui::UiKind::Menu);
                }
                let next_button = egui::Button::new(">>").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::N),
                ));
                if ui.add(next_button).clicked() {
                    self.navigation(ctx, 1);
                    ui.close_kind(egui::UiKind::Menu);
                }
                ui.separator();

                let mut frame_copy: Option<image::DynamicImage> = None;
                if let Some(anim) = &self.anim_data {
                    let play_btn = if self.anim_playing {
                        "⏸ Stop"
                    } else {
                        "▶ Play"
                    };
                    if ui.button(play_btn).clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Space))
                    {
                        self.anim_playing = !self.anim_playing;
                        if self.anim_playing
                            && !self.anim_loop
                            && self.current_frame + 1 == anim.total_frames
                        {
                            self.current_frame = 0;
                        }
                        self.last_frame_time = std::time::Instant::now();
                    }

                    if ui.button("⏮").clicked() {
                        self.current_frame = 0;
                    }

                    // Kézi léptetés (csak ha áll az animáció, vagy bárki nyomogatja)
                    if ui.button("◀").clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowLeft))
                    {
                        self.anim_playing = false;
                        if self.current_frame == 0 {
                            self.current_frame = anim.total_frames - 1;
                        } else {
                            self.current_frame -= 1;
                        }
                        // Textúra frissítése a megjelenítéshez
                        frame_copy = Some(anim.anim_frames[self.current_frame].clone());
                    }

                    if ui.button("▶").clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::ArrowRight))
                    {
                        self.anim_playing = false;
                        self.current_frame = (self.current_frame + 1) % anim.total_frames;
                        // Textúra frissítése a megjelenítéshez
                        frame_copy = Some(anim.anim_frames[self.current_frame].clone());
                        
                    }
                    ui.label(format!(
                        "Frame: {} / {}",
                        self.current_frame + 1,
                        anim.total_frames
                    ));
                }
                if let Some(frame) = frame_copy {
                    self.original_image = Some(frame);
                    self.review(ctx, true, false);
                    ctx.request_repaint();
                }
            });
        });
    }

}