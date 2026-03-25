Unicode true
SetCompressor /SOLID lzma
CRCCheck on

!define APP_NAME "Pixel Paint RS"
!define APP_PUBLISHER "Ksiw"
!define APP_EXE "pixel_paint_rs.exe"

Name "${APP_NAME}"
OutFile "${OUT_FILE}"
InstallDir "$LOCALAPPDATA\\Programs\\${APP_NAME}"
RequestExecutionLevel user

Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File "${APP_BINARY}"
  File "/oname=app.ico" "${APP_REPO_ROOT}\\assets\\icon.ico"
  File "/oname=LICENSE" "${APP_REPO_ROOT}\\LICENSE"
  File "/oname=README.md" "${APP_REPO_ROOT}\\README.md"

  CreateDirectory "$SMPROGRAMS\\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\\${APP_NAME}\\${APP_NAME}.lnk" "$INSTDIR\\${APP_EXE}" "" "$INSTDIR\\app.ico" 0
  CreateShortcut "$DESKTOP\\${APP_NAME}.lnk" "$INSTDIR\\${APP_EXE}" "" "$INSTDIR\\app.ico" 0

  WriteUninstaller "$INSTDIR\\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\\${APP_EXE}"
  Delete "$INSTDIR\\app.ico"
  Delete "$INSTDIR\\LICENSE"
  Delete "$INSTDIR\\README.md"
  Delete "$INSTDIR\\Uninstall.exe"
  Delete "$SMPROGRAMS\\${APP_NAME}\\${APP_NAME}.lnk"
  RMDir "$SMPROGRAMS\\${APP_NAME}"
  Delete "$DESKTOP\\${APP_NAME}.lnk"
  RMDir "$INSTDIR"
SectionEnd
