// -- runtime initialization
@256
D=A
@SP
M=D
// -- push constant 0
@0
D=A

@SP
A=M
M=D

@SP
M=M+1
// -- pop LCL 0
@0
D=A
@LCL
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
// -- label LOOP
(LOOP)
// -- push ARG 0
@0
D=A
@ARG
D=D+M

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
// -- push LCL 0
@0
D=A
@LCL
D=D+M

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
// -- pop LCL 0
@0
D=A
@LCL
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
// -- push ARG 0
@0
D=A
@ARG
D=D+M

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
// -- push constant 1
@1
D=A

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
// -- pop ARG 0
@0
D=A
@ARG
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
// -- push ARG 0
@0
D=A
@ARG
D=D+M

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
// if-goto LOOP
@SP
M=M-1
A=M
D=M

@LOOP
D;JNE
// -- push LCL 0
@0
D=A
@LCL
D=D+M

A=D
D=M
@SP
A=M
M=D

@SP
M=M+1
