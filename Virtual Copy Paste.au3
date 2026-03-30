Do
   ; Recieve input
   $input = InputBox("Virtual Copy Paste", "Please enter text to copy paste into the VM! Switch to the VM after submitted.")

   ; Check @error then exit
   If @error Then
	  Exit
   EndIf

   Sleep(1500)

   ; Interate through string
   $input = StringSplit($input, "")
   For $i = 1 To $input[0]
	  ; Input key on keyboard
	  Send($input[$i], 1)
	  Sleep(20)
   Next
Until False
