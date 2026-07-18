cmake_minimum_required(VERSION 3.10)
project(Demo VERSION 1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(ANDROID_STL c++_static)

find_library(ANDROID_LOG_LIB log)
add_subdirectory(xdl)

# --- Dobby (inline hook) as a static library linked into Demo ---
set(DOBBY_DEBUG OFF CACHE BOOL "" FORCE)
set(DOBBY_BUILD_EXAMPLE OFF CACHE BOOL "" FORCE)
set(DOBBY_BUILD_TEST OFF CACHE BOOL "" FORCE)
set(Plugin.Android.BionicLinkerUtil ON CACHE BOOL "" FORCE)
add_subdirectory(Dobby)

add_library(Demo SHARED
    Main.cpp
    il2cpp/il2cpp_dump.cpp
)

target_include_directories(Demo PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/il2cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/xdl/include
    ${CMAKE_CURRENT_SOURCE_DIR}/Dobby/include
)

target_link_libraries(Demo PRIVATE
    xdl
    dobby_static
    ${ANDROID_LOG_LIB}
    dl
)

target_compile_options(Demo PRIVATE
    -O3
    -DNDEBUG
    -ffunction-sections
    -fdata-sections
    -fvisibility=hidden
    -fvisibility-inlines-hidden
    -flto
    -s
)

target_link_options(Demo PRIVATE
    -O3
    -flto
    -Wl,--gc-sections
    -Wl,--strip-all
    -Wl,--exclude-libs,ALL
    -Wl,-z,relro
    -Wl,-z,now
    -Wl,--icf=all
)
