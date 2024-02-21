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
}