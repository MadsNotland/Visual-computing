#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;

out vec4 vertexColor;
out vec3 vertexNormal;
out vec3 fragmentPosition;

uniform mat4 transformation; //denne matrisen kommer utenfra, fra CPU/Rust koden
uniform mat4 modelMatrix; //denne matrisen kommer utenfra, fra CPU/Rust koden

void main()
{
   
    gl_Position = transformation * vec4(position.x, position.y, position.z, 1.0f);//gjør om til homogen koordinat, som er nødvendig for å kunne tegne i OpenGL
    vertexColor = color;
    vertexNormal = normalize(mat3(modelMatrix) * normal); // Transform the normal to world space
    fragmentPosition = vec3(modelMatrix * vec4(position, 1.0)); // Transform the position to world space
}