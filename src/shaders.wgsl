// src/shader.wgsl

struct Params {
    setted: u32,
    gamma: f32,
    contrast: f32,
    brightness: f32,
    hue_shift: f32,
    saturation: f32,
    invert: u32,
    show_r: u32,
    show_g: u32,
    show_b: u32,
    sharpen_amount: f32,
    sharpen_radius: f32,
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> params: Params;
@group(0) @binding(3) var t_lut: texture_3d<f32>;
@group(0) @binding(4) var s_lut: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    var processed = base_color;

    // 1. BLUR / SHARP (Unsharp Masking elv)
    if ((params.setted & 1u) != 0u) {
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
    if ((params.setted & 2u) != 0u) {
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
    // vertex, ami egyetlen háromszögként lefedi a (0,0) -> (1,1) UV teret
    let x = f32(i32(in_vertex_index) << 1u & 2i);
    let y = f32(i32(in_vertex_index) & 2i);
    
    var out: VertexOutput;
    // GPU koordináták: x, y (-1..1 tartományban a teljes viewportra)
    out.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    // Textúra koordináták: x, y (0..1 tartományban)
    out.tex_coords = vec2<f32>(x, y);
    return out;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ColorSettings {
    gamma: f32,
    contrast: f32,
    brightness: f32,
    hue_shift: f32,      // fokban: -180.0 .. 180.0
    saturation: f32,     // -1.0 .. 1.0
    invert: u32,         // 0 vagy 1
    show_r: u32,
    show_g: u32,
    show_b: u32,
};

@group(0) @binding(0) var t_input: texture_2d<f32>;
@group(0) @binding(1) var t_output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> settings: ColorSettings;

fn rgb_to_hsv(c: vec3<f32>) -> vec3<f32> {
    let v = max(c.r, max(c.g, c.b));
    let delta = v - min(c.r, min(c.g, c.b));
    var h = 0.0;
    var s = 0.0;
    if (v > 0.0) { s = delta / v; }
    if (delta > 0.0) {
        if (v == c.r) { h = (c.g - c.b) / delta + select(6.0, 0.0, c.g >= c.b); }
        else if (v == c.g) { h = (c.b - c.r) / delta + 2.0; }
        else { h = (c.r - c.g) / delta + 4.0; }
        h /= 6.0;
    }
    return vec3<f32>(h, s, v);
}

fn hsv_to_rgb(c: vec3<f32>) -> vec3<f32> {
    let h = c.x * 6.0;
    let i = floor(h);
    let f = h - i;
    let p = c.z * (1.0 - c.y);
    let q = c.z * (1.0 - c.y * f);
    let t = c.z * (1.0 - c.y * (1.0 - f));
    let m = i32(i) % 6;
    if (m == 0) { return vec3(c.z, t, p); }
    if (m == 1) { return vec3(q, c.z, p); }
    if (m == 2) { return vec3(p, c.z, t); }
    if (m == 3) { return vec3(p, q, c.z); }
    if (m == 4) { return vec3(t, p, c.z); }
    return vec3(c.z, p, q);
}

fn apply_color_math(in_color: vec3<f32>, s: ColorSettings) -> vec3<f32> {
    var out = in_color;

    // 1. Invertálás
    if (s.invert == 1u) { out = 1.0 - out; }

    // 2. HSV korrekciók
    var hsv = rgb_to_hsv(out);
    hsv.x = fract(hsv.x + s.hue_shift / 360.0);
    if (s.saturation > 0.0) {
        hsv.y = hsv.y + (1.0 - hsv.y) * s.saturation;
    } else {
        hsv.y = hsv.y * (1.0 + s.saturation);
    }
    out = hsv_to_rgb(hsv);

    // 3. Brightness, Contrast, Gamma
    let factor = (1.015 * (s.contrast + 1.0)) / (1.015 - s.contrast);
    out = factor * (out + s.brightness - 0.5) + 0.5;
    out = pow(max(out, vec3(0.0)), vec3(1.0 / s.gamma));

    return clamp(out * s.show_channels, vec3(0.0), vec3(1.0));
}

// RGB -> HSV és HSV -> RGB függvények (a korábbiak alapján)
fn apply_math(in_col: vec3<f32>) -> vec3<f32> {
    var col = in_col;
    if (settings.invert == 1u) { col = 1.0 - col; }
    
    // ... ide jön az RGB-HSV-RGB konverzió és a többi matek ...
    // (A korábbi válaszban megadott apply_color_math törzse)
    
    let factor = (1.015 * (settings.contrast + 1.0)) / (1.015 - settings.contrast);
    col = factor * (col + settings.brightness - 0.5) + 0.5;
    col = pow(max(col, vec3(0.0)), vec3(1.0 / settings.gamma));
    
    let mask = vec3<f32>(f32(settings.show_r), f32(settings.show_g), f32(settings.show_b));
    return clamp(col * mask, vec3(0.0), vec3(1.0));
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(t_input);
    if (id.x >= size.x || id.y >= size.y) { return; }

    let input_color = textureLoad(t_input, vec2<i32>(id.xy), 0);
    let output_color = apply_math(input_color.rgb);
    
    textureStore(t_output, vec2<i32>(id.xy), vec4<f32>(output_color, 1.0));
}

/////////////////////

@group(0) @binding(0) var t_input: texture_2d<f32>; // Identity LUT (1089x33)
@group(0) @binding(1) var t_output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> s: GpuColorSettings;

// ... ide jönnek a korábban megírt rgb_to_hsv és hsv_to_rgb függvények ...

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let coords = vec2<i32>(id.xy);
    let size = textureDimensions(t_input);
    if (coords.x >= size.x || coords.y >= size.y) { return; }

    var color = textureLoad(t_input, coords, 0).rgb;

    // 1. Invert
    if (s.invert == 1u) { color = 1.0 - color; }

    // 2. HSV (Saturation, Hue)
    var hsv = rgb_to_hsv(color);
    hsv.x = fract(hsv.x + s.hue_shift / 360.0);
    if (s.saturation > 0.0) { hsv.y += (1.0 - hsv.y) * s.saturation; }
    else { hsv.y *= (1.0 + s.saturation); }
    color = hsv_to_rgb(hsv);

    // 3. Brightness, Contrast, Gamma
    let factor = (1.015 * (s.contrast + 1.0)) / (1.015 - s.contrast);
    color = factor * (color + s.brightness - 0.5) + 0.5;
    color = pow(max(color, vec3(0.0)), vec3(1.0 / s.gamma));

    // 4. Csatornák szűrése
    color = color * vec3(f32(s.show_r), f32(s.show_g), f32(s.show_b));

    textureStore(t_output, coords, vec4(clamp(color, vec3(0.0), vec3(1.0)), 1.0));
}

//////////////////////////////////////
 // 1. Első eljárás: LUT Generálása (Compute Shader)
@group(0) @binding(0) var t_identity: texture_3d<f32>;        // Bemenet: Alap kocka
@group(0) @binding(1) var t_processed_lut: texture_storage_3d<rgba8unorm, write>; // Kimenet: Módosított kocka
@group(0) @binding(2) var<uniform> s: ColorSettings;

@compute @workgroup_size(4, 4, 4) // 33x33x33 lefedéséhez
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= 33 || id.y >= 33 || id.z >= 33) { return; }
    
