// -- eq
@SP
M=M-1

A=M
D=M

@SP
M=M-1
A=M
M=M-D
D=M

@is_equal_{0}
D;JEQ
@SP
A=M
M=0
@end_block_{0}
0;JMP

(is_equal_{0})
@SP
A=M
M=-1

(end_block_{0})
@SP
M=M+1
