#import bevy_sprite::{
    mesh2d_view_bindings::view,
    mesh2d_functions as mesh_functions,
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) uv: vec2f,
};

struct PointsSettings {
    color: vec4f,
    radius: f32,
};
@group(2) @binding(0)
var<uniform> settings: PointsSettings;

// Offset Y so the center of the circle aligns with the incoming point
// The unit square is 3r high, so radius is 1/3 and center in Y is 0.5
const offset_y = 0.5 - (1.0 / 3.0);
const triangle = array(
    vec3f(0.0, 0.5 + offset_y, 0.0),   // top center
    vec3f(-0.5, -0.5 + offset_y, 0.0), // bottom left
    vec3f(0.5, -0.5 + offset_y, 0.0),  // bottom right
);
const uv = array(
    vec2f(0.5, 0.0),  // top center
    vec2f(0.0, 1.0),  // bottom left
    vec2f(1.0, 1.0),  // bottom right
);

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Index into the above arrays
    let index = vertex.vertex_index % 3;

    // Height of triangle containing a circle of radius 'r' is '3r'
    let scale = f32(3 * settings.radius);
    let position = vertex.position + (triangle[index] * scale);

    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4f(position, 1.0)
    );

    out.clip_position = mesh_functions::mesh2d_position_world_to_clip(world_position);
    out.uv = uv[index];

    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    const radius = 1.0 / 3.0;
    // Get distance from uv to circle center
    let dist = distance(vec2f(0.5, 2.0 / 3.0), input.uv);
    if dist > radius {
        discard;
    }
    return settings.color;
}