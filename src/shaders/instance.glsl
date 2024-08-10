#ifndef INSTANCE_GLSL
#define INSTANCE_GLSL

struct Instance {
    mat4 to_world;
    uint shape_buf;
    uint shape_offset;
    uint intersection;
    uint compute_surface_intersection;
    uint bsdf_buf;
    uint bsdf_offset;
    uint bsdf_eval;
    uint bsdf_sample;
    uint emitter_buf;
    uint emitter_offset;
    uint emitter_eval;
    uint emitter_sample;
};

#endif //INSTANCE_GLSL
