#include "Core/Window.h"

#include <stdexcept>
#include <iostream>

void framebuffer_size_callback(GLFWwindow *window, int width, int height)
{
    Hydro::Window *hydroWindow = static_cast<Hydro::Window *>(glfwGetWindowUserPointer(window));
    if (hydroWindow)
    {
        hydroWindow->setWidth(width);
        hydroWindow->setHeight(height);
        glViewport(0, 0, width, height);
    }
}

namespace Hydro
{
    Window::Window(int width, int height, const char *title)
    {

        glfwInit();
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

#ifdef __APPLE__
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, true);
#endif

        h_window = glfwCreateWindow(width, height, title, NULL, NULL);

        if (h_window == NULL)
        {
            glfwTerminate();
            throw std::runtime_error("\x1b[1m\x1b[31m[ERROR]\x1b[0m: Failed to create GLFW window");
        }

        glfwSetWindowUserPointer(h_window, this);

#ifdef __APPLE__
        width *= 2;
        height *= 2;
#endif

        setWidth(width);
        setHeight(height);

        glfwSetFramebufferSizeCallback(h_window, framebuffer_size_callback);

        glfwMakeContextCurrent(h_window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
        {
            glfwTerminate();
            throw std::runtime_error("\x1b[1m\x1b[31m[ERROR]\x1b[0m: Failed to initialize GLAD");
        }

        glViewport(0, 0, h_width, h_height);
    }

    void Window::setWidth(int width)
    {
        h_width = width;
    }

    void Window::setHeight(int height)
    {
        h_height = height;
    }

    void Window::swapBuffers()
    {
        glfwSwapBuffers(h_window);
    }

    void Window::pollEvents()
    {
        glfwPollEvents();
    }

    bool Window::shouldClose()
    {
        return glfwWindowShouldClose(h_window);
    }

}