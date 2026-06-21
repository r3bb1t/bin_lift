# Examples

Here you can take a look of simple examples on how the library can be used.


## simple_add.rs

 ```assembly
 mov [rsp+0x18], r8d
 mov [rsp+0x10], edx
 mov [rsp+0x08], ecx
 push rbp
 push rdi
 sub rsp, 0xE8
 lea rbp, [rsp+0x20]
 lea rcx, [0x000000000000F85D]
 call 0xFFFFFFFFFFFFFBAF ; modeled as a call-side stack update by the lifter
 mov eax, [rbp+0xE8]
 mov ecx, [rbp+0xE0]
 add ecx, eax
 mov eax, ecx
 add eax, [rbp+0xF0]
 lea rsp, [rbp+0xC8]
 pop rdi
 pop rbp
 ret
 ```
which decompiles to:

```C
__int64 __fastcall add(int a, int b, int c)
{
  j___CheckForDebuggerJustMyCode(&_5923EECC_simple_target_cpp);
  return (c + b + a);
}
```


Examples print verified textual LLVM IR. Native optimization or object output is outside the core migration.
If you want to compile the emitted IR as a complete program, add the surrounding caller/entry-point or run it through an explicit backend step.

Original comparison link:
https://godbolt.org/z/rv1YhsYjM
