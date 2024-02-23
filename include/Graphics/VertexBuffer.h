#ifndef VERTEX_BUFFER_H
#define VERTEX_BUFFER_H

#include <vector>

namespace Hydro
{
    class VertexBuffer
    {
    public:
        VertexBuffer(float *verticies, unsigned int size);
        ~VertexBuffer();

        void Bind() const;
        void Unbind() const;

    private:
        unsigned int m_id;
    };
}

#endif