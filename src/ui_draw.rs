use crate::image_processing::*;
use crate::ui_elements::*;
use crate::ImageViewer;


impl ImageViewer {

    pub fn draw_image_area(&mut self, ctx: &egui::Context, change_magnify: &mut f32, mouse_zoom: &mut bool){
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.inner_margin(0.0)) // Margók eltüntetése
            .show(ctx, |ui| {
                let mut in_w;
                let mut in_h;
                let old_size = self.image_size * self.magnify;
                if self.first_appear > 0 {
                    if self.first_appear == 1 {
                        let outer_size = ctx.input(|i| i.viewport().outer_rect.unwrap().size());
                        let inner_size = ctx.input(|i| i.viewport_rect().size());
                        self.frame = outer_size - inner_size;
                        self.frame.y += 30.0;
                        self.display_size_netto =
                            ctx.input(|i| i.viewport().monitor_size.unwrap()) - self.frame;
                        //println!("out:{:?} in:{:?} frame:{:?} netto:{:?}", outer_size,inner_size,self.frame,self.display_size_netto);
                    }
                    let ratio = (self.display_size_netto) / self.image_size;
                    self.magnify = ratio.x.min(ratio.y);

                    if let Some(_) = &self.texture {
                    } else {
                        self.magnify /= 2.0;
                    }

                    let round_ = if self.magnify < 1.0 { 0.0 } else { 0.5 };
                    self.magnify = (((self.magnify * 20.0 + round_) as i32) as f32) / 20.0; // round
                    in_w = (self.image_size.x * self.magnify).min(self.display_size_netto.x);
                    in_h = (self.image_size.y * self.magnify).min(self.display_size_netto.y);
                    let inner_size = egui::Vec2 {
                        x: in_w + 4.0,
                        y: in_h + 26.0,
                    };
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(inner_size));
                    if self.first_appear > 0 {
                        let pos = if self.center {
                            egui::pos2(
                                (self.display_size_netto.x - in_w) / 2.0 - 8.0,
                                (self.display_size_netto.y - in_h) / 2.0 - 10.0,
                            )
                        } else {
                            egui::pos2(-8.0, 0.0)
                        };
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                        //println!("inner:{:?} pos:{:?} magn: {:?}", inner_size, pos, self.magnify);
                    }
                }

                let mut zoom = 1.0;
                if *change_magnify != 0.0 {
                    let regi_nagyitas = self.magnify;
                    if self.magnify >= 1.0 {
                        *change_magnify *= 2.0;
                    }
                    if self.magnify >= 4.0 {
                        *change_magnify *= 2.0;
                    }
                    self.magnify = (regi_nagyitas * 1.0 + (0.05 * *change_magnify)).clamp(0.1, 10.0);
                    self.magnify = (((self.magnify * 100.0 + 0.5) as i32) as f32) / 100.0; // round

                    if self.magnify != regi_nagyitas {
                        zoom = self.magnify / regi_nagyitas;
                        in_w = (self.image_size.x * self.magnify).min(self.display_size_netto.x);
                        in_h = (self.image_size.y * self.magnify).min(self.display_size_netto.y);
                        let inner_size = egui::Vec2 {
                            x: in_w + 4.0,
                            y: in_h + 26.0,
                        };
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(inner_size));
                        let pos = if self.center {
                            egui::pos2(
                                (self.display_size_netto.x - in_w) / 2.0,
                                (self.display_size_netto.y - in_h) / 2.0,
                            )
                        } else {
                            egui::pos2(0.0, 0.0)
                        };
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                        //println!("inner:{:?} pos:{:?} magn: {:?}", inner_size, pos, self.magnify);
                    }
                }

