#include "Core/Window.h"
#include "Core/Events.h"
#include "Graphics/Renderer.h"

#include <iostream>

void handle_event(Hydro::Event *event)
{
    if (event != nullptr)
    {

        switch (event->getType())
        {
        case Hydro::EventType::KeyPress:
        {
            Hydro::KeyPressEvent *keyPressEvent = (Hydro::KeyPressEvent *)(event);

            if (keyPressEvent->getKey() == Hydro::KeyCode::KeyW)
            {
                std::cout << "W key is pressed" << std::endl;
            }

            break;
        }
        case Hydro::EventType::KeyRelease:
        {
            Hydro::KeyReleaseEvent *keyReleaseEvent = (Hydro::KeyReleaseEvent *)(event);
            std::cout << "Key Released : " << (int)keyReleaseEvent->getKey() << std::endl;
            break;
        }
        default:
            break;
        }

        delete event;
    }
}

int main()
{
    Hydro::Window window(800, 600, "Hydro");
    Hydro::Renderer renderer;

    renderer.clearColor(0.18, 0.18, 0.18, 1.0);
    while (!window.shouldClose())
    {
        renderer.clear();
        window.pollEvents();

        handle_event(window.getEvent());

        window.swapBuffers();
    }
}
