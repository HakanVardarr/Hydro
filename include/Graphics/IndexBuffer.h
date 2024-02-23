#ifndef INDEX_BUFFER_H
#define INDEX_BUFFER_H

#include <vector>

namespace Hydro
{
    class IndexBuffer
    {
    public:
        IndexBuffer(unsigned int *indicies, unsigned int size);
        ~IndexBuffer();

        void Bind() const;
        void Unbind() const;

        unsigned int Count() const;

    private:
        unsigned int m_id;
        unsigned int m_size;
    };
}

#endif