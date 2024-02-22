#ifndef KEY_EVENT_H
#define KEY_EVENT_H

#include "Events/Event.h"

#include <string>
#include <sstream>

namespace Hydro
{
    // clang-format off
    enum class KeyCode
    {
        Key0 = 48, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key10,
        KeySemiColon, KeyEqual, 
        KeyA = 65, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
        KeyEscape = 256, KeyEnter, KeyTab, KeyBackSpace, KeyInsert, KeyDelete,
        KeyRight, KeyLeft, KeyDown, KeyUp,
        // Function keys
        KeyF1 = 290, KeyF2, KeyF3, KeyF4, KeyF5, KeyF6, KeyF7, KeyF8, KeyF9, KeyF10, KeyF11, KeyF12,
        // Number pad keys
        KeyKP0 = 320, KeyKP1, KeyKP2, KeyKP3, KeyKP4, KeyKP5, KeyKP6, KeyKP7, KeyKP8, KeyKP9,
        KeyKPDecimal, KeyKPDivide, KeyKPMultiply, KeyKPSubtract, KeyKPAdd, KeyKPEnter, KeyKPEqual,
        // Modifier keys
        KeyLeftShift = 340, KeyLeftControl, KeyLeftAlt, KeyLeftSuper, KeyRightShift, KeyRightControl, KeyRightAlt, KeyRightSuper,
        KeyMenu
    };
    // clang-format on

    class KeyPressEvent : public Event
    {
    public:
        KeyPressEvent(int key) : m_key((KeyCode)key) {}
        ~KeyPressEvent() = default;

        EventType getType();
        std::string toString();
        KeyCode getKey();

    private:
        KeyCode m_key;
    };

    class KeyReleaseEvent : public Event
    {
    public:
        KeyReleaseEvent(int key) : m_key((KeyCode)key) {}
        ~KeyReleaseEvent() = default;

        EventType getType();
        std::string toString();
        KeyCode getKey();

    private:
        KeyCode m_key;
    };
}

#endif