#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::maths::PI_2

@group(2) @binding(0)
var<uniform> color: vec4f;
@group(2) @binding(1)
var<uniform> value: f32;

fn mod_glsl(x: vec3f, y: vec3f) -> vec3f {
    return x - y * floor(x / y);
}

//  Function from Iñigo Quiles
//  https://www.shadertoy.com/view/MsS3Wc
fn hsv2rgb(c: vec3f) -> vec3f {
    var rgb: vec3f = clamp(abs(mod_glsl((c.x * 6.0 + vec3f(0.0, 4.0, 2.0)), vec3f(6.0)) - 3.0) - 1., vec3f(0.0), vec3f(1.0));
	rgb = rgb * rgb * (3.0 - 2.0 * rgb);
	return c.z * mix(vec3f(1.0), rgb, c.y);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    // Use polar coordinates instead of cartesian
    let coord = vec2f(0.5) - input.uv;
    let angle = atan2(-coord.y, coord.x);
    let radius = length(coord) * 2.0;

    if (radius > 1.0) {
        return color;
    }

    // Map the angle (-PI to PI) to the Hue (from 0 to 1)
    // and the Saturation to the radius
    return vec4f(hsv2rgb(vec3f((angle / PI_2) + 0.5, radius, value)), 1.0);
}
