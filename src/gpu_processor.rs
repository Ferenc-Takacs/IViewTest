//use eframe::egui;
use eframe::wgpu;
use wgpu::util::DeviceExt;
use crate::ColorSettings;
use std::sync::Arc;
use std::sync::Mutex;
//use image::DynamicImage;
const _: &str = include_str!("image_processor.wgsl");

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuParams {
    pub amount: f32,
    pub radius: f32,
    pub lut_weight: f32,
    pub enabled_mask: u32, // bit 0: blur/sharp, bit 1: lut
}

impl GpuParams {
    pub fn new(amount: f32, radius: f32, lut_weight: f32, use_bs: bool, use_lut: bool) -> Self {
        let mut mask = 0u32;
        if use_bs { mask |= 1; }
        if use_lut { mask |= 2; }
        
        Self {
            amount,
            radius,
            lut_weight,
            enabled_mask: mask,
        }
    }

    pub fn from_color_settings(cs: &ColorSettings) -> Self {
        let mut mask = 0u32;
        // Ha van élesítés vagy elmosás
        if cs.sharpen_amount != 0.0 { mask |= 1; }
        // A LUT-ot szinte mindig bekapcsoljuk, ha van színkorrekció
        mask |= 2;

        Self {
            amount: cs.sharpen_amount,
            radius: cs.sharpen_radius,
            lut_weight: 1.0,
            enabled_mask: mask,
        }
    }
}
///////////////////////////////////////////////////////////////////////////////////


pub fn create_identity_lut_image() -> Vec<u8> {
    let size = 33;
    // 33 darab 33x33-as blokk egymás mellett: szélesség = 33 * 33 = 1089
    let width = size * size; 
    let height = size;
    let mut data = vec![0u8; width * height * 4];

    for z in 0..size { // Kék rétegek (0..32)
        for y in 0..size { // Zöld (függőleges)
            for x in 0..size { // Piros (vízszintes a blokkon belül)
                let r = (x as f32 / (size - 1) as f32 * 255.0) as u8;
                let g = (y as f32 / (size - 1) as f32 * 255.0) as u8;
                let b = (z as f32 / (size - 1) as f32 * 255.0) as u8;

                // Kiszámoljuk a pixel helyét a 1089x33-as képen
                let px = (z * size + x) * 4;
                let py = y * width * 4;
                let idx = py + px;

                data[idx] = r;
                data[idx + 1] = g;
                data[idx + 2] = b;
                data[idx + 3] = 255;
            }
        }
    }
    data
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorSettings {
    gamma: f32,
    contrast: f32,
    brightness: f32,
    hue_shift: f32,
    saturation: f32,
    invert: u32,       // u32-t használunk bool helyett a shader kompatibilitás miatt
    show_r: u32,
    show_g: u32,
    show_b: u32,
    _padding: [u32; 3], // A 16 bájtos igazítás miatt érdemes kitölteni
}

fn work_stream () {
    
    // colorsetting buffer
    let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Color Settings Uniform Buffer"),
        size: std::mem::size_of::<ColorSettings>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    // ... a bind_group többi része (texture_view-k) után:
    wgpu::BindGroupEntry {
        binding: 2,
        resource: params_buffer.as_entire_binding(),
    },


    // referencia kép létrehozása, és betöltése.
    let lut_size = 33;
    let identity_data = create_identity_lut_image();

    let identity_lut_texture = device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some("Identity_LUT_Base"),
            size: wgpu::Extent3d {
                width: lut_size * lut_size, // 1089
                height: lut_size,           // 33
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &identity_data,
    );

    // konvertált kép buffer allokálása
    let processed_lut_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Processed_LUT_Result"),
        size: wgpu::Extent3d {
            width: 1089,
            height: 33,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    
    // program 
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("LUT Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("lut_processor.wgsl").into()),
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("LUT Compute Pipeline"),
        layout: None, // Automatikus layout
        module: &shader,
        entry_point: Some("main"), // 2026-ban ez már lehet Option
        compilation_options: Default::default(),
        cache: None,
    });

    // Bind Group a két textúrához és a bufferekhez
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &compute_pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&identity_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&processed_view) },
            wgpu::BindGroupEntry { binding: 2, resource: params_buffer.as_entire_binding() },
        ],
    });

}

