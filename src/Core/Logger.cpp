#include "Core/Logger.h"

namespace Hydro
{
    void Logger::info(std::string message)
    {
        spdlog::info(message);
    }
}