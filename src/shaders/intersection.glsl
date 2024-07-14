#ifndef SI_GLSL
#define SI_GLSL

struct PreliminaryIntersection {
    vec3 barycentric;
    uint instance;
    uint primitive;
    bool valid;
};

struct SurfaceInteraction {
    vec3 barycentric;
    uint instance;
    uint primitive;
    bool valid;

    vec3 p;
    vec3 n;
    float dist;
    float area;

    vec2 uv;

    mat3 tbn;

    vec3 wi;

    uvec2 bsdf;
};

#endif
