cmake_minimum_required(VERSION 3.15)
project(wxDragon LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# --- Helper Function: Convert CMake boolean values to C preprocessor 1/0 values
function(bool_to_int input_var output_var)
    if(${input_var})
        set(${output_var} 1 PARENT_SCOPE)
    else()
        set(${output_var} 0 PARENT_SCOPE)
    endif()
endfunction()

# --- Feature Flags (from Rust build.rs) ---
set(wxdUSE_AUI ON CACHE BOOL "Use AUI (Advanced User Interface)")
set(wxdUSE_MEDIACTRL OFF CACHE BOOL "Use Media Ctrl widgets, not completed yet, so default to OFF")
set(wxdUSE_STC ON CACHE BOOL "Use Styled Text Control widget")
set(wxdUSE_XRC ON CACHE BOOL "Use XML Resource (XRC) support")
set(wxdUSE_WEBVIEW ON CACHE BOOL "Use the Webview widget")
set(wxdUSE_WEBVIEW_EDGE ON CACHE BOOL "Use Edge/WebView2 backend (modern Chromium-based, preferred over IE)")
set(wxdUSE_RICHTEXT ON CACHE BOOL "Use Rich Text Control widget")

# --- Output Directories ---
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# --- Feature Flags (from Rust build.rs) ---
set(wxUSE_AUI ${wxdUSE_AUI} CACHE BOOL "Use AUI (Advanced User Interface)")
set(wxUSE_MEDIACTRL ${wxdUSE_MEDIACTRL} CACHE BOOL "Use Media Ctrl widgets")
set(wxUSE_STC ${wxdUSE_STC} CACHE BOOL "Use Styled Text Control widget")
set(wxUSE_XRC ON CACHE BOOL "Use XML Resource (XRC) support, always enabled, cannot reuse from wxdUSE_XRC. We don't know the reason why.")
set(wxUSE_WEBVIEW ${wxdUSE_WEBVIEW} CACHE BOOL "Use the Webview widget")
set(wxUSE_WEBVIEW_EDGE ${wxdUSE_WEBVIEW_EDGE} CACHE BOOL "Use Edge/WebView2 backend")
# For MinGW (GNU) builds, we must use dynamic loading because the Microsoft WebView2LoaderStatic.lib
# is in COFF format (MSVC-only). MinGW needs to dynamically load WebView2Loader.dll at runtime.
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR MINGW)
    set(wxUSE_WEBVIEW_EDGE_STATIC OFF CACHE BOOL "Use Edge/WebView2 with static loader (disabled for MinGW)" FORCE)
    message(STATUS "WebView2 static loader disabled for MinGW - will use dynamic loading")
else()
    set(wxUSE_WEBVIEW_EDGE_STATIC ON CACHE BOOL "Use Edge/WebView2 with static loader (no WebView2Loader.dll needed)")
endif()
set(wxUSE_WEBVIEW_IE ON CACHE BOOL "Use IE backend (fallback for systems without WebView2)")
set(wxUSE_WEBVIEW_WEBKIT ON CACHE BOOL "Use WebKit backend (required for macOS)")
set(wxUSE_RICHTEXT ${wxdUSE_RICHTEXT} CACHE BOOL "Use Rich Text Control widget")

set(wxUSE_GUI ON CACHE BOOL "Use wxWidgets GUI features")
set(wxUSE_BASE ON CACHE BOOL "Use wxWidgets base features")
set(wxUSE_ADV ON CACHE BOOL "Use wxWidgets advanced features")
set(wxUSE_HTML ON CACHE BOOL "Use wxWidgets HTML features")
set(wxUSE_OPENGL ON CACHE BOOL "Use wxWidgets OpenGL features")
set(wxUSE_SOCKETS ON CACHE BOOL "Use wxWidgets sockets")
set(wxUSE_PROPGRID ON CACHE BOOL "Use wxWidgets property grid")
set(wxUSE_DEBUGREPORT ON CACHE BOOL "Use wxWidgets debug report features")
set(wxUSE_RIBBON ON CACHE BOOL "Use wxWidgets ribbon control")
set(wxUSE_XML ON CACHE BOOL "Use XML support for XRC. Since wxUSE_XRC is always ON, wxUSE_XML must also be ON.")
set(wxUSE_LIBWEBP OFF CACHE BOOL "Disable WebP support to avoid extra dependencies")

# wxWidgets source directory, normally this needn't be changed, but can be adjusted if needed
set(WXWIDGETS_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../../target/debug/wxWidgets CACHE PATH "Path to the wxWidgets libraries")
message(STATUS "Using wxWidgets libraries source from: ${WXWIDGETS_LIB_DIR}")

