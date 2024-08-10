#version 460

#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable

#include "bindings.glsl"
#include "bsdf/sample.glsl"
#include "bsdf/diffuse_bsdf/common.glsl"

layout(location = 0) callableDataInEXT SampleParams params;

BUFFER_BINDING buffer DiffuseBSDFBuffer {
    DiffuseBSDF diffuse_bsdfs[];
} diffuse_bsdf_buffer[];

TEXTURE_BINDING uniform sampler2D textures[];

void main() {
    if (!params.si.valid)
        return;

    DiffuseBSDF bsdf = diffuse_bsdf_buffer[params.bsdf_buf].diffuse_bsdfs[params.bsdf_offset];

    params.bsdf_sample.value = texture(textures[bsdf.value], params.si.uv).rgb;
}
