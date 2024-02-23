#include "Graphics/IndexBuffer.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>

namespace Hydro
{
    IndexBuffer::IndexBuffer(unsigned int *indicies, unsigned int size) : m_size(size)
    {
        glGenBuffers(1, &m_id);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_id);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER, size, indicies, GL_STATIC_DRAW);
    }

    IndexBuffer::~IndexBuffer()
    {
        glDeleteBuffers(1, &m_id);
    }

    void IndexBuffer::Bind() const
    {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, m_id);
    }

    void IndexBuffer::Unbind() const
    {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    }

    unsigned int IndexBuffer::Count() const
    {
        return m_size;
    }
}