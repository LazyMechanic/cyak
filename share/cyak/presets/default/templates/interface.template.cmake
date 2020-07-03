set(SUBPROJ_NAME                                                {{.Name}})
set(${SUBPROJ_NAME}_LANG                                        {{if eq .IsInherited true}}${_PROJECT_LANG}{{else}}{{.Lang}}{{end}})
set(${SUBPROJ_NAME}_NAMESPACE                                   {{if eq .IsInherited true}}${_PROJECT_NAMESPACE}{{else}}{{.Namespace}}{{end}})

set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD            {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD}{{else}}{{.LangStandard}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_EXTENSIONS          {{if eq .IsInherited true}}${_PROJECT_LANG_EXTENSIONS}{{else}}{{.LangExtensions}}{{end}})
set(${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED   {{if eq .IsInherited true}}${_PROJECT_LANG_STANDARD_REQUIRED}{{else}}{{.LangStandardRequired}}{{end}})

set(${SUBPROJ_NAME}_MAJOR_VERSION                               {{if eq .IsInherited true}}${_PROJECT_MAJOR_VERSION}{{else}}{{.MajorVersion}}{{end}})
set(${SUBPROJ_NAME}_MINOR_VERSION                               {{if eq .IsInherited true}}${_PROJECT_MINOR_VERSION}{{else}}{{.MinorVersion}}{{end}})
set(${SUBPROJ_NAME}_PATCH_VERSION                               {{if eq .IsInherited true}}${_PROJECT_PATCH_VERSION}{{else}}{{.PatchVersion}}{{end}})

# Insert here your source files
set(${SUBPROJ_NAME}_HEADERS
    "")

# ############################################################### #
# Options ####################################################### #
# ############################################################### #

include(OptionHelpers)
generate_basic_options_headeronly(${SUBPROJ_NAME})

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
# Library version ############################################### #
# ############################################################### #

set(${SUBPROJ_NAME}_VERSION
    ${${SUBPROJ_NAME}_MAJOR_VERSION}.${${SUBPROJ_NAME}_MINOR_VERSION}.${${SUBPROJ_NAME}_PATCH_VERSION})

# ############################################################### #
# Set all target sources ######################################## #
# ############################################################### #

set(
    ${SUBPROJ_NAME}_ALL_SRCS
    ${${SUBPROJ_NAME}_HEADERS})

# ############################################################### #
# Create target for build ####################################### #
# ############################################################### #

# Interface library target
add_library(
    ${SUBPROJ_NAME}
    INTERFACE)

# Enable C++ standard on this project
set_target_properties(
    ${SUBPROJ_NAME} PROPERTIES
    INTERFACE_${${SUBPROJ_NAME}_LANG}_STANDARD          ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD}
    INTERFACE_${${SUBPROJ_NAME}_LANG}_EXTENSIONS        ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_EXTENSIONS}
    INTERFACE_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED ${${SUBPROJ_NAME}_${${SUBPROJ_NAME}_LANG}_STANDARD_REQUIRED})

target_include_directories(
    ${SUBPROJ_NAME}
    INTERFACE $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/..>
              $<INSTALL_INTERFACE:include>)

# ############################################################### #
# Link libraries ################################################ #
# ############################################################### #

# Insert here linking libraries:
# For example:
### find_package(Boost CONFIG COMPONENTS random REQUIRED)
### target_link_libraries(
###     ${SUBPROJ_NAME}
###     INTERFACE Boost::random)
# .............................................

# ############################################################### #
# Installing #################################################### #
# ############################################################### #

# Create export targets
install(
    TARGETS ${SUBPROJ_NAME}
    EXPORT  ${SUBPROJ_NAME}-targets)

# Install headers
install(
    FILES       ${${SUBPROJ_NAME}_HEADERS}
    DESTINATION ${${SUBPROJ_NAME}_INSTALL_INCLUDE_PREFIX})

set(SUBPROJ_TARGETS_FILE "${SUBPROJ_NAME}-targets.cmake")

# Create config-targets cmake file
install(
    EXPORT      ${SUBPROJ_NAME}-targets
    FILE        ${SUBPROJ_TARGETS_FILE}
    NAMESPACE   ${${SUBPROJ_NAME}_NAMESPACE}::
    DESTINATION ${${SUBPROJ_NAME}_INSTALL_CMAKE_PREFIX})

# Create config files
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    "${PROJECT_BINARY_DIR}/${SUBPROJ_NAME}-config-version.cmake"
    VERSION ${cmake-test-headeronly_VERSION}
    COMPATIBILITY AnyNewerVersion)

configure_package_config_file(
    "${PROJECT_ROOT_DIR}/cmake/${SUBPROJ_NAME}-config.cmake.in"
    "${PROJECT_BINARY_DIR}/${SUBPROJ_NAME}-config.cmake"
    INSTALL_DESTINATION ${${SUBPROJ_NAME}_INSTALL_CMAKE_PREFIX})

# Install config files
install(
    FILES
        "${PROJECT_BINARY_DIR}/${SUBPROJ_NAME}-config.cmake"
        "${PROJECT_BINARY_DIR}/${SUBPROJ_NAME}-config-version.cmake"
    DESTINATION ${${SUBPROJ_NAME}_INSTALL_CMAKE_PREFIX})