    let raw_color = textureLoad(t_identity, vec3<i32>(id), 0).rgb;
    let new_color = apply_color_math(raw_color, s); // A korábban megírt matek
    
    textureStore(t_processed_lut, vec3<i32>(id), vec4<f32>(new_color, 1.0));
}
//2. Második eljárás: Kép feldolgozása (Compute Shader)
@group(1) @binding(0) var t_image: texture_2d<f32>;    // Eredeti kép
@group(1) @binding(1) var s_image: sampler;
@group(1) @binding(2) var t_lut: texture_3d<f32>;      // A generált 3D LUT
@group(1) @binding(3) var s_lut: sampler;              // Linear Sampler! (Ez végzi az interpolációt)
@group(1) @binding(4) var t_output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let coords = vec2<i32>(id.xy);
    let size = textureDimensions(t_image);
    if (coords.x >= size.x || coords.y >= size.y) { return; }

    let pixel = textureLoad(t_image, coords, 0).rgb;
    
    // A varázslat: a 33x33x33-as LUT-ból lekérjük a színt. 
    // A 's_lut' (linear sampler) magától elvégzi az interpolációt 256 színmélységre.
    let corrected = textureSampleLevel(t_lut, s_lut, pixel, 0.0).rgb;
    
    textureStore(t_output, coords, vec4<f32>(corrected, 1.0));
}
