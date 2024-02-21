#include "Core/Window.h"
#include "Core/Event.h"
#include "Core/KeyEvent.h"

#include <iostream>

int main()
{

    Hydro::Window window(800, 600, "Hydro");

    while (!window.shouldClose())
    {
        window.pollEvents();

        Hydro::Event *event = window.getEvent();

        if (event != nullptr)
        {
            switch (event->getType())
            {
            case Hydro::EventType::KeyPress:
            {
                event = (Hydro::KeyPressEvent *)event;
                std::cout << event->toString() << std::endl;
            }
            default:

                break;
            }
        }

        delete event;

        window.swapBuffers();
    }
}