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

        void bind() const;
        void unbind() const;

        unsigned int count() const;

    private:
        unsigned int m_id;
        unsigned int m_size;
    };
}

#endif