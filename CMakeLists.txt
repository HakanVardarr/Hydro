cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 17)

project(Hydro)

include_directories(
    ${CMAKE_SOURCE_DIR}/include 
    ${CMAKE_SOURCE_DIR}/vendor
)

file(GLOB CORE_SOURCES "src/Core/*.cpp" "src/Graphics/*.cpp")
list(APPEND SOURCES ${CORE_SOURCES})

# Adding the rest of the source files
aux_source_directory(src SOURCES)

link_directories(
    ${CMAKE_SOURCE_DIR}/lib 
)

add_executable(${PROJECT_NAME} main.cpp ${SOURCES} vendor/glad.c)
target_link_libraries(${PROJECT_NAME} glfw.3)