fn csuszka() {
    
    // Ezt hívd meg a compute_pass indítása előtt:
    queue.write_buffer(
        &params_buffer, 
        0, 
        bytemuck::bytes_of(colset) // A 'colset' a ColorSettings példányod
    );
    
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { 
            label: Some("LUT Calculation"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        
        // 1089x33 pixel lefedése 16x16-os munkacsoportokkal
        // (1089/16 felfelé kerekítve = 69, 33/16 felfelé kerekítve = 3)
        compute_pass.dispatch_workgroups(69, 3, 1);
    }
    queue.submit(Some(encoder.finish()));
}

fn aaa(){
    // encoder.begin_compute_pass(...) befejezése után:
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &processed_lut_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width as u32),
                rows_per_image: Some(height as u32),
            },
        },
        wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(Some(encoder.finish()));
}

pub fn download_lut(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<u8> {
    let buffer_slice = buffer.slice(..);

    // 1. Kérjük a hozzáférést (aszinkron)
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });

    // 2. Megvárjuk, amíg a GPU végez (Maintain::Wait kényszeríti a befejezést)
    device.poll(wgpu::Maintain::Wait);

    if let Ok(Ok(_)) = rx.recv() {
        // 3. Adatok másolása a Rust-ba
        let data = buffer_slice.get_mapped_range().to_vec();
        
        // 4. Fontos: unmap-olni kell a buffert, hogy a GPU újra használhassa!
        drop(data); // Előbb dobjuk a nézetet
        buffer.unmap();
        
        return buffer_slice.get_mapped_range().to_vec(); // Visszaadjuk az adatokat
    }
    vec![]
}

pub fn update_animation_frame(queue: &wgpu::Queue, texture: &wgpu::Texture, new_frame: &egui::ColorImage) {
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        new_frame.as_raw(),
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * new_frame.width() as u32),
            rows_per_image: Some(new_frame.height() as u32),
        },
        wgpu::Extent3d {
            width: new_frame.width() as u32,
            height: new_frame.height() as u32,
            depth_or_array_layers: 1,
        },
    );
}

CPU: Módosítod a csúszkát.
GPU: Compute Shader pillanatok alatt átszámolja a 1089x33-as táblát.
GPU: copy_texture_to_buffer átteszi az eredményt a staging bufferbe.
CPU: A download_lut függvénnyel kiveszed a friss bájtokat.
App: Ezt a Vec<u8>-at használod fel a fő kép megjelenítéséhez (mint friss 3D LUT).

    // Az update() függvényben:
    if is_animation_playing && last_frame_elapsed {
        let next_frame = animation.get_next_frame();
        
        // 1. Feltöltjük az új kockát a GPU-ra
        update_animation_frame(&render_state.queue, &self.main_image_texture, &next_frame);
        
        // 2. Ha változtak a színek is, frissítjük a LUT-ot a Compute Shaderrel
        if self.settings_dirty {
            self.run_lut_compute_pass(render_state); 
            self.settings_dirty = false;
        }
        
        // 3. Kérjük az egui-t, hogy rajzoljon (repaint), így látszik az új frame
        ctx.request_repaint();
    }


///////////////////////////////////////////////////////////////////
// Ez villámgyors: csak egy uniform buffert ír és elindítja a compute shadert
queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&gpu_settings));

let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
{
    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
    compute_pass.set_pipeline(&self.lut_compute_pipeline);
    compute_pass.set_bind_group(0, &self.lut_bind_group, &[]);
    // 33x33x33 rácspont feldolgozása (munkacsoportokra osztva)
    compute_pass.dispatch_workgroups(33, 33, 33);
}
queue.submit(Some(encoder.finish()));



