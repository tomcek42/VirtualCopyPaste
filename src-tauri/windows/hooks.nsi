; Include WebView2Loader.dll next to the main executable when present
; (required for gnu toolchain builds, absent with msvc toolchain).
!macro NSIS_HOOK_POSTINSTALL
  !searchreplace RELEASE_DIR "${MAINBINARYSRCPATH}" "\${MAINBINARYNAME}.exe" ""
  File /nonfatal "/oname=$INSTDIR\WebView2Loader.dll" "${RELEASE_DIR}\WebView2Loader.dll"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  Delete "$INSTDIR\WebView2Loader.dll"
!macroend
