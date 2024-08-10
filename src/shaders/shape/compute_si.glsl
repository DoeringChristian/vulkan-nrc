#include "intersection.glsl"
#include "instance.glsl"

struct ComputeSIParams {
    // in
    PreliminaryIntersection pi;
    Instance instance;
    // out
    SurfaceInteraction si;
};
