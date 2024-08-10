#version 460

#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_nonuniform_qualifier : enable

#include "bindings.glsl"
#include "shape/compute_si.glsl"
#include "shape/mesh/common.glsl"

layout(location = 0) callableDataInEXT ComputeSIParams params;

BUFFER_BINDING buffer MehsBuffer {
    Mesh meshes[];
} mesh_buffers[];
BUFFER_BINDING buffer IndicesBuffer {
    uint[3] indices[];
} index_buffers[];
BUFFER_BINDING buffer PositionsBuffer {
    float[3] positions[];
} position_buffers[];

void main() {
    if (!params.pi.valid)
        return;

    Mesh mesh = mesh_buffers[params.instance.shape_buf].meshes[params.instance.shape_offset];

    uint[3] triangle = index_buffers[mesh.indices].indices[params.pi.primitive];
}