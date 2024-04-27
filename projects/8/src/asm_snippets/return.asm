// -- return
/// endFrame = LCL

/// Frame = LCL
@LCL
D=M
@__frame
M=D

/// RET = *(Frame - 5)
@__frame
D=M
@5
D=D-A
A=D
D=M
@__ret
M=D

/// *ARG = pop()
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D

/// SP = ARG + 1
@ARG
D=M
@SP
M=D+1

/// THAT = *(Frame - 1)
@__frame
D=M
D=D-1
A=D
D=M
@THAT
M=D

/// THIS = *(Frame - 2)
@__frame
D=M
@2
D=D-A
A=D
D=M
@THIS
M=D

/// ARG = *(Frame - 3)
@__frame
D=M
@3
D=D-A
A=D
D=M
@ARG
M=D

/// LCL = *(Frame - 4)
@__frame
D=M
@4
D=D-A
A=D
D=M
@LCL
M=D
