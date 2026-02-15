use crate::colors::*;
use crate::file_handlers::*;
//use crate::image_processing::*;
use crate::ImageViewer;


impl ImageViewer {

    pub fn dialogs(&mut self, ctx: &egui::Context){
        let mut pending_action = None;

        if self.show_recent_window {
            let screen_pos = ctx.input(|i| {
                let main_window_rect = i.viewport().outer_rect.unwrap_or(egui::Rect::EVERYTHING);
                main_window_rect.min + egui::vec2(5.0, 57.0)
            });

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("recent_files_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("IView - Recent Paths")
                    .with_position(screen_pos)
                    .with_minimize_button(false)
                    .with_maximize_button(false)
                    .with_resizable(false)
                    .with_inner_size([50.0, 50.0]),
                //.with_always_on_top(), // Legyen a főablak felett
                |ctx, _class| {
                    if ctx
                        .input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::A))
                    {
                        self.show_recent_window = false;
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        let is_new_window = ctx.viewport_rect().width() <= 51.0;

                        ui.vertical(|ui| {
                            let mut action = None; // (ActionType, PathBuf)

                            for path in &self.config.recent_files {
                                let file_name = path
                                    .file_name()
                                    .map(|n| n.to_string_lossy())
                                    .unwrap_or_default();
                                let folder_path = path
                                    .parent()
                                    .map(|p| p.to_string_lossy().into_owned())
                                    .unwrap_or_else(|| "Root".to_string());
                                let hover_msg = format!(
                                    "{}\n                   Clicks:\n\
                                    Left:              -> Open\n\
                                    Shift Left:    -> Open as ...\n\
                                    Right:           -> Save as ...\n\
                                    Shift+Right: -> Save View as ...",
                                    folder_path
                                );
                                let button = ui.button(&*file_name);
                                button.clone().on_hover_text(hover_msg);
                                if button.secondary_clicked() {
                                    if ui.input(|i| i.modifiers.shift || i.modifiers.command) {
                                        action = Some(("SAVEVIEW_DIAL", path.clone()));
                                    } else {
                                        action = Some(("SAVE_DIAL", path.clone()));
                                    }
                                    ui.close_kind(egui::UiKind::Menu);
                                }
                                if button.clicked() {
                                    if ui.input(|i| i.modifiers.shift || i.modifiers.command) {
                                        action = Some(("OPEN_DIAL", path.clone()));
                                    } else {
                                        action = Some(("OPEN", path.clone()));
                                    }
                                    ui.close_kind(egui::UiKind::Menu);
                                }
                            }

                            if let Some(act) = action {
                                pending_action = Some(act);
                                self.show_recent_window = false;
                            }
                        });

                        if self.recent_file_modified || is_new_window {
                            self.recent_window_size = ui.min_size().into();
                            if self.recent_window_size.x > 1.0 {
                                self.recent_file_modified = false;
                                //println!("{:?}",self.recent_window_size);
                                let new_size = egui::vec2(
                                    self.recent_window_size.x + 15.0,
                                    self.recent_window_size.y + 15.0,
                                );
                                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(new_size));
                            }
                        }
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_recent_window = false;
                    }
                },
            );
        }

        // Műveletek végrehajtása
        if let Some((type_str, path)) = pending_action {
            match type_str {
                "OPEN" => self.open_image(ctx, &path, true),
                "OPEN_DIAL" => self.open_image_dialog(ctx, &Some(path)),
                "SAVE_DIAL" => {
                    self.save_original = true;
                    self.starting_save(&Some(path));
                }
                "SAVEVIEW_DIAL" => {
                    self.save_original = false;
                    self.starting_save(&Some(path));
                }
                _ => {}
            }
        }

        if self.show_about_window {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("about_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("About IView")
                    .with_inner_size([350.0, 550.0])
                    .with_resizable(false)
                    .with_minimize_button(false)
                    .with_maximize_button(false)
                    .with_always_on_top(),
                |ctx, _class| {
                    // Bezárás Esc-re
                    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_about_window = false;
                    }

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.heading(egui::RichText::new("IView 2026").size(30.0).strong());
                            ui.label("The high-speed Rust image viewer");
                            ui.label("Version: 0.4.0");
                            ui.separator();

                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Fejlesztette:").strong());
                            ui.label("Ferenc Takács"); // Ide írd be a neved

                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("AI Contributor:").strong());
                            ui.label("Google Gemini (Pro)");

                            ui.add_space(20.0);
                            ui.label(egui::RichText::new("Technologies used:").strong());
                        });

                        ui.add_space(10.0);
                        egui::ScrollArea::vertical()
                            .max_height(250.0)
                            .show(ui, |ui| {
                                ui.group(|ui| {
                                    ui.set_width(320.0); // Fix szélesség, hogy a csoport maga is középen legyen
                                    ui.vertical_centered(|ui| {
                                        ui.label("• egui & eframe (0.30) - Graphical interface");
                                        ui.label("• image (0.25) - Image decoding and animation");
                                        ui.label("• tiff (0.9) - Precision metadata management");
                                        ui.label("• png (0.17) - Chunk level analysis");
                                        ui.label("• kamadak-exif - EXIF database");
                                        ui.label("• rfd - Native file dialogs");
                                        ui.label("• serde - Configuration backup");
                                    });
                                });
                            });

                        ui.add_space(20.0);
                        ui.vertical_centered(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_about_window = false;
                            }
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Made in Rust, for speed.")
                                    .italics()
                                    .size(10.0),
                            );
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_about_window = false;
                    }
                },
            );
        }

        if let Some(save_data) = &mut self.save_dialog {
            let mut need_save = false;
            let mut cancel_save = false;
            // modal(true) blokkolja az alatta lévő felületet
            egui::Window::new("Save Settings")
                .collapsible(false)
                .resizable(false)
                .pivot(egui::Align2::CENTER_CENTER) // Középre tesszük
                .default_pos(ctx.viewport_rect().center())
                .show(ctx, |ui| {
                    match save_data.saveformat {
                        SaveFormat::Jpeg => {
                            ui.add(
                                egui::Slider::new(&mut save_data.quality, 1..=100)
                                    .text("Quality (JPEG)"),
                            );
                        }
                        SaveFormat::Webp => {
                            ui.checkbox(&mut save_data.lossless, "Lossless Compression");
                            if !save_data.lossless {
                                ui.add(
                                    egui::Slider::new(&mut save_data.quality, 1..=100)
                                        .text("Quality (WebP)"),
                                );
                            }
                        }
                        _ => {}
                    }
                    if save_data.can_include_exif {
                        if let Some(exif) = self.exif.clone() {
                            if save_data.saveformat != SaveFormat::Bmp && save_data.saveformat != SaveFormat::Png {
                                ui.separator();
                            }
                            let txt = format!("📝 Include EXIF metadata (+ {} bytes) ",exif.raw_exif_length);
                            ui.checkbox(&mut save_data.include_exif, txt);
                        }
                    }
                    ui.add_space(10.0);
                    
                    if save_data.is_animation {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Animation detected:").strong());
                            ui.radio_value(&mut save_data.save_all_frames, false, "Current Frame Only");
                            ui.radio_value(&mut save_data.save_all_frames, true, "Full Animation");
                        });
                    }

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            need_save = true;
                        }
                        if ui.button("❌ Cancel").clicked() {
                            cancel_save = true;
                        }
                    });
                });
            if cancel_save {
                self.save_dialog = None;
            } else if need_save {
                self.completing_save(); // Ez belül állítja None-ra a save_dialog-ot
            }
        }

        if self.show_info {
            egui::Window::new("Image Info")
                .open(&mut self.show_info) // Bezáró gomb (X) kezelése
                .show(ctx, |ui| {
                    egui::Grid::new("info_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0]) // Oszlopok közötti távolság
                        .striped(true) // Sávos festés a jobb olvashatóságért
                        .show(ui, |ui| {
                            ui.label("Name of file:");
                            ui.label(self.image_name.clone());
                            ui.end_row();

                            ui.label("Size of image:");
                            ui.label(format!(
                                "{} x {} pixel",
                                self.image_size.x, self.image_size.y
                            ));
                            ui.end_row();

                            // Fájlméret és dátum lekérése
                            if let Some(meta) = &self.file_meta {
                                ui.label("Size of file:");
                                let mut s = format!("{}", meta.len()).to_string();
                                let l = s.len();
                                if l > 3 {
                                    s = format!(
                                        "{} {}",
                                        s[..l - 3].to_string(),
                                        s[l - 3..].to_string()
                                    );
                                }
                                if l > 6 {
                                    s = format!(
                                        "{} {}",
                                        s[..l - 6].to_string(),
                                        s[l - 6..].to_string()
                                    );
                                }
                                if l > 9 {
                                    s = format!(
                                        "{} {}",
                                        s[..l - 9].to_string(),
                                        s[l - 9..].to_string()
                                    );
                                }
                                ui.label(format!("{} Byte", s));
                                ui.end_row();
                                if let Ok(time) = meta.created() {
                                    ui.label("Time of file:");
                                    let ts = time_format::from_system_time(time).unwrap();
                                    let c = time_format::components_utc(ts).unwrap();
                                    ui.label(format!(
                                        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                                        c.year, c.month, c.month_day, c.hour, c.min, c.sec
                                    ));
                                    ui.end_row();
                                }
                            }

                            // EXIF save_data kiírása (Dátum, Gépmodell)
                            if let Some(resol) = &self.resolution {
                                let x_res = resol.xres;
                                let y_res = resol.yres;
                                let dpi = resol.dpi;
                                let x_val = x_res.to_string();
                                let y_val = y_res.to_string();
                                ui.label("Resolution:");
                                let unit_str = if dpi { "dpi" } else { "dpcm" };
                                if x_val == y_val {
                                    ui.label(format!("{} {}", x_val, unit_str));
                                } else {
                                    ui.label(format!("{}x{} {}", x_val, y_val, unit_str));
                                }
                                ui.end_row();
                            }

                            if let Some(exif) = &self.exif {
                                if let Some(f) = exif.get_field("DateTimeOriginal".into())
                                {
                                    ui.label("Created:");
                                    ui.label(f/*.display_value().to_string()*/);
                                    ui.end_row();
                                }
                                if let Some(f) = exif.get_field("Model".into())
                                {
                                    ui.label("Machine:");
                                    ui.label(f/*.display_value().to_string()*/);
                                    ui.end_row();
                                }

                                let la = exif .get_num_field("GPSLatitude".into());
                                    //.and_then(exif_to_decimal);
                                let lo = exif.get_num_field("GPSLongitude".into());
                                    //.and_then(exif_to_decimal);
                                let lar = exif.get_field("GPSLatitudeRef".into());
                                let lor = exif.get_field("GPSLongitudeRef".into());

                                if let (Some(mut la_), Some(mut lo_), Some(lar_), Some(lor_), ) = (la, lo, lar, lor) {
                                    // S (Dél) és W (Nyugat) esetén negatív előjel
                                    if lar_.contains('S') {
                                        la_ = -la_;
                                    }
                                    if lor_.contains('W') {
                                        lo_ = -lo_;
                                    }
                                    ui.label("GeoLocation:");
                                    let koord_szoveg = format!("{:.6}, {:.6}", la_, lo_);
                                    ui.label(&koord_szoveg);
                                    ui.end_row();

                                    ui.label("Map:");
                                    let map_url = format!(
                                        "https://www.google.com/maps/place/{:.6},{:.6}",
                                        la_, lo_
                                    );
                                    if ui.link("Open in browser 🌍").clicked() {
                                        if let Err(e) = webbrowser::open(&map_url) {
                                            eprintln!("Can not open the Browser: {}", e);
                                        }
                                    }
                                    ui.end_row();
                                }
                            }
                        });
                        if self.exif.is_some() {
                            ui.separator();
                            if ui.button("EXIF adatok részletezése").clicked() {
                                self.show_exif_details = !self.show_exif_details;
                            }

                            if self.show_exif_details {
                                egui::ScrollArea::vertical()
                                    .max_height(300.0) // Korlátozzuk a magasságot, hogy ne nyúljon túl
                                    .show(ui, |ui| {
                                        ui.group(|ui| {
                                            if let Some(exif) = &self.exif {
                                                for (name, val, _off) in exif.fields() {
                                                    ui.horizontal(|ui| {
                                                        // Tag neve (pl. "Make", "DateTime")
                                                        ui.label(egui::RichText::new(format!("{}:", name.to_string())).strong());
                                                        if let Some(v) = val.get("val") {
                                                            ui.label(v.to_string());
                                                        }
                                                        else { ui.label(val.to_string()); }
                                                    });
                                                }
                                            }
                                        });
                                    });
                            }
                        }
                });
        }

        if self.color_correction_dialog {
            let mut changed = false;
            //let mut dialog_copy = self.color_correction_dialog;
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("colorcorrection_viewport"),
                egui::ViewportBuilder::default()
                .with_title("Color Correction for iView")
                .with_inner_size([440.0, 350.0])
                .with_resizable(false)
                //.with_minimize_button(false)
                .with_maximize_button(false)
                .with_always_on_top(),
                |ctx, _| {
                if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::C)) {
                    self.color_correction_dialog = false;
                    ctx.send_viewport_cmd_to( egui::ViewportId::ROOT, egui::ViewportCommand::Focus );
                }
                egui::CentralPanel::default()
                .frame(egui::Frame::default().fill(ctx.style().visuals.window_fill()).inner_margin(2.0))
                .show( ctx, |ui| {
                    
            /*let mut changed = false;
            let mut dialog_copy = self.color_correction_dialog;
            egui::Window::new("Color corrections")
            .open(&mut dialog_copy) // Bezáró gomb (X) kezelése
            .resizable(false)
            .show(ctx, |ui| {*/
                ui.spacing_mut().slider_width = 300.0;

                 ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Global Corrections").strong());
                    ui.add_space(135.0); 
                    ui.label("Channels:");
                        ui.style_mut().spacing.item_spacing.x = 2.0; // Szorosabb gombok
                        if ui.selectable_label(self.color_settings.invert, " INV ").clicked() {
                            self.color_settings.invert = !self.color_settings.invert;
                            changed = true;
                        }
                        ui.add_space(10.0);
                        let r_btn = ui.selectable_label(self.color_settings.show_r, " R ");
                        if r_btn.clicked() {
                            self.color_settings.show_r = !self.color_settings.show_r;
                            changed = true;
                        }
                        let g_btn = ui.selectable_label(self.color_settings.show_g, " G ");
                        if g_btn.clicked() {
                            self.color_settings.show_g = !self.color_settings.show_g;
                            changed = true;
                        }
                        let b_btn = ui.selectable_label(self.color_settings.show_b, " B ");
                        if b_btn.clicked() {
                            self.color_settings.show_b = !self.color_settings.show_b;
                            changed = true;
                        }
                });

                let gam = ui.add(egui::Slider::new(
                    &mut self.color_settings.gamma, 0.1..=3.0)
                    .text("Gamma"));
                if self.gpu_interface.is_none() {
                    if gam.drag_stopped() || (gam.changed() && !ui.input(|i| i.pointer.any_down())) {
                        changed = true;
                    }
                }
                else {
                    if gam.changed() {
                        changed = true;
                    }
                }
                let con = ui.add(egui::Slider::new(
                    &mut self.color_settings.contrast, -1.0..=1.0)
                    .text("Contrast"));
                if self.gpu_interface.is_none() {
                    if con.drag_stopped() || (con.changed() && !ui.input(|i| i.pointer.any_down())) {
                        changed = true;
                    }
                }
                else {
                    if con.changed() {
                        changed = true;
                    }
                }

                // --- HSV (Színvilág) ---
                //ui.label(egui::RichText::new("Hsv/Oklab Color Shift").strong());
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Color Correction Algorithm: ").strong());
                        if ui.radio_value(&mut self.color_settings.oklab, true, "Oklab (Natural)").clicked() {
                            changed = true;
                        }
                        if ui.radio_value(&mut self.color_settings.oklab, false, "HSV (Classic)").clicked() {
                            changed = true;
                        }
                    });
                    let hue = ui.add(egui::Slider::new(
                        &mut self.color_settings.hue_shift, -180.0..=180.0)
                        .text("Hue Shift"));
                    if self.gpu_interface.is_none() {
                        if hue.drag_stopped() || (hue.changed() && !ui.input(|i| i.pointer.any_down())) {
                            changed = true;
                        }
                    }
                    else {
                        if hue.changed() {
                            changed = true;
                        }
                    }
                    let sat = ui.add(egui::Slider::new(
                        &mut self.color_settings.saturation, -1.0..=1.0)
                        .text("Saturation"));
                    if self.gpu_interface.is_none() {
                        if sat.drag_stopped() || (sat.changed() && !ui.input(|i| i.pointer.any_down())) {
                            changed = true;
                        }
                    }
                    else {
                        if sat.changed() {
                            changed = true;
                        }
                    }
                    let bri = ui.add(egui::Slider::new(
                        &mut self.color_settings.brightness, -1.0..=1.0)
                        .text("Brightness"));
                    if self.gpu_interface.is_none() {
                        if bri.drag_stopped() || (bri.changed() && !ui.input(|i| i.pointer.any_down())) {
                            changed = true;
                        }
                    }
                    else {
                        if bri.changed() {
                            changed = true;
                        }
                    }

                });                

                // --- Élesítés / Blur (GPU előkészítés) ---
                ui.label(egui::RichText::new("Sharpen (Amount > 0) & Blur (Amount < 0)").strong());
                ui.horizontal(|ui| {
                    let res = ui.add(egui::Slider::new(
                        &mut self.color_settings.sharpen_amount, -1.0..=9.0)
                        .text("Amount"));
                    if self.gpu_interface.is_none() {
                        if res.drag_stopped() || (res.changed() && !ui.input(|i| i.pointer.any_down())) {
                            changed = true;
                        }
                    }
                    else {
                        if res.changed() {
                            changed = true;
                        }
                    }
                    if ui.button("⟲").on_hover_text("Reset Amount").clicked() {
                        self.color_settings.sharpen_amount = 0.0;
                        changed = true;
                    }
                });
            
                ui.horizontal(|ui| {
                    let res = ui.add(egui::Slider::new(
                        &mut self.color_settings.sharpen_radius, 0.2..=7.0)
                        .text("Radius"));
                    if self.gpu_interface.is_none() {
                        if res.drag_stopped() || (res.changed() && !ui.input(|i| i.pointer.any_down())) {
                            changed = true;
                        }
                    }
                    else {
                        if res.changed() {
                            changed = true;
                        }
                    }
                    if ui.button("⟲").on_hover_text("Reset Radius").clicked() {
                        self.color_settings.sharpen_radius = 0.2;
                        changed = true;
                    }
                });


                ui.group(|ui| {
                    if ui.checkbox(&mut self.color_settings.use_transparency, "Use transparency color").changed() {
                        changed = true;
                    };
                    ui.horizontal(|ui| {
                        let res = ui.add(egui::Slider::new(
                            &mut self.color_settings.transparency_tolerance, 0.0..=1.0)
                            .text("Tolerance"));
                        if self.gpu_interface.is_none() {
                            if res.drag_stopped() || (res.changed() && !ui.input(|i| i.pointer.any_down())) {
                                changed = true;
                            }
                        }
                        else {
                            if res.changed() {
                                changed = true;
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Transparent Color ");
                        let color = self.color_settings.transparent_color;
                        
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(26.0, 16.0), egui::Sense::hover());
                        let col = egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], 255);
                        ui.painter().rect_filled(rect, 2.0, col);
                        //ui.painter().frame(rect,1.0);
                        
                        ui.label("Red:");
                        let mut r_txt = format!("{}",color[0]);
                        let r_res = ui.add(egui::TextEdit::singleline(&mut r_txt).desired_width(30.0));
                        if r_res.changed() {
                            self.color_settings.transparent_color[0] = r_txt.parse::<u8>().unwrap_or(color[0]);
                            changed=true;
                        }
                        ui.label("Green:");
                        let mut g_txt = format!("{}",color[1]);
                        let g_res = ui.add(egui::TextEdit::singleline(&mut g_txt).desired_width(30.0));
                        if g_res.changed() {
                            self.color_settings.transparent_color[1] = g_txt.parse::<u8>().unwrap_or(color[1]);
                            changed=true;
                        }
                        ui.label("Blue:");
                        let mut b_txt = format!("{}",color[2]);
                        let b_res = ui.add(egui::TextEdit::singleline(&mut b_txt).desired_width(30.0));
                        if b_res.changed() {
                            self.color_settings.transparent_color[2] = b_txt.parse::<u8>().unwrap_or(color[2]);
                            changed=true;
                        }
                    });
                });

                

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Reset All Settings").clicked() {
                        self.color_settings = ColorSettings::default();
                        changed = true;
                    }
                    ui.add_space(160.0); 

                    let btn = ui.add(egui::Button::new("Show Original (Shift+Alt)"));
                    
                    let keys_active = (btn.contains_pointer() && ui.input(|i| i.pointer.any_down())) 
                            || ctx.input(|i| i.modifiers.shift && i.modifiers.alt);
                       
                    if (keys_active && !self.show_original_only) || (!keys_active && self.show_original_only) {
                        self.show_original_only = keys_active;
                        changed = true;
                    }
                       
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.color_correction_dialog = false;
                    ctx.send_viewport_cmd_to( egui::ViewportId::ROOT, egui::ViewportCommand::Focus );
                }
            });
            });
            if changed {
                self.settings_dirty = true;
                self.review(ctx, true, false);
            }
            //self.color_correction_dialog = dialog_copy;
        }
        
        self.menvar.after_all_menus(ctx, self.act());

    }

}