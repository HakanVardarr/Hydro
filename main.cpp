#include "Core/Logger.h"
#include "Core/Window.h"
#include "Core/Events.h"

#include "Graphics/Renderer.h"

void handle_event(Hydro::Event *event, Hydro::Window &window);

int main()
{
    Hydro::Window window(800, 600, "Hydro");
    Hydro::Renderer renderer;

    renderer.clearColor(0.18, 0.18, 0.18, 1.0);
    while (!window.shouldClose())
    {
        renderer.clear();
        window.pollEvents();

        handle_event(window.getEvent(), window);

        window.swapBuffers();
    }
}

void handle_event(Hydro::Event *event, Hydro::Window &window)
{
    if (event != nullptr)
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

        delete event;
    }
}