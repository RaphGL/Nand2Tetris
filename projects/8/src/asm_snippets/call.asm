// -- call {0} {1}
/// 0 = function name
/// 1 = arg number
/// 2 = return number

/// push retAddr
@{0}$ret.{2}
D=A
@SP
A=M
M=D
@SP
M=M+1

/// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1

/// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1

/// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1

/// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1

/// ARG = sp - n - 5
@SP
D=M
@5
D=D-A
@{1}
D=D-A
@ARG
M=D

/// LCL = SP
@SP
D=M
@LCL
M=D

/// goto {0}
@{0}
0;JMP

({0})
