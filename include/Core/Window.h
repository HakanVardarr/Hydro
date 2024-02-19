#ifndef WINDOW_H
#define WINDOW_H

#include <glad/glad.h>
#include <GLFW/glfw3.h>

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
        void swapBuffers();
        void pollEvents();

        bool shouldClose();

    private:
        GLFWwindow *h_window;
        int h_width;
        int h_height;
    };
}

#endif