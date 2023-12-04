#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::mesh_functions

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(0.0,0.0,0.0,1.0);
    return out;
}