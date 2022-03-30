#version 330
in vec2 vert_position;
out vec2 frag_position;

void main() {
    frag_position = vert_position;
    gl_Position = vec4(vert_position.x, vert_position.y, 0.0, 1.0);
}
