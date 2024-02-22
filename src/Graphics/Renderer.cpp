#include "Graphics/Renderer.h"

namespace Hydro
{
    void Renderer::clearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a)
    {
        glClearColor(r, g, b, a);
    }

    void Renderer::clear()
    {
        glClear(GL_COLOR_BUFFER_BIT);
    }

    void Renderer::draw(const VertexArray &vertexArray, const IndexBuffer &indexBuffer, const Shader &shader)
    {
        shader.bind();
        vertexArray.bind();
        indexBuffer.bind();

        glDrawElements(GL_TRIANGLES, indexBuffer.count(), GL_UNSIGNED_INT, 0);
    }
}