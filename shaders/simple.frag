#version 430 core

out vec4 fragmentColor;

in vec4 vertexColor;
in vec3 vertexNormal;
in vec3 fragmentPosition;

uniform vec3 cameraPosition;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

    vec3 normal = normalize(vertexNormal);
    vec3 lightDir = normalize(-lightDirection);
    vec3 viewDir = normalize(cameraPosition - fragmentPosition);

    // Ambient
    float ambient = 0.2;

    // Diffuse / Lambertian
    float diffuse = max(0.0, dot(normal, lightDir));

    // Specular
    vec3 reflectDir = reflect(-lightDir, normal);

    float specular = pow(
        max(0.0, dot(viewDir, reflectDir)),
        32.0
    );

    vec3 color =
        vertexColor.rgb * (ambient + diffuse)
        + vec3(specular);

    fragmentColor = vec4(color, vertexColor.a);
}