#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::color_operations::hsv_to_rgb

@group(2) @binding(0)
var<uniform> color: vec4f;
@group(2) @binding(1)
var<uniform> value: f32;

// https://www.shadertoy.com/view/ldfSDj
fn round_rect(p: vec2f, center: vec2f, radius: f32) -> f32 {
    return length(max(abs(p) - center + radius, vec2f(0.0))) - radius;
}

// https://github.com/bevyengine/bevy/discussions/8937
// https://github.com/bevyengine/bevy/blob/b66c3ceb0ee39374ff1759ffb1b5bee2e4b93e99/crates/bevy_color/src/srgba.rs#L211
fn to_linear(nonlinear: vec3f) -> vec3f {
    let cutoff = step(nonlinear, vec3f(0.04045));
    let higher = pow((nonlinear + vec3f(0.055)) / vec3f(1.055), vec3f(2.4));
    let lower = nonlinear / vec3f(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    // Rounded rect
    // seee https://www.shadertoy.com/view/ldfSDj
    const center = vec2f(0.5);
    const corner_radius = 0.1;
    let b = round_rect(input.uv - center, center, corner_radius);
    if (b > 0.0) {
        discard;
    }

    // Use polar coordinates instead of cartesian
    let coord = vec2f(0.5) - input.uv;
    let angle = atan2(coord.y, coord.x);
    let radius = length(coord) * 2.0;

    if (radius > 1.0) {
        return color;
    }

    // Map the angle (0 to 2PI) to the Hue (from 0 to 1)
    // and the Saturation to the radius
    return vec4f(to_linear(hsv_to_rgb(vec3f(angle, radius, value))), 1.0);
}
