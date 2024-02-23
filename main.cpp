#include "Application.h"

int main()
{
    try
    {
        Application app;

        app.Run();

        return 0;
    }
    catch (std::runtime_error &err)
    {
        Hydro::Logger::Error(err.what());
        return 1;
    }
}