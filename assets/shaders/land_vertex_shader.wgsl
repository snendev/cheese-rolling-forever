#import bevy_pbr::{
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}
#import bevy_render::instance_index::get_instance_index

#import bevy_shader_utils::simplex_noise_3d simplex_noise_3d
#import bevy_shader_utils::simplex_noise_2d

struct StandardMaterial {
    time: f32,
    ship_position: vec3<f32>,
    // base_color: vec4<f32>;
    // emissive: vec4<f32>;
    // perceptual_roughness: f32;
    // metallic: f32;
    // reflectance: f32;
    // // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    // flags: u32;
    // alpha_cutoff: f32;
};

@group(1) @binding(0)
var<uniform> material: StandardMaterial;
@group(1) @binding(1)
var<uniform> ship_position: vec3<f32>;

#ifdef MORPH_TARGETS
fn morph_vertex(vertex_in: Vertex) -> Vertex {
    var vertex = vertex_in;
    let weight_count = bevy_pbr::morph::layer_count();
    for (var i: u32 = 0u; i < weight_count; i ++) {
        let weight = bevy_pbr::morph::weight_at(i);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph(vertex.index, bevy_pbr::morph::position_offset, i);
#ifdef VERTEX_NORMALS
        vertex.normal += weight * morph(vertex.index, bevy_pbr::morph::normal_offset, i);
#endif
#ifdef VERTEX_TANGENTS
        vertex.tangent += vec4(weight * morph(vertex.index, bevy_pbr::morph::tangent_offset, i), 0.0);
#endif
    }
    return vertex;
}
#endif

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph);
#else
    var vertex = vertex_no_morph;
#endif

var noise = simplex_noise_3d(vertex.position);

#ifdef SKINNED
    var model = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    var model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(model, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position.x, noise, vertex.position.z, 1.0));
    var t = ship_position;
    out.world_position.y = out.world_position.y - 4.0 * sin(ship_position.z - out.world_position.z) / 100.;
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        model,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vec4<f32>(0.5, noise, 0.5, 1.0);
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = get_instance_index(vertex_no_morph.instance_index);
#endif

#ifdef BASE_INSTANCE_WORKAROUND
    // Hack: this ensures the push constant is always used, which works around this issue:
    // https://github.com/bevyengine/bevy/issues/10509
    // This can be removed when wgpu 0.19 is released
    out.position.x += min(f32(get_instance_index(0u)), 0.0);
#endif

    return out;
}
