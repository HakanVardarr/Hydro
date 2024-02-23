#ifndef WINDOW_H
#define WINDOW_H

#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include "Events/Event.h"

#include <queue>

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

        void SetWidth(int width);
        void SetHeight(int height);
        void SetEvent(Event *event);
        void SetClose();
        void SwapBuffers();
        void PollEvents();

        Event *GetEvent();

        float GetTime();

        bool ShouldClose();

    private:
        GLFWwindow *m_window;
        std::queue<Event *> m_eventQueue;
        int m_width;
        int m_height;
    };
}

#endif