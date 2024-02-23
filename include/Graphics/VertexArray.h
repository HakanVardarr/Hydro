#ifndef VERTEX_ARRAY_H
#define VERTEX_ARRAY_H

#include "Graphics/VertexBuffer.h"

#include <vector>

namespace Hydro
{

    class VertexArray
    {
    public:
        VertexArray(VertexBuffer &vertexBuffer, std::vector<unsigned int> attributes);
        ~VertexArray();

        void Bind() const;
        void Unbind() const;

    private:
        unsigned int m_id;
    };
}

#endif