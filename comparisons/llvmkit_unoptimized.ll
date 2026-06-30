; ModuleID = 'protected'
define i64 @protected(i64 %rax, i64 %rbx, i64 %rcx, i64 %rdx, i64 %rsi, i64 %rdi, i64 %rsp, i64 %rbp, i64 %r8, i64 %r9, i64 %r10, i64 %r11, i64 %r12, i64 %r13, i64 %r14, i64 %r15, i64 %rip, i1 %CF, i1 %PF, i1 %AF, i1 %ZF, i1 %SF, i1 %TF, i1 %IF, i1 %DF, i1 %OF, i1 %IOPL, i1 %NT, i1 %RF, i1 %VM, i1 %AC, i1 %VIF, i1 %VIP, i1 %ID, i1 %RFLAGS) {
entry:
  %stackmemory = alloca i128, i128 4096
  %0 = trunc i64 %r8 to i32
  %1 = add i64 %rsp, 24
  %2 = getelementptr i8, ptr %stackmemory, i64 %1
  store i32 %0, ptr %2
  %3 = trunc i64 %rdx to i32
  %4 = add i64 %rsp, 16
  %5 = getelementptr i8, ptr %stackmemory, i64 %4
  store i32 %3, ptr %5
  %6 = trunc i64 %rcx to i32
  %7 = add i64 %rsp, 8
  %8 = getelementptr i8, ptr %stackmemory, i64 %7
  store i32 %6, ptr %8
  %push_sp = sub i64 %rsp, 8
  %9 = getelementptr i8, ptr %stackmemory, i64 %push_sp
  store i64 %rbp, ptr %9
  %push_sp1 = sub i64 %push_sp, 8
  %10 = getelementptr i8, ptr %stackmemory, i64 %push_sp1
  store i64 %rdi, ptr %10
  %sub_result = sub i64 %push_sp1, 232
  %sub_cf = icmp ult i64 %push_sp1, 232
  %sub_of_lhs_rhs = xor i64 %push_sp1, 232
  %sub_of_lhs_result = xor i64 %push_sp1, %sub_result
  %sub_of_bits = and i64 %sub_of_lhs_rhs, %sub_of_lhs_result
  %computed_sub_of = icmp slt i64 %sub_of_bits, 0
  %af_lhs_rhs = xor i64 %push_sp1, 232
  %af_changed = xor i64 %af_lhs_rhs, %sub_result
  %af_masked = and i64 %af_changed, 16
  %computed_af = icmp ne i64 %af_masked, 0
  %11 = trunc i64 %sub_result to i8
  %pf_shift4 = lshr i8 %11, 4
  %pf_xor4 = xor i8 %11, %pf_shift4
  %pf_shift2 = lshr i8 %pf_xor4, 2
  %pf_xor2 = xor i8 %pf_xor4, %pf_shift2
  %pf_shift1 = lshr i8 %pf_xor2, 1
  %pf_xor1 = xor i8 %pf_xor2, %pf_shift1
  %pf_low_bit = and i8 %pf_xor1, 1
  %computed_pf = icmp eq i8 %pf_low_bit, 0
  %computed_sf = icmp slt i64 %sub_result, 0
  %computed_zf = icmp eq i64 %sub_result, 0
  %12 = add i64 %sub_result, 32
  %13 = add i64 %rip, 63574
  %call_sp = sub i64 %sub_result, 8
  %14 = getelementptr i8, ptr %stackmemory, i64 %call_sp
  store i64 undef, ptr %14
  %15 = add i64 %12, 232
  %16 = getelementptr i8, ptr %stackmemory, i64 %15
  %17 = load i32, ptr %16
  %18 = add i64 %12, 224
  %19 = getelementptr i8, ptr %stackmemory, i64 %18
  %20 = load i32, ptr %19
  %add_result = add i32 %20, %17
  %add_cf = icmp ult i32 %add_result, %20
  %add_of_lhs_result = xor i32 %20, %add_result
  %add_of_rhs_result = xor i32 %17, %add_result
  %add_of_bits = and i32 %add_of_lhs_result, %add_of_rhs_result
  %computed_add_of = icmp slt i32 %add_of_bits, 0
  %af_lhs_rhs2 = xor i32 %20, %17
  %af_changed3 = xor i32 %af_lhs_rhs2, %add_result
  %af_masked4 = and i32 %af_changed3, 16
  %computed_af5 = icmp ne i32 %af_masked4, 0
  %21 = trunc i32 %add_result to i8
  %pf_shift46 = lshr i8 %21, 4
  %pf_xor47 = xor i8 %21, %pf_shift46
  %pf_shift28 = lshr i8 %pf_xor47, 2
  %pf_xor29 = xor i8 %pf_xor47, %pf_shift28
  %pf_shift110 = lshr i8 %pf_xor29, 1
  %pf_xor111 = xor i8 %pf_xor29, %pf_shift110
  %pf_low_bit12 = and i8 %pf_xor111, 1
  %computed_pf13 = icmp eq i8 %pf_low_bit12, 0
  %computed_sf14 = icmp slt i32 %add_result, 0
  %computed_zf15 = icmp eq i32 %add_result, 0
  %22 = add i64 %12, 240
  %23 = getelementptr i8, ptr %stackmemory, i64 %22
  %24 = load i32, ptr %23
  %add_result16 = add i32 %add_result, %24
  %add_cf17 = icmp ult i32 %add_result16, %add_result
  %add_of_lhs_result18 = xor i32 %add_result, %add_result16
  %add_of_rhs_result19 = xor i32 %24, %add_result16
  %add_of_bits20 = and i32 %add_of_lhs_result18, %add_of_rhs_result19
  %computed_add_of21 = icmp slt i32 %add_of_bits20, 0
  %af_lhs_rhs22 = xor i32 %add_result, %24
  %af_changed23 = xor i32 %af_lhs_rhs22, %add_result16
  %af_masked24 = and i32 %af_changed23, 16
  %computed_af25 = icmp ne i32 %af_masked24, 0
  %25 = trunc i32 %add_result16 to i8
  %pf_shift426 = lshr i8 %25, 4
  %pf_xor427 = xor i8 %25, %pf_shift426
  %pf_shift228 = lshr i8 %pf_xor427, 2
  %pf_xor229 = xor i8 %pf_xor427, %pf_shift228
  %pf_shift130 = lshr i8 %pf_xor229, 1
  %pf_xor131 = xor i8 %pf_xor229, %pf_shift130
  %pf_low_bit32 = and i8 %pf_xor131, 1
  %computed_pf33 = icmp eq i8 %pf_low_bit32, 0
  %computed_sf34 = icmp slt i32 %add_result16, 0
  %computed_zf35 = icmp eq i32 %add_result16, 0
  %26 = add i64 %12, 200
  %27 = getelementptr i8, ptr %stackmemory, i64 %26
  %28 = load i64, ptr %27
  %pop_sp = add i64 %26, 8
  %29 = getelementptr i8, ptr %stackmemory, i64 %pop_sp
  %30 = load i64, ptr %29
  %pop_sp36 = add i64 %pop_sp, 8
  %31 = zext i32 %add_result16 to i64
  ret i64 %31
}


define i32 @main() {
    entry:
        %result = call i32 @protected(i64 3, i64 7, i64 1, i64 0, i64 0, i64 0, i64 0 )
        ret i32 %result
}

