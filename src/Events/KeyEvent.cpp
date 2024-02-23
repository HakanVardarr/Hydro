#include "Events/KeyEvent.h"

namespace Hydro
{
    EventType KeyPressEvent::GetType() const
    {
        return EventType::KeyPress;
    }

    std::string KeyPressEvent::ToString() const
    {
        std::stringstream ss;
        ss << "[KEY PRESS] Key Pressed: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyPressEvent::GetKey() const
    {
        return m_key;
    }

    EventType KeyReleaseEvent::GetType() const
    {
        return EventType::KeyRelease;
    }

    std::string KeyReleaseEvent::ToString() const
    {
        std::stringstream ss;
        ss << "[KEY RELEASE] Key Released: " << (int)m_key;
        return ss.str();
    }

    KeyCode KeyReleaseEvent::GetKey() const
    {
        return m_key;
    }
}
