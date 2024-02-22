#ifndef LOGGER_H
#define LOGGER_H

#include "spdlog/spdlog.h"

namespace Hydro
{
    class Logger
    {
    public:
        static void info(std::string message);
    };
}

#endif