use crate::colors::*;
use crate::ImageViewer;
 
impl ImageViewer {

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context, change_magnify :&mut f32, mouse_zoom: &mut bool){

       // Gyorsbillentyűk figyelése
        if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::ALT | egui::Modifiers::SHIFT,
                egui::Key::C,
            ))
        }) {
            // copy view
            self.save_original = false;
            self.copy_to_clipboard();
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::ALT | egui::Modifiers::SHIFT,
                egui::Key::X,
            ))
        }) {
            // change view
            self.save_original = false;
            self.change_with_clipboard(ctx);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::SHIFT,
                egui::Key::S,
            ))
        }) {
            // save view
            self.save_original = false;
            self.starting_save(&None);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::R,
            ))
        }) {
            // red channel
            self.color_settings.show_r = !self.color_settings.show_r;
            self.review(ctx, true, false);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::G,
            ))
        }) {
            // green channel
            self.color_settings.show_g = !self.color_settings.show_g;
            self.review(ctx, true, false);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::B,
            ))
        }) {
            // blue channel
            self.color_settings.show_b = !self.color_settings.show_b;
            self.review(ctx, true, false);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::I,
            ))
        }) {
            // invert color
            self.color_settings.invert = !self.color_settings.invert;
            self.review(ctx, true, false);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::ArrowUp,
            ))
        }) {
            // rotate 180
            self.color_settings.rotate = self.color_settings.rotate.add(Rotate::Rotate180);
            self.review(ctx, true, false);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::ArrowLeft,
            ))
        }) {
            // rotate -90
            self.color_settings.rotate = self.color_settings.rotate.add(Rotate::Rotate270);
            self.review(ctx,true, true);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::ArrowRight,
            ))
        }) {
            // rotate 90
            self.color_settings.rotate = self.color_settings.rotate.add(Rotate::Rotate90);
            self.review(ctx, true, true);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::ArrowDown,
            ))
        }) {
            // rotate  to 0
            let r = self.color_settings.rotate == Rotate::Rotate90
                || self.color_settings.rotate == Rotate::Rotate270;
            self.color_settings.rotate = Rotate::Rotate0;
            self.review(ctx, true, r);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::ALT,
                egui::Key::C,
            ))
        }) {
            // copy
            // not work with Ctrl
            self.save_original = true;
            self.copy_to_clipboard();
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::ALT,
                egui::Key::V,
            ))
        }) {
            // paste
            // not work with Ctrl
            self.copy_from_clipboard(ctx);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::ALT,
                egui::Key::X,
            ))
        }) {
            // change
            // not work with Ctrl
            self.save_original = true;
            self.change_with_clipboard(ctx);
            
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::C,
            ))
        }) {
            // Színkorrekció
            self.color_correction_dialog = !self.color_correction_dialog;
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::O,
            ))
        }) {
            // open
            self.open_image_dialog(ctx, &None);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::R,
            ))
        }) {
            // reopen
            self.load_image(ctx, true);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::S,
            ))
        }) {
            // save
            self.save_original = true;
            self.starting_save(&None);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::A,
            ))
        }) {
            // recent path ...
            //if !self.config.recent_files.is_empty() {
            self.show_recent_window =
                !self.show_recent_window && !self.config.recent_files.is_empty();
            //}
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::N,
            ))
        }) {
            // next
            self.navigation(ctx, 1);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::D,
            ))
        }) {
            // background rotate
            self.bg_style = self.bg_style.clone().inc();
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::B,
            ))
        }) {
            // before
            self.navigation(ctx, -1);
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::I,
            ))
        }) {
            // info
            self.show_info = !self.show_info;
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::NONE,
                egui::Key::Escape,
            ))
        }) {
            // quit
            if self.color_correction_dialog {
                self.color_correction_dialog = false;
            } else if self.show_info {
                self.show_info = false;
            } else if let Some(_adatok) = &mut self.save_dialog {
                self.save_dialog = None;
            } else if self.show_recent_window {
                self.show_recent_window = false;
            } else if self.show_about_window {
                self.show_about_window = false;
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::Plus,
            ))
        }) {
            // eating default menu text magnify
        } else if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::Minus,
            ))
        }) {
            // eating default menu text magnify
        } else {
            ctx.input(|i| {
                if i.modifiers.command {
                    for event in &i.events {
                        if let egui::Event::MouseWheel { unit: _, delta, .. } = event {
                            *change_magnify = delta.y;
                            if *change_magnify != 0.0 {
                                *mouse_zoom = true;
                            }
                        }
                    }
                } else {
                    // magnify without command and text magnify
                    if i.key_pressed(egui::Key::Plus) {
                        // bigger
                        *change_magnify = 1.0;
                    } else if i.key_pressed(egui::Key::Minus) {
                        // smaller
                        *change_magnify = -1.0;
                    }
                }
            });
        }
    }
}