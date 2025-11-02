cmake_minimum_required(VERSION 4.1.0)
project(ylang LANGUAGES CXX)

include("${CMAKE_CURRENT_SOURCE_DIR}/vendor/magic_enum.cmake")

set(YLANG_TARGET_PROPERTIES
    CXX_STANDARD 26
    C_EXTENSIONS OFF
    CXX_EXTENSIONS OFF
    CXX_STANDARD_REQUIRED ON
)

if (CMAKE_BUILD_TYPE MATCHES "Debug")
    list(APPEND YLANG_TARGET_PROPERTIES
        EXPORT_COMPILE_COMMANDS ON
    )
endif()

file(GLOB_RECURSE YLANG_SOURCES src/*.cpp)
add_executable(ylang ${YLANG_SOURCES})
target_include_directories(ylang
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}
)
set_target_properties(ylang PROPERTIES ${YLANG_TARGET_PROPERTIES})
target_link_libraries(ylang
    PRIVATE magic_enum::magic_enum
)
