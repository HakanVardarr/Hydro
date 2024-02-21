#include "Core/Window.h"
#include "Core/KeyEvent.h"

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

void key_callback(GLFWwindow *window, int key, int scancode, int action, int mods)
{
    Hydro::Window *hydroWindow = static_cast<Hydro::Window *>(glfwGetWindowUserPointer(window));
    if (hydroWindow)
    {
        switch (action)
        {
        case GLFW_RELEASE:
        {
            // hydroWindow->setEvent(new Hydro::KeyPressEvent(key));
            break;
        }
        case GLFW_PRESS:
        {
            hydroWindow->setEvent(new Hydro::KeyPressEvent(key));
            break;
        }

        default:
            break;
        }
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

        m_window = glfwCreateWindow(width, height, title, NULL, NULL);

        if (m_window == NULL)
        {
            glfwTerminate();
            throw std::runtime_error("\x1b[1m\x1b[31m[ERROR]\x1b[0m: Failed to create GLFW window");
        }

        glfwSetWindowUserPointer(m_window, this);

#ifdef __APPLE__
        width *= 2;
        height *= 2;
#endif

        setWidth(width);
        setHeight(height);

        glfwSetFramebufferSizeCallback(m_window, framebuffer_size_callback);
        glfwSetKeyCallback(m_window, key_callback);

        glfwMakeContextCurrent(m_window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
        {
            glfwTerminate();
            throw std::runtime_error("\x1b[1m\x1b[31m[ERROR]\x1b[0m: Failed to initialize GLAD");
        }

        glViewport(0, 0, m_width, m_height);
    }

    void Window::setWidth(int width)
    {
        m_width = width;
    }

    void Window::setHeight(int height)
    {
        m_height = height;
    }

    void Window::setEvent(Event *event)
    {
        m_event = event;
    }

    void Window::swapBuffers()
    {
        glfwSwapBuffers(m_window);
    }

    void Window::pollEvents()
    {
        glfwPollEvents();
    }

    bool Window::shouldClose()
    {
        return glfwWindowShouldClose(m_window);
    }

}