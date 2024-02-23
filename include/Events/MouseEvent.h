#ifndef MOUSE_EVENT_H
#define MOUSE_EVENT_H

#include "Events/Event.h"

namespace Hydro
{
    enum class MouseButton
    {
        Right,
        Left,
    };

    class MouseMoveEvent : public Event
    {
    public:
        MouseMoveEvent(double posX, double posY) : m_posX(posX), m_posY(posY) {}
        ~MouseMoveEvent() = default;

        EventType GetType() const;
        std::string ToString() const;

        double GetPosX() const;
        double GetPosY() const;

    private:
        double m_posX, m_posY;
    };

    class MousePress : public Event
    {
    public:
        MousePress(double posX, double posY, MouseButton button) : m_posX(posX), m_posY(posY), m_button(button) {}
        ~MousePress() = default;

        EventType GetType() const;
        std::string ToString() const;

        double GetPosX() const;
        double GetPosY() const;
        MouseButton GetButton() const;

    private:
        double m_posX, m_posY;
        MouseButton m_button;
    };
}

#endif