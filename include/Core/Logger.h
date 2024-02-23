#ifndef LOGGER_H
#define LOGGER_H

#include "spdlog/spdlog.h"

namespace Hydro
{
    class Logger
    {
    public:
        static void Info(std::string message);
        static void Error(std::string message);
    };
}

#endif