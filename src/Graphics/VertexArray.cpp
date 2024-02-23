#include "Graphics/VertexArray.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>

#include <algorithm>
#include <numeric>

namespace Hydro
{

    VertexArray::VertexArray(VertexBuffer &vertexBuffer, std::vector<unsigned int> attributes)
    {
        vertexBuffer.Bind();

        glGenVertexArrays(1, &m_id);
        glBindVertexArray(m_id);

        auto count = std::reduce(attributes.begin(), attributes.end());
        unsigned int idx = 0, last = 0;
        for (auto attr : attributes)
        {
            glVertexAttribPointer(idx, attr, GL_FLOAT, GL_FALSE, count * sizeof(float), (void *)(last * sizeof(float)));
            glEnableVertexAttribArray(idx);
            if (idx < attributes.size())
            {
                last += attributes[idx];
            }
            idx++;
        }
    }

    VertexArray::~VertexArray()
    {
        glDeleteBuffers(1, &m_id);
    }

    void VertexArray::Bind() const
    {
        glBindVertexArray(m_id);
    }

    void VertexArray::Unbind() const
    {
        glBindVertexArray(0);
    }
}