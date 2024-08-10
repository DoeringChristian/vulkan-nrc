#include "instance.glsl"
#include "intersection.glsl"

struct EvalParams {
    // in
    uint bsdf_buf;
    uint bsdf_offset;
    SurfaceInteraction si;
    // out
    vec3 value;
};
