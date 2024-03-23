// -- pop temp {0}
@5
D=A
@{0}
D=D+A
@addr
M=D

@SP
M=M-1

@SP
A=M
D=M
@addr
A=M
M=D
