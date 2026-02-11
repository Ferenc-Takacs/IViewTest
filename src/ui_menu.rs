use crate::colors::*;
use crate::file_handlers::*;
use crate::image_processing::*;
use crate::ImageViewer;
// use egui::WindowLevel;
use crate::Menu;
 use crate::MenuVariables;
 
impl MenuVariables {
    
    pub fn change_menu(&mut self, ctx: &egui::Context, menu: Menu ) -> bool {
        if self.last_menu == menu && menu != Menu::None &&
            ctx.input(|i| i.time) - self.last_closed_time < 0.18
        { return false; } // disable fast repeat (previous lose focus by click)
        self.current_menu = menu;
        self.last_menu = menu;
        true
    }

    pub fn menu_is_active(&mut self, ctx: &egui::Context, menu: Menu ) -> bool {
        if menu == self.current_menu {
            if self.in_focus(ctx,menu) { return true; }
            self.current_menu = Menu::None;
            self.last_closed_time = ctx.input(|i| i.time);
            return false;
        }
        return self.is_in_root(menu); 
    }

    pub fn in_focus(&mut self, ctx: &egui::Context, _menu: Menu ) -> bool {
        if ctx.input(|i| i.viewport().focused.unwrap_or(true)) { return true; }
        /*match menu {
            Menu::File          => ,
            Menu::Options       => ,
            Menu::Recents       => ,
            Menu::RecentFile    => ,
            Menu::Sort          => ,
            Menu::Position      => ,
            Menu::Rotate        => ,
            Menu::Channels      => ,
            Menu::Backgrounds   => ,
            Menu::Zoom          => ,
            _                   => ,
        }*/
        false
    }

    pub fn is_in_root(&mut self, menu: Menu ) -> bool {
        if self.current_menu == menu { return true; }
        match menu {
            Menu::File          => self.current_menu == Menu::Recents || self.current_menu == Menu::RecentFile,
            Menu::Options       => self.current_menu != Menu::Recents && self.current_menu != Menu::RecentFile,
            Menu::Recents       => self.current_menu == Menu::RecentFile,
            Menu::RecentFile    => false,
            Menu::Sort          => false,
            Menu::Position      => false,
            Menu::Rotate        => false,
            Menu::Channels      => false,
            Menu::Backgrounds   => false,
            Menu::Zoom          => false,
            _                   => true,
        }
    }

}

impl ImageViewer {

    pub fn show_menu_viewport<F>(
        &mut self,
        menvar: &mut MenuVariables,
        ctx: &egui::Context, 
        menu: Menu, 
        add_contents: F
    ) 
    where F: FnMut(&mut egui::Ui) 
    {
        // Csak akkor csinálunk bármit, ha a menü aktív
        if !menvar.menu_is_active(ctx, menu) {
            return;
        }

        let (id_str, pos) = match menu {
            Menu::File          => ("file_menu_viewport",       menvar.file_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Options       => ("options_menu_viewport",    menvar.options_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Recents       => ("recents_menu_viewport",    menvar.recents_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::RecentFile    => ("recentfile_menu_viewport", menvar.recentfile_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Sort          => ("sort_menu_viewport",       menvar.sort_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Position      => ("position_menu_viewport",   menvar.position_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Rotate        => ("rotate_menu_viewport",     menvar.rotate_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Channels      => ("channels_menu_viewport",   menvar.channels_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Backgrounds   => ("background_menu_viewport", menvar.background_menu_pos + menvar.menu_pos.to_vec2()),
            Menu::Zoom          => ("zoom_menu_viewport",       menvar.zoom_menu_pos + menvar.menu_pos.to_vec2()),
            _                   => ("menu_viewport",            menvar.menu_pos),
        };

        let mut add_contents_ref = add_contents;
        
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of(id_str),
            egui::ViewportBuilder::default()
                .with_position(pos)
                .with_always_on_top()
                .with_inner_size([400.0, 400.0]) // A méretet majd a tartalom állítja
                .with_decorations(false),
            |ctx, _| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if menu == Menu::None {
                        ui.horizontal(|ui| {
                            // Itt hívjuk meg a kívülről kapott tartalomépítő kódot
                            add_contents_ref(ui);
                            // Automatikus méretezés a tartalomhoz
                            let desired_size = ui.min_size();
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired_size));
                        });
                    }
                    else {
                        ui.vertical(|ui| {
                            // Itt hívjuk meg a kívülről kapott tartalomépítő kódot
                            add_contents_ref(ui);
                            // Automatikus méretezés a tartalomhoz
                            let desired_size = ui.min_size();
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired_size));
                        });
                    }
                });
            }
        );
    }

