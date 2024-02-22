#ifndef WINDOW_H
#define WINDOW_H

#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include "Events/Event.h"

namespace Hydro
{
    class Window
    {
    public:
        Window(int width, int height, const char *title);
        ~Window()
        {
            glfwTerminate();
        }

        void setWidth(int width);
        void setHeight(int height);
        void setEvent(Event *event);
        void setClose();
        void swapBuffers();
        void pollEvents();

        Event *getEvent();

        bool shouldClose();

    private:
        GLFWwindow *m_window;
        Event *m_event = nullptr;
        int m_width;
        int m_height;
    };
}

#endif