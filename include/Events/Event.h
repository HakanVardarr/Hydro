#ifndef EVENT_H
#define EVENT_H

#include <string>

namespace Hydro
{
    // clang-format off
    enum class EventType
    {
        None = 0,
        KeyPress, KeyRelease,
        MouseMove, MousePress, MouseRelease
    };
    // clang-format on

    class Event
    {
    public:
        virtual ~Event() = default;
        virtual EventType getType() const = 0;
        virtual std::string toString() const = 0;
    };
}

#endif