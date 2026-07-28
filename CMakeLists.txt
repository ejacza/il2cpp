
cmake_minimum_required(VERSION 3.10)
project(Demo VERSION 1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(ANDROID_STL c++_static)

find_library(ANDROID_LOG_LIB log)
add_subdirectory(xdl)

set(DOBBY_DEBUG OFF CACHE BOOL "" FORCE)
set(DOBBY_BUILD_EXAMPLE OFF CACHE BOOL "" FORCE)
set(DOBBY_BUILD_TEST OFF CACHE BOOL "" FORCE)
set(Plugin.Android.BionicLinkerUtil ON CACHE BOOL "" FORCE)
add_subdirectory(Dobby)

set(RUST_TARGET_PREFIX aarch64-linux-android)
if(${ANDROID_ABI} STREQUAL "armeabi-v7a")
    set(RUST_TARGET_PREFIX armv7-linux-androideabi)
endif()

set(RUST_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/il2cpp/target/${RUST_TARGET_PREFIX}/release/libil2cpp_dump.a")

add_custom_target(rust_build ALL
    COMMAND cargo build --release --target ${RUST_TARGET_PREFIX}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/il2cpp
    COMMENT "Building Rust il2cpp dump library for ${ANDROID_ABI}"
)

add_library(rust_il2cpp STATIC IMPORTED)
set_target_properties(rust_il2cpp PROPERTIES IMPORTED_LOCATION ${RUST_LIB_PATH})
add_dependencies(rust_il2cpp rust_build)

add_library(Demo SHARED Main.cpp)
target_include_directories(Demo PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/xdl/include)
target_link_libraries(Demo PRIVATE xdl dobby_static ${ANDROID_LOG_LIB} dl rust_il2cpp)

target_compile_options(Demo PRIVATE -O3 -DNDEBUG -ffunction-sections -fdata-sections
    -fvisibility=hidden -fvisibility-inlines-hidden -flto -s)
target_link_options(Demo PRIVATE -O3 -flto -Wl,--gc-sections -Wl,--strip-all
    -Wl,--exclude-libs,ALL -Wl,-z,relro -Wl,-z,now -Wl,--icf=all)
