#include "Events/MouseEvent.h"

#include <string>
#include <sstream>

namespace Hydro
{
    EventType MouseMoveEvent::GetType() const
    {
        return EventType::MouseMove;
    }

    std::string MouseMoveEvent::ToString() const
    {
        std::stringstream ss;
        ss << "[MOUSE MOVE] Mouse position: (" << m_posX << "," << m_posY << ")";
        return ss.str();
    }

    double MouseMoveEvent::GetPosX() const
    {
        return m_posX;
    }

    double MouseMoveEvent::GetPosY() const
    {
        return m_posY;
    }

    EventType MousePress::GetType() const
    {
        return EventType::MousePress;
    }

    std::string MousePress::ToString() const
    {
        std::stringstream ss;
        ss << "[MOUSE CLICK] Mouse position: (" << m_posX << "," << m_posY << ")";
        return ss.str();
    }

    double MousePress::GetPosX() const
    {
        return m_posX;
    }

    double MousePress::GetPosY() const
    {
        return m_posY;
    }

    MouseButton MousePress::GetButton() const
    {
        return m_button;
    }
}
