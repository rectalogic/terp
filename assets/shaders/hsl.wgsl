#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var<uniform> color: vec4f;

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    var t_adj = t;
    if (t_adj < 0.0) { t_adj += 1.0; }
    if (t_adj > 1.0) { t_adj -= 1.0; }
    if (t_adj < 1.0/6.0) { return p + (q - p) * 6.0 * t_adj; }
    if (t_adj < 1.0/2.0) { return q; }
    if (t_adj < 2.0/3.0) { return p + (q - p) * (2.0/3.0 - t_adj) * 6.0; }
    return p;
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    if (s == 0.0) {
        return vec3<f32>(l);
    }

    var q: f32;
    if (l < 0.5) {
        q = l * (1.0 + s);
    } else {
        q = l + s - l * s;
    }
    let p = 2.0 * l - q;

    return vec3<f32>(
        hue_to_rgb(p, q, h + 1.0/3.0),
        hue_to_rgb(p, q, h),
        hue_to_rgb(p, q, h - 1.0/3.0)
    );
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    // Convert from UV coordinates to centered coordinates
    let uv = input.uv * 2.0 - 1.0;
    
    // Calculate polar coordinates
    let radius = length(uv);
    let angle = atan2(uv.y, uv.x);
    
    // Convert angle to hue (normalize from [-π, π] to [0, 1])
    let hue = (angle / (2.0 * 3.14159) + 0.5);
    
    // Use radius as saturation (clamped to circle)
    let saturation = saturate(radius);
    
    // Fixed lightness value (adjust as needed)
    let lightness = 0.5;
    
    // Only render inside the unit circle
    if (radius > 1.0) {
        return color;
    }
    
    let rgb = hsl_to_rgb(hue, saturation, lightness);
    return vec4<f32>(rgb, 1.0);
}