    pub fn draw_main_menu(&mut self, ctx: &egui::Context, _change_magnify: &mut f32, _mouse_zoom: &mut bool) {
        // Menüsor kialakítása

        //let main_window_focused = ctx.input(|i| i.viewport().focused.unwrap_or(true));
        
        let mut menvar = self.menvar.clone();

        menvar.menu_pos = ctx.input(|i| {
            let main_window_rect = i.viewport().outer_rect.unwrap_or(egui::Rect::EVERYTHING);
            main_window_rect.min + egui::vec2(8.0, 31.0)
        });

        egui::TopBottomPanel::top("menu_placeholder").show(ctx, |ui| {
            ui.set_height(20.0); // Pontosan akkora, mint a menüd lesz
        });

        // Főmenü
        self.show_menu_viewport(&mut menvar, ctx, Menu::None, |ui| {
            let file_btn = ui.button("File");
            if file_btn.clicked() {
                menvar.file_menu_pos = file_btn.rect.left_bottom();
                menvar.change_menu(ctx,Menu::File) ;
            }

            let options_btn = ui.button("Options");
            if options_btn.clicked() {
                menvar.options_menu_pos = options_btn.rect.left_bottom();
                menvar.change_menu(ctx,Menu::Options);
            }

            let prev_button = egui::Button::new("<<").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::B),
            ));
            
