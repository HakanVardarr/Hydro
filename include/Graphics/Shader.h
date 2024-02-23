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

        void Bind() const;
        void Unbind() const;
        void SetBool(const std::string &name, bool value) const;
        void SetInt(const std::string &name, int value) const;
        void SetFloat(const std::string &name, float value) const;

    private:
        unsigned int m_id;
    };

}

#endif