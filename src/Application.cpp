#include "Application.h"

// clang-format off
float vertices[] = {
    // positions          // colors           
     0.0f,  0.5f, 0.0f,   1.0f, 0.0f, 0.0f,   
     0.5f, -0.5f, 0.0f,   0.0f, 1.0f, 0.0f,   
    -0.5f, -0.5f, 0.0f,   0.0f, 0.0f, 1.0f,   
};

unsigned int indicies[] = {
    0, 1, 2,
};
// clang-format on

Application::Application()
{
    m_window = new Hydro::Window(800, 600, "Hydro");
    Hydro::Renderer::clearColor(0.18, 0.18, 0.18, 1.0);

    Hydro::Shader shader("shaders/triangle.frag", "shaders/triangle.vert");
    Hydro::VertexBuffer vertexBuffer(vertices, sizeof(vertices));
    Hydro::VertexArray vertexArray(vertexBuffer, {3, 3});
    Hydro::IndexBuffer indexBuffer(indicies, sizeof(indicies));
}