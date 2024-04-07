// -- pop {0} {1}
@{1}
D=A
@{0}
D=D+M
@addr
M=D

@SP
M=M-1

A=M
D=M
@addr
A=M
M=D
