// -- function {0} {1}
/// 0 = function name
/// 1 = number of args

({0})

@{1}
D=A
@{0}$__loop_end
D;JEQ
@count
M=D

({0}$__loop_start)
@SP
A=M
M=0
@SP
M=M+1

@count
M=M-1
D=M
@{0}$__loop_start
D;JLE

({0}$__loop_end)
