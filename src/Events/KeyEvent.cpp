#include "Events/KeyEvent.h"

namespace Hydro
{
    EventType KeyPressEvent::getType()
    {
        return EventType::KeyPress;
    }

    std::string KeyPressEvent::toString()
    {
        std::stringstream ss;
        ss << "Key Pressed: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyPressEvent::getKey()
    {
        return m_key;
    }

    EventType KeyReleaseEvent::getType()
    {
        return EventType::KeyRelease;
    }

    std::string KeyReleaseEvent::toString()
    {
        std::stringstream ss;
        ss << "Key Released: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyReleaseEvent::getKey()
    {
        return m_key;
    }
}
