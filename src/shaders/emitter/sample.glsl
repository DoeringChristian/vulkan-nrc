#include "instance.glsl"
#include "intersection.glsl"

struct EmitterSample {
    vec3 wo;
    vec3 value;
    float pdf;
};

struct SampleParams {
    // in
    uint emitter_buf;
    uint emitter_offset;
    SurfaceInteraction si;
    // out
    EmitterSample emitter_sample;
};