pub fn process_to_cpu(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    processor: &ImageProcessor,
    source_view: &wgpu::TextureView,
    width: u32,
    height: u32,
) -> Vec<u8> {
    // 1. Létrehozunk egy céltextúrát a GPU-n
    let target_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    // 2. Létrehozunk egy Buffert, amibe a GPU bele tudja másolni az eredményt (olvasható a CPU-nak)
    let u32_size = std::mem::size_of::<u32>() as u32;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: (u32_size * width * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // 3. Renderelés a láthatatlan textúrára
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let view = target_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        
        rp.set_pipeline(&processor.pipeline);
        // Itt frissíteni kell a bind group-ot a forrás képpel (source_view)
        // ... set_bind_group ...
        rp.draw(0..3, 0..1);
    }

    // 4. MÁSOLÁS: Textúra -> Buffer
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &target_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );

    queue.submit(Some(encoder.finish()));

    // 5. Buffer kiolvasása (szinkronizálás a CPU-val)
    let buffer_slice = output_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait); // Megvárjuk a GPU-t

    let data = buffer_slice.get_mapped_range().to_vec();
    data // Itt a nyers RGBA pixelhalmaz!
}

pub struct LutGenerator {
    pub pipeline: wgpu::RenderPipeline,
    pub identity_texture: wgpu::Texture, // A fix 2D báziskép
    pub target_lut_texture: wgpu::Texture, // Ebbe rajzol a GPU
}

impl LutGenerator {
    pub fn update(&self, device: &wgpu::Device, queue: &wgpu::Queue, settings: &ColorSettings) {
        // 1. Paraméterek feltöltése a GPU-ra
        queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(settings));

        // 2. Render Pass indítása a 'target_lut_texture'-re
        // ... (standard wgpu render pass) ...
        
        // Ezután a 'target_lut_texture' már a friss, 
        // korrigált színeket tartalmazza 3D vagy 2D formában.
    }
}

///////////////////////////////////////////////////////////////////////////////////

/*fn calculate_2d_coords_from_3d(r: usize, g: usize, b: usize) -> ( usize , usize ) {
    let size = 33;
    let grid_width = 6; // 6 blokk széles a 2D kép (6 * 33 = 198 pixel)
    
    // b (kék) határozza meg, melyik 33x33-as blokkban vagyunk
    let block_x = b % grid_width;
    let block_y = b / grid_width;
    
    // A pontos pixel koordináta a 2D képen belül
    ( block_x * size + r, block_y * size + g)
}*/



pub struct ImageProcessor {
    pub pipeline: wgpu::RenderPipeline,
    pub params_buffer: wgpu::Buffer,
    pub lut_texture: wgpu::Texture,
    pub lut_texture_view: wgpu::TextureView,
    pub lut_sampler: wgpu::Sampler,
    pub bind_group_layout: wgpu::BindGroupLayout,
    // Ezt a bind groupot újra kell építeni, ha a kép textúrája változik
    pub main_bind_group: Mutex<Option<wgpu::BindGroup>>, 
}

impl ImageProcessor {
    
