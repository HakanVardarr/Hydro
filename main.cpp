#include "Core/Logger.h"
#include "Core/Window.h"
#include "Core/Events.h"
#include "Graphics/Renderer.h"

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

void handle_event(Hydro::Window &window);

int main()
{
    try
    {

        Hydro::Window window(800, 600, "Hydro");

        Hydro::Renderer::clearColor(0.18, 0.18, 0.18, 1.0);

        Hydro::Shader shader("shaders/triangle.frag", "shaders/triangle.vert");
        Hydro::VertexBuffer vertexBuffer(vertices, sizeof(vertices));
        Hydro::VertexArray vertexArray(vertexBuffer, {3, 3});
        Hydro::IndexBuffer indexBuffer(indicies, sizeof(indicies));

        vertexArray.unbind();
        vertexBuffer.unbind();
        indexBuffer.unbind();
        shader.unbind();

        while (!window.shouldClose())
        {
            window.pollEvents();

            handle_event(window);

            Hydro::Renderer::clear();
            Hydro::Renderer::draw(vertexArray, indexBuffer, shader);

            window.swapBuffers();
        }
    }
    catch (std::runtime_error &err)
    {
        Hydro::Logger::error(err.what());
    }
}

void handle_event(Hydro::Window &window)
{
    Hydro::Event *event = window.getEvent();
    while (event != nullptr)
    {

        if (event->getType() == Hydro::EventType::KeyPress)
        {
            Hydro::Logger::info(event->toString());
            Hydro::KeyPressEvent *keyPressEvent = static_cast<Hydro::KeyPressEvent *>(event);

            if (keyPressEvent->getKey() == Hydro::KeyCode::KeyEscape)
            {
                window.setClose();
            }
        }
        else if (event->getType() == Hydro::EventType::MouseMove)
        {
            // Hydro::MouseMoveEvent *mouseMoveEvent = static_cast<Hydro::MouseMoveEvent *>(event);
            // GLfloat color = mouseMoveEvent->getPosX() / 400 + mouseMoveEvent->getPosY() / 150;
            // Hydro::Renderer::clearColor(mouseMoveEvent->getPosX() / 800, mouseMoveEvent->getPosY() / 600, mouseMoveEvent->getPosX() / 800, 1.0);
        }
        else if (event->getType() == Hydro::EventType::MousePress)
        {
            Hydro::Logger::info(event->toString());
        }

        delete event;
        event = window.getEvent();
    }
}