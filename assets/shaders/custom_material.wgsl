#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var char_texture: texture_2d<f32>;
@group(1) @binding(2)
var char_sampler: sampler;
@group(1) @binding(3)
var weapon_texture: texture_2d<f32>;
@group(1) @binding(4)
var weapon_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let cts = textureSample(char_texture, char_sampler, mesh.uv);
    let wts = textureSample(weapon_texture, weapon_sampler, mesh.uv);
    let if_char_outline = cts.r == 0.0 && cts.g == 0.0 && cts.b == 0.0 && cts.a == 1.0;
    let if_weapon_outline = wts.r == 0.0 && wts.g == 0.0 && wts.b == 0.0 && wts.a == 1.0;
    let if_char_content = !if_char_outline && cts.a > 0.0;
    let if_weapon_content = !if_weapon_outline && wts.a > 0.0;

    if (if_weapon_content) {
        return wts;
    }
    if (if_char_content) {
        return cts;
    }
    if (if_weapon_outline) {
        return material.color;
    }
    if (if_char_outline) {
        return material.color;
    }

    return cts;
}
