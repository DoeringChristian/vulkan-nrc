#version 460

#extension GL_EXT_ray_tracing : enable

#include "bindings.glsl"

layout(location = 0) callableDataInEXT ComputeSIParams params;

struct MeshData {
    uint indices;
    uint indices_count;
    uint positions;
    uint normals;
    uint uvs;
};
layout(BUFFER_BINDINGS) MeshDataBuffer {
    MeshData data[];
} mesh[];
layout(BUFFER_BINDINGS) IndicesBuffer {
    uint[3] indices[];
} indices[];
layout(BUFFER_BINDINGS) PositionsBuffer {
    float[3] positions[];
} positions[];

void main() {
    if (!params.pi.is_valid)
        return;

    MeshData data = mesh[params.shape_buf].data[params.shape_offset];

    uint[3] index = indices[data.indices];
}
