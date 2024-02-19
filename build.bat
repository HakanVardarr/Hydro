@echo off

MSBuild.exe .\build\Hydro.sln /verbosity:q
.\build\Debug\Hydro.exe