    pub fn new( device: &wgpu::Device, queue: &wgpu::Queue, surface_format: wgpu::TextureFormat ) -> Self {
        // 1. Shader betöltése (beágyazzuk a binárisba, hogy ne kelljen külön fájl)
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Image Processor Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("image_processor.wgsl").into()),
        });

        // 2. Bind Group Layout (amit korábban beszéltünk - 5 entry)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Processor Layout"),
            entries: &[
                // 0: Main Texture (2D), 1: Main Sampler, 2: Params (Uniform), 3: LUT (3D), 4: LUT Sampler
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D3, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });

        // 3. Pipeline Layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Processor Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // 4. A tényleges Pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Image Processor Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // Kell egy vertex shader is a WGSL-be!
                buffers: &[], // Ha a vertexeket a shaderben generáljuk
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // 5. Alapértelmezett erőforrások
        let (lut_texture, lut_view) = Self::create_identity_lut(device, queue);
        let lut_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Params Buffer"),
            size: std::mem::size_of::<GpuParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            params_buffer,
            lut_texture,
            lut_texture_view: lut_view,
            lut_sampler,
            bind_group_layout,
            main_bind_group: None.into(),
        }
    }

    pub fn update_bind_group(
        &self, 
        device: &wgpu::Device, 
        image_view: &wgpu::TextureView, 
        image_sampler: &wgpu::Sampler
    ) {
        let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Update Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(image_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(image_sampler) }, // Ez legyen a képé
                wgpu::BindGroupEntry { binding: 2, resource: self.params_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&self.lut_texture_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.lut_sampler) },
            ],
        });

        let mut mg = self.main_bind_group.lock().unwrap();
        *mg = Some(new_bind_group);
        
    }

    // A te 6x33-as rácsaidat alakítja át 33x33x33-as 3D textúrává
    pub fn create_lut_3d(device: &wgpu::Device, queue: &wgpu::Queue, rgba_data: &[u8]) -> wgpu::Texture {
        let size = 33;
        // Itt történik a 2D rács "kicsomagolása" a GPU által várt 3D folyamba
        // A kód feltételezi a 6x33-as elrendezést
        let mut data_3d = Vec::with_capacity(size * size * size * 4);
        
        for z in 0..size { // Kék
            for y in 0..size { // Zöld
                for x in 0..size { // Piros
                    // Itt számítjuk ki az (x,y,z) helyét a te 2D képeden
                    // A te egyedi indexed alapján:
                    let pixel = get_pixel_from_6x33_grid(rgba_data, x, y, z);
                    data_3d.extend_from_slice(&pixel);
                }
            }
        }

        device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("3D_LUT"),
                size: wgpu::Extent3d { width: size as u32, height: size as u32, depth_or_array_layers: size as u32 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &data_3d
        )
    }
    
    pub fn create_identity_lut(device: &wgpu::Device, queue: &wgpu::Queue) -> (wgpu::Texture, wgpu::TextureView) {
        let size = 33;
        let mut data = Vec::with_capacity(size * size * size * 4);

        for b in 0..size {
            for g in 0..size {
                for r in 0..size {
                    // Minden ponton a koordináta értéke a szín értéke
                    let r_val = (r as f32 / (size - 1) as f32 * 255.0) as u8;
                    let g_val = (g as f32 / (size - 1) as f32 * 255.0) as u8;
                    let b_val = (b as f32 / (size - 1) as f32 * 255.0) as u8;
                    
                    data.push(r_val);
                    data.push(g_val);
                    data.push(b_val);
                    data.push(255); // Alpha
                }
            }
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Identity 3D LUT"),
            size: wgpu::Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: size as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size as u32),
                rows_per_image: Some(size as u32),
            },
            wgpu::Extent3d {
                width: size as u32,
                height: size as u32,
                depth_or_array_layers: size as u32,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
        // Mindkettőt visszaadjuk
        (texture, view)
    }

    pub fn update_lut(&self, queue: &wgpu::Queue, lut_texture: &wgpu::Texture, new_data: &[u8]) {
        let size = 33;
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: lut_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            new_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size),
                rows_per_image: Some(size),
            },
            wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: size,
            },
        );
    }

}

 
// Segédfüggvény a 6x33-as rács leképezéséhez
fn get_pixel_from_6x33_grid(data: &[u8], r: usize, g: usize, b: usize) -> [u8; 4] {
    let size = 33;
    let grid_width = 6; // 6 blokk széles a 2D kép (6 * 33 = 198 pixel)
    
    // b (kék) határozza meg, melyik 33x33-as blokkban vagyunk
    let block_x = b % grid_width;
    let block_y = b / grid_width;
    
    // A pontos pixel koordináta a 2D képen belül
    let x = block_x * size + r;
    let y = block_y * size + g;
    
    // A 2D kép teljes szélessége (pixelben)
    let total_width = grid_width * size; 
    
    let idx = (y * total_width + x) * 4;
    
    if idx + 3 < data.len() {
        [data[idx], data[idx+1], data[idx+2], data[idx+3]]
    } else {
        [0, 0, 0, 255] // Biztonsági fallback
    }
}

