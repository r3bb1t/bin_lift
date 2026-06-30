define i32 @protected(i64 %rax, i64 %rbx, i64 %rcx, i64 %rdx, i64 %rsi, i64 %rdi, i64 %rsp, i64 %rbp, i64 %r8, i64 %r9, i64 %r10, i64 %r11, i64 %r12, i64 %r13, i64 %r14, i64 %r15, i64 %rip, i8 %CF, i8 %PF, i8 %AF, i8 %ZF, i8 %SF, i8 %TF, i8 %IF, i8 %DF, i8 %OF, i8 %IOPL, i8 %NT, i8 %RF, i8 %VM, i8 %AC, i8 %VIF, i8 %VIP, i8 %ID) local_unnamed_addr {
entry:
  %0 = trunc i64 %r8 to i32
  %1 = trunc i64 %rdx to i32
  %2 = trunc i64 %rcx to i32
  %add_result_ = add i32 %1, %2
  %add_result_22 = add i32 %add_result_, %0
  ret i32 %add_result_22
}

define noundef i32 @main() local_unnamed_addr {
entry:
  ret i32 1
}

