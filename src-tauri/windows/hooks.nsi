; Include WebView2Loader.dll next to the main executable (required for gnu toolchain builds).
; The DLL lives in the same directory as the main binary (target/release/).
!macro NSIS_HOOK_POSTINSTALL
  ; Extract directory from MAINBINARYSRCPATH by replacing the exe filename
  !searchreplace RELEASE_DIR "${MAINBINARYSRCPATH}" "\${MAINBINARYNAME}.exe" ""
  File "/oname=$INSTDIR\WebView2Loader.dll" "${RELEASE_DIR}\WebView2Loader.dll"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  Delete "$INSTDIR\WebView2Loader.dll"
!macroend
