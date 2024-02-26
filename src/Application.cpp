#include "Application.h"

void HandleEvent(Hydro::Window *window);

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
    Hydro::Renderer::ClearColor(0.18, 0.18, 0.18, 1.0);

    Hydro::Shader *shader = new Hydro::Shader("shaders/triangle.frag", "shaders/triangle.vert");
    Hydro::VertexBuffer *vertexBuffer = new Hydro::VertexBuffer(vertices, sizeof(vertices));
    Hydro::VertexArray *vertexArray = new Hydro::VertexArray(*vertexBuffer, {3, 3});
    Hydro::IndexBuffer *indexBuffer = new Hydro::IndexBuffer(indicies, sizeof(indicies));

    Object *triangle = new Object("Triangle", shader, vertexBuffer, vertexArray, indexBuffer);
    AddObject(triangle);
}

void Application::AddObject(Object *object)
{
    m_objects.push_back(object);
}

void Application::Run()
{
    while (!m_window->ShouldClose())
    {
        m_window->PollEvents();

        HandleEvent();
        Update();
        Hydro::Renderer::Clear();

        for (auto object : m_objects)
        {
            object->Draw();
        }

        m_window->SwapBuffers();
    }
}

static float speed = 1.0;

void Application::Update()
{
}

void Application::HandleEvent()
{
    Hydro::Event *event = m_window->GetEvent();
    while (event != nullptr)
    {
        if (event->GetType() == Hydro::EventType::KeyPress)
        {
            Hydro::KeyPressEvent *keyPressEvent = static_cast<Hydro::KeyPressEvent *>(event);
            if (keyPressEvent->GetKey() == Hydro::KeyCode::KeyUp)
            {
                speed += 0.1;
            }
            else if (keyPressEvent->GetKey() == Hydro::KeyCode::KeyDown)
            {
                if (speed - 0.1 >= 0.0)
                {
                    speed -= 0.1;
                }
            }
            else if (keyPressEvent->GetKey() == Hydro::KeyCode::KeyEscape || keyPressEvent->GetKey() == Hydro::KeyCode::KeyQ)
            {
                m_window->SetClose();
            }
        }
        event = m_window->GetEvent();
    }
}