# wxWidgets build directory, note that this can be changed to a different path if needed
set(WXWIDGETS_BUILD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../../target/wxwidgets_cmake_build CACHE PATH "Path to the wxWidgets build directory")
message(STATUS "Building wxWidgets binaries to: ${WXWIDGETS_BUILD_DIR}")

set(wxBUILD_SHARED OFF CACHE BOOL "Build wxWidgets as static libraries")
set(wxBUILD_MONOLITHIC OFF CACHE BOOL "Build wxWidgets as monolithic library")
set(wxBUILD_SAMPLES OFF CACHE BOOL "Do not build wxWidgets samples")
set(wxBUILD_TESTS OFF CACHE BOOL "Do not build wxWidgets tests")
set(wxBUILD_DEMOS OFF CACHE BOOL "Do not build wxWidgets demos")
set(wxBUILD_BENCHMARKS OFF CACHE BOOL "Do not build wxWidgets benchmarks")

set(wxUSE_EXCEPTIONS ON CACHE BOOL "Enable wxWidgets exceptions")

# Set CMAKE_TLS_VERIFY if provided via environment (for CI SSL bypass when downloading WebView2 SDK)
# This must be set BEFORE add_subdirectory() to ensure it propagates to the nested wxWidgets build
if(DEFINED ENV{CMAKE_TLS_VERIFY})
    set(CMAKE_TLS_VERIFY $ENV{CMAKE_TLS_VERIFY} CACHE STRING "TLS verification for file downloads" FORCE)
    message(STATUS "CMAKE_TLS_VERIFY set to: $ENV{CMAKE_TLS_VERIFY} (from environment)")
    # Also set it as a regular (non-cache) variable to ensure it's used immediately in this scope
    set(CMAKE_TLS_VERIFY $ENV{CMAKE_TLS_VERIFY})
endif()

add_subdirectory(${WXWIDGETS_LIB_DIR} ${WXWIDGETS_BUILD_DIR})

# --- Platform Detection ---
if(CMAKE_SYSTEM_NAME MATCHES "Darwin")
    message(STATUS "Building for macOS")
    set(PLATFORM_NAME "macos")
    # Define platform-specific preprocessor macros
    add_compile_definitions(__WXOSX_COCOA__ __WXMAC__ __WXOSX__ _FILE_OFFSET_BITS=64 NDEBUG)
elseif(CMAKE_SYSTEM_NAME MATCHES "Windows")
    message(STATUS "Building for Windows")
    set(PLATFORM_NAME "windows")
    # Define WXD_TARGET_WINDOWS for cross-compilation support in platform-aware components
    add_compile_definitions(WXD_TARGET_WINDOWS __WXMSW__ _FILE_OFFSET_BITS=64 wxUSE_UNICODE=1 NDEBUG)
    
    # Windows requires UNICODE and _UNICODE for both MSVC and MinGW/GNU builds
    add_compile_definitions(UNICODE _UNICODE)
    message(STATUS "Added Windows Unicode definitions for all compilers")

elseif(CMAKE_SYSTEM_NAME MATCHES "Linux")
    message(STATUS "Building for Linux")
    set(PLATFORM_NAME "linux")
    add_compile_definitions(__WXGTK__ _FILE_OFFSET_BITS=64 NDEBUG)
    
    # Always enable position independent code on Linux to prevent PIE-related linking errors
    # This is the standard approach for modern Linux distributions
    set(CMAKE_POSITION_INDEPENDENT_CODE ON)
    message(STATUS "Enabled position independent code (-fPIE) for Linux compatibility")
else()
    message(FATAL_ERROR "Unsupported platform: ${CMAKE_SYSTEM_NAME}")
endif()

# --- wxDragon Library Sources ---
set(WXDRAGON_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/core/timer.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/activity_indicator.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/animation_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/app.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/appprogress.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/array_string.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/art_provider.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmap.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmap_button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapbundle.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/bitmapcombobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/calendar_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/checkbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/checklistbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/choice.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/clipboard.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/collapsiblepane.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/cursor.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/colourdialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/colourpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/combobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/commandlinkbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataobject.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataview.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewtreectrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewtreemodel.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewvirtuallistmodel.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dataviewvirtuallistmodel_custom.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/datepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/datetime.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dc.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dir_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dirpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/dnd.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/droptarget.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/editablelistbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/event.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/file_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/file_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/filepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/font_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/fontpickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/frame.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/gauge.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/hyperlink_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/imagelist.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/item.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/list_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/listbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/menu.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/message_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/multi_choice_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/notebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/simplebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/notificationmessage.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/panel.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/progressdialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/radio_button.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/radiobox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/rearrangelist.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/scrollbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/scrolled_window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/search_ctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/single_choice_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/sizer.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/slider.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinbutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/spinctrldouble.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/splitterwindow.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_bitmap.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/generic_static_bitmap.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/platform_aware_staticbitmap_handler.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_line.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/static_text.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/staticbox.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/statusbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/taskbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/text_entry_dialog.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/textctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/timepickerctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/togglebutton.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/toolbar.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/translations.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treectrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treelistctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.h
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_sysopt.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_logging.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_variant.cpp
)