/*pub fn upload_33_lut(device: &wgpu::Device, queue: &wgpu::Queue, lut_image: &DynamicImage) -> wgpu::Texture {
    let size = 33;
    let mut data_3d = vec![0u8; size * size * size * 4]; // RGBA adatok

    // Végigzongorázunk a 33x33x33-as kockán
    for r in 0..size {
        for g in 0..size {
            for b in 0..size {
                // Kiszámítjuk, hol van az (r,g,b) pont a te 6x33-as kiterített képeden
                // (Ez a te egyéni képleted alapján történik)
                let (x, y) = calculate_2d_coords_from_3d(r, g, b); 
                let pixel = lut_image.get_pixel_mut(x, y);
                
                let offset = (r + g * size + b * size * size) * 4;
                data_3d[offset..offset+4].copy_from_slice(&pixel.0);
            }
        }
    }

    // Feltöltés a GPU-ra mint 3D kocka
    device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some("3D LUT"),
            size: wgpu::Extent3d { width: 33, height: 33, depth_or_array_layers: 33 },
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8Unorm,
            // ... egyéb flag-ek
        },
        &data_3d
    )
}*/

// gpu_processor.rs részlet
/*pub fn load_custom_lut(device: &wgpu::Device, queue: &wgpu::Queue, raw_bytes: &[u8]) -> wgpu::Texture {
    // Feltételezzük, hogy a becsomagolt táblából kinyertük a 6x33-as képpontokat
    // A 33x33x33 kocka 35 937 pixel. RGBA-ban ez ~144 KB.
    let size = 33;
    let mut lut_3d_data = Vec::with_capacity(size * size * size * 4);

    // Itt a te algoritmusod alapján járjuk be a 2D-s képet 
    // és töltjük fel a 3D-s tömböt...
    // (A GPU-nak a 3D textúra egy folytonos memóriaterület)
   // get_pixel_from_6x33_grid(raw_bytes,)
    
    device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some("IView_3D_LUT"),
            size: wgpu::Extent3d { width: size as u32, height: size as u32, depth_or_array_layers: size as u32 },
            mip_level_count: 1,    // ÚJ: 1-es szint (nincs mipmap)
            sample_count: 1,       // ÚJ: 1-es minta (nincs MSAA)
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        &lut_3d_data,
    )
}*/

pub struct Lut3D {
    pub data: Vec<u8>, // 33 * 33 * 33 * 4 (RGBA)
}

impl Lut3D {
    pub fn new(colset: &ColorSettings) -> Self {
        let size = 33;
        let mut data = Vec::with_capacity(size * size * size * 4);

        for b in 0..size {
            for g in 0..size {
                for r in 0..size {
                    // 1. Normalizált RGB értékek (0.0 - 1.0)
                    let r_f = r as f32 / (size - 1) as f32;
                    let g_f = g as f32 / (size - 1) as f32;
                    let b_f = b as f32 / (size - 1) as f32;

                    // 2. Színkorrekció alkalmazása
                    let mut color = [r_f, g_f, b_f];
                    
                    // Invertálás (ha van)
                    if colset.invert {
                        color = [1.0 - color[0], 1.0 - color[1], 1.0 - color[2]];
                    }

                    // HSV korrekció (Saturation és Hue Shift)
                    color = apply_hsv_settings(color, colset.hue_shift, colset.saturation);

                    // Brightness, Contrast, Gamma (ahogy eddig csináltad)
                    for channel in color.iter_mut() {
                        // Brightness
                        *channel += colset.brightness;
                        // Contrast
                        let factor = (1.015 * (colset.contrast + 1.0)) / (1.015 - colset.contrast);
                        *channel = factor * (*channel - 0.5) + 0.5;
                        // Gamma
                        *channel = channel.powf(1.0 / colset.gamma);
                        *channel = channel.clamp(0.0, 1.0);
                    }

                    if !colset.show_r { color[0] = 0.0 };
                    if !colset.show_g { color[1] = 0.0 };
                    if !colset.show_b { color[2] = 0.0 };

                    // 3. Eredmény mentése RGBA bájtokként
                    data.push((color[0] * 255.0) as u8);
                    data.push((color[1] * 255.0) as u8);
                    data.push((color[2] * 255.0) as u8);
                    data.push(255); // Alpha
                }
            }
        }
        Self { data }
    }
}

