#ifndef KEY_EVENT_H
#define KEY_EVENT_H

#include "Core/Event.h"

#include <string>
#include <sstream>

namespace Hydro
{
    class KeyPressEvent : public Event
    {
    public:
        KeyPressEvent(int key) : m_key(key) {}
        ~KeyPressEvent() = default;

        EventType getType()
        {
            return EventType::KeyPress;
        }

        std::string toString()
        {
            std::stringstream ss;
            ss << "Key Pressed: " << m_key;
            return ss.str();
        }

    private:
        int m_key;
    };
}

#endif