;NSIS Modern User Interface
;Basic Example Script
;Written by Joost Verburg

;--------------------------------
;Include Modern UI

  !include "MUI2.nsh"
  !include "EnvVarUpdate.nsh"

;--------------------------------
;General

  ;Name and file
  Name "cyak"
  OutFile "cyak_setup.exe"

  ;Default installation folder
  InstallDir "$PROGRAMFILES\cyak"
  
  ;Get installation folder from registry if available
  InstallDirRegKey HKLM "Software\cyak" "Install_Dir"

  ;Request application privileges for Windows Vista
  RequestExecutionLevel admin

;--------------------------------
;Interface Settings

  !define MUI_ABORTWARNING

;--------------------------------
;Pages

  !insertmacro MUI_PAGE_WELCOME
  !insertmacro MUI_PAGE_LICENSE "..\LICENSE"
  !insertmacro MUI_PAGE_DIRECTORY
  !insertmacro MUI_PAGE_INSTFILES
  
  !insertmacro MUI_UNPAGE_CONFIRM
  !insertmacro MUI_UNPAGE_INSTFILES

  Function AddToPath
  ${EnvVarUpdate} $0 "PATH" "A" "HKCU" "$INSTDIR\bin"
  FunctionEnd

  !define MUI_FINISHPAGE_SHOWREADME ""
  !define MUI_FINISHPAGE_SHOWREADME_TEXT "Add to PATH"
  !define MUI_FINISHPAGE_SHOWREADME_FUNCTION AddToPath
  !insertmacro MUI_PAGE_FINISH
  
;--------------------------------
;Languages
 
  !insertmacro MUI_LANGUAGE "English"

;--------------------------------
;Installer Sections

Section ""
  SetOutPath "$INSTDIR\bin"

  ;ADD YOUR OWN FILES HERE...
  File ..\cyak.exe

  Var /GLOBAL PresetsDir
  StrCpy $PresetsDir "$PROFILE\.cyak\"

  ; Install presets
  SetOutPath "$PresetsDir"

  File /r ..\presets\

  ;Store installation folder
  WriteRegStr HKCU "Software\cyak" "Install_Dir" $INSTDIR

  ;Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd
;--------------------------------
;Descriptions



;--------------------------------
;Uninstaller Section

Section "Uninstall"

  ;ADD YOUR OWN FILES HERE...

  RMDir /r "$PROFILE\.cyak\"
  Delete "$INSTDIR\bin\cyak.exe"
  Delete "$INSTDIR\Uninstall.exe"
  ${un.EnvVarUpdate} $0 "PATH" "R" "HKCU" "$INSTDIR\bin"

  RMDir "$INSTDIR\bin"
  RMDir "$INSTDIR"

  DeleteRegKey /ifempty HKCU "Software\cyak"

SectionEnd