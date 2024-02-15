// def count(a, b):
//    if a== 0 or b == 0:
//        return 0
//    count = b
//    res = 0
//    while count > 0:
//        res += a
//        count -= 1
//    return res
// ---

// count = R1
@R1
D=M
@count
M=D
// res = 0
@result
M=0

// if a == 0 or b == 0: return 0
@R0
D=M
@end
D;JEQ
@R1
D=M
@end
D;JEQ

(loop)
  @R0
  D=M
  @result
  M=M+D
  
  @count
  M=M-1
  D=M
  @loop
  D;JNE

(end)
  @result
  D=M
  @R2
  M=D
