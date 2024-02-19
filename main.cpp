#include "Core/Window.h"

int main()
{
    Hydro::Window window(800, 600, "Hydro");

    while (!window.shouldClose())
    {
        window.pollEvents();

        window.swapBuffers();
    }
}