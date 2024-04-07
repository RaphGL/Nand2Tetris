// -- pop temp {0}
@{0}
D=A
@5
D=D+A
@addr
M=D

@SP
M=M-1

A=M
D=M
@addr
A=M
M=D
