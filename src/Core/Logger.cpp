#include "Core/Logger.h"

namespace Hydro
{
    void Logger::Info(std::string message)
    {
        spdlog::info(message);
    }

    void Logger::Error(std::string message)
    {
        spdlog::error(message);
    }
}