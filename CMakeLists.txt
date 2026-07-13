cmake_minimum_required(VERSION 3.10)
project(Demo VERSION 1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Use static C++ runtime to avoid dependency on libc++_shared.so
set(ANDROID_STL c++_static)

# Define Android log library (if targeting Android)
find_library(ANDROID_LOG_LIB log)

# ── xdl static library ──────────────────────────────────
add_subdirectory(xdl)

# ── Demo main library (shared) ──────────────────────────
add_library(Demo SHARED
    Main.cpp
    il2cpp/il2cpp_dump.cpp
)

target_include_directories(Demo PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/il2cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/xdl/include
)

# Link libraries
target_link_libraries(Demo PRIVATE
    xdl                       # static link xdl into Demo
    ${ANDROID_LOG_LIB}        # Android logging
    dl                        # dynamic linker
)

target_compile_options(Demo PRIVATE
    -Wall -Wextra
    -fvisibility=hidden
    -fvisibility-inlines-hidden
)
