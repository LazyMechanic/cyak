set(SUBPROJ_NAME                                                {{.Name}})
set(${SUBPROJ_NAME}_LANG                                        {{if eq .IsInherited true}}${_PROJECT_LANG}{{else}}{{.Lang}}{{end}})

set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD            {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD}{{else}}{{.LangStandard}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_EXTENSIONS          {{if eq .IsInherited true}}${_PROJECT_LANG_EXTENSIONS}{{else}}{{.LangExtensions}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED   {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD_REQUIRED}{{else}}{{.LangStandardRequired}}{{end}})

set(${SUBPROJ_NAME}_MAJOR_VERSION                               {{if eq .IsInherited true}}${_PROJECT_MAJOR_VERSION}{{else}}{{.MajorVersion}}{{end}})
set(${SUBPROJ_NAME}_MINOR_VERSION                               {{if eq .IsInherited true}}${_PROJECT_MINOR_VERSION}{{else}}{{.MinorVersion}}{{end}})
set(${SUBPROJ_NAME}_PATCH_VERSION                               {{if eq .IsInherited true}}${_PROJECT_PATCH_VERSION}{{else}}{{.PatchVersion}}{{end}})

# Insert here your source files
set(${SUBPROJ_NAME}_HEADERS
    "")

set(${SUBPROJ_NAME}_SOURCES
    "")

# ############################################################### #
# Options ####################################################### #
# ############################################################### #

include(OptionHelpers)
generate_basic_options_executable(${SUBPROJ_NAME})

# Insert here your specififc options for build:
# .............................................

# ############################################################### #
# Pre-build commands ############################################ #
# ############################################################### #

# Insert here pre-build commands:
# .............................................

# ############################################################### #
# After-build commands ########################################## #
# ############################################################### #

# Insert here after-build commands:
# .............................................

# ############################################################### #
# Set all target sources ######################################## #
# ############################################################### #

set(
    ${SUBPROJ_NAME}_ALL_SRCS
    ${${SUBPROJ_NAME}_HEADERS}
    ${${SUBPROJ_NAME}_SOURCES})

# ############################################################### #
# Create target for build ####################################### #
# ############################################################### #

add_executable(
    ${SUBPROJ_NAME}
    ${${SUBPROJ_NAME}_ALL_SRCS})

# Enable C++17 on this project
set_target_properties(
    ${SUBPROJ_NAME} PROPERTIES
    ${${SUBPROJ_NAME}_LANG}_STANDARD          ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD}
    ${${SUBPROJ_NAME}_LANG}_EXTENSIONS        ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_EXTENSIONS}
    ${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED})
    
# Set specific properties
set_target_properties(
    ${SUBPROJ_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${PROJECT_BINARY_DIR}/bin"
    ARCHIVE_OUTPUT_DIRECTORY "${PROJECT_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${PROJECT_BINARY_DIR}/lib"
    OUTPUT_NAME              "${SUBPROJ_NAME}$<$<CONFIG:Debug>:d>")

# Set version
set_target_properties(
    ${SUBPROJ_NAME} PROPERTIES
    VERSION ${${SUBPROJ_NAME}_MAJOR_VERSION}.${${SUBPROJ_NAME}_MINOR_VERSION}.${${SUBPROJ_NAME}_PATCH_VERSION})
    
# Set include directories for this project and if it will use as installing
target_include_directories(
    ${SUBPROJ_NAME}
    PRIVATE   $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    INTERFACE $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>)

# ############################################################### #
# Link libraries ################################################ #
# ############################################################### #

# Insert here linking libraries:
# For example:
### find_package(SFML 2.5 COMPONENTS system graphics REQUIRED)
### target_link_libraries(
###     ${SUBPROJ_NAME}
###     sfml-system
###     sfml-graphics)
# .............................................
    
# ############################################################### #
# Installing #################################################### #
# ############################################################### #

# Create export targets
install(
    TARGETS ${SUBPROJ_NAME}
    EXPORT  ${SUBPROJ_NAME}-targets)

# Set out paths
install(
    TARGETS ${SUBPROJ_NAME}
    RUNTIME DESTINATION  ${${SUBPROJ_NAME}_INSTALL_BIN_PREFIX}
    ARCHIVE DESTINATION  ${${SUBPROJ_NAME}_INSTALL_LIB_PREFIX}
    LIBRARY DESTINATION  ${${SUBPROJ_NAME}_INSTALL_LIB_PREFIX})