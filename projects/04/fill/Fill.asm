@SCREEN
D=A
@scr_coord
M=D
@8192
D=A
@scr_coord
M=M+D

@color
M=0
// black screen on keyboard input
@KBD
D=M
@write_pixels
D;JEQ
@color
M=M-1

(write_pixels)
  @color  
  D=M
  @scr_coord
  A=M
  M=D

  @scr_coord
  M=M-1
  D=M
  @SCREEN
  D=D-A 
  @write_pixels
  D;JNE

@0
0;JMP
