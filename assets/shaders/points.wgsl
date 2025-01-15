#import bevy_sprite::{
    mesh2d_view_bindings::view,
    mesh2d_functions as mesh_functions,
}

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3f,
#ifdef INTERPOLATED
    @location(1) target_position: vec3f,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) triangle_position: vec2f,
    @location(1) color: vec4f,
};

struct PointsSettings {
    color: vec4f,
    radius: f32,
#ifdef INTERPOLATED
    target_color: vec4f,
    target_radius: f32,
    t: f32,
#endif
};
@group(2) @binding(0)
var<uniform> settings: PointsSettings;

// Radius is 1.0*sqrt(3)/6
const radius = sqrt(3.0) / 6.0;
// Equilateral triangle with side length 1.0
const triangle = array(
    vec3f( 0.0,  sqrt(3.0) / 3.0, 0.0),   // top center
    vec3f(-0.5, -radius, 0.0),            // bottom left
    vec3f( 0.5, -radius, 0.0)             // bottom right
);

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Index into the above arrays
    let index = vertex.vertex_index % 3;

#ifdef INTERPOLATED
    let scale = 2.0 * mix(settings.radius, settings.target_radius, settings.t) * sqrt(3.0);
    let position = mix(vertex.position, vertex.target_position, settings.t) + (triangle[index] * scale);
    out.color = mix(settings.color, settings.target_color, settings.t);
#else
    // Height of triangle containing a circle of radius 'r' is '3r'
    // Compute scale of equilateral triangle of side length 1 to achieve desired radius
    let scale = 2.0 * settings.radius * sqrt(3.0);
    let position = vertex.position + (triangle[index] * scale);
    out.color = settings.color;
#endif

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
    if dist > radius {
        discard;
    }
    return input.color;
}
