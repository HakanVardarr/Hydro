#ifndef APPLICATION_H
#define APPLICATION_H

#include "Core/Logger.h"
#include "Core/Window.h"
#include "Core/Events.h"
#include "Graphics/Renderer.h"

class Object
{
public:
    Object(std::string name, Hydro::Shader *shader, Hydro::VertexBuffer *vertexBuf, Hydro::VertexArray *vertexArr, Hydro::IndexBuffer *indexBuf) : m_name(name), m_shader(shader), m_vertexBuf(vertexBuf), m_vertexArr(vertexArr), m_indexBuf(indexBuf) {}
    ~Object()
    {
        Hydro::Logger::Info("[OBJECT] Deleting object: " + m_name);
        delete m_shader;
        delete m_vertexBuf;
        delete m_vertexArr;
        delete m_indexBuf;
    };
    void Draw()
    {
        Hydro::Renderer::Draw(*m_vertexArr, *m_indexBuf, *m_shader);
    }
    Hydro::Shader *GetShader()
    {
        return m_shader;
    }
    std::string GetName()
    {
        return m_name;
    }

private:
    std::string m_name;
    Hydro::Shader *m_shader = nullptr;
    Hydro::VertexBuffer *m_vertexBuf = nullptr;
    Hydro::VertexArray *m_vertexArr = nullptr;
    Hydro::IndexBuffer *m_indexBuf = nullptr;
};

class Application
{
public:
    Application();
    ~Application()
    {
        delete m_window;
        for (auto object : m_objects)
        {
            delete object;
        }
        m_objects.clear();
    }
    void AddObject(Object *object);

    void Run();
    void Update();

private:
    Hydro::Window *m_window = nullptr;
    std::vector<Object *> m_objects;
};

#endif