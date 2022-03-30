#version 330
const int MAX_MARCHING_STEPS = 512;
const float MAX_DIST = 100.0;
const float EPSILON = 0.0001;
const float FIELD_OF_VIEW = 45.0;
const float POWER = 8.0;
const int MAX_ITERATIONS = 256;

in vec2 frag_position;

uniform float aspect_ratio;
uniform vec3 eye_pos;

out vec4 frag_color;

float distance_function(vec3 pos) {
    float sphere = length(pos - vec3(0.5)) - 0.7;

    vec3 cube_distances = abs(pos - vec3(0.5, 0.5, 0.5)) - vec3(0.5, 0.5, 0.5);
    float inside_cube_dist = min(max(cube_distances.x, max(cube_distances.y, cube_distances.z)), 0.0);
    float outside_cube_dist = length(max(cube_distances, 0.0));
    float cube = inside_cube_dist + outside_cube_dist;

    return max(cube, -sphere);

    vec3 clamped_pos = clamp(pos, vec3(0.0), vec3(1.0));
}

struct MandelbulbResult {
    float dist;
    int iterations;
};

float mandelbulb(vec3 pos) {
    vec3 w = pos;
    float m = dot(w,w);

    vec4 trap = vec4(abs(w),m);
    float dz = 1.0;

    for(int i = 0; i < 4; i++) {
        float m2 = m*m;
        float m4 = m2*m2;
        dz = 8.0*sqrt(m4*m2*m)*dz + 1.0;

        float x = w.x; float x2 = x*x; float x4 = x2*x2;
        float y = w.y; float y2 = y*y; float y4 = y2*y2;
        float z = w.z; float z2 = z*z; float z4 = z2*z2;

        float k3 = x2 + z2;
        float k2 = inversesqrt(k3*k3*k3*k3*k3*k3*k3);
        float k1 = x4 + y4 + z4 - 6.0*y2*z2 - 6.0*x2*y2 + 2.0*z2*x2;
        float k4 = x2 - y2 + z2;

        w.x = pos.x +  64.0*x*y*z*(x2-z2)*k4*(x4-6.0*x2*z2+z4)*k1*k2;
        w.y = pos.y + -16.0*y2*k3*k4*k4 + k1*k1;
        w.z = pos.z +  -8.0*y*k4*(x4*x4 - 28.0*x4*x2*z2 + 70.0*x4*z4 - 28.0*x2*z2*z4 + z4*z4)*k1*k2;

        trap = min(trap, vec4(abs(w), m));

        m = dot(w, w);
        if(m > 256.0) {
            break;
        }
    }
    //resColor = vec4(m,trap.yzw);

    return 0.25*log(m)*sqrt(m)/dz;
}

vec2 ray_march(vec3 eye, vec3 dir, float start, float end) {
    float depth = start;

    for (int i = 0; i < MAX_MARCHING_STEPS; i++) {
        float dist = mandelbulb(eye + depth * dir);
        if (dist < EPSILON) {
            return vec2(depth, max(1.0 - log(i) / 6.0, 0.0));
        }
        depth += dist;
        if (depth >= end) {
            return vec2(end, max(1.0 - log(i) / 6.0, 0.0));
        }
    }
    return vec2(depth, 0.0);
}

vec3 estimate_normal(vec3 p) {
    return normalize(vec3(
        distance_function(vec3(p.x + EPSILON, p.y, p.z)) - distance_function(vec3(p.x - EPSILON, p.y, p.z)),
        distance_function(vec3(p.x, p.y + EPSILON, p.z)) - distance_function(vec3(p.x, p.y - EPSILON, p.z)),
        distance_function(vec3(p.x, p.y, p.z + EPSILON)) - distance_function(vec3(p.x, p.y, p.z - EPSILON))
    ));
}

vec3 hemisphere(vec3 normal) {
    const vec3 light = vec3(0.1, -1.0, 0.0);
    float n_dot_l = dot(normal, light)*0.5 + 0.5;
    return mix(vec3(0.886, 0.757, 0.337), vec3(0.518, 0.169, 0.0), n_dot_l);
}

mat4 view_matrix(vec3 eye, vec3 center, vec3 up) {
    // Based on gluLookAt man page
    vec3 f = normalize(center - eye);
    vec3 s = normalize(cross(f, up));
    vec3 u = cross(s, f);
    return mat4(
        vec4(s, 0.0),
        vec4(u, 0.0),
        vec4(-f, 0.0),
        vec4(0.0, 0.0, 0.0, 1)
    );
}

float dist2(vec2 a, vec2 b) {
    return sqrt(pow((a.x - b.x), 2.0) + pow((a.y - b.y), 2.0));
}

void main() {
    float dir_z = 1.0 / tan(radians(FIELD_OF_VIEW) / 2.0);
    vec3 dir = normalize(vec3(frag_position * vec2(aspect_ratio, 1.0), -dir_z));
    mat4 view_to_world = view_matrix(eye_pos, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
    vec3 world_dir = (view_to_world * vec4(dir, 0.0)).xyz;

    vec2 march_res = ray_march(eye_pos, world_dir, 0.01, MAX_DIST);
    float dist = march_res[0];
    float ambient_occlusion = march_res[1];
    vec2 pixel = vec2(frag_position.x * 400.0 + 400.0, -frag_position.y * 300.0 + 300.0);
    // gl_FragDepth = dist

    if (pixel.x > 0.0 && pixel.x < 300.0 && pixel.y > 0.0 && pixel.y < 300.0) {
        frag_color = vec4(0.3, 0.3, 0.3, 1.0);
    } else {
        frag_color = vec4(1.0);
    }
}
