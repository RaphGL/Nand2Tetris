// -- runtime initialization
@256
D=A
@SP
M=D
// push constant 3030
@3030
D=A

@SP
A=M
M=D

@SP
M=M+1
// -- push pointer THIS
@THIS
D=A
@SP
A=M
M=D

@SP
M=M+1
// push constant 3040
@3040
D=A

@SP
A=M
M=D

@SP
M=M+1
// -- push pointer THAT
@THAT
D=A
@SP
A=M
M=D

@SP
M=M+1
// push constant 32
@32
D=A

@SP
A=M
M=D

@SP
M=M+1
// -- pop THIS 2
@2
D=A
@THIS
D=D+A
@addr
M=D

@SP
M=M-1

A=M
M=D
@addr
A=M
M=D
// push constant 46
@46
D=A

@SP
A=M
M=D

@SP
M=M+1
// -- pop THAT 6
@6
D=A
@THAT
D=D+A
@addr
M=D

@SP
M=M-1

A=M
M=D
@addr
A=M
M=D
// -- push pointer THIS
@THIS
D=A
@SP
A=M
M=D

@SP
M=M+1
// -- push pointer THAT
@THAT
D=A
@SP
A=M
M=D

@SP
M=M+1
// -- add
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
M=M+D

@SP
M=M+1
// -- push THIS 2
@2
D=A
@THIS
D=D+A

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
// -- sub
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
M=M-D

@SP
M=M+1
// -- push THAT 6
@6
D=A
@THAT
D=D+A

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
// -- add
@SP
M=M-1
A=M
D=M

@SP
M=M-1
A=M
M=M+D

@SP
M=M+1
