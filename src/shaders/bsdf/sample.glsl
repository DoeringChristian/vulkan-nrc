#include "instance.glsl"
#include "intersection.glsl"

struct BSDFSample {
    vec3 wo;
    vec3 value;
    float pdf;
};

struct SampleParams {
    // in
    uint bsdf_buf;
    uint bsdf_offset;
    SurfaceInteraction si;
    // out
    BSDFSample bsdf_sample;
};
