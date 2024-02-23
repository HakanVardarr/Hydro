#include "Graphics/Renderer.h"

namespace Hydro
{
    void Renderer::ClearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a)
    {
        glClearColor(r, g, b, a);
    }

    void Renderer::Clear()
    {
        glClear(GL_COLOR_BUFFER_BIT);
    }

    void Renderer::Draw(const VertexArray &vertexArray, const IndexBuffer &indexBuffer, const Shader &shader)
    {
        shader.Bind();
        vertexArray.Bind();
        indexBuffer.Bind();

        glDrawElements(GL_TRIANGLES, indexBuffer.Count(), GL_UNSIGNED_INT, 0);
    }
}