#include "Events/KeyEvent.h"

namespace Hydro
{
    EventType KeyPressEvent::getType() const
    {
        return EventType::KeyPress;
    }

    std::string KeyPressEvent::toString() const
    {
        std::stringstream ss;
        ss << "[KEY PRESS] Key Pressed: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyPressEvent::getKey() const
    {
        return m_key;
    }

    EventType KeyReleaseEvent::getType() const
    {
        return EventType::KeyRelease;
    }

    std::string KeyReleaseEvent::toString() const
    {
        std::stringstream ss;
        ss << "[KEY RELEASE] Key Released: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyReleaseEvent::getKey() const
    {
        return m_key;
    }
}
