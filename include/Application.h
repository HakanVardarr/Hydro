#ifndef APPLICATION_H
#define APPLICATION_H

#include "Core/Logger.h"
#include "Core/Window.h"
#include "Core/Events.h"
#include "Graphics/Renderer.h"

class Application
{
public:
    Application();
    ~Application()
    {
        delete m_window;
    }

private:
    Hydro::Window *m_window = nullptr;
};

#endif