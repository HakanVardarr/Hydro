#include "Core/Logger.h"
#include "Core/Window.h"
#include "Core/Events.h"

#include <stdexcept>
#include <iostream>

void framebuffer_size_callback(GLFWwindow *window, int width, int height)
{
    Hydro::Window *hydroWindow = static_cast<Hydro::Window *>(glfwGetWindowUserPointer(window));
    if (hydroWindow)
    {
        hydroWindow->SetWidth(width);
        hydroWindow->SetHeight(height);

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
            hydroWindow->SetEvent(new Hydro::KeyReleaseEvent(key));
            break;
        }
        case GLFW_PRESS:
        {
            hydroWindow->SetEvent(new Hydro::KeyPressEvent(key));
            break;
        }
        default:
            break;
        }
    }
}

void cursor_position_callback(GLFWwindow *window, double xpos, double ypos)
{
    Hydro::Window *hydroWindow = static_cast<Hydro::Window *>(glfwGetWindowUserPointer(window));
    if (hydroWindow)
    {
        hydroWindow->SetEvent(new Hydro::MouseMoveEvent(xpos, ypos));
    }
}

void mouse_button_callback(GLFWwindow *window, int button, int action, int mods)
{

    Hydro::Window *hydroWindow = static_cast<Hydro::Window *>(glfwGetWindowUserPointer(window));
    if (hydroWindow)
    {
        double xPos, yPos;
        glfwGetCursorPos(window, &xPos, &yPos);
        if (button == GLFW_MOUSE_BUTTON_RIGHT && action == GLFW_PRESS)
        {
            hydroWindow->SetEvent(new Hydro::MousePress(xPos, yPos, Hydro::MouseButton::Right));
        }
        else if (button == GLFW_MOUSE_BUTTON_LEFT && action == GLFW_PRESS)
        {
            hydroWindow->SetEvent(new Hydro::MousePress(xPos, yPos, Hydro::MouseButton::Left));
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
            throw std::runtime_error("[WINDOW] Failed to create GLFW window");
        }

        glfwSetWindowUserPointer(m_window, this);

#ifdef __APPLE__
        width *= 2;
        height *= 2;
#endif

        SetWidth(width);
        SetHeight(height);

        glfwSetFramebufferSizeCallback(m_window, framebuffer_size_callback);
        glfwSetKeyCallback(m_window, key_callback);
        glfwSetCursorPosCallback(m_window, cursor_position_callback);
        glfwSetMouseButtonCallback(m_window, mouse_button_callback);

        glfwMakeContextCurrent(m_window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
        {
            glfwTerminate();
            throw std::runtime_error("[WINDOW] Failed to initialize GLAD");
        }

        glViewport(0, 0, m_width, m_height);
        Logger::Info("[WINDOW] Window successfully created");
    }

    void Window::SetWidth(int width)
    {
        m_width = width;
    }

    void Window::SetHeight(int height)
    {
        m_height = height;
    }

    void Window::SetEvent(Event *event)
    {
        m_eventQueue.push(event);
    }

    void Window::SetClose()
    {
        glfwSetWindowShouldClose(m_window, true);
    }

    void Window::SwapBuffers()
    {
        glfwSwapBuffers(m_window);
    }

    void Window::PollEvents()
    {
        glfwPollEvents();
    }

    Event *Window::GetEvent()
    {

        if (!m_eventQueue.empty())
        {
            Event *event = m_eventQueue.front();
            m_eventQueue.pop();
            return event;
        }

        return nullptr;
    }

    float Window::GetTime()
    {
        return glfwGetTime();
    }

    bool Window::ShouldClose()
    {
        return glfwWindowShouldClose(m_window);
    }

}