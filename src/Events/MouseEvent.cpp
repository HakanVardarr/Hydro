#include "Events/MouseEvent.h"

#include <string>
#include <sstream>

namespace Hydro
{
    EventType MouseMoveEvent::getType() const
    {
        return EventType::MouseMove;
    }

    std::string MouseMoveEvent::toString() const
    {
        std::stringstream ss;
        ss << "[MOUSE MOVE] Mouse position: (" << m_posX << "," << m_posY << ")";
        return ss.str();
    }

    double MouseMoveEvent::getPosX() const
    {
        return m_posX;
    }

    double MouseMoveEvent::getPosY() const
    {
        return m_posY;
    }

    EventType MousePress::getType() const
    {
        return EventType::MousePress;
    }

    std::string MousePress::toString() const
    {
        std::stringstream ss;
        ss << "[MOUSE CLICK] Mouse position: (" << m_posX << "," << m_posY << ")";
        return ss.str();
    }

    double MousePress::getPosX() const
    {
        return m_posX;
    }

    double MousePress::getPosY() const
    {
        return m_posY;
    }

    MouseButton MousePress::getButton() const
    {
        return m_button;
    }
}
