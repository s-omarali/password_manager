cmake_minimum_required(VERSION 3.20)
project(password_manager CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()

option(ENABLE_SANITIZERS "Build with AddressSanitizer + UndefinedBehaviorSanitizer" ON)

add_compile_options(-Wall -Wextra -Wpedantic)

if(ENABLE_SANITIZERS AND CMAKE_BUILD_TYPE STREQUAL "Debug")
  add_compile_options(-fsanitize=address,undefined -fno-omit-frame-pointer -g)
  add_link_options(-fsanitize=address,undefined)
endif()

find_package(PkgConfig REQUIRED)
pkg_check_modules(SODIUM REQUIRED libsodium)

add_subdirectory(src/core)
add_subdirectory(src/cli)

# src/gui is wired in once Dear ImGui is added (M4).