            if ui.add(prev_button).clicked() {
                menvar.change_menu(ctx,Menu::None);
                self.navigation(ctx, -1);
                ui.close_kind(egui::UiKind::Menu);
            }
            let next_button = egui::Button::new(">>").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::N),
            ));
            if ui.add(next_button).clicked() {
                menvar.change_menu(ctx,Menu::None);
                self.navigation(ctx, 1);
                ui.close_kind(egui::UiKind::Menu);
            }
            ui.separator();

            let mut frame_copy: Option<usize> = None;
            if self.anim_data.is_some() {
                let play_btn = if self.anim_playing {
                    "⏸ Stop"
                } else {
                    "▶ Play"
                };
                if ui.button(play_btn).clicked()
                    || ui.input(|i| i.key_pressed(egui::Key::Space))
                {
                    menvar.change_menu(ctx,Menu::None);
                    self.anim_playing = !self.anim_playing;
                    if self.anim_playing
                        && !self.anim_loop
                        && self.current_frame + 1 == self.total_frames
                    {
                        self.current_frame = 0;
                    }
                    self.last_frame_time = std::time::Instant::now();
                }

                if ui.button("⏮").clicked() {
                    menvar.change_menu(ctx,Menu::None);
                    self.current_frame = 0;
                }

                // Kézi léptetés (csak ha áll az animáció, vagy bárki nyomogatja)
                if ui.button("◀").clicked() || ui.input(|i| i.key_pressed(egui::Key::ArrowLeft))
                {
                    menvar.change_menu(ctx,Menu::None);
                    self.anim_playing = false;
                    if self.current_frame == 0 {
                        self.current_frame = self.total_frames - 1;
                    } else {
                        self.current_frame -= 1;
                    }
                    // Textúra frissítése a megjelenítéshez
                    frame_copy = Some(self.current_frame);
                }

                if ui.button("▶").clicked()
                    || ui.input(|i| i.key_pressed(egui::Key::ArrowRight))
                {
                    menvar.change_menu(ctx,Menu::None);
                    self.anim_playing = false;
                    self.current_frame = (self.current_frame + 1) % self.total_frames;
                    // Textúra frissítése a megjelenítéshez
                    frame_copy = Some(self.current_frame);
                    
                }
                ui.label(format!(
                    "Frame: {} / {}",
                    self.current_frame + 1,
                    self.total_frames
                ));
            }
            if let Some(frame) = frame_copy {
                if let Some(anim) = &self.anim_data {
                    self.original_image = Some(anim.anim_frames[frame].clone());
                    self.review(ctx, true, false);
                    ctx.request_repaint();
                }
            }
        });

        // File menü
        self.show_menu_viewport(&mut menvar, ctx,Menu::File, |ui| {
            let open_button =
                egui::Button::new("Open ...").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::O),
                ));
            if ui.add(open_button).clicked() {
                self.open_image_dialog(ctx, &None);
                menvar.change_menu(ctx,Menu::None);
            }

            let reopen_button =
                egui::Button::new("Reopen").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::R),
                ));
            if ui.add(reopen_button).clicked() {
                self.load_image(ctx, true);
                menvar.change_menu(ctx,Menu::None);
            }

            let save_button =
                egui::Button::new("Save as ...").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::S),
                ));
            if ui.add(save_button).clicked() {
                self.save_original = true;
                self.starting_save(&None);
                menvar.change_menu(ctx,Menu::None);
            }

            let save_button =
                egui::Button::new("Save view as ...").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::SHIFT, egui::Key::S),
                ));
            if ui.add(save_button).clicked() {
                self.save_original = false;
                self.starting_save(&None);
                menvar.change_menu(ctx,Menu::None);
            }
            
            let recents_btn = ui.button("Recent Paths ...");
            if recents_btn.clicked() {
                menvar.recents_menu_pos = recents_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Recents);
            }

            ui.separator();

            let copy_button = egui::Button::new("Copy").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::C),
            ));
            if ui.add(copy_button).clicked() {
                self.save_original = true;
                self.copy_to_clipboard();
                menvar.change_menu(ctx,Menu::None);
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
                menvar.change_menu(ctx,Menu::None);
            }

            let paste_button =
                egui::Button::new("Paste").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::V),
                ));
            if ui.add(paste_button).clicked() {
                self.copy_from_clipboard(ctx);
                menvar.change_menu(ctx,Menu::None);
            }

            let copy_button = egui::Button::new("Change").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::X),
            ));
            if ui.add(copy_button).clicked() {
                self.save_original = false;
                self.change_with_clipboard(ctx);
                menvar.change_menu(ctx,Menu::None);
            }

            let copy_button = egui::Button::new("Change view").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::ALT | egui::Modifiers::SHIFT, egui::Key::X),
            ));
            if ui.add(copy_button).clicked() {
                self.save_original = true;
                self.change_with_clipboard(ctx);
                menvar.change_menu(ctx,Menu::None);
            }

            ui.separator();

            let exit_button = egui::Button::new("Exit").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Escape),
            ));
            if ui.add(exit_button).clicked() {
                menvar.change_menu(ctx,Menu::None);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            if ui.button("About IView...").clicked() {
                menvar.change_menu(ctx,Menu::None);
                self.show_about_window = true;
            }

        });

        // Recent paths list
        self.show_menu_viewport(&mut menvar, ctx,Menu::Recents, |ui| {
           let files: Vec<_> = self.config.recent_files.iter().cloned().collect();
           for path in files {
                let file_name = path.file_name().map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                let folder_path = path.parent().map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "Root".to_string());
                let button = ui.button(&*file_name);
                button.clone().on_hover_text(folder_path);
                if button.clicked() {
                    menvar.recentfile_menu_pos = button.rect.right_top();
                    menvar.change_menu(ctx,Menu::RecentFile);
                }
            }
        });
        

        // Recent file options
        self.show_menu_viewport(&mut menvar, ctx,Menu::RecentFile, |ui| {
            if ui.button("Open file").clicked() {
                self.open_image(ctx, &menvar.recentfile, true);
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.button("Open File Here").clicked() {
                self.open_image_dialog(ctx, &Some(menvar.recentfile.clone()));
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.button("Save Here").clicked() {
                self.save_original = true;
                self.starting_save(&Some(menvar.recentfile.clone()));
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.button("Save View Here ").clicked() {
                self.save_original = false;
                self.starting_save(&Some(menvar.recentfile.clone()));
                menvar.change_menu(ctx,Menu::None);
            }
        });
        
        
        // Options menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Options, |ui| {
            let sort_btn = ui.button("Order of images");
            if sort_btn.clicked() {
                menvar.sort_menu_pos = sort_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Sort);
            }
            let position_btn = ui.button("Window position");
            if position_btn.clicked() {
                menvar.position_menu_pos = position_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Position);
            }
            let rotate_btn = ui.button("Rotate");
            if rotate_btn.clicked() {
                menvar.rotate_menu_pos = rotate_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Rotate);
            }
            let background_btn = ui.button("Background");
            if background_btn.clicked() {
                menvar.background_menu_pos = background_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Backgrounds);
            }
            let channels_btn = ui.button("Channels");
            if channels_btn.clicked() {
                menvar.channels_menu_pos = channels_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Channels);
            }
            let zoom_btn = ui.button("Zoom");
            if zoom_btn.clicked() {
                menvar.zoom_menu_pos = zoom_btn.rect.right_top();
                menvar.change_menu(ctx,Menu::Zoom);
            }
            let col_button = egui::Button::new("Color correction").shortcut_text(ctx.format_shortcut(
                    &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::C),
                ));
            if ui.add(col_button).clicked() {
                self.color_correction_dialog = true;
                menvar.change_menu(ctx,Menu::None);
            }

            if ui.selectable_label(self.refit_reopen, "Refit at Reopen").clicked()
            {
                self.refit_reopen = !self.refit_reopen;
                menvar.change_menu(ctx,Menu::None);
            }

            if ui.selectable_label(self.use_gpu, "Use Gpu (off at restart)").clicked()
            {
                self.use_gpu = !self.use_gpu;
                if !self.use_gpu {
                    self.gpu_interface = None;
                } else {
                    self.gpu_tried_init = false;
                    ctx.request_repaint();
                }
                menvar.change_menu(ctx,Menu::None);
            }

            if ui.selectable_label(self.fit_open, "Fit at Open").clicked() {
                self.fit_open = !self.fit_open;
                menvar.change_menu(ctx,Menu::None);
            }

            if ui.selectable_label(self.same_correction_open, "No Correction at Open").clicked() {
                self.same_correction_open = !self.same_correction_open;
                menvar.change_menu(ctx,Menu::None);
            }

            let info_button = egui::Button::new("Info").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::I),
            ));
            if ui.add(info_button).clicked() {
                self.show_info = true;
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.selectable_label(self.anim_loop, "Animation Loop").clicked()
            {
                self.anim_loop = !self.anim_loop;
                menvar.change_menu(ctx,Menu::None);
            }
        });

        // sort menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Sort, |ui| {
            let mut changed = false;
            if ui.selectable_value(&mut self.sort, SortDir::Name, "by name").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut self.sort, SortDir::Ext, "by  extension").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut self.sort, SortDir::Date, "by date").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut self.sort, SortDir::Size, "by syze").clicked() {
                changed = true;
            }
            if changed {
                self.make_image_list(); // Újrarendezzük a listát az új szempont szerint
                menvar.change_menu(ctx,Menu::None);
            }
        });
        

        // position menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Position, |ui| {
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
                menvar.change_menu(ctx,Menu::None);
            }
        });

        // channels menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Channels, |ui| {
            let red_button = egui::Button::new(format!( "Red{}",
                if self.color_settings.show_r { "✔" } else { "" }
            ))
            .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::R,
            )));
            if ui.add(red_button).clicked() {
                self.color_settings.show_r = !self.color_settings.show_r;
                self.review(ctx, true, false);
                menvar.change_menu(ctx,Menu::None);
            }

            let green_button = egui::Button::new(format!( "Green{}",
                if self.color_settings.show_g { "✔" } else { "" }
            ))
            .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::G,
            )));
            if ui.add(green_button).clicked() {
                self.color_settings.show_g = !self.color_settings.show_g;
                self.review(ctx, true, false);
                menvar.change_menu(ctx,Menu::None);
            }

            let blue_button = egui::Button::new(format!( "Blue{}",
                if self.color_settings.show_b { "✔" } else { "" }
            ))
            .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::B,
            )));
            if ui.add(blue_button).clicked() {
                self.color_settings.show_b = !self.color_settings.show_b;
                self.review(ctx, true, false);
                menvar.change_menu(ctx,Menu::None);
            }

            let invert_button = egui::Button::new(format!( "Invert{}",
                if self.color_settings.invert { "✔" } else { "" }
            ))
            .shortcut_text(ctx.format_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::I,
            )));
            if ui.add(invert_button).clicked() {
                self.color_settings.invert = !self.color_settings.invert;
                self.review(ctx, true, false);
                menvar.change_menu(ctx,Menu::None);
            }
        });

        // rotate menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Rotate, |ui| {
            let up_button = egui::Button::new("Up").shortcut_text(ctx.format_shortcut(
                &egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::ArrowUp),
            ));
            if ui.add(up_button).clicked() {
                self.color_settings.rotate =
                    self.color_settings.rotate.add(Rotate::Rotate180);
                self.review(ctx, true, false);
                menvar.change_menu(ctx,Menu::None);
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
                menvar.change_menu(ctx,Menu::None);
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
                menvar.change_menu(ctx,Menu::None);
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
                menvar.change_menu(ctx,Menu::None);
            }
        });

        // rotate menu
        self.show_menu_viewport(&mut menvar, ctx,Menu::Backgrounds, |ui| {
            if ui.radio_value(&mut self.bg_style, BackgroundStyle::Black, "Black").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.radio_value(&mut self.bg_style, BackgroundStyle::Gray, "Gray").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.radio_value(&mut self.bg_style, BackgroundStyle::White, "White").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.radio_value(&mut self.bg_style, BackgroundStyle::Green, "Green").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            ui.separator();
            if ui.radio_value(&mut self.bg_style,BackgroundStyle::DarkBright,"🏁 DarkBright").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.radio_value(&mut self.bg_style,BackgroundStyle::GreenMagenta,"🏁 GreenMagenta").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
            if ui.radio_value(&mut self.bg_style,BackgroundStyle::BlackBrown,"🏁 BlackBrown").clicked()
            {
                menvar.change_menu(ctx,Menu::None);
            }
        });
        
        self.menvar = menvar;

    }

}