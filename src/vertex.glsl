#version 100
attribute vec2 pos;
uniform vec2 offset;
void main() {
    gl_Position = vec4(pos + offset, 0, 1);
}