#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_sampler: sampler;
@group(1) @binding(3)
var mask_texture: texture_2d<f32>;
@group(1) @binding(4)
var maks_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var base = textureSample(base_texture, base_sampler, mesh.uv);
    var mask = textureSample(mask_texture, maks_sampler, mesh.uv);
//    return base;
//    return vec4<f32>(base.xyz, (1.0 - mask.w) * base.w);
//    return vec4<f32>(1.0,1.0,1.0,1.0);
    return vec4<f32>(mesh.world_position.xyz,1.0);
}
