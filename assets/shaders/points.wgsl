#import bevy_sprite::{
    mesh2d_view_bindings::view,
    mesh2d_functions as mesh_functions,
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

struct PointsSettings {
    color: vec4<f32>,
    point_radius: f32,
};
@group(2) @binding(0)
var<uniform> settings: PointsSettings;

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    const triangle = array(
        vec3<f32>(0.0, 0.5, 0.0),   // top center
        vec3<f32>(-0.5, -0.5, 0.0), // bottom left
        vec3<f32>(0.5, -0.5, 0.0),  // bottom right
    );

    let position = vertex.position + (triangle[vertex.vertex_index % 3] * settings.point_radius);

    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4<f32>(position, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(world_position);
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return settings.color;
}