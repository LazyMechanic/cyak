cmake_minimum_required(VERSION 3.13)

set(_PROJECT_NAME          {{.Name}})
set(_PROJECT_LANGUAGE      {{.Language}})

set(_PROJECT_MAJOR_VERSION {{.MajorVersion}})
set(_PROJECT_MINOR_VERSION {{.MinorVersion}})
set(_PROJECT_PATCH_VERSION {{.PatchVersion}})

set(SUBPROJECT_LIST
    {{range .Targets}}"{{if or (eq .Type "executable") (eq .Type "library")}}src/{{else if eq .Type "interface"}}include/{{end}}{{.Name}}"
    {{else}}""{{end}})
set(TEST_LIST
    {{range .Targets}}{{if .CreateTest}}"test/{{.Name}}"{{end}}
    {{else}}""{{end}})

# Cmake module path
set(PROJECT_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${PROJECT_ROOT_DIR}/cmake/modules")

set(_PROJECT_VERSION
  ${_PROJECT_MAJOR_VERSION}.${_PROJECT_MINOR_VERSION}.${_PROJECT_PATCH_VERSION})

project(${_PROJECT_NAME} LANGUAGES ${_PROJECT_LANGUAGE} VERSION ${_PROJECT_VERSION})

# ############################################################### #
# Find global libraries ######################################### #
# ############################################################### #

# Insert here finding libraries if they use in several subdirectories:
# For example:
### find_package(SFML 2.5 COMPONENTS system graphics REQUIRED)
### find_package(Boost CONFIG COMPONENTS random REQUIRED)
# .............................................

# ############################################################### #
# Add subdirectories ############################################ #
# ############################################################### #

foreach(SUBPROJ ${SUBPROJECT_LIST})
    add_subdirectory(${SUBPROJ})
endforeach()

# ############################################################### #
# Add test subdirectories ####################################### #
# ############################################################### #

enable_testing()
foreach(TEST ${TEST_LIST})
    string(REGEX REPLACE "^test\/" "" TEST_NAME ${TEST})
    if(${${TEST_NAME}_BUILD_TESTS})
        add_subdirectory(${TEST})
    endif()
endforeach()