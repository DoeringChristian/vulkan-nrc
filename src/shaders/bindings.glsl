#ifndef BINDINGS_GLSL
#define BINDINGS_GLSL

#define BUFFER_BINDING layout(set =  0, binding = 0)
#define TEXTURE_BINDING layout(set = 1, binding = 1)
#define ACCEL_BINDING layout(set = 2, binding = 2)

layout(push_constant) uniform PushConstants {
    uint instance_buffer;
} push_constants;

#endif // BINDINGS_GLSL