# Add Linux compatibility layer for older systems
if(PLATFORM_NAME STREQUAL "linux")
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/linux_compat.cpp)
    message(STATUS "Added Linux compatibility layer (GLib + glibc compatibility)")
endif()

# Add conditional sources based on features
if (wxdUSE_AUI)
    list(APPEND WXDRAGON_SOURCES 
        ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_manager.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_mdi_child_frame.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_mdi_parent_frame.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_notebook.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/aui_toolbar.cpp
    )
endif()

if (wxdUSE_MEDIACTRL)
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/media_ctrl.cpp)
endif()

if (wxdUSE_STC)
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/styledtextctrl.cpp)
endif()

if (wxdUSE_XRC)
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/xrc.cpp)
endif()

if (wxdUSE_WEBVIEW)
    # Add webview wrapper when implemented
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/webview.cpp)
endif()

if (wxdUSE_RICHTEXT)
    list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/richtextctrl.cpp)
endif()

message(STATUS "wxDragon sources: ${WXDRAGON_SOURCES}")

# --- Create wxDragon Static Library ---
add_library(wxdragon STATIC ${WXDRAGON_SOURCES})

# --- Set Include Directories ---
target_include_directories(wxdragon PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# --- Set link libraries ---
target_link_libraries(wxdragon PRIVATE wx::base)
if(wxUSE_GUI)
    target_link_libraries(wxdragon PRIVATE wx::core wx::adv)
endif()
if(wxUSE_AUI)
    target_link_libraries(wxdragon PRIVATE wx::aui)
endif()
if(wxUSE_OPENGL)
    target_link_libraries(wxdragon PRIVATE wx::gl)
endif()
if(wxUSE_HTML)
    target_link_libraries(wxdragon PRIVATE wx::html)
endif()
if(wxUSE_MEDIACTRL)
    target_link_libraries(wxdragon PRIVATE wx::media)
endif()
if(wxUSE_SOCKETS)
    target_link_libraries(wxdragon PRIVATE wx::net)
endif()
if(wxUSE_PROPGRID)
    target_link_libraries(wxdragon PRIVATE wx::propgrid)
endif()
if(wxUSE_DEBUGREPORT)
    target_link_libraries(wxdragon PRIVATE wx::qa)
endif()
if(wxUSE_RIBBON)
    target_link_libraries(wxdragon PRIVATE wx::ribbon)
endif()
if(wxUSE_RICHTEXT)
    target_link_libraries(wxdragon PRIVATE wx::richtext)
endif()
if(wxUSE_STC)
    target_link_libraries(wxdragon PRIVATE wx::stc)
endif()
if(wxUSE_WEBVIEW)
    target_link_libraries(wxdragon PRIVATE wx::webview)
endif()
if(wxUSE_XML)
    target_link_libraries(wxdragon PRIVATE wx::xml)
endif()
if(wxUSE_XRC)
    target_link_libraries(wxdragon PRIVATE wx::xrc)
endif()

# --- Set Preprocessor Definitions Based on Features ---
bool_to_int(wxdUSE_AUI aui_value)
bool_to_int(wxdUSE_MEDIACTRL mediactrl_value)
bool_to_int(wxdUSE_WEBVIEW webview_value)
bool_to_int(wxdUSE_STC stc_value)
bool_to_int(wxdUSE_XRC xrc_value)
bool_to_int(wxdUSE_RICHTEXT richtext_value)

target_compile_definitions(wxdragon PRIVATE 
    wxdUSE_AUI=${aui_value}
    wxdUSE_MEDIACTRL=${mediactrl_value}
    wxdUSE_WEBVIEW=${webview_value}
    wxdUSE_STC=${stc_value}
    wxdUSE_XRC=${xrc_value}
    wxdUSE_RICHTEXT=${richtext_value}
)

# --- Add Library Search Directory ---
target_link_directories(wxdragon PRIVATE ${WXWIDGETS_BUILD_DIR})

message(STATUS "wxDragon wrapper library configured successfully for ${PLATFORM_NAME}")
message(STATUS "CMake configuration for libwxdragon finished.")
