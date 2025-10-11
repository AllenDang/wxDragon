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
set(wxdUSE_MEDIACTRL ON CACHE BOOL "Use Media Ctrl widgets")
set(wxdUSE_STC ON CACHE BOOL "Use Styled Text Control widget")
set(wxdUSE_XRC ON CACHE BOOL "Use XML Resource (XRC) support")
set(wxdUSE_WEBVIEW ON CACHE BOOL "Use the Webview widget")
set(wxdUSE_RICHTEXT ON CACHE BOOL "Use Rich Text Control widget")

# --- Output Directories ---
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# If WXWIDGETS_LIB_DIR is not set or does not exist
if(NOT DEFINED WXWIDGETS_LIB_DIR OR NOT EXISTS "${WXWIDGETS_LIB_DIR}")
    message(STATUS "CMAKE_CURRENT_SOURCE_DIR = ${CMAKE_CURRENT_SOURCE_DIR}")
    # Auto-detect the prebuilt wxWidgets directory
    message(STATUS "WXWIDGETS_LIB_DIR not set or does not exist, auto-detecting...")
    file(GLOB WXWIDGETS_LIBS_PATH
        "${CMAKE_CURRENT_SOURCE_DIR}/../../../target/wxwidgets-*"
    )
    if(NOT WXWIDGETS_LIBS_PATH)
        message(FATAL_ERROR "Could not find prebuilt wxWidgets directory. Please run 'cargo build' first to download prebuilt libraries.")
    endif()
    # Use the first match (there should only be one)
    list(GET WXWIDGETS_LIBS_PATH 0 WXWIDGETS_LIB_DIR)
    message(STATUS "Auto-detected wxWidgets at: ${WXWIDGETS_LIB_DIR}")
else()
    message(STATUS "Using provided WXWIDGETS_LIB_DIR: ${WXWIDGETS_LIB_DIR}")
endif()

# --- Option for wxWidgets Library Directory (passed from build.rs) ---
option(WXWIDGETS_LIB_DIR "Path to the pre-built wxWidgets libraries" "")
if(NOT WXWIDGETS_LIB_DIR OR NOT EXISTS "${WXWIDGETS_LIB_DIR}")
    message(FATAL_ERROR "WXWIDGETS_LIB_DIR ('${WXWIDGETS_LIB_DIR}') is not set or does not exist. It should be passed from the Rust build script via -D.")
endif()
message(STATUS "Using pre-built wxWidgets libraries from: ${WXWIDGETS_LIB_DIR}")

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
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treebook.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treectrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/treelistctrl.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/window.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_utils.h
    ${CMAKE_CURRENT_SOURCE_DIR}/src/wxd_sysopt.cpp
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
    # list(APPEND WXDRAGON_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/webview.cpp)
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

# --- Find and Add wxWidgets Include Directories ---
set(WX_INCLUDE_PATHS)

# First, add main wxWidgets source headers (required for wx/wx.h, wx/arrstr.h, etc.)
set(WX_MAIN_INCLUDE_DIR "${WXWIDGETS_LIB_DIR}/include")
if(EXISTS "${WX_MAIN_INCLUDE_DIR}")
    list(APPEND WX_INCLUDE_PATHS "${WX_MAIN_INCLUDE_DIR}")
    message(STATUS "Found main wxWidgets headers: ${WX_MAIN_INCLUDE_DIR}")
else()
    message(FATAL_ERROR "Cannot find main wxWidgets include directory at ${WX_MAIN_INCLUDE_DIR}")
endif()

# Second, add platform-specific generated headers (for setup.h, etc.)
if(PLATFORM_NAME STREQUAL "macos")
    # For macOS, check multiple possible locations for setup.h
    set(WX_PLATFORM_DIRS_TO_CHECK
        "${WXWIDGETS_LIB_DIR}/osx_cocoa-unicode-static-3.3"
        "${WXWIDGETS_LIB_DIR}/wx/include/osx_cocoa-unicode-static-3.3"
    )
    foreach(dir IN LISTS WX_PLATFORM_DIRS_TO_CHECK)
        if(EXISTS "${dir}")
            list(APPEND WX_INCLUDE_PATHS "${dir}")
            message(STATUS "Found macOS-specific headers: ${dir}")
            break()
        endif()
    endforeach()
elseif(PLATFORM_NAME STREQUAL "windows")
    # For Windows builds, ensure setup.h is available at the expected location
    # Copy from the working setup.h in include/wx/msw to include/wx if needed
    set(WX_MSW_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/msw/setup.h")
    set(WX_TARGET_SETUP "${WXWIDGETS_LIB_DIR}/include/wx/setup.h")
    
    if(EXISTS "${WX_MSW_SETUP}" AND NOT EXISTS "${WX_TARGET_SETUP}")
        file(COPY "${WX_MSW_SETUP}" DESTINATION "${WXWIDGETS_LIB_DIR}/include/wx/")
        message(STATUS "Copied Windows setup.h from msw to wx directory")
    endif()
    
    if(EXISTS "${WX_TARGET_SETUP}")
        message(STATUS "Windows setup.h available at: ${WX_TARGET_SETUP}")
    else()
        message(WARNING "Windows setup.h not found at expected location")
    endif()
elseif(PLATFORM_NAME STREQUAL "linux")
    # For Linux, check multiple possible locations for setup.h
    set(WX_PLATFORM_DIRS_TO_CHECK
        "${WXWIDGETS_LIB_DIR}/gtk3-unicode-static-3.3"
        "${WXWIDGETS_LIB_DIR}/wx/include/gtk3-unicode-static-3.3"
    )
    foreach(dir IN LISTS WX_PLATFORM_DIRS_TO_CHECK)
        if(EXISTS "${dir}")
            list(APPEND WX_INCLUDE_PATHS "${dir}")
            message(STATUS "Found Linux-specific headers: ${dir}")
            break()
        endif()
    endforeach()
endif()

message(STATUS "Using wxWidgets include directories: ${WX_INCLUDE_PATHS}")
target_include_directories(wxdragon PRIVATE ${WX_INCLUDE_PATHS})

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
target_link_directories(wxdragon PRIVATE ${WXWIDGETS_LIB_DIR})

message(STATUS "wxDragon wrapper library configured successfully for ${PLATFORM_NAME}")
message(STATUS "CMake configuration for libwxdragon finished.")
