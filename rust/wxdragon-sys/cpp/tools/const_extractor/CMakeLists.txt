cmake_minimum_required(VERSION 3.16)
project(const_extractor)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WXWIDGETS_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../../../../target/debug/wxWidgets CACHE PATH "Path to the wxWidgets libraries")
message(STATUS "Using wxWidgets libraries source from: ${WXWIDGETS_LIB_DIR}")

# wxWidgets build directory
set(WXWIDGETS_BUILD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../../../../target/wxwidgets_cmake_build CACHE PATH "Path to the wxWidgets build directory")
message(STATUS "Building wxWidgets binaries to: ${WXWIDGETS_BUILD_DIR}")

set(wxBUILD_SHARED OFF CACHE BOOL "Build wxWidgets as static libraries")
set(wxBUILD_MONOLITHIC OFF CACHE BOOL "Build wxWidgets as monolithic library")
set(wxBUILD_SAMPLES OFF CACHE BOOL "Do not build wxWidgets samples")
set(wxBUILD_TESTS OFF CACHE BOOL "Do not build wxWidgets tests")
set(wxBUILD_DEMOS OFF CACHE BOOL "Do not build wxWidgets demos")
set(wxBUILD_BENCHMARKS OFF CACHE BOOL "Do not build wxWidgets benchmarks")

set(wxUSE_EXCEPTIONS ON CACHE BOOL "Enable wxWidgets exceptions")

add_subdirectory(${WXWIDGETS_LIB_DIR} ${WXWIDGETS_BUILD_DIR})

add_executable(const_extractor main.cpp)

target_link_libraries(const_extractor PRIVATE wx::base wx::core)
# target_link_libraries(const_extractor PRIVATE wx::adv wx::aui wx::gl wx::html wx::media wx::net wx::propgrid)
# target_link_libraries(const_extractor PRIVATE wx::qa wx::ribbon wx::richtext wx::stc wx::webview wx::xrc wx::xml)

message(STATUS "const_extractor configured successfully") 
