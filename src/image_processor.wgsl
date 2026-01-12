// src/image_processor.wgsl

struct Params {
    amount: f32,       
    radius: f32,       
    lut_weight: f32,   
    enabled_mask: u32, 
};

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> params: Params;
@group(0) @binding(3) var t_lut: texture_3d<f32>;
@group(0) @binding(4) var s_lut: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    var processed = base_color;

    // 1. BLUR / SHARP (Unsharp Masking elv)
    if ((params.enabled_mask & 1u) != 0u) {
        let tex_size = vec2<f32>(textureDimensions(t_diffuse));
        let step = params.radius / tex_size;
        
        var blurred = vec4<f32>(0.0);
        // 5x5-ös kernel a simább eredményért, a sugarat (radius) használva offszetként
        // A súlyozás itt egyszerűsített, de hatásos
        for (var x: f32 = -2.0; x <= 2.0; x += 1.0) {
            for (var y: f32 = -2.0; y <= 2.0; y += 1.0) {
                let offset = vec2<f32>(x, y) * step;
                blurred += textureSample(t_diffuse, s_diffuse, in.tex_coords + offset);
            }
        }
        blurred /= 25.0;
        
        // Képlet: Eredeti + (Eredeti - Elmosott) * Mennyiség
        // Ha amount > 0: Élesít (túllövi a kontrasztos éleket)
        // Ha amount < 0: Elmos (blur)
        processed = vec4<f32>(base_color.rgb + (base_color.rgb - blurred.rgb) * params.amount, base_color.a);
    }

    // 2. 3D LUT ALKALMAZÁSA
    if ((params.enabled_mask & 2u) != 0u) {
        // Precíziós korrekció a 3D textúra mintavételezéshez
        let lut_size = 33.0;
        let uvw = processed.rgb * ((lut_size - 1.0) / lut_size) + (0.5 / lut_size);
        let lut_color = textureSample(t_lut, s_lut, uvw);
        
        processed = vec4<f32>(mix(processed.rgb, lut_color.rgb, params.lut_weight), processed.a);
    }

    return clamp(processed, vec4<f32>(0.0), vec4<f32>(1.0));
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    // Ez egy trükk: egyetlen hatalmas háromszöget rajzolunk, ami lefedi a képernyőt
    // Nincs szükség vertex bufferre
    let x = f32(i32(in_vertex_index) << 1u & 2i) - 1.0;
    let y = f32(i32(in_vertex_index) & 2i) - 1.0;
    var out: VertexOutput;
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    //out.tex_coords = vec2<f32>(x * 0.5 + 0.5, 1.0 - (y * 0.5 + 0.5));
	out.tex_coords = vec2<f32>(x * 0.5 + 0.5, y * -0.5 + 0.5); // Megfordított Y
    return out;
}

