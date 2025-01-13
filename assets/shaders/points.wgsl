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
    @location(0) triangle_position: vec2f,
};

struct PointsSettings {
    color: vec4f,
    radius: f32,
};
@group(2) @binding(0)
var<uniform> settings: PointsSettings;

// Equilateral triangle with side length 1.0
const triangle = array(
  vec3f( 0.0,  sqrt(3.0) / 3.0, 0.0),   // top center
  vec3f(-0.5, -sqrt(3.0) / 6.0, 0.0),   // bottom left
  vec3f( 0.5, -sqrt(3.0) / 6.0, 0.0)    // bottom right
);
// Radius is 1.0*sqrt(3)/6
const radius = sqrt(3.0) / 6.0;

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
    out.triangle_position = triangle[index].xy;

    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4f {
    let dist = distance(vec2f(0.0, 0.0), input.triangle_position);

    if dist < radius {
        return vec4<f32>(0.0, 0.0, 1.0, 1.0);
    } else {
        discard;
    }
}

/*
@fragment
fn fragmentY(input: VertexOutput) -> @location(0) vec4f {
    const radius = 1.0 / 3.0;
    // Get distance from uv to circle center
    let dist = distance(vec2f(0.5, 2.0 / 3.0), input.uv);
    // 1 pixel smooth border
    let edge_size = 1.0 / view.viewport.w; // 1px in UV space
    let alpha = 1.0 - smoothstep(radius - edge_size, radius, dist);
    // if alpha <= 0.0 {
    //     discard;
    // }
    return vec4f(alpha, alpha, alpha, 1.0);
    // return vec4f(settings.color.rgb, settings.color.a * alpha);
}
*/
