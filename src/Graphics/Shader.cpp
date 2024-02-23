#include "Graphics/Shader.h"
#include "Core/Logger.h"

#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <fstream>
#include <stdexcept>

namespace Hydro
{
    unsigned int ReadShader(std::string shaderName, ShaderType shaderType)
    {
        std::ifstream shaderFile(shaderName);
        std::string shaderContent;

        if (shaderFile.is_open())
        {
            std::string temp;
            while (getline(shaderFile, temp))
            {
                shaderContent.append(temp + "\n");
            }

            Logger::Info("[SHADER] Compiling: " + shaderName);
            const char *shaderSource = shaderContent.c_str();
            unsigned int shader = glCreateShader((shaderType == ShaderType::VERTEX) ? GL_VERTEX_SHADER : GL_FRAGMENT_SHADER);

            glShaderSource(shader, 1, &shaderSource, NULL);
            glCompileShader(shader);

            int success;
            char infoLog[512];
            glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
            if (!success)
            {
                glGetShaderInfoLog(shader, 512, NULL, infoLog);
                throw std::runtime_error("[SHADER] Compile shader failed (" + (shaderType == ShaderType::VERTEX) ? std::string("Vertex Shader") : std::string("Fragment Shader") + std::string(")") + std::string(infoLog));
            }

            Logger::Info("[SHADER] Compiled: " + shaderName);
            return shader;
        }
        else
        {
            throw std::runtime_error("[SHADER] Cannot locate shader named " + shaderName);
        }
    }

    Shader::Shader(const std::string fragmentPath, const std::string vertexPath)
    {
        unsigned int vertexShader, fragmentShader, shaderProgram;

        vertexShader = ReadShader(vertexPath, ShaderType::VERTEX);
        fragmentShader = ReadShader(fragmentPath, ShaderType::FRAGMENT);
        shaderProgram = glCreateProgram();

        glAttachShader(shaderProgram, vertexShader);
        glAttachShader(shaderProgram, fragmentShader);
        glLinkProgram(shaderProgram);

        int success;
        char infoLog[512];

        // Check the program compile-time errors
        glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
        if (!success)
        {
            glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
            throw std::runtime_error("[SHADER] Program link failed " + std::string(infoLog));
        }

        m_id = shaderProgram;
    }

    Shader::~Shader()
    {
        Logger::Info("[SHADER] Deleting shader with id " + std::to_string(m_id));
        glDeleteProgram(m_id);
    }

    void Shader::Bind() const
    {
        glUseProgram(m_id);
    }

    void Shader::Unbind() const
    {
        glUseProgram(0);
    }

    void Shader::SetBool(const std::string &name, bool value) const
    {
        glUniform1i(glGetUniformLocation(m_id, name.c_str()), (int)value);
    }
    void Shader::SetInt(const std::string &name, int value) const
    {
        glUniform1i(glGetUniformLocation(m_id, name.c_str()), value);
    }
    void Shader::SetFloat(const std::string &name, float value) const
    {
        glUniform1f(glGetUniformLocation(m_id, name.c_str()), value);
    }
}