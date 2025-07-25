#version 330

in vec4 position;

uniform mat4 transforms;

void main() {
    gl_Position = transforms * position;
}
