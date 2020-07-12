set(SUBPROJ_NAME                                                {{.Name}})
set(TESTABLE_TARGET                                             {{.TestableName}})
set(${SUBPROJ_NAME}_LANG                                        {{if eq .IsInherited true}}${_PROJECT_LANG}{{else}}{{.Lang}}{{end}})

set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD            {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD}{{else}}{{.LangStandard}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_EXTENSIONS          {{if eq .IsInherited true}}${_PROJECT_LANG_EXTENSIONS}{{else}}{{.LangExtensions}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED   {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD_REQUIRED}{{else}}{{.LangStandardRequired}}{{end}})

# Insert here your source files
set(${SUBPROJ_NAME}_HEADERS
    "")

set(${SUBPROJ_NAME}_SOURCES
    "")

# ############################################################### #
# Options ####################################################### #
# ############################################################### #

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

# Set include directories for this project and if it will use as installing
target_include_directories(
    ${SUBPROJ_NAME}
    PRIVATE   $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    INTERFACE $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>)

target_link_libraries(
    ${SUBPROJ_NAME}
    ${TESTABLE_TARGET})

add_test(
    NAME All${TESTABLE_TARGET}Tests
    COMMAND ${SUBPROJ_NAME})