#ifndef EVENT_H
#define EVENT_H

#include <string>
#include <functional>

namespace Hydro
{
    // clang-format off
    enum class EventType
    {
        None = 0,
        KeyPress, KeyRelease
    };
    // clang-format on

    class Event
    {
    public:
        virtual EventType getType() = 0;
        virtual std::string toString() = 0;

    private:
        bool m_handled = false;
    };
}

#endif