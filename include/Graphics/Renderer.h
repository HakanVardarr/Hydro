#ifndef RENDERER_H
#define RENDERER_H

#include "Graphics/IndexBuffer.h"
#include "Graphics/VertexBuffer.h"
#include "Graphics/VertexArray.h"
#include "Graphics/Shader.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>

namespace Hydro
{
    class Renderer
    {
    public:
        static void ClearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a);
        static void Clear();
        static void Draw(const VertexArray &vertexArray, const IndexBuffer &indexBuffer, const Shader &shader);
    };
}

#endif