                if let Some(tex) = self.texture.as_ref() { // CPU
                    let magnify = self.magnify;
                    let image_size = self.image_size;
                    let display_size_netto = self.display_size_netto;
                    let current_offset = self.aktualis_offset;
                    let title = format!( "IView - {}. {}  {}", self.actual_index, self.image_name, self.magnify );
                    
                    let output = egui::Frame::canvas(ui.style())
                        .fill(egui::Color32::TRANSPARENT)
                        .show(ui, |ui| {

                            draw_custom_background(ui, &self.bg_style);

                            let new_size = image_size * magnify;
                            let scroll_id = ui.make_persistent_id("kep_scroll");
                            let mut off = egui::Vec2 { x: 0.0, y: 0.0 };

                            if zoom != 1.0 || self.first_appear > 0 {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));

                                let ui_rect = ui.max_rect();
                                let inside = ui_rect.max - ui_rect.min;

                                let mut pointer = if *mouse_zoom {
                                    // mouse position
                                    if let Some(p) = ctx.pointer_latest_pos() {
                                        p - ui_rect.min
                                    } else {
                                        inside / 2.0
                                    }
                                } else {
                                    inside / 2.0
                                }; // image center
                                pointer.x = pointer.x.clamp(0.0, old_size.x);
                                pointer.y = pointer.y.clamp(0.0, old_size.y);

                                let mut offset = current_offset;
                                offset += pointer;
                                offset *= zoom;
                                offset -= pointer;

                                if new_size.x > display_size_netto.x {
                                    // need horizontal scrollbar
                                    off.x = offset.x;
                                }
                                if new_size.y > display_size_netto.y {
                                    // need vertical scrollbar
                                    off.y = offset.y;
                                }
                                if self.first_appear > 0 {
                                    //println!("p:{:?} c_of:{:?} o_of:{:?} o_si:{:?} n_si{:?} in:{:?} mag:{}",
                                    //    pointer, current_offset, off, old_size, new_size, inside, self.magnify);
                                    //println!();
                                }
                            }
                            let mut scroll_area = egui::ScrollArea::both()
                                .id_salt(scroll_id)
                                .auto_shrink([false; 2]);

                            if zoom != 1.0 {
                                scroll_area = scroll_area
                                    .vertical_scroll_offset(off.y)
                                    .horizontal_scroll_offset(off.x);
                            }


                            let scroll_output  = scroll_area.show(ui, |ui2| {
                                ui2.add(egui::Image::from_texture(tex).fit_to_exact_size(new_size));
                            });
                            scroll_output
                        }).inner;
                        
                    self.aktualis_offset = output.state.offset;

                    // Csak akkor fut le, ha a Ctrl ÉS a Shift le van nyomva
                    if ctx.input(|i| i.modifiers.ctrl || (i.modifiers.ctrl && i.modifiers.shift)) {
                        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                            let inner_rect = output.inner_rect;
                            if inner_rect.contains(pointer_pos) {
                                let relative_pos = pointer_pos - inner_rect.min + output.state.offset;
                                let pixel_x = (relative_pos.x / self.magnify) as u32;
                                let pixel_y = (relative_pos.y / self.magnify) as u32;

                                if let Some(color) = self.pick_color(pixel_x, pixel_y) {
                                    let tooltip_id = egui::Id::new("pixel_info");
                                    #[allow(deprecated)]     
                                    egui::show_tooltip_at(
                                        ctx,
                                        ui.layer_id(),
                                        tooltip_id,
                                        ctx.pointer_latest_pos().unwrap_or(egui::Pos2::ZERO) + egui::vec2(16.0, 16.0),
                                        |ui: &mut egui::Ui| {
                                            ui.horizontal(|ui: &mut egui::Ui| {
                                                ui.label(format!("Pos: {}, {} ", pixel_x, pixel_y));
                                                let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                                                ui.painter().rect_filled(rect, 2.0, color);
                                                ui.label(format!("Rgb: {}, {}, {}", color.r(), color.g(), color.b()));
                                            });
                                        }
                                    );
                                }
                            }
                        }
                    }

                } else {
                    draw_custom_background(ui, &self.bg_style);

                    ui.vertical_centered(|ui| {
                        ui.add_space(ui.max_rect().height() / 3.0); // Kicsit feljebb a közepénél

                        label_with_shadow(ui, "IView - No image opened", 24.0);

                        ui.add_space(10.0);
                        label_with_shadow(
                            ui,
                            "Drag & Drop a file or select it from the File menu!",
                            20.0,
                        );

                        if !self.config.recent_files.is_empty() {
                            ui.add_space(20.0);
                            label_with_shadow(ui, "Choose from the latests", 20.0);
                            // Itt akár listázhatod is a legutóbbi 3-at gombként...
                        }
                    });
                }
                self.first_appear = 0;
            });
    }

}