define range(i64 0, 4294967296) i64 @protected(i64 %rax, i64 %rbx, i64 %rcx, i64 %rdx, i64 %rsi, i64 %rdi, i64 %rsp, i64 %rbp, i64 %r8, i64 %r9, i64 %r10, i64 %r11, i64 %r12, i64 %r13, i64 %r14, i64 %r15, i64 %rip, i1 %CF, i1 %PF, i1 %AF, i1 %ZF, i1 %SF, i1 %TF, i1 %IF, i1 %DF, i1 %OF, i1 %IOPL, i1 %NT, i1 %RF, i1 %VM, i1 %AC, i1 %VIF, i1 %VIP, i1 %ID, i1 %RFLAGS) local_unnamed_addr {
entry:
  %add_result = add i64 %rdx, %rcx
  %add_result16 = add i64 %add_result, %r8
  %0 = and i64 %add_result16, 4294967295
  ret i64 %0
}

define i32 @main() local_unnamed_addr {
entry:
  %result = tail call i32 @protected(i64 3, i64 7, i64 1, i64 0, i64 0, i64 0, i64 0)
  ret i32 %result
}

