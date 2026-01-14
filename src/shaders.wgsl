// src/shader.wgsl

// 1. ELJÁRÁS: LUT Generálás (33x33x33)

struct GpuColorSettings {
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
}

@group(0) @binding(0) var<uniform> colset: GpuColorSettings;
@group(0) @binding(1) var t_identity: texture_3d<f32>;
@group(0) @binding(2) var t_lut_out: texture_storage_3d<rgba8unorm, write>;

@compute @workgroup_size(4, 4, 4)
fn generate_lut(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= 33u || id.y >= 33u || id.z >= 33u) { return; }
    
    let raw = textureLoad(t_identity, vec3<i32>(id), 0).rgb;
    var color = raw; 
    if (colset.setted == 1u) {
        color = apply_color_math(color);
    }
    textureStore(t_lut_out, vec3<i32>(id), vec4<f32>(color, 1.0));
}

fn apply_color_math(in_color: vec3<f32>) -> vec3<f32> {
    var out = in_color;

    // 1. Invertálás
    if (colset.invert == 1u) { out = 1.0 - out; }

    // 2. HSV korrekciók
    var hsv = rgb_to_hsv(out);
    hsv.x = fract(hsv.x + colset.hue_shift / 360.0);
    if (colset.saturation > 0.0) {
        hsv.y = hsv.y + (1.0 - hsv.y) * colset.saturation;
    } else {
        hsv.y = hsv.y * (1.0 + colset.saturation);
    }
    out = hsv_to_rgb(hsv);

    // 3. Brightness, Contrast, Gamma
    let factor = (1.015 * (colset.contrast + 1.0)) / (1.015 - colset.contrast);
    out = factor * (out + colset.brightness - 0.5) + 0.5;
    out = pow(max(out, vec3(0.0)), vec3(1.0 / colset.gamma));

    // 4. channel mask
    let mask = vec3<f32>(f32(colset.show_r), f32(colset.show_g), f32(colset.show_b));
    return clamp(out * mask, vec3(0.0), vec3(1.0));
}


fn rgb_to_hsv(c: vec3<f32>) -> vec3<f32> {
    let v = max(c.r, max(c.g, c.b));
    let delta = v - min(c.r, min(c.g, c.b));
    var h = 0.0;
    var col = 0.0;
    if (v > 0.0) { col = delta / v; }
    if (delta > 0.0) {
        if (v == c.r) { h = (c.g - c.b) / delta + select(6.0, 0.0, c.g >= c.b); }
        else if (v == c.g) { h = (c.b - c.r) / delta + 2.0; }
        else { h = (c.r - c.g) / delta + 4.0; }
        h /= 6.0;
    }
    return vec3<f32>(h, col, v);
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


// 2. ELJÁRÁS: Kép feldolgozása

struct FilterSettings {
    blur_radius: f32,      // 0.0 = kikapcsolva
    sharpen_amount: f32,   // 0.0 = kikapcsolva
    image_width: f32,
    image_height: f32,
}

// Bindingok az alkalmazáshoz
@group(1) @binding(0) var t_src: texture_2d<f32>;       // Eredeti kép
@group(1) @binding(1) var s_linear: sampler;            // Lineáris szűrő a LUT-hoz
@group(1) @binding(2) var t_lut: texture_3d<f32>;       // A már generált 3D LUT
@group(1) @binding(3) var<uniform> f: FilterSettings;
@group(1) @binding(4) var t_out: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn apply_effects(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims_u32  = textureDimensions(t_src);
    
    if (id.x >= dims_u32 .x || id.y >= dims_u32 .y) { return; }

    let dims = vec2<i32>(dims_u32);
    let coords = vec2<i32>(id.xy);

    let min_limit = vec2<i32>(0, 0);
    let max_limit = dims - vec2<i32>(1, 1);

    // 1. ALAP PIXEL ÉS SZŰRÉS (Blur/Sharpen)
    // Egy egyszerű 3x3-as kernel példa a Sharpen-hez
    var center = textureLoad(t_src, coords, 0).rgb;
    var final_color = center;

    if (f.sharpen_amount > 0.0) {
        let left = textureLoad(t_src, clamp(coords + vec2<i32>(-1, 0), min_limit, max_limit), 0).rgb;
        let right = textureLoad(t_src, clamp(coords + vec2<i32>(1, 0), min_limit, max_limit), 0).rgb;
        let top = textureLoad(t_src, clamp(coords + vec2<i32>(0, -1), min_limit, max_limit), 0).rgb;
        let bottom = textureLoad(t_src, clamp(coords + vec2<i32>(0, 1), min_limit, max_limit), 0).rgb;
        
        // Laplaciana alapú élesítés
        let laplacian = center * 4.0 - (left + right + top + bottom);
        final_color = final_color + laplacian * f.sharpen_amount;
    }

    // 2. LUT ALKALMAZÁSA (Színkorrekció)
    // A szűrt színt koordinátaként használjuk a 3D LUT-ban.
    // A textureSampleLevel hardveresen végzi a 33->256 interpolációt.
    let lut_coords = clamp(final_color, vec3(0.0), vec3(1.0));
    let corrected = textureSampleLevel(t_lut, s_linear, lut_coords, 0.0).rgb;

    // 3. MENTÉS
    textureStore(t_out, coords, vec4<f32>(corrected, 1.0));
}


