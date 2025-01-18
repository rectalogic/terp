#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::color_operations::hsv_to_rgb

@group(2) @binding(0)
var<uniform> color: vec4f;
@group(2) @binding(1)
var<uniform> value: f32;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    // Use polar coordinates instead of cartesian
    let coord = vec2f(0.5) - input.uv;
    let angle = atan2(coord.y, coord.x);
    let radius = length(coord) * 2.0;

    if (radius > 1.0) {
        return color;
    }

    // Map the angle (0 to 2PI) to the Hue (from 0 to 1)
    // and the Saturation to the radius
    return vec4f(hsv_to_rgb(vec3f(angle, radius, value)), 1.0);
}
