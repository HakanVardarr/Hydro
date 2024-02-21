#ifndef RENDERER_H
#define RENDERER_H

#include <glad/glad.h>
#include <GLFW/glfw3.h>

namespace Hydro
{
    class Renderer
    {
    public:
        void clearColor(GLfloat r, GLfloat g, GLfloat b, GLfloat a);
        void clear();
    };
}

#endif