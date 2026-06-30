define i32 @protected(i64 %rax, i64 %rbx, i64 %rcx, i64 %rdx, i64 %rsi, i64 %rdi, i64 %rsp, i64 %rbp, i64 %r8, i64 %r9, i64 %r10, i64 %r11, i64 %r12, i64 %r13, i64 %r14, i64 %r15, i64 %rip, i8 %CF, i8 %PF, i8 %AF, i8 %ZF, i8 %SF, i8 %TF, i8 %IF, i8 %DF, i8 %OF, i8 %IOPL, i8 %NT, i8 %RF, i8 %VM, i8 %AC, i8 %VIF, i8 %VIP, i8 %ID) {
entry:
  %stackmemory = alloca i128, i128 4096, align 8
  %debug_mov_ = alloca i128, align 8
  %0 = trunc i64 %r8 to i32
  %1 = add i64 %rsp, 24
  %GEPSTORE = getelementptr i8, ptr %stackmemory, i64 %1
  store i32 %0, ptr %GEPSTORE, align 4
  %debug_mov_1 = alloca i128, align 8
  %2 = trunc i64 %rdx to i32
  %3 = add i64 %rsp, 16
  %GEPSTORE2 = getelementptr i8, ptr %stackmemory, i64 %3
  store i32 %2, ptr %GEPSTORE2, align 4
  %debug_mov_3 = alloca i128, align 8
  %4 = trunc i64 %rcx to i32
  %5 = add i64 %rsp, 8
  %GEPSTORE4 = getelementptr i8, ptr %stackmemory, i64 %5
  store i32 %4, ptr %GEPSTORE4, align 4
  %debug_push_ = alloca i128, align 8
  %pushing_new_rsp_ = sub i64 %rsp, 8
  %GEPSTORE5 = getelementptr i8, ptr %stackmemory, i64 %pushing_new_rsp_
  store i64 %rbp, ptr %GEPSTORE5, align 4
  %debug_push_6 = alloca i128, align 8
  %pushing_new_rsp_7 = sub i64 %pushing_new_rsp_, 8
  %GEPSTORE8 = getelementptr i8, ptr %stackmemory, i64 %pushing_new_rsp_7
  store i64 %rdi, ptr %GEPSTORE8, align 4
  %debug_sub_ = alloca i128, align 8
  %sub_result_ = sub i64 %pushing_new_rsp_7, 232
  %6 = and i64 %pushing_new_rsp_7, 15
  %7 = sub i64 %6, 8
  %8 = icmp ugt i64 %7, 15
  %9 = icmp ult i64 %sub_result_, %pushing_new_rsp_7
  %10 = xor i64 %pushing_new_rsp_7, 232
  %11 = xor i64 %pushing_new_rsp_7, %sub_result_
  %12 = and i64 %10, %11
  %13 = icmp slt i64 %12, 0
  %sign_flag_ = icmp slt i64 %sub_result_, 0
  %sign_flag_9 = icmp eq i64 %sub_result_, 0
  %14 = trunc i64 %sub_result_ to i8
  %15 = call i8 @llvm.ctpop.i8(i8 %14)
  %16 = and i8 %15, 1
  %17 = icmp eq i8 %16, 0
  %debug_lea_ = alloca i128, align 8
  %18 = add i64 %sub_result_, 32
  %debug_lea_10 = alloca i128, align 8
  %19 = add i64 %rip, 63574
  %debug_mov_11 = alloca i128, align 8
  %20 = add i64 %18, 232
  %GEPSTORE12 = getelementptr i8, ptr %stackmemory, i64 %20
  %Loadxd = load i32, ptr %GEPSTORE12, align 4
  %debug_mov_13 = alloca i128, align 8
  %21 = add i64 %18, 224
  %GEPSTORE14 = getelementptr i8, ptr %stackmemory, i64 %21
  %Loadxd15 = load i32, ptr %GEPSTORE14, align 4
  %debug_add_ = alloca i128, align 8
  %add_result_ = add i32 %Loadxd15, %Loadxd
  %22 = and i32 %Loadxd15, 15
  %23 = and i32 %Loadxd, 15
  %24 = add i32 %22, %23
  %25 = icmp ugt i32 %24, 15
  %26 = icmp ult i32 %add_result_, %Loadxd15
  %27 = xor i32 %Loadxd15, %add_result_
  %28 = xor i32 %Loadxd, %add_result_
  %29 = and i32 %27, %28
  %30 = icmp slt i32 %29, 0
  %sign_flag_16 = icmp slt i32 %add_result_, 0
  %sign_flag_17 = icmp eq i32 %add_result_, 0
  %31 = trunc i32 %add_result_ to i8
  %32 = call i8 @llvm.ctpop.i8(i8 %31)
  %33 = and i8 %32, 1
  %34 = icmp eq i8 %33, 0
  %debug_mov_18 = alloca i128, align 8
  %debug_add_19 = alloca i128, align 8
  %35 = add i64 %18, 240
  %GEPSTORE20 = getelementptr i8, ptr %stackmemory, i64 %35
  %Loadxd21 = load i32, ptr %GEPSTORE20, align 4
  %add_result_22 = add i32 %add_result_, %Loadxd21
  %36 = and i32 %add_result_, 15
  %37 = and i32 %Loadxd21, 15
  %38 = add i32 %36, %37
  %39 = icmp ugt i32 %38, 15
  %40 = icmp ult i32 %add_result_22, %add_result_
  %41 = xor i32 %add_result_, %add_result_22
  %42 = xor i32 %Loadxd21, %add_result_22
  %43 = and i32 %41, %42
  %44 = icmp slt i32 %43, 0
  %sign_flag_23 = icmp slt i32 %add_result_22, 0
  %sign_flag_24 = icmp eq i32 %add_result_22, 0
  %45 = trunc i32 %add_result_22 to i8
  %46 = call i8 @llvm.ctpop.i8(i8 %45)
  %47 = and i8 %46, 1
  %48 = icmp eq i8 %47, 0
  %debug_lea_25 = alloca i128, align 8
  %49 = add i64 %18, 200
  %debug_pop_ = alloca i128, align 8
  %GEPSTORE26 = getelementptr i8, ptr %stackmemory, i64 %49
  %Loadxd27 = load i64, ptr %GEPSTORE26, align 4
  %popping_new_rsp_ = add i64 %49, 8
  %debug_pop_28 = alloca i128, align 8
  %GEPSTORE29 = getelementptr i8, ptr %stackmemory, i64 %popping_new_rsp_
  %Loadxd30 = load i64, ptr %GEPSTORE29, align 4
  %popping_new_rsp_31 = add i64 %popping_new_rsp_, 8
  %debug_ret_ = alloca i128, align 8
  %updated_sp_by_ret = add i64 %popping_new_rsp_31, 8
  %50 = inttoptr i64 %updated_sp_by_ret to ptr
  %51 = load i64, ptr %50, align 4
  ret i32 %add_result_22
}




define i32 @main() {
    entry:
        %result = call i32 @protected(i64 3, i64 7, i64 1, i64 0, i64 0, i64 0, i64 0 )
        ret i32 %result
}

