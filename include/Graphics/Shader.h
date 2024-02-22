#ifndef SHADER_H
#define SHADER_H

#include <string>

namespace Hydro
{
    enum class ShaderType
    {
        VERTEX,
        FRAGMENT,
    };

    class Shader
    {
    public:
        Shader(const std::string fragmentPath, const std::string vertexPath);
        ~Shader();

        void bind() const;
        void unbind() const;
        void setBool(const std::string &name, bool value) const;
        void setInt(const std::string &name, int value) const;
        void setFloat(const std::string &name, float value) const;

    private:
        unsigned int m_id;
    };

}

#endif