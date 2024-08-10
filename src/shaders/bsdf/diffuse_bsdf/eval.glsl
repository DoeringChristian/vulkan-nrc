#version 460

#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable

#include "bindings.glsl"
#include "bsdf/eval.glsl"
#include "bsdf/diffuse_bsdf/common.glsl"

layout(location = 0) callableDataInEXT EvalParams params;

BUFFER_BINDING buffer DiffuseBSDFBuffer {
    DiffuseBSDF diffuse_bsdfs[];
} diffuse_bsdf_buffer[];

TEXTURE_BINDING uniform sampler2D textures[];

void main() {
    if (!params.si.valid)
        return;

    DiffuseBSDF bsdf = diffuse_bsdf_buffer[params.bsdf_buf].diffuse_bsdfs[params.bsdf_offset];

    params.value = texture(textures[bsdf.value], params.si.uv).rgb;
}