fn apply_hsv_settings(rgb: [f32; 3], hue_shift_deg: f32, saturation: f32) -> [f32; 3] {
    let mut hsv = rgb_to_hsv(rgb);

    let shift = hue_shift_deg / 360.0;
    hsv[0] = (hsv[0] + shift).rem_euclid(1.0); // Biztonságos körbefordulás Rustban

    // Saturation tolása: 0.0 az alap, -1.0 a szürke, 1.0 a dupla szaturáció
    if saturation > 0.0 {
        hsv[1] = hsv[1] + (1.0 - hsv[1]) * saturation;
    } else {
        hsv[1] = hsv[1] * (1.0 + saturation);
    }

    hsv_to_rgb(hsv)
}

fn rgb_to_hsv(rgb: [f32; 3]) -> [f32; 3] {
    let r = rgb[0];
    let g = rgb[1];
    let b = rgb[2];

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h = 0.0;
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    if delta != 0.0 {
        if max == r {
            h = (g - b) / delta + (if g < b { 6.0 } else { 0.0 });
        } else if max == g {
            h = (b - r) / delta + 2.0;
        } else {
            h = (r - g) / delta + 4.0;
        }
        h /= 6.0; // Normalizálás 0.0 - 1.0 közé
    }

    [h, s, v]
}

fn hsv_to_rgb(hsv: [f32; 3]) -> [f32; 3] {
    let h = hsv[0];
    let s = hsv[1];
    let v = hsv[2];

    if s <= 0.0 {
        // Ha a telítettség 0, akkor a szín a szürke árnyalata (v)
        return [v, v, v];
    }

    // A színkört 6 szektorra osztjuk (0-tól 5-ig)
    // A modulo 1.0 biztosítja, hogy a 1.0 feletti értékek is körbeforduljanak
    let hh = (h % 1.0) * 6.0;
    let i = hh.floor() as i32;
    let ff = hh - hh.floor(); // A szektoron belüli relatív pozíció

    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * ff));
    let t = v * (1.0 - (s * (1.0 - ff)));

    match i {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q], // Az 5. szektor és biztonsági fallback
    }
}

pub struct ImageCallback {
    // Itt csak olyan adatokat tárolunk, amik a rajzoláshoz kellenek
    pub lut_weight: f32, // Példa extra paraméterre
}

impl egui_wgpu::CallbackTrait for ImageCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        // Itt készíthetnél elő adatokat minden frame előtt, ha kell
        Vec::new()
    }

    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(processor) = callback_resources.get::<Arc<ImageProcessor>>() {
            let mg_lock = processor.main_bind_group.lock().unwrap();
            if let Some(bind_group) = &*mg_lock {
                render_pass.set_pipeline(&processor.pipeline);
                render_pass.set_bind_group(0, bind_group, &[]);

                // A viewport beállítása a tényleges képernyőpixel-helyre
                let ppp = info.pixels_per_point;
                let clip_rect = info.clip_rect; 
                
                render_pass.set_viewport(
                    clip_rect.min.x * ppp,
                    clip_rect.min.y * ppp,
                    clip_rect.width() * ppp,
                    clip_rect.height() * ppp,
                    0.0,
                    1.0,
                );

                render_pass.draw(0..3, 0..1);
            }
        }
    }
}
