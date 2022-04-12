#version 450

varying vec3 inPosition;
varying vec4 inColor;
varying vec2 texCoord;
varying vec3 normals;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = ftranform();
    fragColor = inColor.xyz;
}