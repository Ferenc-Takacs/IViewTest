/*
iview/src/main.rs

Created by Ferenc Takács in 2026

TODO
    saving gif, és webp animations
    saving resolution
    modularize

*/

// disable terminal window beyond graphic window in release version
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//mod exif;
mod gpu_colors;
mod colors;
mod file_handlers;
mod ui_elements;
mod ui_shortcuts;
mod ui_menu;
mod ui_draw;
mod ui_dialogs;
mod image_processing;
mod exif_my;
use colors::*;
use crate::image_processing::*;
use crate::file_handlers::*;
use crate::exif_my::*;
//use arboard::Clipboard;
use eframe::egui;
//use image::AnimationDecoder;
//use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
//use std::io::{Read, Seek};
use std::path::PathBuf;
//use std::time::SystemTime;
//use webp::Encoder;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (start_image, clipboard) = if args.len() > 1 {
        // Ha van argumentum, azt útvonalként kezeljük
        (Some(PathBuf::from(&args[1])), false)
    } else {
        // 2. Ha nincs, megnézzük a vágólapot (Ctrl+C-vel másolt kép)
        (save_clipboard_image(), true)
    };

    let icon = load_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon) // Itt állítjuk be az ikont
            .with_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "IView",
        options,
        Box::new(|cc| {
            let mut app = ImageViewer::default();
            app.load_settings();
            
            if let Some(path) = start_image {
                if clipboard {
                    // az előző könyvtárt vesszük
                    app.make_image_list()
                }
                app.open_image(&cc.egui_ctx, &path, !clipboard);
            } else {
                app.open_image_dialog(&cc.egui_ctx, &None);
            }
            Ok(Box::new(app))
        }),
    )
}

struct ImageViewer {
    pub image_full_path: Option<PathBuf>, // a kép neve a teljes utvonallal
    pub file_meta: Option<fs::Metadata>,
    pub exif: Option<ExifBlock>,
    pub image_name: String, // kép neve a könyvtár nélkül
    pub image_format: SaveFormat,
    pub image_folder: Option<PathBuf>,     // a képek könyvtára
    pub list_of_images: Vec<fs::DirEntry>, // kép nevek listája a könyvtárban
    pub actual_index: usize,               // a kép indexe a listában
    pub magnify: f32,
    pub resize: f32,
    pub first_appear: u32,
    pub texture: Option<egui::TextureHandle>,
    pub original_image: Option<image::DynamicImage>,
    pub rgba_image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub image_size: egui::Vec2, // beolvasott kép mérete pixelben
    pub center: bool,           // igaz, ha középe tesszük az ablakot, egyébként a bal felső sarokba
    pub show_info: bool,
    pub display_size_netto: egui::Vec2, // a képernyő méretből levonva az ablak keret
    pub frame: egui::Vec2,              // ablak keret
    pub aktualis_offset: egui::Vec2,    // megjelenítés kezdőpozíció a nagyított képen
    pub sort: SortDir,
    pub save_dialog: Option<SaveSettings>,
    pub color_settings: ColorSettings,
    pub settings_dirty: bool, // Jelzi, ha újra kell számolni a LUT-ot
    pub lut: Option<Lut4ColorSettings>,
    pub color_correction_dialog: bool,
    pub show_about_window: bool,
    pub refit_reopen: bool,
    pub fit_open: bool,
    pub same_correction_open: bool,
    pub save_original: bool,
    pub bg_style: BackgroundStyle,
    pub config: AppSettings,
    pub resolution: Option<Resolution>,
    pub recent_file_modified: bool,
    pub recent_window_size: egui::Vec2,
    pub show_recent_window: bool,
    pub show_exif_details: bool,
    pub is_animated: bool,    // Ez a fájl animálható-e?
    pub anim_playing: bool,   // Fut-e most az animáció?
    pub anim_loop: bool,      // Ismétlődjön-e (default: true)?
    pub current_frame: usize, // Hol tartunk?
    pub total_frames: usize,
    pub last_frame_time: std::time::Instant,
    pub anim_data: Option<AnimatedImage>,
    pub show_original_only: bool,
    pub gpu_interface : Option<gpu_colors::GpuInterface>,
    pub gpu_tried_init: bool,
    pub use_gpu: bool,
    pub modified: bool,
    pub menvar: MenuVariables,
}


impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            image_full_path: None,
            file_meta: None,
            exif: None,
            //raw_exif: None,
            image_name: "".to_string(),
            image_format: SaveFormat::Bmp,
            image_folder: None,
            list_of_images: Vec::new(),
            actual_index: 0,
            magnify: 1.0,
            resize: 1.0,
            first_appear: 1,
            texture: None,
            original_image: None,
            rgba_image: None,
            image_size: [800.0, 600.0].into(),
            center: false,
            show_info: false,
            display_size_netto: (0.0, 0.0).into(),
            frame: (0.0, 0.0).into(),
            aktualis_offset: (0.0, 0.0).into(),
            sort: SortDir::Name,
            save_dialog: None,
            color_settings: ColorSettings::default(),
            //image_processor: None,
            settings_dirty: false,
            lut: None,
            color_correction_dialog: false,
            show_about_window: false,
            refit_reopen: false,
            fit_open: true,
            same_correction_open: false,
            save_original: false, //always set before use
            bg_style: BackgroundStyle::DarkBright,
            config: AppSettings::default(),
            resolution: None,
            recent_file_modified: false,
            recent_window_size: (0.0, 0.0).into(),
            show_recent_window: false,
            show_exif_details: false,
            is_animated: false,  // Ez a fájl animálható-e?
            anim_playing: false, // Fut-e most az animáció?
            anim_loop: true,     // Ismétlődjön-e (default: true)?
            current_frame: 0,    // Hol tartunk?
            total_frames: 0,
            last_frame_time: std::time::Instant::now(),
            anim_data: None,
            show_original_only: false,
            gpu_interface : None,
            gpu_tried_init: false,
            use_gpu: true,
            modified: false,
            menvar: MenuVariables::default(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Menu {
    None,
    File,
    Options,
    Recents,
    RecentFile,
    Sort,
    Position,
    Rotate,
    Channels,
    Backgrounds,
    Zoom,
}


pub struct MenuVariables {
    pub recentfile: PathBuf,
    pub recentidx_last: usize,
    pub recentidx_curr: usize,
    pub recentidx_parm: usize,
    pub last_closed_time: f64,
    pub current_menu : Menu,
    pub last_menu : Menu,
    pub menu_pos:           egui::Pos2,
    pub file_menu_pos:      egui::Pos2,
    pub options_menu_pos:   egui::Pos2,
    pub recents_menu_pos:   egui::Pos2,
    pub recentfile_menu_pos: egui::Pos2,
    pub sort_menu_pos:      egui::Pos2,
    pub position_menu_pos:  egui::Pos2,
    pub rotate_menu_pos:    egui::Pos2,
    pub channels_menu_pos:  egui::Pos2,
    pub background_menu_pos: egui::Pos2,
    pub zoom_menu_pos:      egui::Pos2,
}

impl Default for MenuVariables {
    fn default() -> Self {
        Self {
            recentfile: PathBuf::default(),
            recentidx_last: 1000,
            recentidx_curr: 1000,
            recentidx_parm: 1000,
            last_closed_time: 0.0,
            current_menu : Menu::None,
            last_menu : Menu::None,
            menu_pos:           (0.0,0.0).into(),
            file_menu_pos:      (0.0,0.0).into(),
            options_menu_pos:   (0.0,0.0).into(),
            recents_menu_pos:   (0.0,0.0).into(),
            recentfile_menu_pos: (0.0,0.0).into(),
            sort_menu_pos:      (0.0,0.0).into(),
            position_menu_pos:  (0.0,0.0).into(),
            rotate_menu_pos:    (0.0,0.0).into(),
            channels_menu_pos:  (0.0,0.0).into(),
            background_menu_pos: (0.0,0.0).into(),
            zoom_menu_pos:      (0.0,0.0).into(),
        }
    }
}



impl eframe::App for ImageViewer {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        self.anim_and_gpu(ctx, frame);
        
        let mut change_magnify = 0.0;
        let mut mouse_zoom = false;
        
        self.handle_shortcuts(ctx, &mut change_magnify, &mut mouse_zoom);
        
        self.draw_main_menu(ctx, &mut change_magnify, &mut mouse_zoom);
        
        let dropped_file = ctx.input_mut(|i| {
            if !i.raw.dropped_files.is_empty() {
                let files = std::mem::take(&mut i.raw.dropped_files);
                files.first().and_then(|f| f.path.clone())
            } else {
                None
            }
        });
        if let Some(path) = dropped_file {
            self.open_image(ctx, &path.to_path_buf(), true);
            //println!("Fájl behúzva: {:?}", path);
        }
        
        self.draw_image_area(ctx, &mut change_magnify, &mut mouse_zoom);
        
        self.dialogs(ctx);
        
    }

    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_settings();
    }
}
