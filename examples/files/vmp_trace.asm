mov r8d, 0x07
mov edx, 0x02
mov ecx, 0x01
call -0x51F
jmp +0x3C6
jmp +0x10DFA8
push 0x63C0B05
call -0x9CEB6
push rdi
movsx di, r10b
bswap rdi
push r11
push rcx
push r8
movsx r11d, r15w
mov r11b, 0x1D
xchg r11w, di
push r10
push rdx
push r9
push r15
push r13
movsx edx, sp
movzx rdx, r8w
push rax
push rsi
pushfq
rol r11b, 0x07
add r9b, 0xA9
bts rdx, rbx
push rbx
stc
push r14
rcl rsi, cl
neg di
test sp, bx
push r12
movsx r10d, r8w
bts r11w, r9w
push rbp
and bl, r11b
shl ax, 0x47
mov rsi, 0x00
xchg r9b, r9b
and r11b, r13b
neg rax
push rsi
mov r11, [rsp+0x90]
movsxd rdx, eax
clc
not r11d
add r11d, 0x5AE00E86
ror dx, cl
bswap r11d
mov r10, r15
ror r11d, 0x01
rcl dh, 0x73
add r11, rsi
btr edi, 0xBB
bsf edx, r11d
mov r9, 0x100000000
rol bh, 0xDE
add r11, r9
mov rdi, rsp
sub rsp, 0x180
add r10w, 0x3041
movzx r10, sp
dec bx
and rsp, 0xFFFFFFFFFFFFFFF0
mov rbx, 0x5ADA748E
mov rbx, r11
sbb r10b, 0x22
sar ah, 0x20
rol ax, 0xED
mov rax, 0x00
bt r10w, 0xE9
sub rbx, rax
add dl, 0xC7
mov dx, si
rcr r10, cl
lea r10, [rip-0x07]
cwd
neg dx
mov edx, [r11]
stc
add r11, 0x04
xor edx, ebx
clc
xor edx, 0x4B4238EA
test r14b, bl
sub edx, 0x769C7D02
clc
ror edx, 0x01
stc
xor edx, 0x19E95F2D
push rbx
rol bh, cl
shrd bx, di, 0xB4
bt rbx, r14
xor [rsp], edx
add bl, 0xBA
pop rbx
movsxd rdx, edx
test rdx, 0x6F3349CA
cmp spl, 0xC9
add r10, rdx
jmp -0x36B1D
push r10
ret
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
mov r8, [r11]
cmovnl dx, r8w
or edx, 0x4FE964E3
add r11, 0x08
cwd
xor r8, rbx
bts edx, r9d
ror dx, 0xEB
cqo
rol r8, 0x05
xchg dx, dx
cdq
bswap r8
bswap rdx
inc r8
shr edx, 0x22
sub dx, r10w
xor r8, 0x18BC78EF
shld edx, r8d, 0xDE
rcr dh, cl
rol dx, 0xA0
sub r8, 0x59F76389
bsf dx, r10w
mov rdx, 0x6ABE4CC4
bswap r8
shl dh, 0x90
xor dl, ch
adc edx, 0x58CF5593
xor rbx, r8
sub rdx, r12
cmp r8b, 0xE2
sub rdi, 0x08
shr rdx, 0x32
sub dl, r12b
xor dh, 0x71
mov [rdi], r8
adc dl, r11b
mov edx, [r11]
stc
cmc
add r11, 0x04
test rbx, 0xF9117E9
cmp r8b, r14b
xor edx, ebx
xor edx, 0x668D6E25
add edx, 0x735D6DA4
not edx
neg edx
inc edx
jmp +0x25365
rol edx, 0x03
clc
push rbx
shl bh, 0x1D
xor [rsp], edx
pop rbx
movsxd rdx, edx
add r10, rdx
jmp -0x879AD
jmp +0x41876
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
jmp -0x3C210
xor r8b, bl
cmp eax, 0x6D434734
neg r8b
rcr sil, 0xE8
ror r8b, 0x01
neg r8b
not r8b
bts esi, r9d
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, 0xA1
bts r8w, cx
sub rdi, 0x08
btc r8d, ebp
mov [rdi], rsi
mov r8d, [r11]
test r10b, 0x36
jmp +0x67B53
add r11, 0x04
jmp +0x6362
xor r8d, ebx
clc
stc
rol r8d, 0x02
jmp -0x3DE4E
neg r8d
not r8d
inc r8d
not r8d
inc r8d
push rbx
xor [rsp], r8d
dec bl
pop rbx
clc
test bl, 0xB5
movsxd r8, r8d
stc
add r10, r8
jmp -0x50256
jmp +0x4A1DA
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
or r8b, r10b
mov rbp, [rdi+0x08]
shld r8, r15, 0x1D
btr r8d, ebp
bsf r8d, r9d
add r9, rbp
movzx r8d, r10w
mov [rdi+0x08], r9
not r8b
movsxd r8, edi
cmovo r8d, ecx
pushfq
sbb r8b, r14b
shl r8b, cl
pop [rdi]
ror r8b, 0xBC
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
ror r8d, 0x01
jmp -0x96DC4
dec r8d
rol r8d, 0x01
inc r8d
clc
test r10w, 0x6508
push rbx
movsx rbx, bp
xor [rsp], r8d
pop rbx
cmp r8b, dl
movsxd r8, r8d
cmp r10b, 0xAC
add r10, r8
jmp +0x84B58
jmp r10
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov rax, rdi
btr dx, 0x95
cmovns rdx, r15
shl dl, cl
sub rdi, 0x08
mov [rdi], rax
shr dl, 0x7E
mov edx, [r11]
stc
clc
add r11, 0x04
cmp r8b, 0x29
clc
stc
xor edx, ebx
jmp -0x69439
rol edx, 0x01
bswap edx
jmp -0x508E2
dec edx
stc
cmc
test r12, 0x11AF6184
bswap edx
push rbx
xor [rsp], edx
btc rbx, rsi
ror bh, 0x12
pop rbx
cmp r11d, r15d
clc
movsxd rdx, edx
cmp dl, r11b
test r13w, 0x214D
jmp -0x1B218
add r10, rdx
jmp +0x8E5B3
jmp +0xB00
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
movzx r8d, byte ptr [r11]
cmovz rsi, rsp
add r11, 0x01
rol sil, cl
rcr esi, 0x4E
shld si, di, 0xDB
xor r8b, bl
shld si, r8w, 0xD8
movzx si, bl
neg r8b
jmp -0x44832
ror r8b, 0x01
sbb sil, 0xCB
or si, r15w
ror rsi, cl
neg r8b
not r8b
bts si, r13w
cmp r9b, 0xF4
bsr esi, r8d
xor bl, r8b
movsxd rsi, esi
xchg si, si
rol sil, cl
mov rsi, [rsp+r8*1]
sub rdi, 0x08
movzx r8d, sp
mov [rdi], rsi
shl r8, 0x13
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
rol r8d, 0x02
clc
cmp sil, cl
stc
neg r8d
not r8d
inc r8d
not r8d
jmp +0x4B618
inc r8d
push rbx
xor [rsp], r8d
sub bh, 0x95
pop rbx
cmp r13b, 0xD0
movsxd r8, r8d
jmp -0xA6128
add r10, r8
jmp +0x5AB78
jmp +0x3F95
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
movzx r8d, byte ptr [r11]
dec esi
rol sil, 0x00
add r11, 0x01
shl sil, 0x43
xor r8b, bl
shl si, 0x87
neg r8b
movsxd rsi, esi
movsx rsi, cx
jmp -0x4AB67
ror r8b, 0x01
sbb si, si
cmovno si, bp
neg r8b
bts esi, ebp
sbb si, 0x6C8
not r8b
rcr esi, cl
xor bl, r8b
shl sil, cl
bsr si, sp
sbb sil, 0x46
mov rsi, [rsp+r8*1]
add r8b, 0xE8
shr r8, cl
sub rdi, 0x08
btr r8w, 0xCE
shl r8w, cl
mov [rdi], rsi
bsr r8w, bp
btr r8w, 0xA8
rol r8w, 0x41
mov r8d, [r11]
cmc
add r11, 0x04
cmc
cmp r11b, r15b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp +0x9C2FB
not r8d
jmp -0xBCBD
inc r8d
jmp -0x8468F
not r8d
inc r8d
clc
stc
push rbx
shrd rbx, rdx, 0x74
sbb ebx, ecx
mov rbx, 0x68DC6371
xor [rsp], r8d
movsx rbx, r14w
shr bh, 0x08
pop rbx
test di, 0x3163
movsxd r8, r8d
stc
add r10, r8
jmp +0x9C323
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
movzx r8d, byte ptr [r11]
xor si, r14w
inc sil
add r11, 0x01
dec esi
sbb si, bp
bt esi, edx
xor r8b, bl
cmovnz si, bx
neg r8b
ror r8b, 0x01
btr esi, esi
mov sil, r9b
sar sil, cl
neg r8b
sar sil, 0x6A
not r8b
xor bl, r8b
xchg sil, sil
movzx si, bpl
mov rsi, [rsp+r8*1]
ror r8d, 0x86
sub rdi, 0x08
xadd r8d, r8d
mov [rdi], rsi
inc r8
bsf r8w, r13w
dec r8w
mov r8d, [r11]
cmp r13d, 0x3D0E02C5
cmp bpl, al
add r11, 0x04
test bpl, 0xFD
test dl, r14b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp -0x29EB9
not r8d
inc r8d
not r8d
jmp +0x83BDD
inc r8d
push rbx
cmovz bx, r12w
xor [rsp], r8d
sar bx, cl
xadd bl, bl
not ebx
pop rbx
movsxd r8, r8d
add r10, r8
jmp +0x11382
jmp -0x151BE
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
movzx r8d, byte ptr [r11]
add r11, 0x01
rcl esi, 0xA1
sub si, 0x5FD2
xor r8b, bl
cmovb esi, r11d
neg r8b
rcr si, 0x24
btc si, di
movzx esi, dx
ror r8b, 0x01
btr si, sp
movsx si, dil
neg r8b
movsx esi, r9w
test r11b, 0xD1
bts si, sp
not r8b
xor si, r14w
xor bl, r8b
setnb sil
movsx rsi, bp
movsx esi, dx
mov rsi, [rsp+r8*1]
sub rdi, 0x08
bsr r8w, r8w
or r8b, 0x94
and r8w, sp
mov [rdi], rsi
test r15b, r8b
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
cmc
rol r8d, 0x02
cmp sp, 0x4775
neg r8d
jmp -0x7484E
not r8d
jmp +0x5719A
inc r8d
not r8d
jmp +0x34C63
inc r8d
cmp bpl, cl
jmp -0x2BC9B
push rbx
shl ebx, cl
xor [rsp], r8d
bts bx, di
pop rbx
movsxd r8, r8d
test r15w, 0x38F
jmp +0x36289
add r10, r8
jmp -0x81C7F
jmp +0x51A8B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
movzx r8d, byte ptr [r11]
cmc
setnp sil
add sil, spl
add r11, 0x01
xor r8b, bl
sbb si, 0x5981
sub r8b, 0x73
ror r8b, 0x01
bsr esi, r12d
btc si, r13w
bts si, cx
not r8b
and si, 0xDB2
bsr rsi, r11
rcr rsi, 0x8D
sub r8b, 0xF1
xor r8b, 0xEF
xchg sil, sil
bts esi, r13d
bswap esi
xor bl, r8b
mov esi, [rsp+r8*1]
sbb r8, 0x685E6004
sub rdi, 0x04
movzx r8, r11w
mov [rdi], esi
btc r8w, bp
mov r8d, [r11]
add r11, 0x04
cmp edx, r11d
xor r8d, ebx
jmp +0x7849D
not r8d
jmp -0x274BA
dec r8d
neg r8d
stc
cmc
ror r8d, 0x03
test spl, 0x40
not r8d
push rbx
xor [rsp], r8d
movzx ebx, r13w
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x68F8D
jmp +0x84476
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
sar dx, cl
sub rdi, 0x08
bt edx, r15d
add dh, 0xD8
mov [rdi], rax
cqo
xadd dx, dx
mov edx, [r11]
test r13b, 0x86
cmp r14b, cl
add r11, 0x04
xor edx, ebx
clc
rol edx, 0x01
jmp -0xB2848
bswap edx
jmp -0x6B8D
dec edx
cmp rbx, 0x78467BB0
bswap edx
cmp r12b, r9b
jmp +0x44F62
push rbx
xor [rsp], edx
pop rbx
cmc
movsxd rdx, edx
stc
add r10, rdx
jmp -0x2F2A
jmp +0x4A0F1
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
rcr dh, 0xEA
add r11, 0x08
inc dl
test dx, 0x2EB1
and dl, dh
xor r8, rbx
movsx edx, r15w
movzx edx, dx
cmc
rol r8, 0x05
movzx edx, r13w
movsx dx, sil
bswap r8
jmp +0xD49A6
inc r8
movzx rdx, r15w
stc
xor r8, 0x18BC78EF
sub r8, 0x59F76389
cmc
neg dl
ror dh, cl
bswap r8
xor rdx, r15
xadd dl, dh
xor rbx, r8
bts edx, edi
inc rdx
bt edx, 0x56
sub rdi, 0x08
shld dx, cx, 0x13
sar edx, 0x4F
mov [rdi], r8
mov edx, [r11]
clc
test r14w, di
cmp r9w, 0x330
add r11, 0x04
cmp r11, r15
jmp -0xB6B3
xor edx, ebx
cmc
xor edx, 0x668D6E25
clc
add edx, 0x735D6DA4
not edx
cmp r14b, 0xEC
neg edx
jmp -0xD5CB7
inc edx
rol edx, 0x03
cmc
test dl, r8b
push rbx
rol ebx, cl
sar bl, cl
add bx, 0x491C
xor [rsp], edx
and bx, 0x575B
shr bx, 0x59
pop rbx
movsxd rdx, edx
add r10, rdx
jmp +0x92D83
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
setbe r8b
bswap r8d
movsx r8d, ax
mov [rdi+0x08], r9
movsx r8, bx
setb r8b
xchg r8b, r8b
pushfq
pop [rdi]
mov r8d, [r11]
stc
add r11, 0x04
xor r8d, ebx
test bh, 0x10
jmp +0x4E813
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x80280
dec r8d
rol r8d, 0x01
jmp +0xE425
inc r8d
cmc
cmp r13w, 0x388E
push rbx
not bh
cmc
xor [rsp], r8d
shrd bx, r11w, 0x00
pop rbx
test al, r15b
cmc
movsxd r8, r8d
cmc
stc
add r10, r8
jmp -0x18712
push r10
ret
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
mov r8, [r11]
test r9, 0x1CD947A8
movzx dx, bpl
adc rdx, r9
add r11, 0x08
shld rdx, rbx, 0xE3
movsxd rdx, edi
dec dh
xor r8, rbx
bt rdx, r13
movzx rdx, r13w
rol r8, 0x05
cqo
movzx rdx, sp
bswap r8
cmovbe rdx, r13
xchg dh, dh
inc r8
btc rdx, r11
rcl dx, cl
or dl, r9b
xor r8, 0x18BC78EF
sub r8, 0x59F76389
and dx, 0x19DB
cdq
xchg dx, dx
bswap r8
shl dh, 0xCE
and dx, 0x3D0B
inc dl
xor rbx, r8
ror dl, cl
movsx edx, r13w
shld edx, r14d, 0xBA
sub rdi, 0x08
mov [rdi], r8
bt dx, r11w
bts rdx, rsp
dec dx
mov edx, [r11]
cmp spl, r10b
jmp +0x388D2
add r11, 0x04
xor edx, ebx
xor edx, 0x668D6E25
test dh, 0x02
add edx, 0x735D6DA4
clc
not edx
cmp ch, bl
test bpl, spl
neg edx
jmp +0x50DBB
inc edx
rol edx, 0x03
stc
push rbx
sar rbx, cl
xor [rsp], edx
bt bx, 0x62
xadd bh, bh
pop rbx
stc
movsxd rdx, edx
cmp r8, 0x3327521
clc
cmc
add r10, rdx
jmp +0xF2FF
jmp -0x5073C
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
or r8b, 0x24
xchg r8b, r8b
btc r8, 0xFC
add r9, rbp
movzx r8, di
sets r8b
mov [rdi+0x08], r9
mov r8d, r13d
cmovbe r8, r8
movsx r8, r12w
pushfq
xor r8b, al
not r8w
mov r8b, sil
pop [rdi]
mov r8d, [r11]
cmc
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
cmc
stc
ror r8d, 0x01
jmp +0x4C894
dec r8d
stc
rol r8d, 0x01
jmp -0x69214
inc r8d
clc
push rbx
bswap bx
adc bx, r10w
xor [rsp], r8d
ror bh, 0x09
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp -0x1A68C
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
mov r9, [rdi]
cdq
mov esi, [rdi+0x08]
add rdi, 0x0C
rol dx, cl
mov [r9], esi
btr dx, di
movsx dx, r13b
mov edx, [r11]
cmc
cmp rcx, 0x13975F0E
add r11, 0x04
test bpl, bl
clc
xor edx, ebx
rol edx, 0x01
add edx, 0x6FF841C6
clc
rol edx, 0x03
inc edx
push rbx
adc bx, r10w
movsx bx, r10b
xor [rsp], edx
pop rbx
test r14b, r8b
movsxd rdx, edx
cmc
jmp -0x68FA1
add r10, rdx
jmp -0x3951
push r10
ret
mov r8, [r11]
not edx
btc edx, r9d
shl dx, 0x88
add r11, 0x08
rcl edx, 0x93
rol dl, cl
xor r8, rbx
cdq
rcl dh, 0xDE
bt edx, r13d
rol r8, 0x05
bswap r8
jmp +0x3BA1C
inc r8
movsx rdx, r11w
bts rdx, rdi
sar dx, cl
xor r8, 0x18BC78EF
sub r8, 0x59F76389
bswap r8
xor rbx, r8
or dh, 0xBB
sub dl, r9b
sub rdi, 0x08
shl dl, cl
xor dh, 0x52
add dx, bp
mov [rdi], r8
movzx rdx, ax
mov edx, [r11]
add r11, 0x04
cmp bp, bx
xor edx, ebx
test r8, rsi
cmp r13w, dx
stc
xor edx, 0x668D6E25
add edx, 0x735D6DA4
not edx
cmc
test r11w, 0x3A3
neg edx
jmp +0x8126
inc edx
clc
rol edx, 0x03
stc
push rbx
movzx rbx, r8w
xor [rsp], edx
movsx ebx, r9w
or bl, r13b
btc rbx, 0xB0
pop rbx
movsxd rdx, edx
test dh, 0x16
add r10, rdx
jmp +0x10388
jmp -0x40FD8
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
shl sil, 0x28
add esi, edi
cmovl rsi, rdi
add r11, 0x01
xor r8b, bl
xchg sil, sil
shl sil, cl
neg r8b
ror r8b, 0x01
neg r8b
adc sil, 0x83
shl sil, cl
bswap rsi
not r8b
dec esi
mov sil, 0x8E
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, cl
sub rdi, 0x08
sar r8b, 0xD4
mov [rdi], rsi
btc r8w, r11w
mov r8d, [r11]
clc
test esi, 0x5FFD2DB2
cmc
add r11, 0x04
test bpl, r11b
xor r8d, ebx
clc
stc
rol r8d, 0x02
stc
cmp r13b, r15b
neg r8d
not r8d
inc r8d
jmp -0x265B0
not r8d
jmp +0x61FFD
inc r8d
push rbx
xor [rsp], r8d
neg ebx
pop rbx
test r11b, 0x60
clc
cmp edi, 0x7F310A36
movsxd r8, r8d
test r15b, 0xBB
add r10, r8
jmp -0x1DE9A
jmp +0x3270B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
mov [rdi+0x08], r9
xchg r8, r8
pushfq
btc r8, 0xF4
pop [rdi]
cmc
not r8b
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
test r13b, cl
xor r8d, 0x369B465A
clc
ror r8d, 0x01
jmp +0x8C06C
dec r8d
jmp -0x9F98B
rol r8d, 0x01
jmp -0x1D76C
inc r8d
push rbx
inc bl
xor [rsp], r8d
pop rbx
stc
movsxd r8, r8d
cmc
add r10, r8
push r10
ret
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
shl esi, 0xCA
adc esi, 0x58D753A7
xor r8b, bl
sub r8b, 0x73
xchg esi, esi
btc si, 0x8B
stc
ror r8b, 0x01
sar rsi, 0xF7
shl sil, cl
not r8b
add sil, 0x8D
sub r8b, 0xF1
xor sil, 0xE8
sar sil, 0x55
xor r8b, 0xEF
movsx esi, bx
xor bl, r8b
shl si, 0x4A
mov esi, [rsp+r8*1]
bt r8, 0x0A
sub rdi, 0x04
mov [rdi], esi
xadd r8b, r8b
mov r8d, [r11]
cmp ax, 0x7070
clc
add r11, 0x04
stc
xor r8d, ebx
jmp -0xB7CC7
not r8d
dec r8d
neg r8d
ror r8d, 0x03
test r11w, 0x4DFF
not r8d
jmp -0x33684
push rbx
sbb bl, 0xF5
xor [rsp], r8d
rcr bh, 0xE0
mov bl, spl
pop rbx
movsxd r8, r8d
stc
clc
add r10, r8
jmp +0x1DC54
jmp +0x7FB9F
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
setl dh
btc dx, bp
sub rdi, 0x08
shl rdx, cl
mov [rdi], rax
jmp -0x3A4C9
mov edx, [r11]
stc
test r13b, 0x92
cmp ecx, 0x22F80CFB
add r11, 0x04
xor edx, ebx
jmp -0xB62C
rol edx, 0x01
jmp +0x843C1
bswap edx
dec edx
clc
bswap edx
cmc
push rbx
seto bh
xor [rsp], edx
bts rbx, rax
pop rbx
jmp -0xACF4A
movsxd rdx, edx
test r11w, 0x72CD
add r10, rdx
jmp +0xA3654
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
add r11, 0x08
sbb dl, dil
sub edx, 0x7A2C011B
cdq
xor r8, rbx
btr edx, 0x3C
mov dl, dil
bt dx, 0xAB
rol r8, 0x05
cdq
bswap r8
cmovo edx, esi
movzx rdx, r12w
movsx rdx, r10w
inc r8
sbb dx, 0x2BAD
movzx rdx, r14w
xor r8, 0x18BC78EF
jmp -0x5C01
sub r8, 0x59F76389
shld dx, r9w, 0xD7
or dl, 0xE2
bswap r8
sbb dh, 0xC4
rcr dh, cl
and dl, r13b
xor rbx, r8
clc
rcr dx, cl
sub rdi, 0x08
xadd dx, dx
movsx edx, si
mov [rdi], r8
rcr dh, cl
cdq
jmp +0x96E4
mov edx, [r11]
cmp r14b, sil
add r11, 0x04
cmp r14b, al
stc
xor edx, ebx
test r11, 0x43D10059
xor edx, 0x668D6E25
add edx, 0x735D6DA4
test ah, 0xF3
cmp bl, spl
not edx
clc
jmp +0x3C894
neg edx
inc edx
clc
rol edx, 0x03
push rbx
xor [rsp], edx
movsx ebx, bp
pop rbx
movsxd rdx, edx
cmc
clc
stc
add r10, rdx
jmp -0x36342
jmp +0x88912
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
bts bp, bp
rcl bp, cl
movsx r8d, bp
mov rbp, [rdi+0x08]
or r8b, 0xB8
ror r8, 0x12
shrd r8w, ax, 0xF6
add r9, rbp
cmovnl r8w, r13w
not r8b
movsx r8, r9w
mov [rdi+0x08], r9
movsx r8d, dx
movzx r8, dx
pushfq
sub r8d, 0x44D249DC
pop [rdi]
movzx r8, r13w
xor r8, rcx
btr r8d, 0xAB
mov r8d, [r11]
jmp -0xB66F6
add r11, 0x04
clc
test si, sp
xor r8d, ebx
clc
xor r8d, 0x369B465A
cmc
ror r8d, 0x01
jmp +0x7BE9A
dec r8d
cmc
jmp +0x2E7FA
rol r8d, 0x01
jmp -0xDD6CA
inc r8d
clc
test r13b, r9b
push rbx
bts bx, r9w
add ebx, esp
xor [rsp], r8d
bt bx, r8w
test r13b, r10b
inc bh
pop rbx
test cl, 0x52
cmp spl, r9b
movsxd r8, r8d
cmp dl, 0x8D
add r10, r8
jmp +0x50E74
jmp r10
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov r8, [r11]
rcl dh, 0x5F
add r11, 0x08
xor r8, rbx
bts dx, r13w
movsx rdx, r11w
rol r8, 0x05
bswap r8
movsx dx, r13b
movzx edx, r14w
mov dl, dil
inc r8
stc
shrd rdx, r11, 0x19
xor r8, 0x18BC78EF
mov dl, 0x58
sub r8, 0x59F76389
bswap r8
xor dx, ax
rcr dh, 0x6A
bsr rdx, r11
xor rbx, r8
movzx rdx, r8w
rcl dl, 0x65
rcr dl, cl
sub rdi, 0x08
sbb dx, ax
movsxd rdx, eax
mov [rdi], r8
mov edx, [r11]
cmc
add r11, 0x04
test r12b, 0xA2
stc
xor edx, ebx
cmp spl, cl
xor edx, 0x668D6E25
add edx, 0x735D6DA4
cmc
cmp si, si
jmp -0x5F6F5
not edx
stc
neg edx
inc edx
rol edx, 0x03
push rbx
btc rbx, rax
sub ebx, 0x5F1B72F6
mov ebx, r9d
xor [rsp], edx
pop rbx
test r14b, r8b
cmc
cmp r11b, bpl
movsxd rdx, edx
add r10, rdx
jmp +0x590F7
jmp -0x24DB7
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
shl r8, cl
shl rbp, 0x78
mov rbp, [rdi+0x08]
sar r8b, 0x7F
add r9, rbp
movsx r8, r13w
mov [rdi+0x08], r9
movzx r8, r11w
pushfq
pop [rdi]
shl r8d, cl
mov r8d, [r11]
clc
stc
add r11, 0x04
cmc
xor r8d, ebx
cmp dh, 0xF5
xor r8d, 0x369B465A
jmp -0x2EDA7
ror r8d, 0x01
dec r8d
clc
rol r8d, 0x01
jmp +0x5AC2B
inc r8d
cmp dil, 0xC9
push rbx
adc ebx, 0x5049777A
shr bl, cl
xor [rsp], r8d
btr bx, 0xC9
pop rbx
movsxd r8, r8d
jmp -0x46FB9
add r10, r8
jmp +0x16726
jmp r10
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov r9, [rdi]
mov esi, [rdi+0x08]
add rdi, 0x0C
mov dl, r8b
sbb edx, 0x56AF20CB
sar rdx, 0xEB
mov [r9], esi
mov edx, [r11]
test si, di
jmp -0x3DD8E
add r11, 0x04
test r11w, 0x5137
xor edx, ebx
stc
rol edx, 0x01
test r10b, 0x8E
cmp esp, ebp
add edx, 0x6FF841C6
jmp +0x3AA31
rol edx, 0x03
jmp +0x2AF71
inc edx
clc
cmc
push rbx
cmc
neg ebx
movsx rbx, r11w
xor [rsp], edx
and bh, 0xB3
dec rbx
movzx rbx, dx
pop rbx
movsxd rdx, edx
add r10, rdx
push r10
ret
mov r8, [r11]
bts edx, r10d
xadd rdx, rdx
add rdx, rsp
add r11, 0x08
bswap rdx
xor r8, rbx
bswap edx
rol r8, 0x05
xchg dx, dx
bswap r8
movzx rdx, r12w
bswap dx
cmovnl dx, r9w
inc r8
ror edx, cl
btc edx, r14d
xor r8, 0x18BC78EF
xchg dx, dx
sub r8, 0x59F76389
btr dx, 0x24
mov dl, r8b
bswap r8
movsx edx, r8w
bt edx, r9d
movsxd rdx, eax
xor rbx, r8
shr dl, cl
rcl dx, cl
bt rdx, r15
sub rdi, 0x08
bsf rdx, rsi
mov [rdi], r8
shl dh, cl
mov edx, [r11]
cmp r14w, 0x6800
add r11, 0x04
cmc
jmp -0xD1889
xor edx, ebx
stc
cmc
test r14b, dl
xor edx, 0x668D6E25
clc
cmc
add edx, 0x735D6DA4
stc
not edx
neg edx
jmp +0x714ED
inc edx
rol edx, 0x03
cmp ax, 0x4DE1
push rbx
movsx rbx, r14w
xor [rsp], edx
cmc
bts bx, bx
btr rbx, r8
pop rbx
movsxd rdx, edx
test bpl, r14b
stc
add r10, rdx
jmp -0x653ED
jmp +0x840C2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
mov rsi, rsp
not si
add r11, 0x01
neg esi
shl si, cl
xor r8b, bl
neg r8b
movzx esi, bp
bt si, di
ror r8b, 0x01
sub rsi, 0x3113452A
mov si, 0x7D49
neg r8b
rcl rsi, 0xB0
cmp r10b, bl
not r8b
movsx si, cl
cmp r9, 0x9BB5EDF
bt esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
bts r8d, r14d
not r8b
adc r8d, r9d
sub rdi, 0x08
mov [rdi], rsi
mov r8d, [r11]
add r11, 0x04
cmp r12b, 0xCF
xor r8d, ebx
clc
cmc
rol r8d, 0x02
cmc
cmp cx, 0x2552
test r12, 0x251600AE
neg r8d
jmp -0x73801
not r8d
inc r8d
jmp +0x628A2
not r8d
inc r8d
push rbx
shr bl, 0xD3
xor [rsp], r8d
rcl bh, 0xAA
test r13d, edx
pop rbx
cmc
test r11w, 0x7844
movsxd r8, r8d
add r10, r8
jmp -0x27AD2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
bts bp, sp
not ebp
mov rbp, [rdi+0x08]
inc r8b
movsx r8w, r15b
add r9, rbp
movsx r8d, r12w
movsxd r8, esp
mov [rdi+0x08], r9
mov r8b, r8b
bswap r8w
movsxd r8, r14d
pushfq
bt r8, 0x0B
rcl r8, 0x37
ror r8b, cl
pop [rdi]
bt r8w, 0x54
or r8b, r10b
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x5F4B5
xor r8d, ebx
test r9b, al
cmc
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x4F1E9
dec r8d
cmc
rol r8d, 0x01
jmp -0x9CABC
inc r8d
push rbx
stc
xadd ebx, ebx
sar rbx, 0x1A
xor [rsp], r8d
pop rbx
cmp r15d, edi
movsxd r8, r8d
test r8d, 0xC6E6F86
add r10, r8
jmp -0x1C633
push r10
ret
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
movzx r8d, byte ptr [r11]
shl si, 0x34
xchg si, si
jmp -0x560FC
add r11, 0x01
inc sil
rol esi, cl
mov esi, 0x4BD95602
xor r8b, bl
adc sil, 0x40
sub r8b, 0x73
bt rsi, r8
rcr rsi, 0x9C
setnz sil
ror r8b, 0x01
not r8b
shr sil, 0x62
sub r8b, 0xF1
rcl sil, 0xF7
shl si, 0xE0
sar si, 0xF7
xor r8b, 0xEF
sbb sil, dl
shr si, 0x54
xor bl, r8b
mov esi, [rsp+r8*1]
sub rdi, 0x04
movzx r8, sp
movsx r8w, bl
mov [rdi], esi
or r8b, 0x0E
movzx r8d, r9w
mov r8d, [r11]
test r14, r9
add r11, 0x04
jmp +0x89B1F
xor r8d, ebx
jmp -0x7C6DF
not r8d
jmp +0x570D4
dec r8d
stc
neg r8d
ror r8d, 0x03
stc
cmp r9w, 0x30B1
not r8d
jmp -0xAF2B8
push rbx
bsf rbx, r8
xchg bl, bl
xor [rsp], r8d
stc
pop rbx
cmc
jmp +0x4DA91
movsxd r8, r8d
add r10, r8
jmp +0x7D8BD
jmp -0x32910
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
shrd dx, r14w, 0x9E
xor edx, edx
sub rdi, 0x08
bsr dx, r8w
inc dh
ror dx, cl
mov [rdi], rax
btr dx, ax
cmovnle dx, r8w
mov edx, [r11]
test r9b, 0xE0
add r11, 0x04
test bpl, r12b
jmp -0x13CA0
xor edx, ebx
clc
rol edx, 0x01
jmp +0x28FC0
bswap edx
jmp -0x608C3
dec edx
bswap edx
push rbx
xor bh, ah
xor [rsp], edx
movzx ebx, r10w
adc bx, 0x7A7C
shl bl, cl
pop rbx
clc
test ax, ax
movsxd rdx, edx
stc
clc
add r10, rdx
jmp +0xD8CA2
jmp -0x4F72E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
movsx rdx, bx
add r11, 0x08
cmc
bt edx, 0xA9
xor r8, rbx
cmovle dx, di
rol r8, 0x05
cmovnz rdx, rdi
bswap r8
inc r8
xor r8, 0x18BC78EF
shl dx, cl
sub r8, 0x59F76389
mov dx, r11w
not dl
sub dh, 0xBE
bswap r8
rcl dh, cl
xchg dh, dl
xor rbx, r8
sub rdi, 0x08
shr dl, cl
cwd
ror dh, cl
mov [rdi], r8
jmp -0x4CD03
mov edx, [r11]
add r11, 0x04
clc
xor edx, ebx
xor edx, 0x668D6E25
cmp di, cx
add edx, 0x735D6DA4
not edx
neg edx
jmp +0x41114
inc edx
jmp -0x364B4
rol edx, 0x03
stc
cmp r15b, r14b
push rbx
xor [rsp], edx
bswap bx
shrd ebx, eax, 0xD5
mov ebx, r8d
pop rbx
cmp cl, 0x91
movsxd rdx, edx
test cx, 0x5EEA
cmp dil, cl
add r10, rdx
jmp +0x527E3
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
jmp -0x3BF7
add r9, rbp
movzx r8, r12w
jmp -0xC78A0
mov [rdi+0x08], r9
mov r8w, ax
movzx r8d, r8w
pushfq
pop [rdi]
mov r8d, [r11]
add r11, 0x04
test bp, 0x44
jmp +0xC373A
xor r8d, ebx
cmp spl, 0x16
xor r8d, 0x369B465A
clc
jmp -0x491B2
ror r8d, 0x01
jmp -0x7E809
dec r8d
rol r8d, 0x01
jmp +0xAC971
inc r8d
test ah, 0xE7
push rbx
xor [rsp], r8d
sbb rbx, 0x60AD13E6
jmp +0x42A91
pop rbx
clc
movsxd r8, r8d
cmp r10b, r11b
add r10, r8
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
mov r8, [r11]
add r11, 0x08
shl edx, cl
xor r8, rbx
btr edx, 0xDB
mov edx, 0x52AE0860
movsx dx, spl
rol r8, 0x05
cwd
cdq
movzx rdx, r13w
bswap r8
inc r8
or dh, 0xEF
jmp +0x2A403
xor r8, 0x18BC78EF
bt dx, sp
sub r8, 0x59F76389
bswap r8
add rdx, rbp
xor rbx, r8
shl dl, 0xCB
sub rdi, 0x08
movzx dx, r15b
rcr dh, cl
not edx
mov [rdi], r8
shr dh, cl
cmc
rcr dl, cl
mov edx, [r11]
jmp -0x189B8
add r11, 0x04
test r12b, bpl
cmc
xor edx, ebx
clc
test r15d, r11d
xor edx, 0x668D6E25
add edx, 0x735D6DA4
test dil, bpl
not edx
neg edx
inc edx
jmp +0x8495E
rol edx, 0x03
test r11b, 0x68
push rbx
xor [rsp], edx
rcl ebx, cl
movzx rbx, cx
cmc
pop rbx
movsxd rdx, edx
add r10, rdx
jmp -0x96BD6
jmp +0x984C5
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
stc
movsx rbp, si
ror bpl, cl
mov rbp, [rdi+0x08]
sar r8b, 0xFF
add r9, rbp
movzx r8w, al
mov [rdi+0x08], r9
cmovle r8, r13
pushfq
pop [rdi]
mov r8d, [r11]
cmp r14b, 0xBE
test r15b, bpl
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x3A5C8
dec r8d
clc
rol r8d, 0x01
inc r8d
cmp dx, 0x506B
clc
push rbx
xor [rsp], r8d
sbb bl, 0xF6
xadd bx, bx
movzx rbx, cx
pop rbx
cmp r12w, 0x1F70
movsxd r8, r8d
stc
test bpl, r10b
add r10, r8
jmp -0x4C463
jmp r10
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
mov r9, [rdi]
rcr dx, cl
and dx, 0x72B
shl si, 0x77
mov esi, [rdi+0x08]
bt edx, r10d
add rdi, 0x0C
shl dx, 0xBC
cqo
mov [r9], esi
mov edx, [r11]
stc
add r11, 0x04
cmp ax, cx
xor edx, ebx
clc
rol edx, 0x01
cmc
add edx, 0x6FF841C6
jmp +0x55878
rol edx, 0x03
jmp -0x9874
inc edx
cmp sil, 0x16
push rbx
btc ebx, 0xFD
movzx ebx, r13w
xor [rsp], edx
test r10, r14
pop rbx
cmc
movsxd rdx, edx
add r10, rdx
jmp -0x89603
jmp r10
mov r8, [r11]
add r11, 0x08
cmovp rdx, r14
xor r8, rbx
rcr dl, cl
rol r8, 0x05
movsx edx, ax
movsxd rdx, esp
jmp +0x2C821
bswap r8
inc r8
bsf rdx, r8
clc
or dx, di
xor r8, 0x18BC78EF
btr dx, 0x90
stc
sub r8, 0x59F76389
dec dh
inc dh
neg dl
bswap r8
btr edx, r11d
shld rdx, rcx, 0x72
xor rbx, r8
bsf dx, r8w
or dl, 0x2C
movsx edx, cx
sub rdi, 0x08
rcr dl, cl
adc dx, 0x300D
bt rdx, r9
mov [rdi], r8
mov edx, [r11]
add r11, 0x04
jmp -0x7D1C0
xor edx, ebx
xor edx, 0x668D6E25
cmp sp, cx
jmp +0x15354
add edx, 0x735D6DA4
stc
test r11w, 0x3217
not edx
test r8d, 0x1ABD7B64
neg edx
jmp +0x423AD
inc edx
clc
cmc
rol edx, 0x03
stc
test r9b, 0x5A
cmp r11, r13
push rbx
clc
xor [rsp], edx
shr ebx, cl
add bl, r12b
test r9d, 0x1C7425BE
pop rbx
movsxd rdx, edx
stc
cmp rsi, 0x6C441FC5
test r10d, 0x1E1A0474
add r10, rdx
jmp +0x28F5B
jmp -0x410A0
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
or sil, cl
btc si, di
test r15d, r12d
add r11, 0x01
xor r8b, bl
cmovnle si, r9w
neg r8b
btr si, si
btc rsi, 0x20
ror r8b, 0x01
rcl sil, 0xFE
sbb rsi, 0x103B38AE
neg r8b
not r8b
sbb si, 0x916
or esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
mov [rdi], rsi
shl r8b, cl
mov r8d, [r11]
jmp +0x901E8
add r11, 0x04
xor r8d, ebx
jmp +0x4B3DC
rol r8d, 0x02
neg r8d
not r8d
jmp +0x1AF10
inc r8d
jmp -0x8521D
not r8d
inc r8d
stc
cmp di, 0x5EB4
push rbx
cmp r13b, sil
xor [rsp], r8d
xchg bx, bx
sub ebx, 0x3C2A06D8
pop rbx
movsxd r8, r8d
cmc
jmp +0x4D760
add r10, r8
jmp -0x3FF9D
jmp +0x28D9F
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
movsxd r8, ebp
mov [rdi+0x08], r9
movsx r8w, al
jmp -0x6045
pushfq
rcr r8b, cl
pop [rdi]
jmp -0x90DCA
mov r8d, [r11]
add r11, 0x04
cmp r11d, edx
test bpl, 0x48
xor r8d, ebx
stc
xor r8d, 0x369B465A
stc
jmp -0xBC9
ror r8d, 0x01
jmp +0x26FC9
dec r8d
jmp +0x8C74
rol r8d, 0x01
jmp +0x7369
inc r8d
test cx, 0x1F08
push rbx
xor [rsp], r8d
adc ebx, r8d
sbb rbx, rsp
pop rbx
movsxd r8, r8d
cmc
stc
add r10, r8
jmp +0x3F180
push r10
ret
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
dec sil
bsr si, r13w
bt si, 0x56
xor r8b, bl
adc sil, spl
bts esi, r8d
inc sil
neg r8b
movsx si, r15b
ror r8b, 0x01
not rsi
bts rsi, r12
neg r8b
mov sil, r10b
clc
not r8b
add sil, sil
rol sil, 0x0D
xor bl, r8b
add esi, r11d
mov rsi, [rsp+r8*1]
adc r8w, dx
sub rdi, 0x08
bsr r8w, r12w
sbb r8w, 0x7A1F
bt r8w, 0xE1
mov [rdi], rsi
sub r8b, 0xF3
mov r8w, 0x94E
shld r8w, bp, 0xA8
mov r8d, [r11]
add r11, 0x04
jmp -0x493BE
xor r8d, ebx
clc
rol r8d, 0x02
cmp dil, bl
test r12w, r15w
test r10b, 0x8F
neg r8d
jmp +0xA7179
not r8d
jmp -0x574BD
inc r8d
jmp -0x58ECD
not r8d
jmp +0x35663
inc r8d
jmp +0x3D34C
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
clc
add r10, r8
jmp -0x3C22C
jmp +0x56F6E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
sar edx, cl
sub rdi, 0x08
mov [rdi], rax
inc dl
shl rdx, 0x8A
cmp r15b, 0xCC
mov edx, [r11]
add r11, 0x04
test spl, 0xE7
test bl, r9b
cmc
xor edx, ebx
rol edx, 0x01
jmp -0x24787
bswap edx
jmp -0x1330E
dec edx
cmp edx, eax
bswap edx
cmp bl, r15b
clc
push rbx
clc
xor [rsp], edx
cmp bl, dil
pop rbx
cmc
test bl, r10b
movsxd rdx, edx
add r10, rdx
jmp +0x523BE
jmp -0x1CBD2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
mov r8, [r11]
jmp -0xE6A9D
add r11, 0x08
sar dx, cl
xor r8, rbx
rol r8, 0x05
cqo
bswap r8
jmp +0x6EDDD
inc r8
and dx, dx
xor r8, 0x18BC78EF
sub r8, 0x59F76389
inc dx
rcl dh, 0x15
bswap r8
test r8b, dil
shl dh, 0x3B
rcr dx, cl
xor rbx, r8
movsx dx, r8b
xchg dl, dh
and dl, r10b
sub rdi, 0x08
mov [rdi], r8
mov edx, [r11]
jmp +0x47051
add r11, 0x04
jmp -0x9E240
xor edx, ebx
test dl, 0x7A
xor edx, 0x668D6E25
stc
test si, r14w
add edx, 0x735D6DA4
test r12b, 0xB6
not edx
neg edx
inc edx
jmp +0x3EBFD
rol edx, 0x03
push rbx
xor [rsp], edx
pop rbx
movsxd rdx, edx
clc
add r10, rdx
jmp +0x5717C
jmp -0xEDB9
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
bts rsi, r14
inc sil
add r11, 0x01
movsx rsi, si
ror sil, 0xBC
xor r8b, bl
movsx esi, di
mov si, 0x325C
setnp sil
neg r8b
movzx rsi, dx
rcr si, cl
ror sil, cl
ror r8b, 0x01
rcr rsi, 0xC7
neg r8b
shl sil, 0xA6
sbb si, r13w
not r8b
adc sil, r10b
rcl sil, 0x44
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
jmp +0xB37E1
mov [rdi], rsi
xadd r8, r8
cmp r12b, 0xA1
mov r8d, [r11]
cmp r9b, 0x9F
cmc
jmp +0x32520
add r11, 0x04
stc
jmp -0xCD81E
xor r8d, ebx
stc
rol r8d, 0x02
cmp r9d, 0x43C35321
neg r8d
not r8d
inc r8d
jmp +0x64EC6
not r8d
jmp -0x4A752
inc r8d
test bx, r12w
push rbx
setp bh
xor [rsp], r8d
movsxd rbx, r14d
pop rbx
cmp r13w, 0x1267
movsxd r8, r8d
add r10, r8
jmp +0x2FB55
jmp +0x39243
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov bpl, 0xA1
adc r8w, 0x4B16
mov rbp, [rdi+0x08]
movzx r8w, bpl
add r9, rbp
mov [rdi+0x08], r9
not r8b
movsxd r8, ecx
pushfq
shrd r8w, r15w, 0xB6
pop [rdi]
shrd r8w, r11w, 0x09
mov r8d, [r11]
cmp esp, 0x40B35B13
add r11, 0x04
cmp r13, r10
stc
xor r8d, ebx
cmp r15b, sil
xor r8d, 0x369B465A
stc
ror r8d, 0x01
jmp +0x285F6
dec r8d
clc
jmp +0x60D2B
rol r8d, 0x01
inc r8d
push rbx
shld bx, r11w, 0xF8
shl bx, cl
add bl, 0xB2
xor [rsp], r8d
bts bx, ax
and rbx, 0x4D980F97
ror bl, 0x36
pop rbx
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0x803CD
jmp r10
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
movzx r8d, byte ptr [r11]
stc
test rcx, 0x34EA4385
sbb sil, r15b
add r11, 0x01
stc
xor r8b, bl
cmc
neg r8b
ror si, cl
bswap rsi
movzx esi, r12w
ror r8b, 0x01
btr esi, ebx
bsr rsi, rdi
shr sil, cl
neg r8b
shr rsi, cl
btr esi, r8d
btc si, r14w
not r8b
cmp r10, 0x532F4095
xor bl, r8b
mov sil, 0xB3
ror si, 0xAD
mov rsi, [rsp+r8*1]
shl r8b, cl
shld r8w, bx, 0xC2
sub rdi, 0x08
shl r8b, cl
btc r8, r15
test r13w, sp
mov [rdi], rsi
bts r8w, bp
movzx r8d, r12w
cmc
mov r8d, [r11]
test dil, 0x7F
add r11, 0x04
xor r8d, ebx
rol r8d, 0x02
stc
neg r8d
jmp +0xC8598
not r8d
jmp +0x16694
inc r8d
jmp -0x8F873
not r8d
jmp -0x4A7D8
inc r8d
stc
push rbx
sbb ebx, 0x27102D1A
setp bl
xor [rsp], r8d
bts rbx, rbx
pop rbx
cmc
test dil, 0x4E
cmp bpl, r10b
movsxd r8, r8d
clc
add r10, r8
jmp +0x86AB6
jmp +0x277E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
movzx dx, spl
shl dx, 0xAF
sub rdi, 0x08
xchg rdx, rdx
mov [rdi], rax
movsxd rdx, ebp
shr dh, 0xD0
mov edx, [r11]
add r11, 0x04
test r13b, 0x8C
cmc
test r8w, sp
xor edx, ebx
cmc
jmp +0xBF843
rol edx, 0x01
jmp -0x60973
bswap edx
jmp +0x586
dec edx
stc
bswap edx
push rbx
rcr bx, 0x0B
bt ebx, edx
xor [rsp], edx
cmp esp, 0x36BB23CD
pop rbx
cmc
cmp rdx, rbx
test r13b, 0x5E
movsxd rdx, edx
test edx, 0x2AC349BF
test rsp, rdi
add r10, rdx
jmp +0x48B12
jmp -0x3B547
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov r8, [r11]
cmovnl dx, r8w
or edx, 0x4FE964E3
add r11, 0x08
cwd
xor r8, rbx
bts edx, r9d
ror dx, 0xEB
cqo
rol r8, 0x05
xchg dx, dx
cdq
bswap r8
bswap rdx
inc r8
shr edx, 0x22
sub dx, r10w
xor r8, 0x18BC78EF
shld edx, r8d, 0xDE
rcr dh, cl
rol dx, 0xA0
sub r8, 0x59F76389
bsf dx, r10w
mov rdx, 0x6ABE4CC4
bswap r8
shl dh, 0x90
xor dl, ch
adc edx, 0x58CF5593
xor rbx, r8
sub rdx, r12
cmp r8b, 0xE2
sub rdi, 0x08
shr rdx, 0x32
sub dl, r12b
xor dh, 0x71
mov [rdi], r8
adc dl, r11b
mov edx, [r11]
stc
cmc
add r11, 0x04
test rbx, 0xF9117E9
cmp r8b, r14b
xor edx, ebx
xor edx, 0x668D6E25
add edx, 0x735D6DA4
not edx
neg edx
inc edx
jmp +0x25365
rol edx, 0x03
clc
push rbx
shl bh, 0x1D
xor [rsp], edx
pop rbx
movsxd rdx, edx
add r10, rdx
jmp -0x879AD
jmp +0x41876
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
jmp -0x3C210
xor r8b, bl
cmp eax, 0x6D434734
neg r8b
rcr sil, 0xE8
ror r8b, 0x01
neg r8b
not r8b
bts esi, r9d
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, 0xA1
bts r8w, cx
sub rdi, 0x08
btc r8d, ebp
mov [rdi], rsi
mov r8d, [r11]
test r10b, 0x36
jmp +0x67B53
add r11, 0x04
jmp +0x6362
xor r8d, ebx
clc
stc
rol r8d, 0x02
jmp -0x3DE4E
neg r8d
not r8d
inc r8d
not r8d
inc r8d
push rbx
xor [rsp], r8d
dec bl
pop rbx
clc
test bl, 0xB5
movsxd r8, r8d
stc
add r10, r8
jmp -0x50256
jmp +0x4A1DA
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
or r8b, r10b
mov rbp, [rdi+0x08]
shld r8, r15, 0x1D
btr r8d, ebp
bsf r8d, r9d
add r9, rbp
movzx r8d, r10w
mov [rdi+0x08], r9
not r8b
movsxd r8, edi
cmovo r8d, ecx
pushfq
sbb r8b, r14b
shl r8b, cl
pop [rdi]
ror r8b, 0xBC
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
ror r8d, 0x01
jmp -0x96DC4
dec r8d
rol r8d, 0x01
inc r8d
clc
test r10w, 0x6508
push rbx
movsx rbx, bp
xor [rsp], r8d
pop rbx
cmp r8b, dl
movsxd r8, r8d
cmp r10b, 0xAC
add r10, r8
jmp +0x84B58
jmp r10
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
mov r8, [r11]
rcr dh, 0xEA
add r11, 0x08
inc dl
test dx, 0x2EB1
and dl, dh
xor r8, rbx
movsx edx, r15w
movzx edx, dx
cmc
rol r8, 0x05
movzx edx, r13w
movsx dx, sil
bswap r8
jmp +0xD49A6
inc r8
movzx rdx, r15w
stc
xor r8, 0x18BC78EF
sub r8, 0x59F76389
cmc
neg dl
ror dh, cl
bswap r8
xor rdx, r15
xadd dl, dh
xor rbx, r8
bts edx, edi
inc rdx
bt edx, 0x56
sub rdi, 0x08
shld dx, cx, 0x13
sar edx, 0x4F
mov [rdi], r8
mov edx, [r11]
clc
test r14w, di
cmp r9w, 0x330
add r11, 0x04
cmp r11, r15
jmp -0xB6B3
xor edx, ebx
cmc
xor edx, 0x668D6E25
clc
add edx, 0x735D6DA4
not edx
cmp r14b, 0xEC
neg edx
jmp -0xD5CB7
inc edx
rol edx, 0x03
cmc
test dl, r8b
push rbx
rol ebx, cl
sar bl, cl
add bx, 0x491C
xor [rsp], edx
and bx, 0x575B
shr bx, 0x59
pop rbx
movsxd rdx, edx
add r10, rdx
jmp +0x92D83
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, rdi
sbb edx, r8d
sub rdi, 0x08
not dx
xchg dh, dl
mov [rdi], rax
mov edx, [r11]
stc
add r11, 0x04
xor edx, ebx
jmp -0x64F9F
rol edx, 0x01
bswap edx
jmp +0xB020F
dec edx
test r8d, eax
bswap edx
test r9w, bp
stc
cmc
push rbx
xor [rsp], edx
test r12b, r13b
bt rbx, rcx
btc bx, r12w
pop rbx
stc
test di, r8w
clc
movsxd rdx, edx
cmc
stc
test dil, r15b
add r10, rdx
jmp -0x9D029
jmp +0x62362
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
test r9, 0x1CD947A8
movzx dx, bpl
adc rdx, r9
add r11, 0x08
shld rdx, rbx, 0xE3
movsxd rdx, edi
dec dh
xor r8, rbx
bt rdx, r13
movzx rdx, r13w
rol r8, 0x05
cqo
movzx rdx, sp
bswap r8
cmovbe rdx, r13
xchg dh, dh
inc r8
btc rdx, r11
rcl dx, cl
or dl, r9b
xor r8, 0x18BC78EF
sub r8, 0x59F76389
and dx, 0x19DB
cdq
xchg dx, dx
bswap r8
shl dh, 0xCE
and dx, 0x3D0B
inc dl
xor rbx, r8
ror dl, cl
movsx edx, r13w
shld edx, r14d, 0xBA
sub rdi, 0x08
mov [rdi], r8
bt dx, r11w
bts rdx, rsp
dec dx
mov edx, [r11]
cmp spl, r10b
jmp +0x388D2
add r11, 0x04
xor edx, ebx
xor edx, 0x668D6E25
test dh, 0x02
add edx, 0x735D6DA4
clc
not edx
cmp ch, bl
test bpl, spl
neg edx
jmp +0x50DBB
inc edx
rol edx, 0x03
stc
push rbx
sar rbx, cl
xor [rsp], edx
bt bx, 0x62
xadd bh, bh
pop rbx
stc
movsxd rdx, edx
cmp r8, 0x3327521
clc
cmc
add r10, rdx
jmp +0xF2FF
jmp -0x5073C
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
setbe r8b
bswap r8d
movsx r8d, ax
mov [rdi+0x08], r9
movsx r8, bx
setb r8b
xchg r8b, r8b
pushfq
pop [rdi]
mov r8d, [r11]
stc
add r11, 0x04
xor r8d, ebx
test bh, 0x10
jmp +0x4E813
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x80280
dec r8d
rol r8d, 0x01
jmp +0xE425
inc r8d
cmc
cmp r13w, 0x388E
push rbx
not bh
cmc
xor [rsp], r8d
shrd bx, r11w, 0x00
pop rbx
test al, r15b
cmc
movsxd r8, r8d
cmc
stc
add r10, r8
jmp -0x18712
push r10
ret
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
mov rax, rdi
adc dl, r9b
sub rdi, 0x08
inc dx
mov [rdi], rax
shr rdx, cl
mov edx, [r11]
add r11, 0x04
xor edx, ebx
stc
clc
rol edx, 0x01
jmp +0x5502F
bswap edx
jmp +0x386D1
dec edx
cmp esp, r10d
bswap edx
cmc
push rbx
xor [rsp], edx
pop rbx
cmp si, r9w
movsxd rdx, edx
cmp r13b, 0x0C
jmp -0x63215
add r10, rdx
jmp -0x1F02C
jmp +0x7CE63
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
not edx
btc edx, r9d
shl dx, 0x88
add r11, 0x08
rcl edx, 0x93
rol dl, cl
xor r8, rbx
cdq
rcl dh, 0xDE
bt edx, r13d
rol r8, 0x05
bswap r8
jmp +0x3BA1C
inc r8
movsx rdx, r11w
bts rdx, rdi
sar dx, cl
xor r8, 0x18BC78EF
sub r8, 0x59F76389
bswap r8
xor rbx, r8
or dh, 0xBB
sub dl, r9b
sub rdi, 0x08
shl dl, cl
xor dh, 0x52
add dx, bp
mov [rdi], r8
movzx rdx, ax
mov edx, [r11]
add r11, 0x04
cmp bp, bx
xor edx, ebx
test r8, rsi
cmp r13w, dx
stc
xor edx, 0x668D6E25
add edx, 0x735D6DA4
not edx
cmc
test r11w, 0x3A3
neg edx
jmp +0x8126
inc edx
clc
rol edx, 0x03
stc
push rbx
movzx rbx, r8w
xor [rsp], edx
movsx ebx, r9w
or bl, r13b
btc rbx, 0xB0
pop rbx
movsxd rdx, edx
test dh, 0x16
add r10, rdx
jmp +0x10388
jmp -0x40FD8
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
or r8b, 0x24
xchg r8b, r8b
btc r8, 0xFC
add r9, rbp
movzx r8, di
sets r8b
mov [rdi+0x08], r9
mov r8d, r13d
cmovbe r8, r8
movsx r8, r12w
pushfq
xor r8b, al
not r8w
mov r8b, sil
pop [rdi]
mov r8d, [r11]
cmc
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
cmc
stc
ror r8d, 0x01
jmp +0x4C894
dec r8d
stc
rol r8d, 0x01
jmp -0x69214
inc r8d
clc
push rbx
bswap bx
adc bx, r10w
xor [rsp], r8d
ror bh, 0x09
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp -0x1A68C
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
mov r9, [rdi]
mov rsi, [rdi+0x08]
not r9
not rsi
stc
jmp +0x66650
and r9, rsi
mov [rdi+0x08], r9
cmovno r9w, di
not r9b
movsxd r9, r12d
pushfq
pop [rdi]
bts r9w, r15w
mov r9d, [r11]
cmp esi, ebp
stc
add r11, 0x04
xor r9d, ebx
dec r9d
stc
cmc
ror r9d, 0x01
jmp -0x55F29
bswap r9d
dec r9d
jmp -0x7003B
push rbx
xor [rsp], r9d
sub ebx, esp
and ebx, r11d
pop rbx
test r13b, spl
movsxd r9, r9d
add r10, r9
jmp -0xE0BE
jmp r10
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
mov [rdi+0x08], r9
xchg r8, r8
pushfq
btc r8, 0xF4
pop [rdi]
cmc
not r8b
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
test r13b, cl
xor r8d, 0x369B465A
clc
ror r8d, 0x01
jmp +0x8C06C
dec r8d
jmp -0x9F98B
rol r8d, 0x01
jmp -0x1D76C
inc r8d
push rbx
inc bl
xor [rsp], r8d
pop rbx
stc
movsxd r8, r8d
cmc
add r10, r8
push r10
ret
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
mov rax, rdi
cqo
sbb dl, spl
cdq
sub rdi, 0x08
sub rdx, r9
setb dl
mov [rdi], rax
shrd dx, r11w, 0x43
mov edx, [r11]
stc
add r11, 0x04
cmc
xor edx, ebx
rol edx, 0x01
jmp +0x9C3CB
bswap edx
jmp +0x2530C
dec edx
jmp -0x49EB9
bswap edx
push rbx
setnl bl
btc bx, 0x29
xor [rsp], edx
pop rbx
test cl, cl
cmc
movsxd rdx, edx
stc
cmp r11w, r10w
add r10, rdx
jmp +0x161E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rdx, [rdi]
rcl ch, cl
xchg sil, cl
mov rcx, [rdx]
mov [rdi], rcx
shl rsi, 0x42
sar si, 0xB4
mov esi, [r11]
cmp di, 0x143F
test spl, 0xB5
stc
add r11, 0x04
test si, 0x371F
cmp r8b, r11b
jmp +0x2D662
xor esi, ebx
cmp ax, 0xE24
test dl, r12b
jmp -0xB16E
sub esi, 0x189679E7
cmp r12b, dil
neg esi
rol esi, 0x01
test r13, r8
cmc
jmp -0x83032
not esi
test r8, 0x8F4596D
clc
push rbx
add rbx, r8
ror bx, cl
shl bx, 0xDD
xor [rsp], esi
inc rbx
ror rbx, cl
pop rbx
test r13b, 0xD2
jmp -0x540E0
movsxd rsi, esi
jmp +0xB14BD
add r10, rsi
jmp -0xBC7D4
jmp r10
mov r9, [rdi]
shrd rcx, r15, 0xD8
add sil, r9b
neg si
mov rcx, [rdi+0x08]
not r9
not rcx
inc sil
xor esi, ecx
or r9, rcx
mov [rdi+0x08], r9
mov si, r10w
jmp +0x8BF59
pushfq
pop [rdi]
jmp -0x3EF79
mov esi, [r11]
cmp bx, di
test rax, r13
add r11, 0x04
test di, sp
stc
test ch, 0xED
xor esi, ebx
cmc
xor esi, 0x5C1A0567
cmp rbx, r10
not esi
xor esi, 0x31A26B50
jmp -0x1D2C
bswap esi
jmp -0x58330
dec esi
clc
stc
push rbx
setnp bh
shr bx, cl
movsx rbx, r13w
xor [rsp], esi
or bl, bh
pop rbx
cmp r8d, 0x250E0097
stc
test r8, r12
movsxd rsi, esi
test r8b, r10b
add r10, rsi
jmp +0xE2A2A
push r10
ret
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
mov rdi, [rdi]
adc dx, 0x3C02
xor edx, 0x698062F3
mov edx, [r11]
stc
test r15b, 0x56
add r11, 0x04
stc
xor edx, ebx
cmp r11b, 0xA9
cmp ebx, r15d
add edx, 0x474B6610
clc
cmc
stc
bswap edx
add edx, 0x3AFC3FF0
neg edx
add edx, 0x397D2A26
jmp +0x5021
neg edx
clc
cmc
push rbx
bt rbx, 0xD0
xor [rsp], edx
movsxd rbx, esp
pop rbx
cmp ax, dx
movsxd rdx, edx
add r10, rdx
jmp +0xA1252
jmp -0x30786
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
mov rdx, rsp
bts r9w, r12w
xadd r9d, esi
cmovnl esi, ebp
mov rcx, 0x100
bswap si
rcl r9b, cl
seto r9b
lea r9, [rdi-0x80]
mov sil, spl
or sil, bl
and r9, 0xFFFFFFFFFFFFFFF0
dec sil
sub r9, rcx
mov rsp, r9
mov sil, spl
movsx esi, r11w
push rdi
jmp -0x1AEF4
pushfq
xchg si, di
mov dil, cl
mov rsi, rdx
xchg rdi, rdi
movsx edi, r10w
mov rdi, r9
jmp -0x1C28A
cld
jmp -0xDB18
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
test r9w, di
xchg dil, dil
btc di, 0x4D
popfq
pop rdi
jmp -0x275C4
jmp r10
mov rax, rdi
sub rdi, 0x08
test r10b, 0x61
rcl rdx, 0x15
mov [rdi], rax
movzx dx, al
and rdx, 0x63B20389
mov edx, [r11]
cmp r15b, r13b
stc
add r11, 0x04
xor edx, ebx
clc
cmc
rol edx, 0x01
jmp +0x258F6
bswap edx
jmp +0x5A5E2
dec edx
bswap edx
clc
push rbx
xor [rsp], edx
movzx bx, bl
bts bx, r10w
movzx ebx, bx
pop rbx
movsxd rdx, edx
add r10, rdx
jmp +0x3487
jmp -0x15E12
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
movzx r8d, byte ptr [r11]
cmovz rsi, rsp
add r11, 0x01
rol sil, cl
rcr esi, 0x4E
shld si, di, 0xDB
xor r8b, bl
shld si, r8w, 0xD8
movzx si, bl
neg r8b
jmp -0x44832
ror r8b, 0x01
sbb sil, 0xCB
or si, r15w
ror rsi, cl
neg r8b
not r8b
bts si, r13w
cmp r9b, 0xF4
bsr esi, r8d
xor bl, r8b
movsxd rsi, esi
xchg si, si
rol sil, cl
mov rsi, [rsp+r8*1]
sub rdi, 0x08
movzx r8d, sp
mov [rdi], rsi
shl r8, 0x13
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
rol r8d, 0x02
clc
cmp sil, cl
stc
neg r8d
not r8d
inc r8d
not r8d
jmp +0x4B618
inc r8d
push rbx
xor [rsp], r8d
sub bh, 0x95
pop rbx
cmp r13b, 0xD0
movsxd r8, r8d
jmp -0xA6128
add r10, r8
jmp +0x5AB78
jmp +0x3F95
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
add r11, 0x08
sbb dl, dil
sub edx, 0x7A2C011B
cdq
xor r8, rbx
btr edx, 0x3C
mov dl, dil
bt dx, 0xAB
rol r8, 0x05
cdq
bswap r8
cmovo edx, esi
movzx rdx, r12w
movsx rdx, r10w
inc r8
sbb dx, 0x2BAD
movzx rdx, r14w
xor r8, 0x18BC78EF
jmp -0x5C01
sub r8, 0x59F76389
shld dx, r9w, 0xD7
or dl, 0xE2
bswap r8
sbb dh, 0xC4
rcr dh, cl
and dl, r13b
xor rbx, r8
clc
rcr dx, cl
sub rdi, 0x08
xadd dx, dx
movsx edx, si
mov [rdi], r8
rcr dh, cl
cdq
jmp +0x96E4
mov edx, [r11]
cmp r14b, sil
add r11, 0x04
cmp r14b, al
stc
xor edx, ebx
test r11, 0x43D10059
xor edx, 0x668D6E25
add edx, 0x735D6DA4
test ah, 0xF3
cmp bl, spl
not edx
clc
jmp +0x3C894
neg edx
inc edx
clc
rol edx, 0x03
push rbx
xor [rsp], edx
movsx ebx, bp
pop rbx
movsxd rdx, edx
cmc
clc
stc
add r10, rdx
jmp -0x36342
jmp +0x88912
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rcx, [rdi+0x08]
btc esi, edi
neg sil
not r9
sbb sil, 0x76
and esi, ebx
xor sil, r13b
not rcx
stc
cmovns si, sp
not sil
or r9, rcx
movsx rsi, r14w
mov sil, 0x75
setno sil
mov [rdi+0x08], r9
bswap esi
mov si, bp
movzx esi, bp
pushfq
ror si, cl
btc si, 0x6A
xchg esi, esi
pop [rdi]
sbb sil, 0x6C
shr sil, 0x54
mov esi, [r11]
cmp r12b, 0x1F
add r11, 0x04
xor esi, ebx
test r13w, 0x242D
clc
jmp +0x3BFC1
xor esi, 0x5C1A0567
test r11b, r14b
not esi
xor esi, 0x31A26B50
jmp -0x2F693
bswap esi
dec esi
test esi, 0x1FA6650F
push rbx
not rbx
adc bx, 0x4C8C
xor [rsp], esi
sar rbx, cl
shl bl, 0xB3
pop rbx
test r12d, r11d
movsxd rsi, esi
clc
cmp dil, r8b
add r10, rsi
jmp -0x84C79
push r10
ret
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, rdi
movsx edx, ax
sub rdi, 0x08
rol dx, cl
xchg dh, dl
shl dl, 0xB2
mov [rdi], rax
rcr dl, cl
and dl, r14b
shl dx, 0xA7
mov edx, [r11]
cmp sil, 0xC7
jmp -0x44817
add r11, 0x04
cmp dil, 0xB0
cmc
test rdx, rsp
xor edx, ebx
cmc
rol edx, 0x01
jmp -0xA5607
bswap edx
jmp +0x31FD5
dec edx
test r10, 0x32231070
bswap edx
test r9d, 0x3880011C
push rbx
and ebx, edx
shl bl, cl
xor bh, 0xA5
xor [rsp], edx
dec bh
or bh, 0x3D
pop rbx
clc
test bp, r10w
stc
movsxd rdx, edx
test r10w, r9w
test r11b, 0x99
cmp sil, r12b
add r10, rdx
jmp +0xBA489
jmp -0x4C78A
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rdx, [rdi]
btc rcx, r9
mov rcx, [rdx]
cmovnz esi, r9d
cmp esi, r12d
mov [rdi], rcx
btc rsi, 0xEC
shrd esi, r13d, 0x6E
mov esi, [r11]
clc
add r11, 0x04
stc
xor esi, ebx
clc
cmp cl, spl
sub esi, 0x189679E7
jmp -0x8B797
neg esi
rol esi, 0x01
test r14b, r10b
cmp ebx, ecx
jmp +0xDE87D
not esi
stc
push rbx
dec bl
xor [rsp], esi
pop rbx
stc
test spl, 0xB3
cmc
movsxd rsi, esi
cmp cl, spl
cmc
add r10, rsi
jmp -0xD3EA0
jmp r10
mov r9, [rdi]
adc esi, 0x7F3F31A2
dec sil
mov rsi, [rdi+0x08]
not r9
cmp r14b, 0xF9
cmc
not rsi
cmc
and r9, rsi
jmp +0x43CC5
mov [rdi+0x08], r9
not r9b
pushfq
and r9b, r11b
pop [rdi]
movzx r9d, cx
and r9b, 0x13
mov r9d, [r11]
test ch, 0x04
add r11, 0x04
xor r9d, ebx
jmp -0x319A
dec r9d
jmp -0x4148
ror r9d, 0x01
bswap r9d
dec r9d
cmp bpl, 0x79
test bx, r15w
stc
push rbx
rcl bx, cl
stc
btc ebx, r15d
xor [rsp], r9d
pop rbx
stc
movsxd r9, r9d
stc
cmp sil, cl
add r10, r9
jmp +0x6F44E
jmp r10
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
movzx r8d, byte ptr [r11]
dec esi
rol sil, 0x00
add r11, 0x01
shl sil, 0x43
xor r8b, bl
shl si, 0x87
neg r8b
movsxd rsi, esi
movsx rsi, cx
jmp -0x4AB67
ror r8b, 0x01
sbb si, si
cmovno si, bp
neg r8b
bts esi, ebp
sbb si, 0x6C8
not r8b
rcr esi, cl
xor bl, r8b
shl sil, cl
bsr si, sp
sbb sil, 0x46
mov rsi, [rsp+r8*1]
add r8b, 0xE8
shr r8, cl
sub rdi, 0x08
btr r8w, 0xCE
shl r8w, cl
mov [rdi], rsi
bsr r8w, bp
btr r8w, 0xA8
rol r8w, 0x41
mov r8d, [r11]
cmc
add r11, 0x04
cmc
cmp r11b, r15b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp +0x9C2FB
not r8d
jmp -0xBCBD
inc r8d
jmp -0x8468F
not r8d
inc r8d
clc
stc
push rbx
shrd rbx, rdx, 0x74
sbb ebx, ecx
mov rbx, 0x68DC6371
xor [rsp], r8d
movsx rbx, r14w
shr bh, 0x08
pop rbx
test di, 0x3163
movsxd r8, r8d
stc
add r10, r8
jmp +0x9C323
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
rcl dh, 0x5F
add r11, 0x08
xor r8, rbx
bts dx, r13w
movsx rdx, r11w
rol r8, 0x05
bswap r8
movsx dx, r13b
movzx edx, r14w
mov dl, dil
inc r8
stc
shrd rdx, r11, 0x19
xor r8, 0x18BC78EF
mov dl, 0x58
sub r8, 0x59F76389
bswap r8
xor dx, ax
rcr dh, 0x6A
bsr rdx, r11
xor rbx, r8
movzx rdx, r8w
rcl dl, 0x65
rcr dl, cl
sub rdi, 0x08
sbb dx, ax
movsxd rdx, eax
mov [rdi], r8
mov edx, [r11]
cmc
add r11, 0x04
test r12b, 0xA2
stc
xor edx, ebx
cmp spl, cl
xor edx, 0x668D6E25
add edx, 0x735D6DA4
cmc
cmp si, si
jmp -0x5F6F5
not edx
stc
neg edx
inc edx
rol edx, 0x03
push rbx
btc rbx, rax
sub ebx, 0x5F1B72F6
mov ebx, r9d
xor [rsp], edx
pop rbx
test r14b, r8b
cmc
cmp r11b, bpl
movsxd rdx, edx
add r10, rdx
jmp +0x590F7
jmp -0x24DB7
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
movzx esi, r9w
mov rcx, [rdi+0x08]
sub esi, r10d
not r9
bts si, r9w
btr esi, 0xC5
rol esi, 0xB3
not rcx
bts esi, r11d
bswap rsi
or r9, rcx
mov [rdi+0x08], r9
pushfq
btc si, 0x88
xor sil, r11b
pop [rdi]
and si, r10w
mov esi, [r11]
add r11, 0x04
xor esi, ebx
xor esi, 0x5C1A0567
not esi
cmp r8, r14
xor esi, 0x31A26B50
jmp -0x65434
bswap esi
jmp +0x64F18
dec esi
clc
cmc
push rbx
shl bh, cl
xor [rsp], esi
dec bl
movsxd rbx, esp
sar bx, 0x02
pop rbx
stc
movsxd rsi, esi
clc
add r10, rsi
jmp -0xB971
push r10
ret
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov rax, rdi
btr dx, 0x95
cmovns rdx, r15
shl dl, cl
sub rdi, 0x08
mov [rdi], rax
shr dl, 0x7E
mov edx, [r11]
stc
clc
add r11, 0x04
cmp r8b, 0x29
clc
stc
xor edx, ebx
jmp -0x69439
rol edx, 0x01
bswap edx
jmp -0x508E2
dec edx
stc
cmc
test r12, 0x11AF6184
bswap edx
push rbx
xor [rsp], edx
btc rbx, rsi
ror bh, 0x12
pop rbx
cmp r11d, r15d
clc
movsxd rdx, edx
cmp dl, r11b
test r13w, 0x214D
jmp -0x1B218
add r10, rdx
jmp +0x8E5B3
jmp +0xB00
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rdx, [rdi]
and ecx, r13d
movsx rsi, r11w
mov rcx, [rdx]
and esi, 0x74B62ED6
btc esi, 0x70
mov [rdi], rcx
cmp r8b, 0xEE
xadd si, si
mov esi, [r11]
clc
cmp bpl, 0xDF
add r11, 0x04
xor esi, ebx
stc
clc
cmp r10w, r12w
sub esi, 0x189679E7
neg esi
rol esi, 0x01
test sil, 0xDE
not esi
jmp -0x2B9F6
push rbx
xor [rsp], esi
rol rbx, 0x33
pop rbx
cmc
clc
movsxd rsi, esi
test r15w, 0x233
cmp r15b, al
add r10, rsi
jmp r10
mov r9, [rdi]
ror esi, 0xC8
mov rsi, [rdi+0x08]
test r15w, 0x6070
stc
not r9
jmp -0x57FD9
not rsi
stc
and r9, rsi
mov [rdi+0x08], r9
setp r9b
pushfq
sub r9b, r10b
rcl r9b, cl
shr r9b, cl
pop [rdi]
sar r9, 0x37
xor r9b, cl
movsx r9d, sp
mov r9d, [r11]
add r11, 0x04
cmp spl, r13b
cmc
xor r9d, ebx
jmp +0x2A2D
dec r9d
ror r9d, 0x01
jmp -0x648ED
bswap r9d
dec r9d
stc
test r15b, al
push rbx
xor [rsp], r9d
movzx rbx, sp
bt rbx, r10
pop rbx
test r15, 0x54921F15
stc
movsxd r9, r9d
stc
add r10, r9
jmp +0xC6E91
push r10
ret
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov r9, [rdi]
bts bp, bp
rcl bp, cl
movsx r8d, bp
mov rbp, [rdi+0x08]
or r8b, 0xB8
ror r8, 0x12
shrd r8w, ax, 0xF6
add r9, rbp
cmovnl r8w, r13w
not r8b
movsx r8, r9w
mov [rdi+0x08], r9
movsx r8d, dx
movzx r8, dx
pushfq
sub r8d, 0x44D249DC
pop [rdi]
movzx r8, r13w
xor r8, rcx
btr r8d, 0xAB
mov r8d, [r11]
jmp -0xB66F6
add r11, 0x04
clc
test si, sp
xor r8d, ebx
clc
xor r8d, 0x369B465A
cmc
ror r8d, 0x01
jmp +0x7BE9A
dec r8d
cmc
jmp +0x2E7FA
rol r8d, 0x01
jmp -0xDD6CA
inc r8d
clc
test r13b, r9b
push rbx
bts bx, r9w
add ebx, esp
xor [rsp], r8d
bt bx, r8w
test r13b, r10b
inc bh
pop rbx
test cl, 0x52
cmp spl, r9b
movsxd r8, r8d
cmp dl, 0x8D
add r10, r8
jmp +0x50E74
jmp r10
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
mov r8, [r11]
bts edx, r10d
xadd rdx, rdx
add rdx, rsp
add r11, 0x08
bswap rdx
xor r8, rbx
bswap edx
rol r8, 0x05
xchg dx, dx
bswap r8
movzx rdx, r12w
bswap dx
cmovnl dx, r9w
inc r8
ror edx, cl
btc edx, r14d
xor r8, 0x18BC78EF
xchg dx, dx
sub r8, 0x59F76389
btr dx, 0x24
mov dl, r8b
bswap r8
movsx edx, r8w
bt edx, r9d
movsxd rdx, eax
xor rbx, r8
shr dl, cl
rcl dx, cl
bt rdx, r15
sub rdi, 0x08
bsf rdx, rsi
mov [rdi], r8
shl dh, cl
mov edx, [r11]
cmp r14w, 0x6800
add r11, 0x04
cmc
jmp -0xD1889
xor edx, ebx
stc
cmc
test r14b, dl
xor edx, 0x668D6E25
clc
cmc
add edx, 0x735D6DA4
stc
not edx
neg edx
jmp +0x714ED
inc edx
rol edx, 0x03
cmp ax, 0x4DE1
push rbx
movsx rbx, r14w
xor [rsp], edx
cmc
bts bx, bx
btr rbx, r8
pop rbx
movsxd rdx, edx
test bpl, r14b
stc
add r10, rdx
jmp -0x653ED
jmp +0x840C2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
xor si, r14w
inc sil
add r11, 0x01
dec esi
sbb si, bp
bt esi, edx
xor r8b, bl
cmovnz si, bx
neg r8b
ror r8b, 0x01
btr esi, esi
mov sil, r9b
sar sil, cl
neg r8b
sar sil, 0x6A
not r8b
xor bl, r8b
xchg sil, sil
movzx si, bpl
mov rsi, [rsp+r8*1]
ror r8d, 0x86
sub rdi, 0x08
xadd r8d, r8d
mov [rdi], rsi
inc r8
bsf r8w, r13w
dec r8w
mov r8d, [r11]
cmp r13d, 0x3D0E02C5
cmp bpl, al
add r11, 0x04
test bpl, 0xFD
test dl, r14b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp -0x29EB9
not r8d
inc r8d
not r8d
jmp +0x83BDD
inc r8d
push rbx
cmovz bx, r12w
xor [rsp], r8d
sar bx, cl
xadd bl, bl
not ebx
pop rbx
movsxd r8, r8d
add r10, r8
jmp +0x11382
jmp -0x151BE
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
shl r8, cl
shl rbp, 0x78
mov rbp, [rdi+0x08]
sar r8b, 0x7F
add r9, rbp
movsx r8, r13w
mov [rdi+0x08], r9
movzx r8, r11w
pushfq
pop [rdi]
shl r8d, cl
mov r8d, [r11]
clc
stc
add r11, 0x04
cmc
xor r8d, ebx
cmp dh, 0xF5
xor r8d, 0x369B465A
jmp -0x2EDA7
ror r8d, 0x01
dec r8d
clc
rol r8d, 0x01
jmp +0x5AC2B
inc r8d
cmp dil, 0xC9
push rbx
adc ebx, 0x5049777A
shr bl, cl
xor [rsp], r8d
btr bx, 0xC9
pop rbx
movsxd r8, r8d
jmp -0x46FB9
add r10, r8
jmp +0x16726
jmp r10
mov rax, [rdi]
xadd r8w, r8w
add rdi, 0x08
bt si, ax
rcr r8b, cl
rol esi, 0xAE
movzx esi, byte ptr [r11]
sar r8w, 0x2F
xadd r8d, r8d
bt r8w, 0x9B
add r11, 0x01
btc r8d, eax
shrd r8w, r12w, 0x65
jmp -0xC7D1
xor sil, bl
rol r8b, 0xB9
movsx r8w, spl
ror sil, 0x01
rcl r8, 0xEF
cmovno r8w, r15w
xadd r8b, r8b
sub sil, 0xE7
xor sil, 0x86
add sil, 0xC4
clc
movzx r8d, si
mov r8w, r13w
rol sil, 0x01
rcl r8w, cl
bt r8w, dx
xchg r8b, r8b
sub sil, 0xD0
mov r8b, 0x5C
btr r8w, r10w
rol sil, 0x01
add sil, 0x43
cmovnl r8d, r9d
movzx r8w, r15b
rcl r8b, 0xC6
xor bl, sil
shr r8b, cl
mov [rsp+rsi*1], rax
add r8d, 0x4820409D
mov r8d, [r11]
add r11, 0x04
stc
cmp edi, r13d
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x26BA2
dec r8d
jmp +0x13D2D
ror r8d, 0x01
xor r8d, 0x29460FCE
cmc
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
bswap r8d
cmc
ror r8d, 0x01
push rbx
shr rbx, cl
test rsp, 0x17B40A2F
sub ebx, 0x4A482BF2
xor [rsp], r8d
pop rbx
stc
test r15b, 0x46
clc
movsxd r8, r8d
test al, 0xA2
clc
add r10, r8
jmp +0x74E8F
push r10
ret
mov rax, [rdi]
movzx r8w, r13b
add r8b, r13b
sbb sil, r9b
add rdi, 0x08
btc si, r11w
movzx esi, byte ptr [r11]
add r11, 0x01
sub r8d, r14d
rol r8b, 0x0A
xor sil, bl
movzx r8w, sil
ror r8b, 0x0E
ror sil, 0x01
bt r8w, 0x6E
bsf r8d, r13d
setns r8b
sub sil, 0xE7
bswap r8w
and r8b, sil
setb r8b
xor sil, 0x86
bt r8w, r14w
shr r8b, 0xD7
add sil, 0xC4
movzx r8d, cx
jmp +0x96A50
rol sil, 0x01
sub sil, 0xD0
movzx r8, r14w
setp r8b
btc r8w, bx
rol sil, 0x01
btr r8w, 0x58
adc r8, 0x77377B6E
add sil, 0x43
shr r8b, 0xB0
rcr r8, 0x66
xor bl, sil
or r8, 0x4AC334F5
rcl r8b, cl
mov [rsp+rsi*1], rax
shl r8b, cl
add r8w, 0x117A
mov r8b, bl
mov r8d, [r11]
cmp r8b, 0x62
add r11, 0x04
cmp r14b, al
stc
test sil, 0xD9
xor r8d, ebx
stc
rol r8d, 0x01
cmp spl, 0xC8
clc
xor r8d, 0x383D7910
dec r8d
jmp +0x8C7F
ror r8d, 0x01
xor r8d, 0x29460FCE
test bpl, 0x2D
clc
neg r8d
add r8d, 0x6BA37BA3
clc
cmc
bswap r8d
clc
ror r8d, 0x01
cmp dl, bpl
cmp r8, 0x129B6E27
push rbx
bt bx, 0x87
clc
sbb bl, r15b
xor [rsp], r8d
test r12w, bx
pop rbx
cmp r13b, 0xD2
test r14w, 0x51A6
cmp r13, rdi
movsxd r8, r8d
cmp r8b, spl
add r10, r8
push r10
ret
mov rax, rdi
sar dx, cl
sub rdi, 0x08
bt edx, r15d
add dh, 0xD8
mov [rdi], rax
cqo
xadd dx, dx
mov edx, [r11]
test r13b, 0x86
cmp r14b, cl
add r11, 0x04
xor edx, ebx
clc
rol edx, 0x01
jmp -0xB2848
bswap edx
jmp -0x6B8D
dec edx
cmp rbx, 0x78467BB0
bswap edx
cmp r12b, r9b
jmp +0x44F62
push rbx
xor [rsp], edx
pop rbx
cmc
movsxd rdx, edx
stc
add r10, rdx
jmp -0x2F2A
jmp +0x4A0F1
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r8, [r11]
movsx rdx, bx
add r11, 0x08
cmc
bt edx, 0xA9
xor r8, rbx
cmovle dx, di
rol r8, 0x05
cmovnz rdx, rdi
bswap r8
inc r8
xor r8, 0x18BC78EF
shl dx, cl
sub r8, 0x59F76389
mov dx, r11w
not dl
sub dh, 0xBE
bswap r8
rcl dh, cl
xchg dh, dl
xor rbx, r8
sub rdi, 0x08
shr dl, cl
cwd
ror dh, cl
mov [rdi], r8
jmp -0x4CD03
mov edx, [r11]
add r11, 0x04
clc
xor edx, ebx
xor edx, 0x668D6E25
cmp di, cx
add edx, 0x735D6DA4
not edx
neg edx
jmp +0x41114
inc edx
jmp -0x364B4
rol edx, 0x03
stc
cmp r15b, r14b
push rbx
xor [rsp], edx
bswap bx
shrd ebx, eax, 0xD5
mov ebx, r8d
pop rbx
cmp cl, 0x91
movsxd rdx, edx
test cx, 0x5EEA
cmp dil, cl
add r10, rdx
jmp +0x527E3
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
bts bp, sp
not ebp
mov rbp, [rdi+0x08]
inc r8b
movsx r8w, r15b
add r9, rbp
movsx r8d, r12w
movsxd r8, esp
mov [rdi+0x08], r9
mov r8b, r8b
bswap r8w
movsxd r8, r14d
pushfq
bt r8, 0x0B
rcl r8, 0x37
ror r8b, cl
pop [rdi]
bt r8w, 0x54
or r8b, r10b
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x5F4B5
xor r8d, ebx
test r9b, al
cmc
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x4F1E9
dec r8d
cmc
rol r8d, 0x01
jmp -0x9CABC
inc r8d
push rbx
stc
xadd ebx, ebx
sar rbx, 0x1A
xor [rsp], r8d
pop rbx
cmp r15d, edi
movsxd r8, r8d
test r8d, 0xC6E6F86
add r10, r8
jmp -0x1C633
push r10
ret
mov rax, [rdi]
test r12, rsi
rol r8b, 0x4F
add rdi, 0x08
sbb rsi, rsp
movzx esi, byte ptr [r11]
shrd r8w, r15w, 0x12
rol r8d, cl
sar r8b, 0xC6
add r11, 0x01
mov r8d, edi
xor sil, bl
ror sil, 0x01
sub sil, 0xE7
xor sil, 0x86
mov r8d, 0xD721CB3
shl r8b, cl
add sil, 0xC4
movsx r8w, dl
movsx r8d, r9w
rol sil, 0x01
ror r8b, cl
sub sil, 0xD0
cmc
rol sil, 0x01
add sil, 0x43
bsr r8w, di
xor bl, sil
cmc
mov [rsp+rsi*1], rax
rol r8d, cl
mov r8d, [r11]
add r11, 0x04
test spl, 0x04
stc
xor r8d, ebx
jmp +0xAE465
rol r8d, 0x01
cmp r11b, 0x0A
xor r8d, 0x383D7910
jmp -0xD0A8
dec r8d
stc
ror r8d, 0x01
test rdi, rcx
jmp -0x50B45
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
stc
cmc
jmp -0x2F503
bswap r8d
jmp +0x25778
ror r8d, 0x01
push rbx
sub ebx, r13d
xor [rsp], r8d
shl bx, 0x79
cmp bl, r11b
pop rbx
movsxd r8, r8d
clc
cmc
add r10, r8
jmp +0x2863F
jmp r10
mov rax, [rdi]
add rdi, 0x08
add sil, 0xFA
movzx esi, byte ptr [r11]
add r11, 0x01
sar r8, cl
dec r8d
xchg r8, r8
xor sil, bl
btr r8, r15
movsxd r8, ebx
ror sil, 0x01
sub sil, 0xE7
jmp -0x32D2F
xor sil, 0x86
add sil, 0xC4
rcl r8w, cl
rol sil, 0x01
sub sil, 0xD0
stc
rol r8b, 0xA5
movsx r8w, dil
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
sbb r8w, r11w
mov r8d, [r11]
cmc
cmp rsi, r15
add r11, 0x04
jmp +0xA5FE
xor r8d, ebx
rol r8d, 0x01
test r11w, 0x13FB
jmp -0x89669
xor r8d, 0x383D7910
jmp -0xAAF2
dec r8d
clc
stc
ror r8d, 0x01
test ch, 0x69
jmp +0x38C54
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
stc
clc
bswap r8d
clc
ror r8d, 0x01
test r12b, dl
cmc
jmp -0x20F28
push rbx
shl bx, cl
xor [rsp], r8d
cmp r9w, 0x2D3
pop rbx
cmc
movsxd r8, r8d
add r10, r8
jmp +0x3C7E2
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
rcl esi, 0xA1
sub si, 0x5FD2
xor r8b, bl
cmovb esi, r11d
neg r8b
rcr si, 0x24
btc si, di
movzx esi, dx
ror r8b, 0x01
btr si, sp
movsx si, dil
neg r8b
movsx esi, r9w
test r11b, 0xD1
bts si, sp
not r8b
xor si, r14w
xor bl, r8b
setnb sil
movsx rsi, bp
movsx esi, dx
mov rsi, [rsp+r8*1]
sub rdi, 0x08
bsr r8w, r8w
or r8b, 0x94
and r8w, sp
mov [rdi], rsi
test r15b, r8b
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
cmc
rol r8d, 0x02
cmp sp, 0x4775
neg r8d
jmp -0x7484E
not r8d
jmp +0x5719A
inc r8d
not r8d
jmp +0x34C63
inc r8d
cmp bpl, cl
jmp -0x2BC9B
push rbx
shl ebx, cl
xor [rsp], r8d
bts bx, di
pop rbx
movsxd r8, r8d
test r15w, 0x38F
jmp +0x36289
add r10, r8
jmp -0x81C7F
jmp +0x51A8B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rax, [rdi]
btc si, r8w
add rdi, 0x08
inc sil
cmp r8b, r12b
movzx esi, byte ptr [r11]
shl r8w, cl
rol r8w, 0x1B
add r11, 0x01
rcl r8w, 0x81
xor sil, bl
ror sil, 0x01
movsx r8w, r12b
adc r8w, 0x4C1D
sub sil, 0xE7
bt r8d, 0x65
xor sil, 0x86
bsf r8w, bp
add sil, 0xC4
rol sil, 0x01
movzx r8d, r8w
setp r8b
sub sil, 0xD0
movsx r8, sp
rcr r8b, 0xEE
rol sil, 0x01
sar r8b, cl
add sil, 0x43
xor bl, sil
shld r8, r10, 0x63
ror r8b, 0x94
mov [rsp+rsi*1], rax
xadd r8b, r8b
mov r8d, [r11]
stc
clc
test bl, 0xCD
add r11, 0x04
cmp si, dx
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
dec r8d
clc
jmp +0x8D1F
ror r8d, 0x01
jmp +0xB283
xor r8d, 0x29460FCE
test sp, 0x4D02
test r8b, sil
cmp r14b, 0xAD
neg r8d
jmp -0x17555
add r8d, 0x6BA37BA3
stc
bswap r8d
ror r8d, 0x01
jmp -0x22CA2
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
add r10, r8
jmp -0x3AF7C
push r10
ret
mov r8, [r11]
add r11, 0x08
shl edx, cl
xor r8, rbx
btr edx, 0xDB
mov edx, 0x52AE0860
movsx dx, spl
rol r8, 0x05
cwd
cdq
movzx rdx, r13w
bswap r8
inc r8
or dh, 0xEF
jmp +0x2A403
xor r8, 0x18BC78EF
bt dx, sp
sub r8, 0x59F76389
bswap r8
add rdx, rbp
xor rbx, r8
shl dl, 0xCB
sub rdi, 0x08
movzx dx, r15b
rcr dh, cl
not edx
mov [rdi], r8
shr dh, cl
cmc
rcr dl, cl
mov edx, [r11]
jmp -0x189B8
add r11, 0x04
test r12b, bpl
cmc
xor edx, ebx
clc
test r15d, r11d
xor edx, 0x668D6E25
add edx, 0x735D6DA4
test dil, bpl
not edx
neg edx
inc edx
jmp +0x8495E
rol edx, 0x03
test r11b, 0x68
push rbx
xor [rsp], edx
rcl ebx, cl
movzx rbx, cx
cmc
pop rbx
movsxd rdx, edx
add r10, rdx
jmp -0x96BD6
jmp +0x984C5
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
shl sil, 0x28
add esi, edi
cmovl rsi, rdi
add r11, 0x01
xor r8b, bl
xchg sil, sil
shl sil, cl
neg r8b
ror r8b, 0x01
neg r8b
adc sil, 0x83
shl sil, cl
bswap rsi
not r8b
dec esi
mov sil, 0x8E
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, cl
sub rdi, 0x08
sar r8b, 0xD4
mov [rdi], rsi
btc r8w, r11w
mov r8d, [r11]
clc
test esi, 0x5FFD2DB2
cmc
add r11, 0x04
test bpl, r11b
xor r8d, ebx
clc
stc
rol r8d, 0x02
stc
cmp r13b, r15b
neg r8d
not r8d
inc r8d
jmp -0x265B0
not r8d
jmp +0x61FFD
inc r8d
push rbx
xor [rsp], r8d
neg ebx
pop rbx
test r11b, 0x60
clc
cmp edi, 0x7F310A36
movsxd r8, r8d
test r15b, 0xBB
add r10, r8
jmp -0x1DE9A
jmp +0x3270B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
jmp -0x3BF7
add r9, rbp
movzx r8, r12w
jmp -0xC78A0
mov [rdi+0x08], r9
mov r8w, ax
movzx r8d, r8w
pushfq
pop [rdi]
mov r8d, [r11]
add r11, 0x04
test bp, 0x44
jmp +0xC373A
xor r8d, ebx
cmp spl, 0x16
xor r8d, 0x369B465A
clc
jmp -0x491B2
ror r8d, 0x01
jmp -0x7E809
dec r8d
rol r8d, 0x01
jmp +0xAC971
inc r8d
test ah, 0xE7
push rbx
xor [rsp], r8d
sbb rbx, 0x60AD13E6
jmp +0x42A91
pop rbx
clc
movsxd r8, r8d
cmp r10b, r11b
add r10, r8
jmp r10
mov rax, [rdi]
add rdi, 0x08
movzx esi, byte ptr [r11]
or r8, 0x9127DD1
add r11, 0x01
cmovno r8, r13
xor sil, bl
btc r8w, ax
ror sil, 0x01
sub sil, 0xE7
movzx r8d, cx
xchg r8w, r8w
inc r8w
xor sil, 0x86
add sil, 0xC4
btc r8d, 0xD5
rol sil, 0x01
shr r8b, 0xEB
movzx r8, ax
mov r8b, 0x3D
sub sil, 0xD0
clc
not r8
rol sil, 0x01
sub r8b, 0xFD
movsxd r8, r10d
test sil, r14b
add sil, 0x43
shl r8w, cl
xor bl, sil
mov [rsp+rsi*1], rax
mov r8d, [r11]
cmp rsi, 0x6FCC7AF5
add r11, 0x04
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0x51873
dec r8d
cmc
clc
ror r8d, 0x01
xor r8d, 0x29460FCE
test r8b, bl
cmp r11b, spl
neg r8d
test rbx, rdi
cmp r8w, r12w
cmc
add r8d, 0x6BA37BA3
stc
bswap r8d
cmc
ror r8d, 0x01
push rbx
movzx rbx, bx
xor [rsp], r8d
pop rbx
test cl, 0x38
jmp -0x48424
movsxd r8, r8d
add r10, r8
push r10
ret
mov rax, [rdi]
stc
add rdi, 0x08
cmp bpl, 0x59
shl r8w, cl
movzx esi, byte ptr [r11]
btc r8d, r9d
movzx r8w, dil
add r11, 0x01
movsxd r8, r11d
sbb r8w, 0x2918
xor sil, bl
bswap r8d
mov r8w, r15w
rcr r8d, cl
ror sil, 0x01
shrd r8d, r10d, 0xE3
rcr r8, 0x6D
sub sil, 0xE7
bts r8w, ax
stc
or r8b, spl
xor sil, 0x86
add sil, 0xC4
rol sil, 0x01
movsx r8d, r13w
sub sil, 0xD0
rol r8b, cl
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
clc
mov r8d, [r11]
add r11, 0x04
jmp -0x7863
xor r8d, ebx
rol r8d, 0x01
cmp r9d, 0x15160928
xor r8d, 0x383D7910
jmp -0x3DACC
dec r8d
jmp -0x3B73B
ror r8d, 0x01
xor r8d, 0x29460FCE
cmp r14w, 0x52FA
neg r8d
add r8d, 0x6BA37BA3
cmc
bswap r8d
jmp +0xF3154
ror r8d, 0x01
stc
push rbx
stc
or bl, 0x32
bt bx, 0xED
xor [rsp], r8d
not rbx
pop rbx
test rbx, 0x75995E7F
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0xCE857
jmp r10
mov r8, [r11]
add r11, 0x08
cmovp rdx, r14
xor r8, rbx
rcr dl, cl
rol r8, 0x05
movsx edx, ax
movsxd rdx, esp
jmp +0x2C821
bswap r8
inc r8
bsf rdx, r8
clc
or dx, di
xor r8, 0x18BC78EF
btr dx, 0x90
stc
sub r8, 0x59F76389
dec dh
inc dh
neg dl
bswap r8
btr edx, r11d
shld rdx, rcx, 0x72
xor rbx, r8
bsf dx, r8w
or dl, 0x2C
movsx edx, cx
sub rdi, 0x08
rcr dl, cl
adc dx, 0x300D
bt rdx, r9
mov [rdi], r8
mov edx, [r11]
add r11, 0x04
jmp -0x7D1C0
xor edx, ebx
xor edx, 0x668D6E25
cmp sp, cx
jmp +0x15354
add edx, 0x735D6DA4
stc
test r11w, 0x3217
not edx
test r8d, 0x1ABD7B64
neg edx
jmp +0x423AD
inc edx
clc
cmc
rol edx, 0x03
stc
test r9b, 0x5A
cmp r11, r13
push rbx
clc
xor [rsp], edx
shr ebx, cl
add bl, r12b
test r9d, 0x1C7425BE
pop rbx
movsxd rdx, edx
stc
cmp rsi, 0x6C441FC5
test r10d, 0x1E1A0474
add r10, rdx
jmp +0x28F5B
jmp -0x410A0
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
mov rsi, rsp
not si
add r11, 0x01
neg esi
shl si, cl
xor r8b, bl
neg r8b
movzx esi, bp
bt si, di
ror r8b, 0x01
sub rsi, 0x3113452A
mov si, 0x7D49
neg r8b
rcl rsi, 0xB0
cmp r10b, bl
not r8b
movsx si, cl
cmp r9, 0x9BB5EDF
bt esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
bts r8d, r14d
not r8b
adc r8d, r9d
sub rdi, 0x08
mov [rdi], rsi
mov r8d, [r11]
add r11, 0x04
cmp r12b, 0xCF
xor r8d, ebx
clc
cmc
rol r8d, 0x02
cmc
cmp cx, 0x2552
test r12, 0x251600AE
neg r8d
jmp -0x73801
not r8d
inc r8d
jmp +0x628A2
not r8d
inc r8d
push rbx
shr bl, 0xD3
xor [rsp], r8d
rcl bh, 0xAA
test r13d, edx
pop rbx
cmc
test r11w, 0x7844
movsxd r8, r8d
add r10, r8
jmp -0x27AD2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
stc
movsx rbp, si
ror bpl, cl
mov rbp, [rdi+0x08]
sar r8b, 0xFF
add r9, rbp
movzx r8w, al
mov [rdi+0x08], r9
cmovle r8, r13
pushfq
pop [rdi]
mov r8d, [r11]
cmp r14b, 0xBE
test r15b, bpl
add r11, 0x04
xor r8d, ebx
xor r8d, 0x369B465A
ror r8d, 0x01
jmp +0x3A5C8
dec r8d
clc
rol r8d, 0x01
inc r8d
cmp dx, 0x506B
clc
push rbx
xor [rsp], r8d
sbb bl, 0xF6
xadd bx, bx
movzx rbx, cx
pop rbx
cmp r12w, 0x1F70
movsxd r8, r8d
stc
test bpl, r10b
add r10, r8
jmp -0x4C463
jmp r10
mov rax, [rdi]
neg r8w
movzx si, r10b
xchg sil, sil
add rdi, 0x08
dec r8b
bsr r8w, cx
sar r8w, 0x24
movzx esi, byte ptr [r11]
cmc
add r11, 0x01
inc r8b
movsx r8, r10w
bsf r8d, esi
xor sil, bl
movzx r8d, ax
sets r8b
bts r8, rbx
ror sil, 0x01
adc r8b, r14b
btr r8, 0x6F
shl r8b, 0x7E
sub sil, 0xE7
ror r8, 0xDD
adc r8b, 0x13
xor sil, 0x86
add sil, 0xC4
ror r8d, 0xA5
bswap r8w
movzx r8d, r13w
rol sil, 0x01
btc r8d, 0x77
sub sil, 0xD0
rol sil, 0x01
ror r8, cl
add sil, 0x43
bts r8, r10
xor bl, sil
bts r8, rbp
btr r8, 0xE8
movsx r8, sp
mov [rsp+rsi*1], rax
ror r8b, cl
rcr r8b, cl
movzx r8d, r14w
mov r8d, [r11]
test di, si
cmc
cmp rsi, 0x70413AC9
add r11, 0x04
clc
xor r8d, ebx
stc
clc
rol r8d, 0x01
jmp -0xC1169
xor r8d, 0x383D7910
dec r8d
ror r8d, 0x01
cmp r10, r14
xor r8d, 0x29460FCE
neg r8d
stc
add r8d, 0x6BA37BA3
clc
stc
bswap r8d
stc
ror r8d, 0x01
jmp +0x14B8
push rbx
inc bl
shl bl, cl
shrd ebx, ebx, 0x46
xor [rsp], r8d
rol bh, 0x74
shld rbx, rdx, 0xA0
pop rbx
cmp r9, rbx
jmp +0x194D2
movsxd r8, r8d
add r10, r8
jmp -0x33587
jmp r10
mov rax, [rdi]
movsx rsi, di
shr sil, cl
bt r8d, 0x76
add rdi, 0x08
sar r8b, cl
movsx r8w, r13b
or rsi, 0x323513FB
movzx esi, byte ptr [r11]
xor r8d, esi
add r11, 0x01
add r8b, al
shl r8b, 0x2B
xor sil, bl
bt r8d, 0x8C
ror sil, 0x01
mov r8w, 0x4FB8
sub sil, 0xE7
movzx r8d, ax
bt r8w, r8w
inc r8w
xor sil, 0x86
shld r8, rdx, 0xC8
add sil, 0xC4
setb r8b
rol r8b, 0x98
rol sil, 0x01
stc
jmp -0x1012E
sub sil, 0xD0
ror r8b, 0x72
clc
rol sil, 0x01
add sil, 0x43
not r8d
xor bl, sil
setl r8b
mov r8, rsp
mov [rsp+rsi*1], rax
sar r8, 0x89
dec r8b
jmp +0x106DA
mov r8d, [r11]
cmp r8b, spl
add r11, 0x04
test r12w, 0x638F
xor r8d, ebx
rol r8d, 0x01
cmp r15b, 0x35
test al, sil
xor r8d, 0x383D7910
jmp -0x3C2FC
dec r8d
clc
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
cmc
add r8d, 0x6BA37BA3
cmc
clc
bswap r8d
cmc
clc
ror r8d, 0x01
push rbx
btr bx, 0xBE
movsxd rbx, r11d
xor [rsp], r8d
ror bl, 0x43
pop rbx
cmp r11b, bl
jmp +0x4BF2A
movsxd r8, r8d
stc
clc
add r10, r8
jmp -0x3332C
jmp r10
mov r8, [r11]
rcr dh, 0xEA
add r11, 0x08
inc dl
test dx, 0x2EB1
and dl, dh
xor r8, rbx
movsx edx, r15w
movzx edx, dx
cmc
rol r8, 0x05
movzx edx, r13w
movsx dx, sil
bswap r8
jmp +0xD49A6
inc r8
movzx rdx, r15w
stc
xor r8, 0x18BC78EF
sub r8, 0x59F76389
cmc
neg dl
ror dh, cl
bswap r8
xor rdx, r15
xadd dl, dh
xor rbx, r8
bts edx, edi
inc rdx
bt edx, 0x56
sub rdi, 0x08
shld dx, cx, 0x13
sar edx, 0x4F
mov [rdi], r8
mov edx, [r11]
clc
test r14w, di
cmp r9w, 0x330
add r11, 0x04
cmp r11, r15
jmp -0xB6B3
xor edx, ebx
cmc
xor edx, 0x668D6E25
clc
add edx, 0x735D6DA4
not edx
cmp r14b, 0xEC
neg edx
jmp -0xD5CB7
inc edx
rol edx, 0x03
cmc
test dl, r8b
push rbx
rol ebx, cl
sar bl, cl
add bx, 0x491C
xor [rsp], edx
and bx, 0x575B
shr bx, 0x59
pop rbx
movsxd rdx, edx
add r10, rdx
jmp +0x92D83
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
or sil, cl
btc si, di
test r15d, r12d
add r11, 0x01
xor r8b, bl
cmovnle si, r9w
neg r8b
btr si, si
btc rsi, 0x20
ror r8b, 0x01
rcl sil, 0xFE
sbb rsi, 0x103B38AE
neg r8b
not r8b
sbb si, 0x916
or esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
mov [rdi], rsi
shl r8b, cl
mov r8d, [r11]
jmp +0x901E8
add r11, 0x04
xor r8d, ebx
jmp +0x4B3DC
rol r8d, 0x02
neg r8d
not r8d
jmp +0x1AF10
inc r8d
jmp -0x8521D
not r8d
inc r8d
stc
cmp di, 0x5EB4
push rbx
cmp r13b, sil
xor [rsp], r8d
xchg bx, bx
sub ebx, 0x3C2A06D8
pop rbx
movsxd r8, r8d
cmc
jmp +0x4D760
add r10, r8
jmp -0x3FF9D
jmp +0x28D9F
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov rbp, [rdi+0x08]
add r9, rbp
movsxd r8, ebp
mov [rdi+0x08], r9
movsx r8w, al
jmp -0x6045
pushfq
rcr r8b, cl
pop [rdi]
jmp -0x90DCA
mov r8d, [r11]
add r11, 0x04
cmp r11d, edx
test bpl, 0x48
xor r8d, ebx
stc
xor r8d, 0x369B465A
stc
jmp -0xBC9
ror r8d, 0x01
jmp +0x26FC9
dec r8d
jmp +0x8C74
rol r8d, 0x01
jmp +0x7369
inc r8d
test cx, 0x1F08
push rbx
xor [rsp], r8d
adc ebx, r8d
sbb rbx, rsp
pop rbx
movsxd r8, r8d
cmc
stc
add r10, r8
jmp +0x3F180
push r10
ret
mov rax, [rdi]
bts r8w, r10w
sar sil, 0x9F
add rdi, 0x08
movzx esi, byte ptr [r11]
bt r8w, cx
add r11, 0x01
sbb r8d, edi
xor sil, bl
bts r8, rcx
ror sil, 0x01
sub sil, 0xE7
shl r8b, cl
and r8w, r15w
shl r8w, 0xB8
xor sil, 0x86
sar r8b, 0xC1
movzx r8, r9w
shl r8, cl
add sil, 0xC4
rol sil, 0x01
sub sil, 0xD0
rol sil, 0x01
add sil, 0x43
xor bl, sil
mov [rsp+rsi*1], rax
stc
or r8d, r10d
xor r8b, spl
mov r8d, [r11]
test r13w, 0x6D85
clc
add r11, 0x04
test r10b, 0x1A
clc
stc
xor r8d, ebx
cmc
stc
rol r8d, 0x01
stc
jmp -0x7C4CF
xor r8d, 0x383D7910
jmp +0x885DB
dec r8d
jmp -0xD1437
ror r8d, 0x01
stc
xor r8d, 0x29460FCE
neg r8d
test bpl, 0xEB
add r8d, 0x6BA37BA3
jmp +0x847F8
bswap r8d
jmp +0x40E5
ror r8d, 0x01
test r12b, 0x86
push rbx
jmp +0x411D6
xor [rsp], r8d
pop rbx
stc
cmp r13w, bp
cmp r13b, 0x42
movsxd r8, r8d
cmp rcx, 0x7AA1929
add r10, r8
jmp -0x5DF9
jmp r10
mov r8, [r11]
cmovnl dx, r8w
or edx, 0x4FE964E3
add r11, 0x08
cwd
xor r8, rbx
bts edx, r9d
ror dx, 0xEB
cqo
rol r8, 0x05
xchg dx, dx
cdq
bswap r8
bswap rdx
inc r8
shr edx, 0x22
sub dx, r10w
xor r8, 0x18BC78EF
shld edx, r8d, 0xDE
rcr dh, cl
rol dx, 0xA0
sub r8, 0x59F76389
bsf dx, r10w
mov rdx, 0x6ABE4CC4
bswap r8
shl dh, 0x90
xor dl, ch
adc edx, 0x58CF5593
xor rbx, r8
sub rdx, r12
cmp r8b, 0xE2
sub rdi, 0x08
shr rdx, 0x32
sub dl, r12b
xor dh, 0x71
mov [rdi], r8
adc dl, r11b
mov edx, [r11]
stc
cmc
add r11, 0x04
test rbx, 0xF9117E9
cmp r8b, r14b
xor edx, ebx
xor edx, 0x668D6E25
add edx, 0x735D6DA4
not edx
neg edx
inc edx
jmp +0x25365
rol edx, 0x03
clc
push rbx
shl bh, 0x1D
xor [rsp], edx
pop rbx
movsxd rdx, edx
add r10, rdx
jmp -0x879AD
jmp +0x41876
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
dec sil
bsr si, r13w
bt si, 0x56
xor r8b, bl
adc sil, spl
bts esi, r8d
inc sil
neg r8b
movsx si, r15b
ror r8b, 0x01
not rsi
bts rsi, r12
neg r8b
mov sil, r10b
clc
not r8b
add sil, sil
rol sil, 0x0D
xor bl, r8b
add esi, r11d
mov rsi, [rsp+r8*1]
adc r8w, dx
sub rdi, 0x08
bsr r8w, r12w
sbb r8w, 0x7A1F
bt r8w, 0xE1
mov [rdi], rsi
sub r8b, 0xF3
mov r8w, 0x94E
shld r8w, bp, 0xA8
mov r8d, [r11]
add r11, 0x04
jmp -0x493BE
xor r8d, ebx
clc
rol r8d, 0x02
cmp dil, bl
test r12w, r15w
test r10b, 0x8F
neg r8d
jmp +0xA7179
not r8d
jmp -0x574BD
inc r8d
jmp -0x58ECD
not r8d
jmp +0x35663
inc r8d
jmp +0x3D34C
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
clc
add r10, r8
jmp -0x3C22C
jmp +0x56F6E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov r9, [rdi]
mov bpl, 0xA1
adc r8w, 0x4B16
mov rbp, [rdi+0x08]
movzx r8w, bpl
add r9, rbp
mov [rdi+0x08], r9
not r8b
movsxd r8, ecx
pushfq
shrd r8w, r15w, 0xB6
pop [rdi]
shrd r8w, r11w, 0x09
mov r8d, [r11]
cmp esp, 0x40B35B13
add r11, 0x04
cmp r13, r10
stc
xor r8d, ebx
cmp r15b, sil
xor r8d, 0x369B465A
stc
ror r8d, 0x01
jmp +0x285F6
dec r8d
clc
jmp +0x60D2B
rol r8d, 0x01
inc r8d
push rbx
shld bx, r11w, 0xF8
shl bx, cl
add bl, 0xB2
xor [rsp], r8d
bts bx, ax
and rbx, 0x4D980F97
ror bl, 0x36
pop rbx
clc
movsxd r8, r8d
stc
add r10, r8
jmp -0x803CD
jmp r10
mov rax, [rdi]
rol r8b, cl
add rdi, 0x08
btc si, 0x6C
cmp ecx, ebp
rcr r8b, 0x86
movzx esi, byte ptr [r11]
sub r8b, 0x4D
stc
shl r8b, 0xD3
add r11, 0x01
shl r8w, cl
rcr r8d, 0xF9
xor sil, bl
movzx r8w, r10b
bt r8, 0x9B
ror sil, 0x01
sub sil, 0xE7
rcl r8b, 0x82
xor sil, 0x86
add sil, 0xC4
movzx r8d, r10w
rol sil, 0x01
shl r8b, 0xAE
test bx, r11w
movsx r8d, bp
sub sil, 0xD0
bts r8d, edx
btr r8d, r15d
ror r8b, cl
rol sil, 0x01
movsx r8, r11w
add sil, 0x43
add r8b, 0x01
xor bl, sil
mov [rsp+rsi*1], rax
mov r8b, dl
bsf r8d, esi
mov r8d, [r11]
clc
cmc
add r11, 0x04
cmc
cmp r11w, r11w
test bpl, r11b
xor r8d, ebx
rol r8d, 0x01
xor r8d, 0x383D7910
jmp -0xC17C6
dec r8d
cmc
ror r8d, 0x01
test r12b, 0xEE
cmp r15d, 0x61253B27
xor r8d, 0x29460FCE
neg r8d
add r8d, 0x6BA37BA3
bswap r8d
ror r8d, 0x01
push rbx
xor [rsp], r8d
bt rbx, 0xB5
pop rbx
movsxd r8, r8d
cmc
clc
test r11, 0x6D00701E
add r10, r8
jmp +0xB42AD
jmp r10
movzx r8d, byte ptr [r11]
bts rsi, r14
inc sil
add r11, 0x01
movsx rsi, si
ror sil, 0xBC
xor r8b, bl
movsx esi, di
mov si, 0x325C
setnp sil
neg r8b
movzx rsi, dx
rcr si, cl
ror sil, cl
ror r8b, 0x01
rcr rsi, 0xC7
neg r8b
shl sil, 0xA6
sbb si, r13w
not r8b
adc sil, r10b
rcl sil, 0x44
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
jmp +0xB37E1
mov [rdi], rsi
xadd r8, r8
cmp r12b, 0xA1
mov r8d, [r11]
cmp r9b, 0x9F
cmc
jmp +0x32520
add r11, 0x04
stc
jmp -0xCD81E
xor r8d, ebx
stc
rol r8d, 0x02
cmp r9d, 0x43C35321
neg r8d
not r8d
inc r8d
jmp +0x64EC6
not r8d
jmp -0x4A752
inc r8d
test bx, r12w
push rbx
setp bh
xor [rsp], r8d
movsxd rbx, r14d
pop rbx
cmp r13w, 0x1267
movsxd r8, r8d
add r10, r8
jmp +0x2FB55
jmp +0x39243
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
stc
test rcx, 0x34EA4385
sbb sil, r15b
add r11, 0x01
stc
xor r8b, bl
cmc
neg r8b
ror si, cl
bswap rsi
movzx esi, r12w
ror r8b, 0x01
btr esi, ebx
bsr rsi, rdi
shr sil, cl
neg r8b
shr rsi, cl
btr esi, r8d
btc si, r14w
not r8b
cmp r10, 0x532F4095
xor bl, r8b
mov sil, 0xB3
ror si, 0xAD
mov rsi, [rsp+r8*1]
shl r8b, cl
shld r8w, bx, 0xC2
sub rdi, 0x08
shl r8b, cl
btc r8, r15
test r13w, sp
mov [rdi], rsi
bts r8w, bp
movzx r8d, r12w
cmc
mov r8d, [r11]
test dil, 0x7F
add r11, 0x04
xor r8d, ebx
rol r8d, 0x02
stc
neg r8d
jmp +0xC8598
not r8d
jmp +0x16694
inc r8d
jmp -0x8F873
not r8d
jmp -0x4A7D8
inc r8d
stc
push rbx
sbb ebx, 0x27102D1A
setp bl
xor [rsp], r8d
bts rbx, rbx
pop rbx
cmc
test dil, 0x4E
cmp bpl, r10b
movsxd r8, r8d
clc
add r10, r8
jmp +0x86AB6
jmp +0x277E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
jmp -0x3C210
xor r8b, bl
cmp eax, 0x6D434734
neg r8b
rcr sil, 0xE8
ror r8b, 0x01
neg r8b
not r8b
bts esi, r9d
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, 0xA1
bts r8w, cx
sub rdi, 0x08
btc r8d, ebp
mov [rdi], rsi
mov r8d, [r11]
test r10b, 0x36
jmp +0x67B53
add r11, 0x04
jmp +0x6362
xor r8d, ebx
clc
stc
rol r8d, 0x02
jmp -0x3DE4E
neg r8d
not r8d
inc r8d
not r8d
inc r8d
push rbx
xor [rsp], r8d
dec bl
pop rbx
clc
test bl, 0xB5
movsxd r8, r8d
stc
add r10, r8
jmp -0x50256
jmp +0x4A1DA
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
cmovz rsi, rsp
add r11, 0x01
rol sil, cl
rcr esi, 0x4E
shld si, di, 0xDB
xor r8b, bl
shld si, r8w, 0xD8
movzx si, bl
neg r8b
jmp -0x44832
ror r8b, 0x01
sbb sil, 0xCB
or si, r15w
ror rsi, cl
neg r8b
not r8b
bts si, r13w
cmp r9b, 0xF4
bsr esi, r8d
xor bl, r8b
movsxd rsi, esi
xchg si, si
rol sil, cl
mov rsi, [rsp+r8*1]
sub rdi, 0x08
movzx r8d, sp
mov [rdi], rsi
shl r8, 0x13
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
rol r8d, 0x02
clc
cmp sil, cl
stc
neg r8d
not r8d
inc r8d
not r8d
jmp +0x4B618
inc r8d
push rbx
xor [rsp], r8d
sub bh, 0x95
pop rbx
cmp r13b, 0xD0
movsxd r8, r8d
jmp -0xA6128
add r10, r8
jmp +0x5AB78
jmp +0x3F95
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
dec esi
rol sil, 0x00
add r11, 0x01
shl sil, 0x43
xor r8b, bl
shl si, 0x87
neg r8b
movsxd rsi, esi
movsx rsi, cx
jmp -0x4AB67
ror r8b, 0x01
sbb si, si
cmovno si, bp
neg r8b
bts esi, ebp
sbb si, 0x6C8
not r8b
rcr esi, cl
xor bl, r8b
shl sil, cl
bsr si, sp
sbb sil, 0x46
mov rsi, [rsp+r8*1]
add r8b, 0xE8
shr r8, cl
sub rdi, 0x08
btr r8w, 0xCE
shl r8w, cl
mov [rdi], rsi
bsr r8w, bp
btr r8w, 0xA8
rol r8w, 0x41
mov r8d, [r11]
cmc
add r11, 0x04
cmc
cmp r11b, r15b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp +0x9C2FB
not r8d
jmp -0xBCBD
inc r8d
jmp -0x8468F
not r8d
inc r8d
clc
stc
push rbx
shrd rbx, rdx, 0x74
sbb ebx, ecx
mov rbx, 0x68DC6371
xor [rsp], r8d
movsx rbx, r14w
shr bh, 0x08
pop rbx
test di, 0x3163
movsxd r8, r8d
stc
add r10, r8
jmp +0x9C323
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
xor si, r14w
inc sil
add r11, 0x01
dec esi
sbb si, bp
bt esi, edx
xor r8b, bl
cmovnz si, bx
neg r8b
ror r8b, 0x01
btr esi, esi
mov sil, r9b
sar sil, cl
neg r8b
sar sil, 0x6A
not r8b
xor bl, r8b
xchg sil, sil
movzx si, bpl
mov rsi, [rsp+r8*1]
ror r8d, 0x86
sub rdi, 0x08
xadd r8d, r8d
mov [rdi], rsi
inc r8
bsf r8w, r13w
dec r8w
mov r8d, [r11]
cmp r13d, 0x3D0E02C5
cmp bpl, al
add r11, 0x04
test bpl, 0xFD
test dl, r14b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp -0x29EB9
not r8d
inc r8d
not r8d
jmp +0x83BDD
inc r8d
push rbx
cmovz bx, r12w
xor [rsp], r8d
sar bx, cl
xadd bl, bl
not ebx
pop rbx
movsxd r8, r8d
add r10, r8
jmp +0x11382
jmp -0x151BE
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
rcl esi, 0xA1
sub si, 0x5FD2
xor r8b, bl
cmovb esi, r11d
neg r8b
rcr si, 0x24
btc si, di
movzx esi, dx
ror r8b, 0x01
btr si, sp
movsx si, dil
neg r8b
movsx esi, r9w
test r11b, 0xD1
bts si, sp
not r8b
xor si, r14w
xor bl, r8b
setnb sil
movsx rsi, bp
movsx esi, dx
mov rsi, [rsp+r8*1]
sub rdi, 0x08
bsr r8w, r8w
or r8b, 0x94
and r8w, sp
mov [rdi], rsi
test r15b, r8b
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
cmc
rol r8d, 0x02
cmp sp, 0x4775
neg r8d
jmp -0x7484E
not r8d
jmp +0x5719A
inc r8d
not r8d
jmp +0x34C63
inc r8d
cmp bpl, cl
jmp -0x2BC9B
push rbx
shl ebx, cl
xor [rsp], r8d
bts bx, di
pop rbx
movsxd r8, r8d
test r15w, 0x38F
jmp +0x36289
add r10, r8
jmp -0x81C7F
jmp +0x51A8B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
shl sil, 0x28
add esi, edi
cmovl rsi, rdi
add r11, 0x01
xor r8b, bl
xchg sil, sil
shl sil, cl
neg r8b
ror r8b, 0x01
neg r8b
adc sil, 0x83
shl sil, cl
bswap rsi
not r8b
dec esi
mov sil, 0x8E
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, cl
sub rdi, 0x08
sar r8b, 0xD4
mov [rdi], rsi
btc r8w, r11w
mov r8d, [r11]
clc
test esi, 0x5FFD2DB2
cmc
add r11, 0x04
test bpl, r11b
xor r8d, ebx
clc
stc
rol r8d, 0x02
stc
cmp r13b, r15b
neg r8d
not r8d
inc r8d
jmp -0x265B0
not r8d
jmp +0x61FFD
inc r8d
push rbx
xor [rsp], r8d
neg ebx
pop rbx
test r11b, 0x60
clc
cmp edi, 0x7F310A36
movsxd r8, r8d
test r15b, 0xBB
add r10, r8
jmp -0x1DE9A
jmp +0x3270B
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
mov rdx, rsp
bts r9w, r12w
xadd r9d, esi
cmovnl esi, ebp
mov rcx, 0x100
bswap si
rcl r9b, cl
seto r9b
lea r9, [rdi-0x80]
mov sil, spl
or sil, bl
and r9, 0xFFFFFFFFFFFFFFF0
dec sil
sub r9, rcx
mov rsp, r9
mov sil, spl
movsx esi, r11w
push rdi
jmp -0x1AEF4
pushfq
xchg si, di
mov dil, cl
mov rsi, rdx
xchg rdi, rdi
movsx edi, r10w
mov rdi, r9
jmp -0x1C28A
cld
jmp -0xDB18
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
rep movsb
test r9w, di
xchg dil, dil
btc di, 0x4D
popfq
pop rdi
jmp -0x275C4
jmp r10
movzx r8d, byte ptr [r11]
mov rsi, rsp
not si
add r11, 0x01
neg esi
shl si, cl
xor r8b, bl
neg r8b
movzx esi, bp
bt si, di
ror r8b, 0x01
sub rsi, 0x3113452A
mov si, 0x7D49
neg r8b
rcl rsi, 0xB0
cmp r10b, bl
not r8b
movsx si, cl
cmp r9, 0x9BB5EDF
bt esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
bts r8d, r14d
not r8b
adc r8d, r9d
sub rdi, 0x08
mov [rdi], rsi
mov r8d, [r11]
add r11, 0x04
cmp r12b, 0xCF
xor r8d, ebx
clc
cmc
rol r8d, 0x02
cmc
cmp cx, 0x2552
test r12, 0x251600AE
neg r8d
jmp -0x73801
not r8d
inc r8d
jmp +0x628A2
not r8d
inc r8d
push rbx
shr bl, 0xD3
xor [rsp], r8d
rcl bh, 0xAA
test r13d, edx
pop rbx
cmc
test r11w, 0x7844
movsxd r8, r8d
add r10, r8
jmp -0x27AD2
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
or sil, cl
btc si, di
test r15d, r12d
add r11, 0x01
xor r8b, bl
cmovnle si, r9w
neg r8b
btr si, si
btc rsi, 0x20
ror r8b, 0x01
rcl sil, 0xFE
sbb rsi, 0x103B38AE
neg r8b
not r8b
sbb si, 0x916
or esi, r12d
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
mov [rdi], rsi
shl r8b, cl
mov r8d, [r11]
jmp +0x901E8
add r11, 0x04
xor r8d, ebx
jmp +0x4B3DC
rol r8d, 0x02
neg r8d
not r8d
jmp +0x1AF10
inc r8d
jmp -0x8521D
not r8d
inc r8d
stc
cmp di, 0x5EB4
push rbx
cmp r13b, sil
xor [rsp], r8d
xchg bx, bx
sub ebx, 0x3C2A06D8
pop rbx
movsxd r8, r8d
cmc
jmp +0x4D760
add r10, r8
jmp -0x3FF9D
jmp +0x28D9F
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
dec sil
bsr si, r13w
bt si, 0x56
xor r8b, bl
adc sil, spl
bts esi, r8d
inc sil
neg r8b
movsx si, r15b
ror r8b, 0x01
not rsi
bts rsi, r12
neg r8b
mov sil, r10b
clc
not r8b
add sil, sil
rol sil, 0x0D
xor bl, r8b
add esi, r11d
mov rsi, [rsp+r8*1]
adc r8w, dx
sub rdi, 0x08
bsr r8w, r12w
sbb r8w, 0x7A1F
bt r8w, 0xE1
mov [rdi], rsi
sub r8b, 0xF3
mov r8w, 0x94E
shld r8w, bp, 0xA8
mov r8d, [r11]
add r11, 0x04
jmp -0x493BE
xor r8d, ebx
clc
rol r8d, 0x02
cmp dil, bl
test r12w, r15w
test r10b, 0x8F
neg r8d
jmp +0xA7179
not r8d
jmp -0x574BD
inc r8d
jmp -0x58ECD
not r8d
jmp +0x35663
inc r8d
jmp +0x3D34C
push rbx
xor [rsp], r8d
pop rbx
movsxd r8, r8d
clc
add r10, r8
jmp -0x3C22C
jmp +0x56F6E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
bts rsi, r14
inc sil
add r11, 0x01
movsx rsi, si
ror sil, 0xBC
xor r8b, bl
movsx esi, di
mov si, 0x325C
setnp sil
neg r8b
movzx rsi, dx
rcr si, cl
ror sil, cl
ror r8b, 0x01
rcr rsi, 0xC7
neg r8b
shl sil, 0xA6
sbb si, r13w
not r8b
adc sil, r10b
rcl sil, 0x44
xor bl, r8b
mov rsi, [rsp+r8*1]
sub rdi, 0x08
jmp +0xB37E1
mov [rdi], rsi
xadd r8, r8
cmp r12b, 0xA1
mov r8d, [r11]
cmp r9b, 0x9F
cmc
jmp +0x32520
add r11, 0x04
stc
jmp -0xCD81E
xor r8d, ebx
stc
rol r8d, 0x02
cmp r9d, 0x43C35321
neg r8d
not r8d
inc r8d
jmp +0x64EC6
not r8d
jmp -0x4A752
inc r8d
test bx, r12w
push rbx
setp bh
xor [rsp], r8d
movsxd rbx, r14d
pop rbx
cmp r13w, 0x1267
movsxd r8, r8d
add r10, r8
jmp +0x2FB55
jmp +0x39243
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
stc
test rcx, 0x34EA4385
sbb sil, r15b
add r11, 0x01
stc
xor r8b, bl
cmc
neg r8b
ror si, cl
bswap rsi
movzx esi, r12w
ror r8b, 0x01
btr esi, ebx
bsr rsi, rdi
shr sil, cl
neg r8b
shr rsi, cl
btr esi, r8d
btc si, r14w
not r8b
cmp r10, 0x532F4095
xor bl, r8b
mov sil, 0xB3
ror si, 0xAD
mov rsi, [rsp+r8*1]
shl r8b, cl
shld r8w, bx, 0xC2
sub rdi, 0x08
shl r8b, cl
btc r8, r15
test r13w, sp
mov [rdi], rsi
bts r8w, bp
movzx r8d, r12w
cmc
mov r8d, [r11]
test dil, 0x7F
add r11, 0x04
xor r8d, ebx
rol r8d, 0x02
stc
neg r8d
jmp +0xC8598
not r8d
jmp +0x16694
inc r8d
jmp -0x8F873
not r8d
jmp -0x4A7D8
inc r8d
stc
push rbx
sbb ebx, 0x27102D1A
setp bl
xor [rsp], r8d
bts rbx, rbx
pop rbx
cmc
test dil, 0x4E
cmp bpl, r10b
movsxd r8, r8d
clc
add r10, r8
jmp +0x86AB6
jmp +0x277E
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
add r11, 0x01
jmp -0x3C210
xor r8b, bl
cmp eax, 0x6D434734
neg r8b
rcr sil, 0xE8
ror r8b, 0x01
neg r8b
not r8b
bts esi, r9d
xor bl, r8b
mov rsi, [rsp+r8*1]
rol r8b, 0xA1
bts r8w, cx
sub rdi, 0x08
btc r8d, ebp
mov [rdi], rsi
mov r8d, [r11]
test r10b, 0x36
jmp +0x67B53
add r11, 0x04
jmp +0x6362
xor r8d, ebx
clc
stc
rol r8d, 0x02
jmp -0x3DE4E
neg r8d
not r8d
inc r8d
not r8d
inc r8d
push rbx
xor [rsp], r8d
dec bl
pop rbx
clc
test bl, 0xB5
movsxd r8, r8d
stc
add r10, r8
jmp -0x50256
jmp +0x4A1DA
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
cmovz rsi, rsp
add r11, 0x01
rol sil, cl
rcr esi, 0x4E
shld si, di, 0xDB
xor r8b, bl
shld si, r8w, 0xD8
movzx si, bl
neg r8b
jmp -0x44832
ror r8b, 0x01
sbb sil, 0xCB
or si, r15w
ror rsi, cl
neg r8b
not r8b
bts si, r13w
cmp r9b, 0xF4
bsr esi, r8d
xor bl, r8b
movsxd rsi, esi
xchg si, si
rol sil, cl
mov rsi, [rsp+r8*1]
sub rdi, 0x08
movzx r8d, sp
mov [rdi], rsi
shl r8, 0x13
mov r8d, [r11]
add r11, 0x04
xor r8d, ebx
stc
rol r8d, 0x02
clc
cmp sil, cl
stc
neg r8d
not r8d
inc r8d
not r8d
jmp +0x4B618
inc r8d
push rbx
xor [rsp], r8d
sub bh, 0x95
pop rbx
cmp r13b, 0xD0
movsxd r8, r8d
jmp -0xA6128
add r10, r8
jmp +0x5AB78
jmp +0x3F95
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
movzx r8d, byte ptr [r11]
dec esi
rol sil, 0x00
add r11, 0x01
shl sil, 0x43
xor r8b, bl
shl si, 0x87
neg r8b
movsxd rsi, esi
movsx rsi, cx
jmp -0x4AB67
ror r8b, 0x01
sbb si, si
cmovno si, bp
neg r8b
bts esi, ebp
sbb si, 0x6C8
not r8b
rcr esi, cl
xor bl, r8b
shl sil, cl
bsr si, sp
sbb sil, 0x46
mov rsi, [rsp+r8*1]
add r8b, 0xE8
shr r8, cl
sub rdi, 0x08
btr r8w, 0xCE
shl r8w, cl
mov [rdi], rsi
bsr r8w, bp
btr r8w, 0xA8
rol r8w, 0x41
mov r8d, [r11]
cmc
add r11, 0x04
cmc
cmp r11b, r15b
xor r8d, ebx
rol r8d, 0x02
neg r8d
jmp +0x9C2FB
not r8d
jmp -0xBCBD
inc r8d
jmp -0x8468F
not r8d
inc r8d
clc
stc
push rbx
shrd rbx, rdx, 0x74
sbb ebx, ecx
mov rbx, 0x68DC6371
xor [rsp], r8d
movsx rbx, r14w
shr bh, 0x08
pop rbx
test di, 0x3163
movsxd r8, r8d
stc
add r10, r8
jmp +0x9C323
lea rdx, [rsp+0x140]
clc
cmc
cmp bpl, r9b
cmp rdi, rdx
jnbe -0x6C1E9
jmp r10
mov rsp, rdi
rol cx, cl
pop rbp
pop r12
inc eax
pop r14
clc
cdqe
or cl, r9b
pop rbx
xadd ax, si
popfq
lahf
setz ah
pop rsi
movzx r11w, spl
pop rax
movzx r15w, r10b
cwd
pop r13
bswap r10w
not r10w
pop r15
cwd
setle dh
pop r9
xchg r11w, cx
movsx r11, sp
pop rdx
movzx r8, cx
mov rdi, rcx
pop r10
mov edi, 0x7F7E3783
pop r8
mov di, cx
movsx r11, sp
pop rcx
movsx rdi, si
pop r11
setl dil
movsx edi, bp
pop rdi
ret
jmp +0x99F
mov [rsp+0x08], rcx
sub rsp, 0x38
mov rax, [rsp+0x40]
mov [rsp+0x20], rax
mov rax, [rsp+0x40]
movzx eax, byte ptr [rax]
test eax, eax
jz +0x1A
cmp dword ptr [rip+0xABBE], 0x00
jz +0x11
add rsp, 0x38
ret
push 0xFFFFFFFFF9A2DB8F
call -0x6FB9E
push r12
jmp +0x24169
push rsi
mov sil, al
push rdx
push rbx
movzx rsi, di
push r15
mov sil, 0x10
push rbp
pushfq
push r13
push rax
push rcx
push r14
push r10
push r8
rol bx, 0x3C
push rdi
bts r8d, r9d
movsx esi, r15w
movsx r8, r9w
push r11
push r9
ror r8w, 0xEC
btr bx, 0x6B
mov rdx, 0x00
movzx bp, r14b
movzx ebx, r11w
push rdx
mov r10, [rsp+0x90]
setb r8b
ror r10d, 0x03
rcr bl, 0xAC
sbb r8b, dl
bt rbx, 0x5E
sub r10d, 0x77342171
cmovle bp, r10w
btr bx, 0x7C
movzx rsi, r9w
rol r10d, 0x01
neg r10d
rcr si, cl
bt rbx, r13
btc si, di
rol r10d, 0x02
not r10d
movsxd rbx, r14d
add r10, rdx
mov rdx, 0x100000000
xadd si, bx
shl bpl, 0xD1
shr si, 0xD6
add r10, rdx
xchg r8b, sil
neg rsi
shl sil, 0xCD
mov rdi, rsp
adc r8w, 0x1785
movsx ebx, cx
sub rsp, 0x180
and rsp, 0xFFFFFFFFFFFFFFF0
rol bpl, cl
btr r8d, esp
mov bh, 0xD6
mov r8, r10
mov rbp, 0x00
neg bh
sub r8, rbp
btr si, 0x55
lea rbx, [rip-0x07]
dec rsi
mov esi, r10d
btc si, r14w
mov esi, [r10]
add r10, 0x04
stc
xor esi, r8d
clc
cmp r12w, 0x1036
cmp r14b, dl
xor esi, 0x5F864B02
jmp +0x30BD
bswap esi
test spl, bl
test r13w, 0x7FE6
xor esi, 0x14354641
sub esi, 0x5DC87738
bswap esi
cmp bpl, 0x36
jmp -0x89209
sub esi, 0x6E5013C2
cmp r8d, 0x17F3E54
cmc
jmp +0x74E3C
xor esi, 0x327E3540
push r8
xor [rsp], esi
bts r8w, r12w
stc
ror r8d, cl
pop r8
clc
movsxd rsi, esi
test r8b, 0x36
add rbx, rsi
jmp +0x3403C
jmp rbx
mov rsi, [rdi]
movsx rdx, r8w
add rdi, 0x08
movzx edx, byte ptr [r10]
bts ax, cx
bt ax, sp
add r10, 0x01
xor dl, r8b
dec dl
cmp di, 0x4F79
neg dl
sub eax, 0x5B5466F7
sbb ax, r13w
add dl, 0x21
shr ah, cl
btr eax, r15d
neg dl
cdqe
cwde
dec dl
sub eax, 0x25F47721
not dl
xor r8b, dl
bsf ax, r11w
add al, dl
mov [rsp+rdx*1], rsi
or al, r12b
mov eax, [r10]
cmp r9w, 0x3EF3
add r10, 0x04
xor eax, r8d
jmp -0x654F1
rol eax, 0x01
jmp -0x570DA
dec eax
cmp spl, r10b
bswap eax
neg eax
cmc
test r8, 0x1B484F73
push r8
sets r8b
cmp r11b, dil
jmp +0x782B6
xor [rsp], eax
jmp -0x6A47A
pop r8
stc
clc
movsxd rax, eax
cmc
cmp sil, 0xFD
test sp, r13w
add rbx, rax
jmp +0xC547D
jmp rbx
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
shl ax, 0xB3
bt eax, esi
movzx ax, r10b
add r10, 0x01
bt ax, 0x43
sbb ax, 0x26B6
xor dl, r8b
movsx rax, sp
setbe al
dec dl
cmp spl, 0x95
sar rax, 0x0A
neg dl
rcl ax, 0x43
sub eax, r9d
add dl, 0x21
sar al, cl
adc ax, 0x565E
sub eax, ecx
neg dl
dec dl
setnle al
btr ax, 0xEA
not dl
shl ah, cl
xor eax, edi
xor r8b, dl
movsx ax, r11b
mov [rsp+rdx*1], rsi
cmc
shl ax, 0x05
mov eax, [r10]
clc
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
dec eax
cmc
bswap eax
neg eax
push r8
cmp r9b, 0x6E
adc r8b, 0x8A
xor [rsp], eax
stc
add r8w, 0x300C
rcl r8b, cl
pop r8
cmc
test r9, r8
jmp -0x97C87
movsxd rax, eax
cmp r11w, r14w
add rbx, rax
jmp +0x3CEA2
jmp rbx
mov rsi, [rdi]
or al, r11b
cmp r14b, 0x11
movsx ax, ch
add rdi, 0x08
bsr eax, r15d
not ax
or dh, 0x23
movzx edx, byte ptr [r10]
mov al, r8b
lahf
add r10, 0x01
xor dl, r8b
xchg al, ah
movzx ax, r9b
movsx eax, r11w
dec dl
stc
neg dl
sub rax, 0x6E482B57
add dl, 0x21
test r15b, 0x7D
inc al
cmc
neg dl
dec dl
ror ah, 0x4D
cdqe
movzx rax, r9w
not dl
shl eax, cl
xor r8b, dl
bsf eax, r13d
mov [rsp+rdx*1], rsi
adc al, bpl
cwde
mov eax, [r10]
stc
add r10, 0x04
cmc
xor eax, r8d
rol eax, 0x01
jmp +0x26F9D
dec eax
stc
bswap eax
jmp -0x2A2A8
neg eax
jmp +0x347CE
push r8
xor [rsp], eax
shl r8b, 0x53
rcr r8b, 0x5C
cmp rdx, rsp
pop r8
movsxd rax, eax
stc
cmp al, r13b
add rbx, rax
jmp rbx
mov rsi, [rdi]
add edx, 0x264F6436
add rdi, 0x08
bt dx, r15w
or ah, 0xDC
movzx edx, byte ptr [r10]
add r10, 0x01
btr ax, 0xF8
xor dl, r8b
movzx eax, sp
movsx eax, bp
dec dl
movzx rax, r10w
neg dl
add dl, 0x21
btc ax, r14w
or al, r10b
bt ax, r11w
neg dl
jmp -0x3235F
dec dl
add al, r10b
not dl
ror al, 0x81
rcr ah, cl
xor r8b, dl
xadd ah, al
sbb al, 0x0E
mov [rsp+rdx*1], rsi
cmovno ax, dx
mov eax, [r10]
cmp cl, r10b
jmp +0x3EC46
add r10, 0x04
cmc
clc
stc
xor eax, r8d
cmc
rol eax, 0x01
jmp +0x59918
dec eax
cmp sp, r10w
bswap eax
neg eax
test r10b, 0x3A
cmc
push r8
clc
inc r8b
movsxd r8, r12d
xor [rsp], eax
bswap r8w
add r8w, 0x691B
pop r8
test spl, r8b
movsxd rax, eax
jmp -0x37994
add rbx, rax
jmp +0x4221F
push rbx
ret
mov rsi, [rdi]
rol dl, 0x29
movzx dx, r14b
cmp esi, 0x4D751F17
add rdi, 0x08
shl dx, cl
sbb al, r12b
dec ax
movzx edx, byte ptr [r10]
add r10, 0x01
btr rax, 0x3A
btc ax, r11w
xor dl, r8b
dec dl
movsxd rax, ebp
bts ax, r10w
add al, spl
neg dl
shl ax, cl
shl ah, 0x0A
add dl, 0x21
btc ax, r8w
shr ax, cl
movsx eax, sp
neg dl
movzx eax, r8w
dec dl
sar al, cl
cdqe
not dl
not ax
add al, r9b
xor r8b, dl
cbw
cdqe
mov [rsp+rdx*1], rsi
sar ax, 0x5D
test rbp, rax
stc
mov eax, [r10]
stc
test r10b, bl
add r10, 0x04
stc
xor eax, r8d
rol eax, 0x01
jmp +0xA9BD8
dec eax
cmc
bswap eax
cmc
neg eax
cmp di, bp
cmc
push r8
and r8d, 0x1F826CDC
sar r8b, 0x70
movzx r8w, r13b
xor [rsp], eax
xor r8b, 0xAB
btc r8d, 0x8F
cmovle r8d, r10d
pop r8
cmc
movsxd rax, eax
clc
add rbx, rax
jmp -0x94358
jmp rbx
mov rsi, [rdi]
sub dl, sil
add rdi, 0x08
movzx edx, byte ptr [r10]
btr ax, 0xA5
sub al, dil
movsx ax, r11b
add r10, 0x01
xor dl, r8b
dec dl
sub rax, 0x326E5F75
neg dl
add dl, 0x21
neg dl
dec dl
adc ax, 0x14A7
not dl
xor r8b, dl
xadd ax, ax
movsx rax, dx
mov [rsp+rdx*1], rsi
shr ax, 0xC7
shl al, 0x94
mov eax, [r10]
test r13b, 0x6A
add r10, 0x04
xor eax, r8d
rol eax, 0x01
jmp -0xE131
dec eax
test spl, 0x2E
stc
bswap eax
cmp al, r14b
stc
neg eax
cmp r10b, 0xB5
test bpl, r15b
push r8
xor [rsp], eax
sub r8w, 0x6EE3
adc r8b, r9b
xadd r8, r8
pop r8
jmp +0x2660
movsxd rax, eax
add rbx, rax
push rbx
ret
mov rsi, [rdi]
lahf
add rdx, r8
xor dx, 0x3760
add rdi, 0x08
cwde
shl dx, 0x22
movzx edx, byte ptr [r10]
lahf
add r10, 0x01
cmp bpl, dil
mov rax, 0x7BE03384
btr rax, 0x9E
xor dl, r8b
cmovnbe ax, r15w
bswap eax
dec dl
movzx rax, r14w
neg dl
add dl, 0x21
shrd ax, r8w, 0x81
lahf
movsxd rax, edi
neg dl
dec dl
cmp dil, dil
or al, r9b
not dl
add al, r9b
not al
xor r8b, dl
rcl eax, cl
mov [rsp+rdx*1], rsi
bts rax, rdx
cbw
mov eax, [r10]
cmp sil, 0xAB
add r10, 0x04
xor eax, r8d
stc
jmp +0x4B20A
rol eax, 0x01
dec eax
test r13b, r13b
clc
bswap eax
cmp r12b, 0x0F
test r15w, 0x7B8F
test rax, r14
neg eax
push r8
rcl r8w, cl
xor [rsp], eax
dec r8
shl r8b, cl
pop r8
movsxd rax, eax
add rbx, rax
jmp +0x80B7D
jmp rbx
mov rsi, [rdi]
sbb dx, r10w
add rdi, 0x08
cwd
bsf ax, r9w
shl dx, 0xCF
movzx edx, byte ptr [r10]
dec rax
neg ah
add r10, 0x01
xor eax, 0x1A09129F
xor dl, r8b
not eax
dec dl
neg dl
ror ah, 0xC8
rol al, 0x53
add dl, 0x21
cmp dil, 0x0D
shl al, 0xD3
cdqe
neg dl
inc ax
movzx ax, r10b
dec dl
rcl ax, cl
stc
btc ax, 0x00
not dl
shr al, cl
mov al, 0x2F
rcl ax, 0xC3
xor r8b, dl
clc
mov [rsp+rdx*1], rsi
adc ah, ah
btc ax, cx
mov eax, [r10]
add r10, 0x04
xor eax, r8d
jmp -0x9BAD0
rol eax, 0x01
jmp +0x4DFD7
dec eax
clc
bswap eax
cmp dil, 0xDF
cmp r14d, r9d
jmp +0x579F5
neg eax
jmp -0x61899
push r8
xor [rsp], eax
dec r8w
clc
pop r8
movsxd rax, eax
cmc
add rbx, rax
jmp +0x1867D
push rbx
ret
mov rsi, [rdi]
cdq
rol dl, 0xFE
bt dx, r9w
add rdi, 0x08
and dx, 0x6F10
adc dl, 0x8E
movzx edx, byte ptr [r10]
bts eax, r12d
clc
add r10, 0x01
jmp +0x367A6
xor dl, r8b
dec dl
neg dl
and ah, 0x37
movsxd rax, ebx
not al
add dl, 0x21
neg dl
movsx rax, r9w
dec dl
btc ax, di
nop
not dl
bsr ax, r8w
stc
bts rax, r13
xor r8b, dl
sar al, 0x1D
cmp dl, 0x07
dec ah
mov [rsp+rdx*1], rsi
bts ax, r10w
shl al, cl
mov eax, [r10]
test rsi, 0x607F51DE
add r10, 0x04
cmp di, dx
cmp r10b, 0x6E
xor eax, r8d
stc
clc
rol eax, 0x01
dec eax
bswap eax
test sil, sil
cmp di, 0x7985
neg eax
cmp r12b, 0x4A
push r8
adc r8b, 0x2C
movsx r8w, r15b
xor [rsp], eax
pop r8
test r9, rcx
clc
test ebx, 0x4BB60E5D
movsxd rax, eax
clc
add rbx, rax
jmp -0x47DDC
jmp rbx
mov rsi, [rdi]
xadd dl, dh
bswap dx
movzx rdx, si
add rdi, 0x08
bt dx, 0x52
movzx edx, byte ptr [r10]
btr eax, 0x91
add r10, 0x01
btc ax, r12w
xor dl, r8b
dec dl
shr ah, cl
movsx rax, r15w
xadd ah, ah
neg dl
bts eax, r9d
lahf
shr rax, 0x93
add dl, 0x21
sub al, r12b
neg dl
dec ax
setnle ah
movsx ax, sil
dec dl
xchg ah, al
adc al, 0x2B
stc
not dl
btr ax, ax
setl al
xor r8b, dl
mov ax, bx
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp spl, dl
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
jmp +0xDF88
dec eax
jmp +0x864E4
bswap eax
test r12b, spl
neg eax
cmp r11w, 0x2A8
push r8
xor [rsp], eax
test r8b, r9b
shr r8b, cl
sbb r8w, di
pop r8
movsxd rax, eax
clc
test r12b, r9b
add rbx, rax
jmp rbx
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
adc ax, 0x5716
add r10, 0x01
shrd ax, sp, 0x45
movsxd rax, r12d
xor dl, r8b
jmp -0x60C2F
dec dl
stc
dec ah
sbb al, 0x9E
neg dl
movsx eax, ax
movzx rax, si
add dl, 0x21
shl al, cl
movzx ax, dl
neg dl
movzx eax, sp
cbw
dec dl
clc
sar al, cl
not dl
shl rax, cl
cbw
xor r8b, dl
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp ebp, edx
stc
cmc
add r10, 0x04
xor eax, r8d
stc
jmp +0x7A9F6
rol eax, 0x01
jmp -0x464E6
dec eax
test r12d, ecx
jmp +0x4F3A2
bswap eax
test r10w, 0x4B8E
neg eax
push r8
cmovns r8w, ax
xor [rsp], eax
shl r8w, cl
pop r8
movsxd rax, eax
cmc
clc
cmp spl, bpl
add rbx, rax
jmp -0x5DCA2
jmp rbx
mov rsi, [rdi]
movsx rdx, r8w
add rdi, 0x08
movzx edx, byte ptr [r10]
bts ax, cx
bt ax, sp
add r10, 0x01
xor dl, r8b
dec dl
cmp di, 0x4F79
neg dl
sub eax, 0x5B5466F7
sbb ax, r13w
add dl, 0x21
shr ah, cl
btr eax, r15d
neg dl
cdqe
cwde
dec dl
sub eax, 0x25F47721
not dl
xor r8b, dl
bsf ax, r11w
add al, dl
mov [rsp+rdx*1], rsi
or al, r12b
mov eax, [r10]
cmp r9w, 0x3EF3
add r10, 0x04
xor eax, r8d
jmp -0x654F1
rol eax, 0x01
jmp -0x570DA
dec eax
cmp spl, r10b
bswap eax
neg eax
cmc
test r8, 0x1B484F73
push r8
sets r8b
cmp r11b, dil
jmp +0x782B6
xor [rsp], eax
jmp -0x6A47A
pop r8
stc
clc
movsxd rax, eax
cmc
cmp sil, 0xFD
test sp, r13w
add rbx, rax
jmp +0xC547D
jmp rbx
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
shl ax, 0xB3
bt eax, esi
movzx ax, r10b
add r10, 0x01
bt ax, 0x43
sbb ax, 0x26B6
xor dl, r8b
movsx rax, sp
setbe al
dec dl
cmp spl, 0x95
sar rax, 0x0A
neg dl
rcl ax, 0x43
sub eax, r9d
add dl, 0x21
sar al, cl
adc ax, 0x565E
sub eax, ecx
neg dl
dec dl
setnle al
btr ax, 0xEA
not dl
shl ah, cl
xor eax, edi
xor r8b, dl
movsx ax, r11b
mov [rsp+rdx*1], rsi
cmc
shl ax, 0x05
mov eax, [r10]
clc
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
dec eax
cmc
bswap eax
neg eax
push r8
cmp r9b, 0x6E
adc r8b, 0x8A
xor [rsp], eax
stc
add r8w, 0x300C
rcl r8b, cl
pop r8
cmc
test r9, r8
jmp -0x97C87
movsxd rax, eax
cmp r11w, r14w
add rbx, rax
jmp +0x3CEA2
jmp rbx
mov rsi, [rdi]
or al, r11b
cmp r14b, 0x11
movsx ax, ch
add rdi, 0x08
bsr eax, r15d
not ax
or dh, 0x23
movzx edx, byte ptr [r10]
mov al, r8b
lahf
add r10, 0x01
xor dl, r8b
xchg al, ah
movzx ax, r9b
movsx eax, r11w
dec dl
stc
neg dl
sub rax, 0x6E482B57
add dl, 0x21
test r15b, 0x7D
inc al
cmc
neg dl
dec dl
ror ah, 0x4D
cdqe
movzx rax, r9w
not dl
shl eax, cl
xor r8b, dl
bsf eax, r13d
mov [rsp+rdx*1], rsi
adc al, bpl
cwde
mov eax, [r10]
stc
add r10, 0x04
cmc
xor eax, r8d
rol eax, 0x01
jmp +0x26F9D
dec eax
stc
bswap eax
jmp -0x2A2A8
neg eax
jmp +0x347CE
push r8
xor [rsp], eax
shl r8b, 0x53
rcr r8b, 0x5C
cmp rdx, rsp
pop r8
movsxd rax, eax
stc
cmp al, r13b
add rbx, rax
jmp rbx
mov rsi, [rdi]
add edx, 0x264F6436
add rdi, 0x08
bt dx, r15w
or ah, 0xDC
movzx edx, byte ptr [r10]
add r10, 0x01
btr ax, 0xF8
xor dl, r8b
movzx eax, sp
movsx eax, bp
dec dl
movzx rax, r10w
neg dl
add dl, 0x21
btc ax, r14w
or al, r10b
bt ax, r11w
neg dl
jmp -0x3235F
dec dl
add al, r10b
not dl
ror al, 0x81
rcr ah, cl
xor r8b, dl
xadd ah, al
sbb al, 0x0E
mov [rsp+rdx*1], rsi
cmovno ax, dx
mov eax, [r10]
cmp cl, r10b
jmp +0x3EC46
add r10, 0x04
cmc
clc
stc
xor eax, r8d
cmc
rol eax, 0x01
jmp +0x59918
dec eax
cmp sp, r10w
bswap eax
neg eax
test r10b, 0x3A
cmc
push r8
clc
inc r8b
movsxd r8, r12d
xor [rsp], eax
bswap r8w
add r8w, 0x691B
pop r8
test spl, r8b
movsxd rax, eax
jmp -0x37994
add rbx, rax
jmp +0x4221F
push rbx
ret
mov rsi, [rdi]
rol dl, 0x29
movzx dx, r14b
cmp esi, 0x4D751F17
add rdi, 0x08
shl dx, cl
sbb al, r12b
dec ax
movzx edx, byte ptr [r10]
add r10, 0x01
btr rax, 0x3A
btc ax, r11w
xor dl, r8b
dec dl
movsxd rax, ebp
bts ax, r10w
add al, spl
neg dl
shl ax, cl
shl ah, 0x0A
add dl, 0x21
btc ax, r8w
shr ax, cl
movsx eax, sp
neg dl
movzx eax, r8w
dec dl
sar al, cl
cdqe
not dl
not ax
add al, r9b
xor r8b, dl
cbw
cdqe
mov [rsp+rdx*1], rsi
sar ax, 0x5D
test rbp, rax
stc
mov eax, [r10]
stc
test r10b, bl
add r10, 0x04
stc
xor eax, r8d
rol eax, 0x01
jmp +0xA9BD8
dec eax
cmc
bswap eax
cmc
neg eax
cmp di, bp
cmc
push r8
and r8d, 0x1F826CDC
sar r8b, 0x70
movzx r8w, r13b
xor [rsp], eax
xor r8b, 0xAB
btc r8d, 0x8F
cmovle r8d, r10d
pop r8
cmc
movsxd rax, eax
clc
add rbx, rax
jmp -0x94358
jmp rbx
mov rsi, [rdi]
sub dl, sil
add rdi, 0x08
movzx edx, byte ptr [r10]
btr ax, 0xA5
sub al, dil
movsx ax, r11b
add r10, 0x01
xor dl, r8b
dec dl
sub rax, 0x326E5F75
neg dl
add dl, 0x21
neg dl
dec dl
adc ax, 0x14A7
not dl
xor r8b, dl
xadd ax, ax
movsx rax, dx
mov [rsp+rdx*1], rsi
shr ax, 0xC7
shl al, 0x94
mov eax, [r10]
test r13b, 0x6A
add r10, 0x04
xor eax, r8d
rol eax, 0x01
jmp -0xE131
dec eax
test spl, 0x2E
stc
bswap eax
cmp al, r14b
stc
neg eax
cmp r10b, 0xB5
test bpl, r15b
push r8
xor [rsp], eax
sub r8w, 0x6EE3
adc r8b, r9b
xadd r8, r8
pop r8
jmp +0x2660
movsxd rax, eax
add rbx, rax
push rbx
ret
mov rsi, [rdi]
lahf
add rdx, r8
xor dx, 0x3760
add rdi, 0x08
cwde
shl dx, 0x22
movzx edx, byte ptr [r10]
lahf
add r10, 0x01
cmp bpl, dil
mov rax, 0x7BE03384
btr rax, 0x9E
xor dl, r8b
cmovnbe ax, r15w
bswap eax
dec dl
movzx rax, r14w
neg dl
add dl, 0x21
shrd ax, r8w, 0x81
lahf
movsxd rax, edi
neg dl
dec dl
cmp dil, dil
or al, r9b
not dl
add al, r9b
not al
xor r8b, dl
rcl eax, cl
mov [rsp+rdx*1], rsi
bts rax, rdx
cbw
mov eax, [r10]
cmp sil, 0xAB
add r10, 0x04
xor eax, r8d
stc
jmp +0x4B20A
rol eax, 0x01
dec eax
test r13b, r13b
clc
bswap eax
cmp r12b, 0x0F
test r15w, 0x7B8F
test rax, r14
neg eax
push r8
rcl r8w, cl
xor [rsp], eax
dec r8
shl r8b, cl
pop r8
movsxd rax, eax
add rbx, rax
jmp +0x80B7D
jmp rbx
mov rsi, [rdi]
sbb dx, r10w
add rdi, 0x08
cwd
bsf ax, r9w
shl dx, 0xCF
movzx edx, byte ptr [r10]
dec rax
neg ah
add r10, 0x01
xor eax, 0x1A09129F
xor dl, r8b
not eax
dec dl
neg dl
ror ah, 0xC8
rol al, 0x53
add dl, 0x21
cmp dil, 0x0D
shl al, 0xD3
cdqe
neg dl
inc ax
movzx ax, r10b
dec dl
rcl ax, cl
stc
btc ax, 0x00
not dl
shr al, cl
mov al, 0x2F
rcl ax, 0xC3
xor r8b, dl
clc
mov [rsp+rdx*1], rsi
adc ah, ah
btc ax, cx
mov eax, [r10]
add r10, 0x04
xor eax, r8d
jmp -0x9BAD0
rol eax, 0x01
jmp +0x4DFD7
dec eax
clc
bswap eax
cmp dil, 0xDF
cmp r14d, r9d
jmp +0x579F5
neg eax
jmp -0x61899
push r8
xor [rsp], eax
dec r8w
clc
pop r8
movsxd rax, eax
cmc
add rbx, rax
jmp +0x1867D
push rbx
ret
mov rax, [r10]
jmp +0x4C94B
add r10, 0x08
rol cx, 0x94
not ecx
bsf cx, cx
xor rax, r8
dec rax
rol rax, 0x01
mov cx, r13w
cmovs ecx, r13d
movzx cx, dh
not rax
movzx rcx, r9w
inc rax
bts rcx, rbp
shr cl, cl
neg rax
xor r8, rax
neg cx
clc
xchg cx, cx
sub rdi, 0x08
mov [rdi], rax
rcr ch, cl
mov ecx, [r10]
cmp r10b, al
cmp r9w, 0x3C51
stc
add r10, 0x04
jmp +0x3A0E6
xor ecx, r8d
cmc
rol ecx, 0x03
bswap ecx
test r10d, 0xB7823C7
sub ecx, 0x6F335F3
stc
cmc
not ecx
jmp +0x26E58
rol ecx, 0x01
push r8
bsr r8w, r14w
xor [rsp], ecx
pop r8
clc
test r8b, 0xA5
test r15b, r13b
movsxd rcx, ecx
cmp r10b, r13b
test dil, r15b
add rbx, rcx
jmp +0xE607
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
sar cx, 0x75
cwde
add r10, 0x01
xor r11b, r8b
test r12b, 0x9F
neg r11b
xchg cl, cl
shl ax, cl
movsxd rax, r10d
add r11b, 0x3E
cdqe
neg ch
xor r11b, 0xC9
inc r11b
movsxd rcx, esi
mov ch, 0x16
rcr rcx, cl
xor r8b, r11b
and cl, bh
btc cx, si
mov rax, [rsp+r11*1]
sub rdi, 0x08
stc
jmp +0x26208
mov [rdi], rax
shl rcx, 0x7C
sar cl, 0xBD
neg ch
mov ecx, [r10]
clc
add r10, 0x04
test r9, rsi
xor ecx, r8d
stc
neg ecx
test r10d, 0x410A5956
sub ecx, 0x50D714E5
stc
clc
ror ecx, 0x01
jmp +0x3745F
dec ecx
cmc
rol ecx, 0x02
jmp +0x330B5
dec ecx
rol ecx, 0x01
dec ecx
push r8
shr r8d, cl
movsx r8d, di
xor [rsp], ecx
clc
pop r8
movsxd rcx, ecx
clc
cmp r10b, 0x86
add rbx, rcx
jmp +0x2114
jmp +0x42197
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11, [rdi]
btr ecx, 0xE8
xchg ch, ch
mov rcx, [rdi+0x08]
cmp r14w, r9w
clc
add r11, rcx
jmp +0x596FD
mov [rdi+0x08], r11
not r11w
pushfq
mov r11d, 0x4809274E
or r11b, spl
pop [rdi]
rcr r11b, cl
rol r11, 0x18
cmc
mov r11d, [r10]
add r10, 0x04
xor r11d, r8d
jmp +0x4E928
inc r11d
cmc
rol r11d, 0x01
jmp -0x9BEFC
xor r11d, 0x29811801
clc
cmp r11, 0x75797C15
test al, cl
not r11d
cmc
cmp dl, 0x05
neg r11d
stc
bswap r11d
jmp +0x877F4
push r8
movzx r8, r12w
xor [rsp], r11d
clc
sbb r8, 0x574C6A24
pop r8
clc
movsxd r11, r11d
stc
test r15b, 0xDA
cmp cx, 0x6928
add rbx, r11
jmp -0x22FC
push rbx
ret
mov rsi, [rdi]
cdq
rol dl, 0xFE
bt dx, r9w
add rdi, 0x08
and dx, 0x6F10
adc dl, 0x8E
movzx edx, byte ptr [r10]
bts eax, r12d
clc
add r10, 0x01
jmp +0x367A6
xor dl, r8b
dec dl
neg dl
and ah, 0x37
movsxd rax, ebx
not al
add dl, 0x21
neg dl
movsx rax, r9w
dec dl
btc ax, di
nop
not dl
bsr ax, r8w
stc
bts rax, r13
xor r8b, dl
sar al, 0x1D
cmp dl, 0x07
dec ah
mov [rsp+rdx*1], rsi
bts ax, r10w
shl al, cl
mov eax, [r10]
test rsi, 0x607F51DE
add r10, 0x04
cmp di, dx
cmp r10b, 0x6E
xor eax, r8d
stc
clc
rol eax, 0x01
dec eax
bswap eax
test sil, sil
cmp di, 0x7985
neg eax
cmp r12b, 0x4A
push r8
adc r8b, 0x2C
movsx r8w, r15b
xor [rsp], eax
pop r8
test r9, rcx
clc
test ebx, 0x4BB60E5D
movsxd rax, eax
clc
add rbx, rax
jmp -0x47DDC
jmp rbx
mov rsi, [rdi]
xadd dl, dh
bswap dx
movzx rdx, si
add rdi, 0x08
bt dx, 0x52
movzx edx, byte ptr [r10]
btr eax, 0x91
add r10, 0x01
btc ax, r12w
xor dl, r8b
dec dl
shr ah, cl
movsx rax, r15w
xadd ah, ah
neg dl
bts eax, r9d
lahf
shr rax, 0x93
add dl, 0x21
sub al, r12b
neg dl
dec ax
setnle ah
movsx ax, sil
dec dl
xchg ah, al
adc al, 0x2B
stc
not dl
btr ax, ax
setl al
xor r8b, dl
mov ax, bx
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp spl, dl
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
jmp +0xDF88
dec eax
jmp +0x864E4
bswap eax
test r12b, spl
neg eax
cmp r11w, 0x2A8
push r8
xor [rsp], eax
test r8b, r9b
shr r8b, cl
sbb r8w, di
pop r8
movsxd rax, eax
clc
test r12b, r9b
add rbx, rax
jmp rbx
mov rsi, rdi
shld rdx, rsi, 0x42
sub rdi, 0x08
bt dx, di
test sp, 0x1108
xadd edx, edx
mov [rdi], rsi
or dx, 0x1327
shr rdx, cl
mov edx, [r10]
add r10, 0x04
cmp cx, sp
xor edx, r8d
neg edx
bswap edx
jmp -0x653A
not edx
dec edx
bswap edx
cmp r12w, si
cmc
push r8
xor [rsp], edx
movsx r8d, r12w
rcr r8b, cl
movzx r8d, ax
pop r8
movsxd rdx, edx
cmc
add rbx, rdx
jmp +0x8BB28
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
adc ax, 0x5716
add r10, 0x01
shrd ax, sp, 0x45
movsxd rax, r12d
xor dl, r8b
jmp -0x60C2F
dec dl
stc
dec ah
sbb al, 0x9E
neg dl
movsx eax, ax
movzx rax, si
add dl, 0x21
shl al, cl
movzx ax, dl
neg dl
movzx eax, sp
cbw
dec dl
clc
sar al, cl
not dl
shl rax, cl
cbw
xor r8b, dl
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp ebp, edx
stc
cmc
add r10, 0x04
xor eax, r8d
stc
jmp +0x7A9F6
rol eax, 0x01
jmp -0x464E6
dec eax
test r12d, ecx
jmp +0x4F3A2
bswap eax
test r10w, 0x4B8E
neg eax
push r8
cmovns r8w, ax
xor [rsp], eax
shl r8w, cl
pop r8
movsxd rax, eax
cmc
clc
cmp spl, bpl
add rbx, rax
jmp -0x5DCA2
jmp rbx
movzx r11d, byte ptr [r10]
bt ax, si
dec cx
lahf
add r10, 0x01
bswap rax
xor r11b, r8b
jmp -0x84A13
neg r11b
shld cx, si, 0x9E
bsr eax, ebp
rcr cl, 0x99
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
shr cl, 0x51
btc rcx, 0x18
test r13b, 0xDB
xor r8b, r11b
xchg ah, al
mov rax, [rsp+r11*1]
not cl
and cx, r8w
movsxd rcx, eax
sub rdi, 0x08
or cx, 0x5BCA
mov [rdi], rax
rol rcx, cl
shrd ecx, ebp, 0xA0
shld cx, r9w, 0x7D
mov ecx, [r10]
jmp +0x2FCBD
add r10, 0x04
clc
cmp r12b, r8b
jmp -0xFFA3
xor ecx, r8d
stc
test r11b, 0x30
neg ecx
clc
cmc
test rbp, 0x70C629E2
sub ecx, 0x50D714E5
clc
ror ecx, 0x01
jmp +0x9DB4D
dec ecx
rol ecx, 0x02
jmp -0xE56EB
dec ecx
cmc
stc
rol ecx, 0x01
jmp +0x546EE
dec ecx
cmp bx, 0x3F00
push r8
bt r8d, 0xE4
xor [rsp], ecx
ror r8, 0xE8
adc r8b, al
bsf r8, rbp
pop r8
movsxd rcx, ecx
test rbp, 0x71290159
add rbx, rcx
jmp +0x1BE13
jmp +0x60AF7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
movsx rdx, r8w
add rdi, 0x08
movzx edx, byte ptr [r10]
bts ax, cx
bt ax, sp
add r10, 0x01
xor dl, r8b
dec dl
cmp di, 0x4F79
neg dl
sub eax, 0x5B5466F7
sbb ax, r13w
add dl, 0x21
shr ah, cl
btr eax, r15d
neg dl
cdqe
cwde
dec dl
sub eax, 0x25F47721
not dl
xor r8b, dl
bsf ax, r11w
add al, dl
mov [rsp+rdx*1], rsi
or al, r12b
mov eax, [r10]
cmp r9w, 0x3EF3
add r10, 0x04
xor eax, r8d
jmp -0x654F1
rol eax, 0x01
jmp -0x570DA
dec eax
cmp spl, r10b
bswap eax
neg eax
cmc
test r8, 0x1B484F73
push r8
sets r8b
cmp r11b, dil
jmp +0x782B6
xor [rsp], eax
jmp -0x6A47A
pop r8
stc
clc
movsxd rax, eax
cmc
cmp sil, 0xFD
test sp, r13w
add rbx, rax
jmp +0xC547D
jmp rbx
movzx r11d, byte ptr [r10]
movsx ecx, si
cdqe
add r10, 0x01
xor r11b, r8b
sbb rax, r8
rol eax, 0xB5
xadd rcx, rax
neg r11b
or al, spl
shr ax, 0xB7
add r11b, 0x3E
rcl cx, cl
xor r11b, 0xC9
movsxd rax, r14d
inc r11b
xor r8b, r11b
lahf
mov rax, [rsp+r11*1]
sub rdi, 0x08
bsr ecx, ecx
btc ecx, esi
mov [rdi], rax
xadd ch, cl
bts cx, cx
mov ecx, [r10]
test cl, r9b
cmp dh, 0xB6
test r12b, 0xDC
add r10, 0x04
cmp rbx, rbx
test r8, 0x7389064F
xor ecx, r8d
neg ecx
test edx, 0x34820148
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp -0x1286D
dec ecx
stc
cmc
rol ecx, 0x02
jmp +0x11E2C
dec ecx
clc
stc
rol ecx, 0x01
jmp -0x1BF42
dec ecx
push r8
neg r8
mov r8w, 0xFD
xor [rsp], ecx
pop r8
cmc
movsxd rcx, ecx
add rbx, rcx
jmp +0xACE18
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
shl ax, 0xB3
bt eax, esi
movzx ax, r10b
add r10, 0x01
bt ax, 0x43
sbb ax, 0x26B6
xor dl, r8b
movsx rax, sp
setbe al
dec dl
cmp spl, 0x95
sar rax, 0x0A
neg dl
rcl ax, 0x43
sub eax, r9d
add dl, 0x21
sar al, cl
adc ax, 0x565E
sub eax, ecx
neg dl
dec dl
setnle al
btr ax, 0xEA
not dl
shl ah, cl
xor eax, edi
xor r8b, dl
movsx ax, r11b
mov [rsp+rdx*1], rsi
cmc
shl ax, 0x05
mov eax, [r10]
clc
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
dec eax
cmc
bswap eax
neg eax
push r8
cmp r9b, 0x6E
adc r8b, 0x8A
xor [rsp], eax
stc
add r8w, 0x300C
rcl r8b, cl
pop r8
cmc
test r9, r8
jmp -0x97C87
movsxd rax, eax
cmp r11w, r14w
add rbx, rax
jmp +0x3CEA2
jmp rbx
movzx r11d, byte ptr [r10]
dec cl
add r10, 0x01
and cx, 0x3627
shl rcx, 0x35
xor r11b, r8b
neg r11b
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
sar cx, cl
rcr cl, 0x12
lahf
xor r8b, r11b
bswap rcx
mov rax, [rsp+r11*1]
mov cl, 0x4E
rol cx, cl
sub rdi, 0x08
mov [rdi], rax
cmovnl cx, r13w
mov ecx, [r10]
test dil, r11b
add r10, 0x04
xor ecx, r8d
cmc
neg ecx
cmp esi, r8d
cmp ax, 0x1974
sub ecx, 0x50D714E5
stc
ror ecx, 0x01
dec ecx
rol ecx, 0x02
jmp +0x57B6
dec ecx
stc
jmp -0xC1845
rol ecx, 0x01
jmp +0x5335A
dec ecx
test rsp, rax
stc
push r8
xor [rsp], ecx
adc r8w, r15w
jmp +0x26D6A
pop r8
cmp spl, 0x8F
jmp -0x7BFB
movsxd rcx, ecx
test sil, bpl
cmc
add rbx, rcx
jmp +0x1512
jmp +0x38883
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
or al, r11b
cmp r14b, 0x11
movsx ax, ch
add rdi, 0x08
bsr eax, r15d
not ax
or dh, 0x23
movzx edx, byte ptr [r10]
mov al, r8b
lahf
add r10, 0x01
xor dl, r8b
xchg al, ah
movzx ax, r9b
movsx eax, r11w
dec dl
stc
neg dl
sub rax, 0x6E482B57
add dl, 0x21
test r15b, 0x7D
inc al
cmc
neg dl
dec dl
ror ah, 0x4D
cdqe
movzx rax, r9w
not dl
shl eax, cl
xor r8b, dl
bsf eax, r13d
mov [rsp+rdx*1], rsi
adc al, bpl
cwde
mov eax, [r10]
stc
add r10, 0x04
cmc
xor eax, r8d
rol eax, 0x01
jmp +0x26F9D
dec eax
stc
bswap eax
jmp -0x2A2A8
neg eax
jmp +0x347CE
push r8
xor [rsp], eax
shl r8b, 0x53
rcr r8b, 0x5C
cmp rdx, rsp
pop r8
movsxd rax, eax
stc
cmp al, r13b
add rbx, rax
jmp rbx
movzx r11d, byte ptr [r10]
rcr al, cl
movsx ax, r8b
bts cx, dx
add r10, 0x01
adc ah, al
xor r11b, r8b
shl rcx, 0xB8
neg r11b
test r14d, 0x19164107
add r11b, 0x3E
ror rcx, 0x46
movzx cx, r13b
xor r11b, 0xC9
nop
cwde
mov cl, 0x1F
inc r11b
rcl cl, 0x35
ror ax, cl
movsx rcx, sp
xor r8b, r11b
cdqe
adc cl, 0xBB
mov rax, [rsp+r11*1]
sub rdi, 0x08
btc rcx, 0xD5
mov [rdi], rax
mov ecx, [r10]
test r13b, r15b
add r10, 0x04
clc
xor ecx, r8d
test rsp, r10
neg ecx
jmp -0x78E2D
sub ecx, 0x50D714E5
jmp +0x8833C
ror ecx, 0x01
dec ecx
rol ecx, 0x02
jmp -0x3823D
dec ecx
rol ecx, 0x01
jmp -0x32491
dec ecx
test r9d, 0x55A62CD5
push r8
xor [rsp], ecx
bt r8w, r8w
adc r8b, 0x46
rol r8, cl
pop r8
movsxd rcx, ecx
clc
stc
test si, r8w
add rbx, rcx
jmp -0x5F8D
jmp +0x9C6BE
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add edx, 0x264F6436
add rdi, 0x08
bt dx, r15w
or ah, 0xDC
movzx edx, byte ptr [r10]
add r10, 0x01
btr ax, 0xF8
xor dl, r8b
movzx eax, sp
movsx eax, bp
dec dl
movzx rax, r10w
neg dl
add dl, 0x21
btc ax, r14w
or al, r10b
bt ax, r11w
neg dl
jmp -0x3235F
dec dl
add al, r10b
not dl
ror al, 0x81
rcr ah, cl
xor r8b, dl
xadd ah, al
sbb al, 0x0E
mov [rsp+rdx*1], rsi
cmovno ax, dx
mov eax, [r10]
cmp cl, r10b
jmp +0x3EC46
add r10, 0x04
cmc
clc
stc
xor eax, r8d
cmc
rol eax, 0x01
jmp +0x59918
dec eax
cmp sp, r10w
bswap eax
neg eax
test r10b, 0x3A
cmc
push r8
clc
inc r8b
movsxd r8, r12d
xor [rsp], eax
bswap r8w
add r8w, 0x691B
pop r8
test spl, r8b
movsxd rax, eax
jmp -0x37994
add rbx, rax
jmp +0x4221F
push rbx
ret
movzx r11d, byte ptr [r10]
btr eax, esi
stc
add r10, 0x01
not al
cbw
rcr ch, cl
xor r11b, r8b
neg r11b
stc
add r11b, 0x3E
xor r11b, 0xC9
jmp +0xE0398
inc r11b
xor r8b, r11b
mov rax, [rsp+r11*1]
sub rdi, 0x08
mov [rdi], rax
adc cl, r10b
mov ecx, [r10]
add r10, 0x04
clc
xor ecx, r8d
test r10, 0x1D007430
jmp -0x6FD15
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp +0x19EAC
dec ecx
rol ecx, 0x02
jmp -0x49101
dec ecx
rol ecx, 0x01
jmp -0x3BFD2
dec ecx
test r13w, r9w
push r8
sub r8b, 0xA4
xor [rsp], ecx
xadd r8b, r8b
mov r8w, 0x2277
rcl r8d, cl
pop r8
movsxd rcx, ecx
cmc
cmp r14b, bpl
add rbx, rcx
jmp +0xCE94A
jmp -0x35F1
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
add r10, 0x01
bt ecx, r15d
movsxd rax, ebp
btr ax, 0x2A
xor r11b, r8b
adc ecx, 0x1344781D
neg r11b
bt ax, 0x21
not cx
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
movsx ecx, r14w
jmp -0x86DD
xor r8b, r11b
bsf ax, ax
mov rax, [rsp+r11*1]
rol cx, cl
mov ecx, 0x8772461
sub rdi, 0x08
add cx, sp
mov [rdi], rax
mov ecx, [r10]
clc
add r10, 0x04
xor ecx, r8d
neg ecx
cmp si, 0xD7A
cmc
sub ecx, 0x50D714E5
ror ecx, 0x01
dec ecx
jmp -0x44435
rol ecx, 0x02
jmp +0xAC982
dec ecx
clc
rol ecx, 0x01
jmp -0x866C8
dec ecx
cmc
stc
push r8
xor [rsp], ecx
movsx r8w, spl
xchg r8d, r8d
add r8b, 0x0D
pop r8
cmp r11b, 0xE9
movsxd rcx, ecx
add rbx, rcx
jmp +0x6B84C
jmp +0x33CDF
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rax, [r10]
neg cx
movsx ecx, di
add r10, 0x08
xchg cl, cl
xor rax, r8
dec rax
rol rax, 0x01
movzx rcx, ax
movsx rcx, r12w
movsx ecx, r12w
not rax
inc rax
neg rax
sub ecx, esi
ror cx, 0xBE
xor r8, rax
btc cx, 0x15
bsr cx, bx
sub rdi, 0x08
mov [rdi], rax
movsx ecx, si
mov ecx, [r10]
stc
add r10, 0x04
cmp edx, r12d
test bpl, 0x84
xor ecx, r8d
rol ecx, 0x03
stc
cmp r9b, 0xA4
jmp -0x5A4D8
bswap ecx
sub ecx, 0x6F335F3
not ecx
stc
cmc
rol ecx, 0x01
push r8
sbb r8w, 0x2B0B
btr r8d, 0x66
xor [rsp], ecx
pop r8
movsxd rcx, ecx
stc
add rbx, rcx
jmp -0x13BCA
jmp +0x962FF
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11, [rdi]
mov rcx, [rdi+0x08]
cmc
cmp spl, sil
clc
add r11, rcx
mov [rdi+0x08], r11
bswap r11w
pushfq
or r11b, 0x2A
jmp +0x43974
pop [rdi]
shrd r11w, r8w, 0x0F
mov r11d, [r10]
test dil, 0x09
cmp r10, rbx
add r10, 0x04
cmc
xor r11d, r8d
jmp -0x20B80
inc r11d
stc
cmc
rol r11d, 0x01
clc
xor r11d, 0x29811801
not r11d
test dil, 0x58
cmp r11w, 0x32B3
cmc
neg r11d
test rbp, r8
test dil, 0xA5
jmp +0x2A824
bswap r11d
test r10b, 0xAD
cmp cl, r14b
push r8
sbb r8b, dil
rcl r8w, 0xE5
xor [rsp], r11d
adc r8b, r10b
pop r8
movsxd r11, r11d
add rbx, r11
jmp +0xA594
jmp rbx
mov rsi, [rdi]
rol dl, 0x29
movzx dx, r14b
cmp esi, 0x4D751F17
add rdi, 0x08
shl dx, cl
sbb al, r12b
dec ax
movzx edx, byte ptr [r10]
add r10, 0x01
btr rax, 0x3A
btc ax, r11w
xor dl, r8b
dec dl
movsxd rax, ebp
bts ax, r10w
add al, spl
neg dl
shl ax, cl
shl ah, 0x0A
add dl, 0x21
btc ax, r8w
shr ax, cl
movsx eax, sp
neg dl
movzx eax, r8w
dec dl
sar al, cl
cdqe
not dl
not ax
add al, r9b
xor r8b, dl
cbw
cdqe
mov [rsp+rdx*1], rsi
sar ax, 0x5D
test rbp, rax
stc
mov eax, [r10]
stc
test r10b, bl
add r10, 0x04
stc
xor eax, r8d
rol eax, 0x01
jmp +0xA9BD8
dec eax
cmc
bswap eax
cmc
neg eax
cmp di, bp
cmc
push r8
and r8d, 0x1F826CDC
sar r8b, 0x70
movzx r8w, r13b
xor [rsp], eax
xor r8b, 0xAB
btc r8d, 0x8F
cmovle r8d, r10d
pop r8
cmc
movsxd rax, eax
clc
add rbx, rax
jmp -0x94358
jmp rbx
mov rax, [rdi]
mov r11b, 0x1A
mov r11d, [rax]
stc
cmp cx, r10w
jmp +0x75F6C
add rdi, 0x04
mov [rdi], r11d
shl r11d, 0x4C
xor r11b, 0x06
mov r11d, [r10]
jmp -0x7825C
add r10, 0x04
xor r11d, r8d
ror r11d, 0x02
clc
add r11d, 0x5AFC075D
stc
bswap r11d
clc
cmc
rol r11d, 0x01
test r8b, dil
test ecx, 0xEDB11C9
stc
not r11d
test sil, 0x7D
cmp di, 0xA7B
clc
push r8
xor r8w, ax
xor [rsp], r11d
shl r8, 0x61
pop r8
stc
movsxd r11, r11d
clc
add rbx, r11
push rbx
ret
mov ebp, [rdi]
bts r11w, r13w
btc r9, r11
add rdi, 0x04
bswap r11w
movsx r11w, bl
bt r9w, r8w
movzx r9d, byte ptr [r10]
shrd r11, r14, 0x67
movzx r11, r12w
setnp r11b
add r10, 0x01
cmp ecx, esp
rcr r11b, cl
xor r9b, r8b
cmp dl, r9b
xor r9b, 0x31
test r15b, al
add r9b, 0xEE
rcl r11w, 0x38
rol r11w, 0xE7
ror r9b, 0x01
dec r9b
inc r11w
btr r11d, r14d
shl r11b, 0x89
xor r8b, r9b
clc
mov [rsp+r9*1], ebp
xadd r11b, r11b
cmp r15b, 0xF9
or r11d, 0x1B0C538B
mov r11d, [r10]
cmp r10w, r12w
cmc
add r10, 0x04
test r9b, r9b
clc
xor r11d, r8d
jmp -0xDE89A
dec r11d
jmp +0x5B52F
ror r11d, 0x03
neg r11d
jmp +0x7D6F6
inc r11d
cmp ecx, esp
jmp -0x13AEA
push r8
bsr r8w, bx
xor r8d, r12d
xor [rsp], r11d
btr r8w, bx
clc
inc r8w
pop r8
movsxd r11, r11d
add rbx, r11
push rbx
ret
mov r9d, [r10]
add r10, 0x04
shl ax, cl
xor r9d, r8d
xchg ah, ah
cwde
movsx eax, r8w
inc r9d
bts ax, bp
ror r9d, 0x02
jmp -0x1F321
neg r9d
movzx ax, r14b
setnl ah
cmovp rax, r8
add r9d, 0x31114D8A
push r8
bsr r8w, ax
shl r8, 0x10
or r8b, 0xA5
xor [rsp], r9d
cmp rdx, 0x325312D3
rcr r8w, cl
pop r8
btr eax, edx
bsr ax, r11w
sub rdi, 0x04
mov [rdi], r9d
sar rax, 0x2F
add ax, bx
bsf rax, r9
mov eax, [r10]
clc
add r10, 0x04
clc
xor eax, r8d
test sil, al
test r10b, 0x64
add eax, 0x38A453FB
cmc
ror eax, 0x03
cmc
stc
sub eax, 0xF9677CA
test sp, bp
not eax
cmp r15b, al
cmc
test r13b, 0x7D
push r8
test si, 0x678C
xor [rsp], eax
dec r8w
xor r8b, r11b
pop r8
movsxd rax, eax
test r14b, 0x51
add rbx, rax
jmp -0x92226
jmp +0x9148D
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
add rdi, 0x04
jmp -0x14CF5
movzx r9d, byte ptr [r10]
xor r11w, r8w
add r10, 0x01
xchg r11b, r11b
and r11d, eax
sar r11b, 0xBF
xor r9b, r8b
xor r9b, 0x31
mov r11w, dx
bt r11w, 0x50
adc r11b, cl
add r9b, 0xEE
ror r9b, 0x01
bswap r11w
movzx r11d, r10w
dec r9b
setno r11b
shr r11w, cl
xor r8b, r9b
btr r11w, 0x83
movzx r11d, r9w
mov [rsp+r9*1], ebp
cmp r14b, 0x8D
mov r11d, [r10]
test bh, dh
add r10, 0x04
cmc
cmp r8b, r12b
xor r11d, r8d
jmp -0x30C0
dec r11d
jmp +0x6D6ED
ror r11d, 0x03
neg r11d
jmp -0x1C71
inc r11d
cmp r10b, 0xFD
push r8
xor [rsp], r11d
shrd r8, rsi, 0xFD
btc r8w, r13w
pop r8
stc
movsxd r11, r11d
stc
add rbx, r11
jmp -0x19DC6
push rbx
ret
mov rax, [r10]
cmovbe cx, r15w
add r10, 0x08
stc
btc cx, 0x11
xor rax, r8
inc rcx
dec rax
rol ch, cl
rol rax, 0x01
jmp -0x20F2D
not rax
inc cx
inc rax
sar ch, cl
neg rax
sbb cl, r11b
xor r8, rax
btr cx, 0x6B
sub rdi, 0x08
mov [rdi], rax
bsr ecx, ecx
sub cx, si
mov ecx, [r10]
stc
test r9b, dil
add r10, 0x04
cmp sil, r13b
xor ecx, r8d
cmc
jmp -0x4C3B3
rol ecx, 0x03
jmp +0x6DBA8
bswap ecx
cmp r10b, cl
stc
sub ecx, 0x6F335F3
jmp -0x22A66
not ecx
rol ecx, 0x01
push r8
xor [rsp], ecx
xor r8b, bpl
add r8, 0x4980614D
pop r8
movsxd rcx, ecx
test bp, 0x52CA
add rbx, rcx
jmp +0x32B04
jmp +0xA68
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11, [rdi]
mov rcx, [rdi+0x08]
add r11, rcx
jmp -0x6B068
mov [rdi+0x08], r11
bswap r11
movzx r11d, r15w
pushfq
shl r11b, cl
clc
pop [rdi]
btr r11, r15
shl r11d, cl
mov r11d, [r10]
test cl, r15b
cmp r10w, r11w
add r10, 0x04
jmp +0x2C698
xor r11d, r8d
inc r11d
jmp -0x93657
rol r11d, 0x01
stc
xor r11d, 0x29811801
not r11d
test dh, ah
neg r11d
test r8b, r8b
cmc
bswap r11d
cmp rsp, 0x4F7321C5
stc
push r8
xor [rsp], r11d
rcr r8b, cl
shr r8b, 0x76
pop r8
cmp r13b, dil
jmp +0x46032
movsxd r11, r11d
cmp r8b, 0xB0
clc
add rbx, r11
jmp -0x57EFB
jmp rbx
mov rsi, [rdi]
sub dl, sil
add rdi, 0x08
movzx edx, byte ptr [r10]
btr ax, 0xA5
sub al, dil
movsx ax, r11b
add r10, 0x01
xor dl, r8b
dec dl
sub rax, 0x326E5F75
neg dl
add dl, 0x21
neg dl
dec dl
adc ax, 0x14A7
not dl
xor r8b, dl
xadd ax, ax
movsx rax, dx
mov [rsp+rdx*1], rsi
shr ax, 0xC7
shl al, 0x94
mov eax, [r10]
test r13b, 0x6A
add r10, 0x04
xor eax, r8d
rol eax, 0x01
jmp -0xE131
dec eax
test spl, 0x2E
stc
bswap eax
cmp al, r14b
stc
neg eax
cmp r10b, 0xB5
test bpl, r15b
push r8
xor [rsp], eax
sub r8w, 0x6EE3
adc r8b, r9b
xadd r8, r8
pop r8
jmp +0x2660
movsxd rax, eax
add rbx, rax
push rbx
ret
mov rax, [rdi]
mov r11d, [rax]
test edi, ecx
cmc
test r15d, 0x567344A1
add rdi, 0x04
mov [rdi], r11d
or r11b, r11b
mov r11d, [r10]
test r9, 0x33824DA2
add r10, 0x04
test r13d, 0x413925E2
stc
jmp -0x510AD
xor r11d, r8d
ror r11d, 0x02
test si, 0x6F50
jmp +0xDEE14
add r11d, 0x5AFC075D
cmc
bswap r11d
clc
rol r11d, 0x01
not r11d
cmp sil, 0xFE
test r9, 0x405D6D26
push r8
xor [rsp], r11d
movzx r8w, bl
pop r8
movsxd r11, r11d
add rbx, r11
jmp -0xC4D6D
jmp rbx
mov ebp, [rdi]
shrd r9d, ebx, 0xF9
ror r11b, 0xB3
xchg r9, r9
add rdi, 0x04
movsx r9, r9w
bt r9w, 0x85
movzx r9d, byte ptr [r10]
btc r11w, 0x77
add r10, 0x01
shld r11, rcx, 0x52
bts r11w, bp
setz r11b
xor r9b, r8b
adc r11d, ebp
shl r11, 0x09
rcl r11b, 0x64
xor r9b, 0x31
shl r11b, cl
adc r11b, 0x88
add r9b, 0xEE
movzx r11w, r11b
movzx r11d, dx
bswap r11d
ror r9b, 0x01
movsxd r11, edi
bswap r11d
dec r9b
not r11d
rcl r11b, 0x66
ror r11, cl
xor r8b, r9b
mov [rsp+r9*1], ebp
bswap r11w
cmp r13b, 0xBC
btc r11, 0x53
mov r11d, [r10]
cmp sil, r13b
stc
jmp -0x3A42F
add r10, 0x04
cmc
cmp ch, 0x4E
xor r11d, r8d
jmp -0x62974
dec r11d
jmp +0x52685
ror r11d, 0x03
clc
neg r11d
jmp -0x278B7
inc r11d
test r9b, 0x84
push r8
bsr r8w, r15w
xor [rsp], r11d
not r8w
shl r8b, 0x6F
cmc
pop r8
clc
movsxd r11, r11d
add rbx, r11
jmp +0x44CB6
jmp rbx
movzx esi, byte ptr [r10]
movsx eax, r11w
add r10, 0x01
xor sil, r8b
neg sil
inc al
inc sil
bt edx, 0x78
clc
ror sil, 0x01
rdtsc
movzx dx, r10b
cwde
dec sil
movzx dx, al
neg sil
rdtsc
xor r8b, sil
btc ax, 0x6E
mov eax, [rsp+rsi*1]
movzx edx, r13w
btc rdx, r10
movsx dx, dl
sub rdi, 0x04
mov rdx, r12
mov [rdi], eax
mov edx, [r10]
cmp al, 0x5C
test cx, 0x3EBA
add r10, 0x04
cmc
stc
xor edx, r8d
neg edx
xor edx, 0x4A271450
jmp -0x3558A
rol edx, 0x02
test r15b, 0x60
jmp +0x4460B
add edx, 0x764814F6
cmc
push r8
bswap r8d
mov r8w, 0x2E03
xor [rsp], edx
mov r8w, r10w
pop r8
cmc
stc
movsxd rdx, edx
stc
cmp r11w, 0x33C9
add rbx, rdx
jmp +0x33E4
jmp +0x7C97E
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx esi, byte ptr [r10]
xadd dx, dx
cwde
add r10, 0x01
movzx rdx, r12w
xor sil, r8b
add dh, 0x1D
or dx, di
neg sil
movzx eax, r9w
inc sil
rdtsc
ror sil, 0x01
bswap rax
cdqe
cmovle dx, cx
dec sil
adc dx, 0x3A36
cmc
cdq
neg sil
shrd rax, r14, 0x4A
xor r8b, sil
mov eax, [rsp+rsi*1]
movsx dx, r15b
movsxd rdx, r10d
sub rdi, 0x04
shrd dx, r13w, 0xDF
inc edx
or dl, r8b
mov [rdi], eax
btr edx, 0x04
btc dx, r14w
mov edx, [r10]
cmc
test bl, 0x4A
stc
add r10, 0x04
xor edx, r8d
cmp r13b, r13b
neg edx
cmp r12w, si
xor edx, 0x4A271450
jmp +0x3F475
rol edx, 0x02
add edx, 0x764814F6
cmp r14b, 0x45
clc
push r8
and r8d, edi
sar r8b, cl
rcr r8d, cl
xor [rsp], edx
shl r8d, cl
sar r8b, cl
btr r8d, ebx
pop r8
clc
movsxd rdx, edx
add rbx, rdx
jmp +0x3B0F1
jmp +0x529F4
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11d, [rdi]
bswap rdx
not cl
sar ecx, 0x28
mov ecx, [rdi+0x04]
cmovnp rdx, r8
cqo
add dx, r9w
sub rdi, 0x04
add r11d, ecx
mov [rdi+0x08], r11d
pushfq
rcl dl, 0x6D
adc dx, 0x699E
pop [rdi]
bts dx, r12w
movsx edx, r11w
mov edx, [r10]
cmp r15b, 0x1D
add r10, 0x04
stc
xor edx, r8d
clc
test sp, 0x6E12
neg edx
ror edx, 0x03
test r12w, si
test cl, 0x16
neg edx
jmp -0xB45EA
add edx, 0x2798110A
stc
jmp +0x91D0E
push r8
xor [rsp], edx
cmc
add r8b, 0xDE
pop r8
test dl, dh
clc
stc
movsxd rdx, edx
test r9b, r10b
cmp si, 0x449A
add rbx, rdx
jmp -0x5882C
jmp +0x87F2D
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
lahf
add rdx, r8
xor dx, 0x3760
add rdi, 0x08
cwde
shl dx, 0x22
movzx edx, byte ptr [r10]
lahf
add r10, 0x01
cmp bpl, dil
mov rax, 0x7BE03384
btr rax, 0x9E
xor dl, r8b
cmovnbe ax, r15w
bswap eax
dec dl
movzx rax, r14w
neg dl
add dl, 0x21
shrd ax, r8w, 0x81
lahf
movsxd rax, edi
neg dl
dec dl
cmp dil, dil
or al, r9b
not dl
add al, r9b
not al
xor r8b, dl
rcl eax, cl
mov [rsp+rdx*1], rsi
bts rax, rdx
cbw
mov eax, [r10]
cmp sil, 0xAB
add r10, 0x04
xor eax, r8d
stc
jmp +0x4B20A
rol eax, 0x01
dec eax
test r13b, r13b
clc
bswap eax
cmp r12b, 0x0F
test r15w, 0x7B8F
test rax, r14
neg eax
push r8
rcl r8w, cl
xor [rsp], eax
dec r8
shl r8b, cl
pop r8
movsxd rax, eax
add rbx, rax
jmp +0x80B7D
jmp rbx
mov ebp, [rdi]
add rdi, 0x04
rol r9w, 0x02
movzx r9d, byte ptr [r10]
add r10, 0x01
test rbx, r8
bsr r11d, r13d
shr r11b, 0xE2
xor r9b, r8b
sbb r11b, r13b
xor r9b, 0x31
adc r11w, 0x407A
add r11b, 0xEF
add r9b, 0xEE
cmovz r11d, r9d
movzx r11, cx
ror r9b, 0x01
movzx r11w, r9b
movzx r11d, sp
movsx r11, r15w
dec r9b
cmovnl r11w, r9w
xor r11b, r9b
cmp r15, r10
xor r8b, r9b
cmc
mov r11b, spl
add r11d, esi
mov [rsp+r9*1], ebp
mov r11d, [r10]
stc
test r8b, 0x71
add r10, 0x04
stc
test spl, spl
test dil, 0xE0
xor r11d, r8d
jmp -0x64A48
dec r11d
jmp -0x499A
ror r11d, 0x03
stc
clc
test dil, 0x2F
neg r11d
jmp +0x2B202
inc r11d
cmp r12, r11
push r8
xadd r8, r8
xor [rsp], r11d
rcl r8b, cl
sbb r8b, spl
pop r8
stc
test dil, bpl
movsxd r11, r11d
test bl, spl
add rbx, r11
jmp -0x21FA4
push rbx
ret
mov r9d, [r10]
cmovnz rax, rsi
bt ax, 0x50
add r10, 0x04
bt ax, 0xEA
inc ax
rol al, 0xE6
xor r9d, r8d
inc r9d
movzx ax, dil
movzx eax, sp
ror r9d, 0x02
rcl ax, 0xA7
movsxd rax, ecx
neg r9d
shl ax, 0x9E
bts rax, r13
add r9d, 0x31114D8A
movsx ax, r12b
dec al
stc
push r8
xor al, 0x4B
rcr rax, cl
xor [rsp], r9d
dec ax
pop r8
jmp -0x3D705
sub rdi, 0x04
bswap ax
btr eax, 0xBC
movzx ax, r9b
mov [rdi], r9d
not al
mov ax, r15w
mov eax, [r10]
cmp r9, 0x2F57547B
cmc
add r10, 0x04
xor eax, r8d
add eax, 0x38A453FB
clc
cmc
ror eax, 0x03
cmp bpl, r15b
sub eax, 0xF9677CA
test rsp, r15
not eax
cmc
cmp r15b, 0xEB
push r8
sub r8b, 0x29
xor [rsp], eax
bsr r8d, ebp
pop r8
movsxd rax, eax
stc
cmp r14d, 0x67DC4C61
add rbx, rax
jmp +0x4912A
jmp +0x5039D
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
adc r11b, cl
btr r9d, ebx
add rdi, 0x04
shr r11, 0xD4
add r9b, r11b
adc r9w, 0x4142
movzx r9d, byte ptr [r10]
add r11, r9
mov r11b, r9b
adc r11b, 0x02
add r10, 0x01
xadd r11w, r11w
movzx r11d, r9w
xchg r11b, r11b
xor r9b, r8b
dec r11d
xor r9b, 0x31
or r11b, al
and r11d, 0x99F3DC9
clc
add r9b, 0xEE
btc r11w, si
clc
ror r9b, 0x01
movsx r11d, r8w
not r11
mov r11w, 0x5B38
dec r9b
btc r11d, 0x70
bsf r11w, r10w
mov r11w, 0x3F9F
xor r8b, r9b
mov [rsp+r9*1], ebp
ror r11b, cl
mov r11d, [r10]
add r10, 0x04
jmp +0xA3C6A
xor r11d, r8d
jmp -0xC2FC3
dec r11d
cmc
clc
ror r11d, 0x03
jmp +0x50A9E
neg r11d
jmp +0x1FC3B
inc r11d
cmc
test r12b, r11b
push r8
bts r8w, cx
xor [rsp], r11d
shl r8w, 0x76
pop r8
cmp rcx, 0x65004A4A
movsxd r11, r11d
test r14b, sil
cmp r13w, r10w
add rbx, r11
jmp +0x2A4D7
push rbx
ret
mov r9d, [r10]
rcr ah, cl
cbw
sub ah, bh
add r10, 0x04
shl ah, 0x3E
xor r9d, r8d
inc r9d
rcr al, cl
ror r9d, 0x02
neg r9d
shld ax, dx, 0x7E
add r9d, 0x31114D8A
cmc
push r8
cbw
xor [rsp], r9d
pop r8
sub rdi, 0x04
and ax, r15w
xor ax, r12w
lahf
mov [rdi], r9d
add ax, 0x5FA1
mov eax, [r10]
cmp ecx, r9d
add r10, 0x04
xor eax, r8d
add eax, 0x38A453FB
ror eax, 0x03
cmc
stc
cmp sil, bl
sub eax, 0xF9677CA
stc
test r9b, r14b
not eax
cmp r13w, r9w
push r8
xor [rsp], eax
mov r8w, cx
bsr r8, rbp
pop r8
cmc
jmp -0x9F8F4
movsxd rax, eax
test r14b, dl
stc
clc
add rbx, rax
jmp +0x800EA
jmp +0x34CBD
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
add rdi, 0x04
movzx r9d, byte ptr [r10]
bt r11d, 0x44
add r10, 0x01
sub r11, 0x43703C58
xor r9b, r8b
rcl r11b, cl
ror r11b, cl
xor r9b, 0x31
cmp r11b, r15b
add r9b, 0xEE
ror r9b, 0x01
dec r9b
sbb r11d, 0x4F7A6A07
and r11, 0x7AAF15EE
xor r8b, r9b
rcr r11w, cl
setp r11b
mov [rsp+r9*1], ebp
mov r11d, [r10]
add r10, 0x04
stc
xor r11d, r8d
dec r11d
ror r11d, 0x03
test sil, 0xFB
stc
clc
neg r11d
jmp +0xBF02
inc r11d
cmc
stc
cmp r12d, esi
push r8
movzx r8w, r11b
btr r8, 0xB1
xor [rsp], r11d
stc
or r8b, al
btc r8w, r15w
pop r8
cmp r14b, spl
jmp -0xA65F0
movsxd r11, r11d
add rbx, r11
jmp +0xA7B87
push rbx
ret
movzx r11d, byte ptr [r10]
add r10, 0x01
or ah, dl
xor r11b, r8b
neg r11b
shl ch, 0x88
bswap ax
add r11b, 0x3E
shrd ax, sp, 0xF4
bt ax, cx
xor ch, 0x68
xor r11b, 0xC9
inc r11b
xor r8b, r11b
mov rax, [rsp+r11*1]
movzx cx, al
shl ecx, 0xEC
sub rdi, 0x08
mov [rdi], rax
mov ecx, [r10]
cmc
test sil, bl
jmp +0x2FB42
add r10, 0x04
cmp r9b, 0xC9
test spl, 0x47
xor ecx, r8d
clc
test r8b, spl
neg ecx
cmc
sub ecx, 0x50D714E5
jmp -0x1DEA7
ror ecx, 0x01
dec ecx
clc
stc
rol ecx, 0x02
jmp +0x14CAA
dec ecx
clc
rol ecx, 0x01
jmp -0x47588
dec ecx
cmp si, 0x5E93
push r8
or r8b, 0x95
sub r8w, 0x6585
xor [rsp], ecx
xor r8b, 0x92
mov r8b, 0x1E
pop r8
test di, bx
cmp r11b, r14b
clc
movsxd rcx, ecx
test r10b, 0xC7
test r12b, r13b
add rbx, rcx
jmp +0x723CC
jmp +0x30EA7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rax, [r10]
bt ecx, 0x2F
shrd rcx, rdi, 0x9C
ror ch, cl
add r10, 0x08
cmovp cx, r13w
sar cx, cl
shrd rcx, rsi, 0x06
xor rax, r8
dec rax
movsxd rcx, r15d
rol rax, 0x01
dec ecx
not rax
mov cx, r13w
inc rax
shr cl, cl
neg rax
btc cx, 0xBE
rcr cl, cl
xadd cl, cl
xor r8, rax
sub rdi, 0x08
xor cx, 0x32F5
cmp r11d, 0x3ECE6EDC
neg cx
mov [rdi], rax
dec cl
mov ecx, [r10]
cmp spl, r13b
cmc
add r10, 0x04
test cx, bp
cmp cx, sp
clc
xor ecx, r8d
rol ecx, 0x03
cmc
bswap ecx
stc
sub ecx, 0x6F335F3
clc
not ecx
jmp -0x88279
rol ecx, 0x01
push r8
xor [rsp], ecx
pop r8
cmp r13b, r12b
movsxd rcx, ecx
cmp r13b, r12b
add rbx, rcx
jmp +0x5D986
jmp +0x589D7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx esi, byte ptr [r10]
add r10, 0x01
clc
xor sil, r8b
or dh, 0x22
bsf ax, r10w
neg sil
bswap ax
inc sil
ror sil, 0x01
cmovnp edx, r11d
not dx
dec sil
neg sil
bts ax, di
xor r8b, sil
mov eax, [rsp+rsi*1]
shld edx, r15d, 0x86
sub rdi, 0x04
shl dl, cl
mov [rdi], eax
btr dx, cx
cwd
mov edx, [r10]
test r10d, r9d
add r10, 0x04
cmp r12, 0x195D3C09
test rbp, r15
xor edx, r8d
neg edx
stc
cmp r10w, 0x3D02
xor edx, 0x4A271450
rol edx, 0x02
add edx, 0x764814F6
push r8
neg r8
xor [rsp], edx
shl r8b, cl
and r8w, dx
pop r8
movsxd rdx, edx
clc
test rbx, r9
add rbx, rdx
jmp +0x26868
jmp +0x6C101
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
movsx r11d, r15w
shrd r11w, cx, 0xBA
add rdi, 0x04
ror r11d, cl
xchg r9, r11
movzx r9d, byte ptr [r10]
add r10, 0x01
clc
rol r11w, 0x7B
shrd r11w, ax, 0xE6
xor r9b, r8b
movzx r11, r9w
dec r11b
not r11b
xor r9b, 0x31
adc r11d, esp
bsr r11w, r8w
ror r11b, 0x6F
add r9b, 0xEE
jmp -0x8638E
ror r9b, 0x01
setnp r11b
mov r11d, 0x43684035
movzx r11w, r11b
dec r9b
dec r11d
xor r11d, 0x57986129
xor r8b, r9b
bt r11w, 0x51
mov [rsp+r9*1], ebp
mov r11d, [r10]
cmp r13, 0x4CC95C08
add r10, 0x04
clc
test r12b, bl
xor r11d, r8d
jmp +0x38606
dec r11d
ror r11d, 0x03
test r9w, 0x6D7C
cmp r8w, 0x7662
neg r11d
inc r11d
test r13w, dx
clc
push r8
xor [rsp], r11d
clc
or r8, rbx
rcr r8b, cl
pop r8
cmc
clc
movsxd r11, r11d
clc
cmp di, 0x4D72
add rbx, r11
jmp rbx
mov r9d, [r10]
adc al, r12b
add r10, 0x04
bsr ax, di
and rax, 0x32A22A68
movzx eax, r11w
xor r9d, r8d
movsxd rax, ebx
setbe ah
inc r9d
rcl al, 0x14
ror r9d, 0x02
btr ax, 0x26
not rax
btc rax, 0x74
neg r9d
add r9d, 0x31114D8A
sub al, r14b
shl ax, cl
rcl eax, 0xFA
push r8
and al, 0x4A
xor [rsp], r9d
not rax
test r8w, r12w
rcr al, cl
pop r8
movsx rax, sp
ror ax, 0x87
sub rdi, 0x04
cdqe
ror rax, cl
bsf eax, r15d
mov [rdi], r9d
shr rax, cl
mov eax, [r10]
jmp -0xA7173
add r10, 0x04
xor eax, r8d
add eax, 0x38A453FB
stc
jmp +0x32AA6
ror eax, 0x03
test bl, bl
sub eax, 0xF9677CA
stc
not eax
cmp eax, edi
stc
push r8
shl r8w, 0x09
not r8b
cmovp r8, rdx
xor [rsp], eax
bt r8, rdi
sbb r8d, r13d
pop r8
test r9b, spl
cmc
stc
movsxd rax, eax
cmp al, 0x16
clc
add rbx, rax
jmp +0x6AE42
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
rcr r11b, cl
add rdi, 0x04
and r9b, bl
btc r11w, r9w
movzx r9d, byte ptr [r10]
add r10, 0x01
xor r9b, r8b
movsxd r11, r12d
xor r9b, 0x31
rcr r11, cl
xor r11d, esp
add r9b, 0xEE
btc r11d, 0x9B
cmc
rcl r11, cl
ror r9b, 0x01
dec r9b
cmp r11d, 0x61E70565
xor r8b, r9b
mov [rsp+r9*1], ebp
sub r11, rdi
mov r11d, [r10]
test r13b, 0x8B
cmp dil, 0xA6
add r10, 0x04
test r15b, 0x8A
xor r11d, r8d
jmp -0x34CD4
dec r11d
cmc
clc
ror r11d, 0x03
jmp -0x168C1
neg r11d
jmp -0x5222C
inc r11d
push r8
sub r8w, r12w
btr r8w, 0x08
xor [rsp], r11d
shrd r8, rax, 0x0E
shl r8w, 0xB0
pop r8
stc
test r15b, 0x68
movsxd r11, r11d
cmc
test dx, r12w
add rbx, r11
jmp +0x4C82
push rbx
ret
mov r11, [rdi]
movsxd rcx, r13d
sbb cl, r15b
mov rcx, [rdi+0x08]
stc
add r11, rcx
jmp +0x56923
mov [rdi+0x08], r11
movzx r11w, al
pushfq
pop [rdi]
shl r11d, 0x09
mov r11d, [r10]
clc
add r10, 0x04
cmc
test sp, 0x37C4
xor r11d, r8d
inc r11d
cmc
rol r11d, 0x01
test r15w, r8w
xor r11d, 0x29811801
not r11d
jmp +0x5E43A
neg r11d
test r13b, r14b
cmp bpl, 0x10
bswap r11d
jmp -0x4C2CC
push r8
movsx r8d, ax
sar r8w, cl
xor [rsp], r11d
rcl r8b, cl
btc r8w, di
btc r8d, 0xE7
pop r8
test bh, 0xAC
movsxd r11, r11d
jmp +0x2570C
add rbx, r11
jmp -0x33755
push rbx
ret
mov rsi, [rdi]
sbb dx, r10w
add rdi, 0x08
cwd
bsf ax, r9w
shl dx, 0xCF
movzx edx, byte ptr [r10]
dec rax
neg ah
add r10, 0x01
xor eax, 0x1A09129F
xor dl, r8b
not eax
dec dl
neg dl
ror ah, 0xC8
rol al, 0x53
add dl, 0x21
cmp dil, 0x0D
shl al, 0xD3
cdqe
neg dl
inc ax
movzx ax, r10b
dec dl
rcl ax, cl
stc
btc ax, 0x00
not dl
shr al, cl
mov al, 0x2F
rcl ax, 0xC3
xor r8b, dl
clc
mov [rsp+rdx*1], rsi
adc ah, ah
btc ax, cx
mov eax, [r10]
add r10, 0x04
xor eax, r8d
jmp -0x9BAD0
rol eax, 0x01
jmp +0x4DFD7
dec eax
clc
bswap eax
cmp dil, 0xDF
cmp r14d, r9d
jmp +0x579F5
neg eax
jmp -0x61899
push r8
xor [rsp], eax
dec r8w
clc
pop r8
movsxd rax, eax
cmc
add rbx, rax
jmp +0x1867D
push rbx
ret
mov rax, [rdi]
ror r11b, 0x2B
cmc
shr r11b, cl
mov r11d, [rax]
add rdi, 0x04
mov [rdi], r11d
shl r11w, cl
mov r11d, [r10]
add r10, 0x04
cmp dil, bl
jmp -0x6C090
xor r11d, r8d
cmc
clc
ror r11d, 0x02
test r12, 0x1B1F2BEC
add r11d, 0x5AFC075D
stc
bswap r11d
rol r11d, 0x01
test r10, 0x7ADF760D
cmp sp, sp
not r11d
test r15b, 0x2B
cmc
test dil, cl
push r8
and r8w, 0x7DFD
movsx r8d, r15w
sbb r8w, 0x76EF
xor [rsp], r11d
pop r8
jmp +0x42D6B
movsxd r11, r11d
add rbx, r11
jmp +0x6228A
push rbx
ret
movzx esi, byte ptr [r10]
movsx dx, r10b
rol ah, cl
or dx, 0x6595
add r10, 0x01
xor sil, r8b
movzx dx, r13b
neg sil
movsxd rdx, ecx
inc sil
ror sil, 0x01
bswap rax
not dh
dec sil
shl edx, cl
sub edx, r8d
movzx dx, bpl
neg sil
btc edx, r11d
xor r8b, sil
xor dh, 0xE0
mov eax, [rsp+rsi*1]
shl dl, cl
sub rdi, 0x04
and dh, 0x0B
ror dl, cl
mov [rdi], eax
test bpl, 0x3C
movsx rdx, r12w
mov edx, [r10]
clc
test sil, dl
add r10, 0x04
clc
cmp r8b, 0x15
xor edx, r8d
neg edx
cmp esi, 0xE4E362D
xor edx, 0x4A271450
clc
rol edx, 0x02
test r9b, 0x4E
test cl, r12b
add edx, 0x764814F6
push r8
neg r8b
sub r8d, edi
xor [rsp], edx
pop r8
jmp -0xC2A9C
movsxd rdx, edx
test sp, 0x7585
add rbx, rdx
jmp +0x8E906
jmp +0x1DA92
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11d, [rdi]
mov ecx, [rdi+0x04]
sub rdi, 0x04
add r11d, ecx
movsxd rdx, r11d
mov dx, 0x3A24
bswap rdx
mov [rdi+0x08], r11d
movzx rdx, bx
pushfq
pop [rdi]
shr dl, cl
shld dx, bp, 0x25
sub edx, esp
mov edx, [r10]
test r11w, r12w
add r10, 0x04
cmp r13d, 0x7EFD3087
clc
xor edx, r8d
stc
cmc
neg edx
jmp -0xA3B92
ror edx, 0x03
neg edx
cmp r13b, 0xF1
add edx, 0x2798110A
test r14b, dil
cmp r13w, 0x377E
push r8
bts r8w, r14w
stc
sar r8b, cl
xor [rsp], edx
movzx r8w, r9b
pop r8
jmp +0x11208
movsxd rdx, edx
cmp r13b, 0xD4
cmp r8w, bx
jmp -0x32A37
add rbx, rdx
jmp +0xC26B0
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
cdq
rol dl, 0xFE
bt dx, r9w
add rdi, 0x08
and dx, 0x6F10
adc dl, 0x8E
movzx edx, byte ptr [r10]
bts eax, r12d
clc
add r10, 0x01
jmp +0x367A6
xor dl, r8b
dec dl
neg dl
and ah, 0x37
movsxd rax, ebx
not al
add dl, 0x21
neg dl
movsx rax, r9w
dec dl
btc ax, di
nop
not dl
bsr ax, r8w
stc
bts rax, r13
xor r8b, dl
sar al, 0x1D
cmp dl, 0x07
dec ah
mov [rsp+rdx*1], rsi
bts ax, r10w
shl al, cl
mov eax, [r10]
test rsi, 0x607F51DE
add r10, 0x04
cmp di, dx
cmp r10b, 0x6E
xor eax, r8d
stc
clc
rol eax, 0x01
dec eax
bswap eax
test sil, sil
cmp di, 0x7985
neg eax
cmp r12b, 0x4A
push r8
adc r8b, 0x2C
movsx r8w, r15b
xor [rsp], eax
pop r8
test r9, rcx
clc
test ebx, 0x4BB60E5D
movsxd rax, eax
clc
add rbx, rax
jmp -0x47DDC
jmp rbx
mov ebp, [rdi]
movsx r9, ax
sbb r11w, 0x4828
not r9d
add rdi, 0x04
bts r9w, bp
bswap r9w
xor r11w, 0x4D17
movzx r9d, byte ptr [r10]
add r10, 0x01
or r11w, si
test r9, 0x16771753
xor r9b, r8b
rcl r11b, 0xBD
xor r9b, 0x31
rol r11b, 0xA1
add r9b, 0xEE
bts r11d, r11d
movzx r11w, bl
bts r11d, esp
ror r9b, 0x01
movsxd r11, r8d
movsx r11d, r9w
jmp +0x12645
dec r9b
movsx r11, sp
rol r11b, 0xE7
xor r8b, r9b
adc r11w, 0x123A
mov [rsp+r9*1], ebp
bsf r11w, r9w
movsxd r11, edx
mov r11d, [r10]
stc
add r10, 0x04
cmp r12w, 0x208F
xor r11d, r8d
jmp -0x5E273
dec r11d
ror r11d, 0x03
cmp sp, 0x6192
cmc
neg r11d
jmp +0x4849F
inc r11d
test eax, 0x241628D7
push r8
jmp +0x7EB87
xor [rsp], r11d
test r10b, dl
setz r8b
shl r8, cl
pop r8
stc
cmp dil, 0x4F
test di, 0x3973
movsxd r11, r11d
add rbx, r11
jmp rbx
mov r9d, [r10]
add r10, 0x04
adc al, r8b
xor r9d, r8d
inc r9d
ror r9d, 0x02
cmp r11b, r9b
bt ax, bx
ror al, 0x79
neg r9d
movzx rax, r12w
sar ah, cl
rcl ax, cl
add r9d, 0x31114D8A
shl ax, cl
shr al, cl
push r8
movsx r8w, cl
xor [rsp], r9d
inc r8b
pop r8
bsf ax, r9w
adc eax, r9d
sub rdi, 0x04
cmc
bswap ax
btc ax, 0x47
mov [rdi], r9d
mov eax, [r10]
stc
add r10, 0x04
cmc
test sil, 0xB4
test r12w, r8w
xor eax, r8d
cmp r8w, 0x5359
clc
add eax, 0x38A453FB
ror eax, 0x03
sub eax, 0xF9677CA
stc
not eax
push r8
stc
ror r8b, 0xFC
xor [rsp], eax
pop r8
cmc
cmp r11d, r11d
movsxd rax, eax
add rbx, rax
jmp -0x2A4FA
jmp +0xA3E2
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov ebp, [rdi]
movsxd r9, r11d
shr r9b, cl
neg r9b
add rdi, 0x04
rol r9b, 0xD5
and r9b, bpl
adc r11b, 0x2D
movzx r9d, byte ptr [r10]
add r10, 0x01
xor r9b, r8b
xor r9b, 0x31
shr r11b, 0x9A
add r11w, cx
add r9b, 0xEE
rol r11b, cl
cmovns r11, r11
movsxd r11, r12d
ror r9b, 0x01
movzx r11d, r12w
dec r9b
movsx r11w, bpl
neg r11b
xor r8b, r9b
mov [rsp+r9*1], ebp
sar r11w, cl
mov r11d, [r10]
stc
add r10, 0x04
xor r11d, r8d
dec r11d
ror r11d, 0x03
neg r11d
inc r11d
cmc
push r8
movsx r8w, r9b
xor [rsp], r11d
movzx r8w, bl
add r8b, sil
ror r8b, cl
pop r8
movsxd r11, r11d
add rbx, r11
jmp -0x2D91F
push rbx
ret
movzx r11d, byte ptr [r10]
shr ah, cl
sbb cl, r13b
movsx rcx, r14w
add r10, 0x01
sbb cl, 0x56
xor r11b, r8b
and cl, 0x99
sub cx, cx
neg r11b
shl ecx, cl
xadd ax, ax
sbb cx, cx
add r11b, 0x3E
lahf
movzx ax, dil
bsf ax, r9w
xor r11b, 0xC9
movsx eax, r10w
movsx rax, di
inc r11b
movsx rax, bx
cwde
xor r8b, r11b
not ax
mov rax, [rsp+r11*1]
mov cl, sil
btc cx, r10w
mov rcx, 0x3EA52C29
sub rdi, 0x08
btr ecx, r12d
adc cl, spl
movsx ecx, r13w
mov [rdi], rax
bts rcx, r10
mov ecx, [r10]
cmc
add r10, 0x04
jmp -0x4C9BB
xor ecx, r8d
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp +0x378B6
dec ecx
rol ecx, 0x02
jmp +0x95EBB
dec ecx
stc
cmc
rol ecx, 0x01
jmp -0x7CEC4
dec ecx
test bl, r14b
test r9d, 0x42633C06
push r8
rol r8d, 0x83
sub r8b, 0xD0
btc r8w, r14w
xor [rsp], ecx
rcr r8, cl
pop r8
movsxd rcx, ecx
add rbx, rcx
jmp -0x5C6C4
jmp +0xC79E1
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rax, [r10]
xadd ch, ch
shr ch, cl
add r10, 0x08
sar rcx, 0x76
movzx ecx, si
shr ch, cl
xor rax, r8
dec rax
not cx
btc cx, di
rol rax, 0x01
not rax
movzx rcx, r10w
not ch
inc rax
shld cx, si, 0xAB
ror rcx, cl
neg rax
xor r8, rax
or rcx, r8
movzx rcx, r11w
sub rdi, 0x08
mov rcx, r15
mov [rdi], rax
movzx rcx, ax
adc cx, 0x165F
mov ecx, [r10]
clc
add r10, 0x04
cmc
xor ecx, r8d
stc
rol ecx, 0x03
bswap ecx
test r13b, 0x7B
cmp r8d, eax
cmp r15b, 0x03
sub ecx, 0x6F335F3
jmp -0x167DC
not ecx
jmp -0x32821
rol ecx, 0x01
push r8
xor [rsp], ecx
pop r8
cmp sil, 0xD7
jmp +0x5B939
movsxd rcx, ecx
add rbx, rcx
jmp -0x47DCD
jmp +0xBE76D
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov r11, [rdi]
mov rcx, [rdi+0x08]
jmp +0x7B862
add r11, rcx
jmp -0xC8A55
mov [rdi+0x08], r11
xchg r11b, r11b
movsx r11w, spl
setnbe r11b
pushfq
mov r11b, spl
and r11b, 0xB4
pop [rdi]
xadd r11b, r11b
mov r11d, [r10]
cmp r14d, r8d
cmp ax, 0x4A2D
stc
add r10, 0x04
cmp r14b, r9b
cmc
cmp dil, 0xB5
xor r11d, r8d
inc r11d
clc
jmp +0xC3C1D
rol r11d, 0x01
stc
xor r11d, 0x29811801
test r9w, di
not r11d
cmp rbx, 0x45820D5B
test edi, r12d
neg r11d
test r15, 0x3EB37F9D
clc
bswap r11d
test ebx, 0x8AB654D
push r8
rcl r8b, cl
xor [rsp], r11d
bswap r8w
cmc
pop r8
movsxd r11, r11d
clc
test r13w, r8w
stc
add rbx, r11
jmp -0xC1C7
push rbx
ret
mov rsi, [rdi]
xadd dl, dh
bswap dx
movzx rdx, si
add rdi, 0x08
bt dx, 0x52
movzx edx, byte ptr [r10]
btr eax, 0x91
add r10, 0x01
btc ax, r12w
xor dl, r8b
dec dl
shr ah, cl
movsx rax, r15w
xadd ah, ah
neg dl
bts eax, r9d
lahf
shr rax, 0x93
add dl, 0x21
sub al, r12b
neg dl
dec ax
setnle ah
movsx ax, sil
dec dl
xchg ah, al
adc al, 0x2B
stc
not dl
btr ax, ax
setl al
xor r8b, dl
mov ax, bx
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp spl, dl
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
jmp +0xDF88
dec eax
jmp +0x864E4
bswap eax
test r12b, spl
neg eax
cmp r11w, 0x2A8
push r8
xor [rsp], eax
test r8b, r9b
shr r8b, cl
sbb r8w, di
pop r8
movsxd rax, eax
clc
test r12b, r9b
add rbx, rax
jmp rbx
mov rdi, [rdi]
sbb al, r11b
btc ax, 0x94
mov eax, [r10]
clc
add r10, 0x04
cmp r8b, r10b
xor eax, r8d
jmp +0x22C70
inc eax
cmc
clc
rol eax, 0x01
cmp edx, r11d
add eax, 0x33068D4
neg eax
test r10d, r15d
clc
test ebx, 0x37D05F8F
push r8
xor [rsp], eax
movsxd r8, edi
cmp rsp, r14
pop r8
cmc
movsxd rax, eax
add rbx, rax
jmp +0xB60A6
jmp -0x302E
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, rdi
sub rdi, 0x08
mov [rdi], rsi
cwd
mov edx, [r10]
test rdx, 0x3C681FBF
add r10, 0x04
xor edx, r8d
stc
jmp +0x22716
neg edx
jmp -0x50582
bswap edx
jmp +0xCA2F2
not edx
jmp +0x2B2CC
dec edx
clc
cmp r14b, r11b
cmp r10d, 0x7C1D4F13
bswap edx
push r8
shl r8b, 0x9A
xor [rsp], edx
add r8b, 0x3A
bsr r8, r10
pop r8
jmp -0xBA8B1
movsxd rdx, edx
test rcx, rdi
add rbx, rdx
jmp +0x9CBF1
jmp +0x22CC
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
adc ax, 0x5716
add r10, 0x01
shrd ax, sp, 0x45
movsxd rax, r12d
xor dl, r8b
jmp -0x60C2F
dec dl
stc
dec ah
sbb al, 0x9E
neg dl
movsx eax, ax
movzx rax, si
add dl, 0x21
shl al, cl
movzx ax, dl
neg dl
movzx eax, sp
cbw
dec dl
clc
sar al, cl
not dl
shl rax, cl
cbw
xor r8b, dl
mov [rsp+rdx*1], rsi
mov eax, [r10]
cmp ebp, edx
stc
cmc
add r10, 0x04
xor eax, r8d
stc
jmp +0x7A9F6
rol eax, 0x01
jmp -0x464E6
dec eax
test r12d, ecx
jmp +0x4F3A2
bswap eax
test r10w, 0x4B8E
neg eax
push r8
cmovns r8w, ax
xor [rsp], eax
shl r8w, cl
pop r8
movsxd rax, eax
cmc
clc
cmp spl, bpl
add rbx, rax
jmp -0x5DCA2
jmp rbx
mov rsi, [rdi]
movsx rdx, r8w
add rdi, 0x08
movzx edx, byte ptr [r10]
bts ax, cx
bt ax, sp
add r10, 0x01
xor dl, r8b
dec dl
cmp di, 0x4F79
neg dl
sub eax, 0x5B5466F7
sbb ax, r13w
add dl, 0x21
shr ah, cl
btr eax, r15d
neg dl
cdqe
cwde
dec dl
sub eax, 0x25F47721
not dl
xor r8b, dl
bsf ax, r11w
add al, dl
mov [rsp+rdx*1], rsi
or al, r12b
mov eax, [r10]
cmp r9w, 0x3EF3
add r10, 0x04
xor eax, r8d
jmp -0x654F1
rol eax, 0x01
jmp -0x570DA
dec eax
cmp spl, r10b
bswap eax
neg eax
cmc
test r8, 0x1B484F73
push r8
sets r8b
cmp r11b, dil
jmp +0x782B6
xor [rsp], eax
jmp -0x6A47A
pop r8
stc
clc
movsxd rax, eax
cmc
cmp sil, 0xFD
test sp, r13w
add rbx, rax
jmp +0xC547D
jmp rbx
mov rsi, rdi
sub rdi, 0x08
cdq
sbb dx, r12w
mov [rdi], rsi
cmovnbe edx, r12d
mov edx, [r10]
cmp dil, r12b
add r10, 0x04
jmp -0x3B5C6
xor edx, r8d
neg edx
jmp +0x512
bswap edx
not edx
jmp +0x90C40
dec edx
cmc
jmp -0x9F5BB
bswap edx
clc
cmc
push r8
shl r8b, cl
inc r8w
xor [rsp], edx
shl r8b, cl
xadd r8w, r8w
pop r8
movsxd rdx, edx
cmp ch, 0x91
stc
add rbx, rdx
jmp +0xE614B
jmp -0x1E3A4
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add rdi, 0x08
movzx edx, byte ptr [r10]
shl ax, 0xB3
bt eax, esi
movzx ax, r10b
add r10, 0x01
bt ax, 0x43
sbb ax, 0x26B6
xor dl, r8b
movsx rax, sp
setbe al
dec dl
cmp spl, 0x95
sar rax, 0x0A
neg dl
rcl ax, 0x43
sub eax, r9d
add dl, 0x21
sar al, cl
adc ax, 0x565E
sub eax, ecx
neg dl
dec dl
setnle al
btr ax, 0xEA
not dl
shl ah, cl
xor eax, edi
xor r8b, dl
movsx ax, r11b
mov [rsp+rdx*1], rsi
cmc
shl ax, 0x05
mov eax, [r10]
clc
add r10, 0x04
xor eax, r8d
clc
rol eax, 0x01
dec eax
cmc
bswap eax
neg eax
push r8
cmp r9b, 0x6E
adc r8b, 0x8A
xor [rsp], eax
stc
add r8w, 0x300C
rcl r8b, cl
pop r8
cmc
test r9, r8
jmp -0x97C87
movsxd rax, eax
cmp r11w, r14w
add rbx, rax
jmp +0x3CEA2
jmp rbx
mov rsi, [rdi]
or al, r11b
cmp r14b, 0x11
movsx ax, ch
add rdi, 0x08
bsr eax, r15d
not ax
or dh, 0x23
movzx edx, byte ptr [r10]
mov al, r8b
lahf
add r10, 0x01
xor dl, r8b
xchg al, ah
movzx ax, r9b
movsx eax, r11w
dec dl
stc
neg dl
sub rax, 0x6E482B57
add dl, 0x21
test r15b, 0x7D
inc al
cmc
neg dl
dec dl
ror ah, 0x4D
cdqe
movzx rax, r9w
not dl
shl eax, cl
xor r8b, dl
bsf eax, r13d
mov [rsp+rdx*1], rsi
adc al, bpl
cwde
mov eax, [r10]
stc
add r10, 0x04
cmc
xor eax, r8d
rol eax, 0x01
jmp +0x26F9D
dec eax
stc
bswap eax
jmp -0x2A2A8
neg eax
jmp +0x347CE
push r8
xor [rsp], eax
shl r8b, 0x53
rcr r8b, 0x5C
cmp rdx, rsp
pop r8
movsxd rax, eax
stc
cmp al, r13b
add rbx, rax
jmp rbx
movzx r11d, byte ptr [r10]
bsf ax, r13w
cmp r11b, 0xB3
add r10, 0x01
sub rax, 0x7085651
cdqe
and al, spl
xor r11b, r8b
rcl cl, 0xE2
neg cl
adc al, bpl
neg r11b
not rcx
movzx rax, r13w
shl ecx, 0x8B
add r11b, 0x3E
xor ecx, 0x742A55B6
neg ah
xor r11b, 0xC9
xchg rcx, rcx
inc r11b
rcl eax, 0x8D
sbb al, 0xF5
cdqe
xor r8b, r11b
cdqe
mov rax, [rsp+r11*1]
sub rdi, 0x08
mov [rdi], rax
mov ecx, [r10]
cmc
test r10b, 0x9F
clc
add r10, 0x04
test r14w, 0x9D1
xor ecx, r8d
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
dec ecx
rol ecx, 0x02
dec ecx
stc
clc
cmc
rol ecx, 0x01
jmp -0xC4206
dec ecx
stc
cmp di, r10w
cmp si, 0x788F
push r8
sar r8b, cl
add r8b, 0x6E
sbb r8b, r12b
xor [rsp], ecx
xor r8b, dl
pop r8
cmc
cmp esi, r15d
movsxd rcx, ecx
cmp rdx, r11
add rbx, rcx
jmp +0xAC48
jmp +0xC0DC8
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
add edx, 0x264F6436
add rdi, 0x08
bt dx, r15w
or ah, 0xDC
movzx edx, byte ptr [r10]
add r10, 0x01
btr ax, 0xF8
xor dl, r8b
movzx eax, sp
movsx eax, bp
dec dl
movzx rax, r10w
neg dl
add dl, 0x21
btc ax, r14w
or al, r10b
bt ax, r11w
neg dl
jmp -0x3235F
dec dl
add al, r10b
not dl
ror al, 0x81
rcr ah, cl
xor r8b, dl
xadd ah, al
sbb al, 0x0E
mov [rsp+rdx*1], rsi
cmovno ax, dx
mov eax, [r10]
cmp cl, r10b
jmp +0x3EC46
add r10, 0x04
cmc
clc
stc
xor eax, r8d
cmc
rol eax, 0x01
jmp +0x59918
dec eax
cmp sp, r10w
bswap eax
neg eax
test r10b, 0x3A
cmc
push r8
clc
inc r8b
movsxd r8, r12d
xor [rsp], eax
bswap r8w
add r8w, 0x691B
pop r8
test spl, r8b
movsxd rax, eax
jmp -0x37994
add rbx, rax
jmp +0x4221F
push rbx
ret
mov rsi, rdi
rcl dx, cl
sub rdi, 0x08
rol dx, 0x3E
shr dl, cl
mov [rdi], rsi
stc
mov edx, [r10]
clc
add r10, 0x04
xor edx, r8d
neg edx
jmp +0xC731B
bswap edx
jmp -0xC9CA
not edx
jmp -0x7B345
dec edx
cmp r8w, di
bswap edx
test sp, dx
push r8
sbb r8b, r12b
xor [rsp], edx
cmp r8, rax
btr r8, 0xC3
pop r8
test bpl, bpl
cmp r11b, r13b
movsxd rdx, edx
cmp r11b, r9b
jmp +0x98931
add rbx, rdx
jmp -0x5EE09
jmp +0x490C4
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsi, [rdi]
rol dl, 0x29
movzx dx, r14b
cmp esi, 0x4D751F17
add rdi, 0x08
shl dx, cl
sbb al, r12b
dec ax
movzx edx, byte ptr [r10]
add r10, 0x01
btr rax, 0x3A
btc ax, r11w
xor dl, r8b
dec dl
movsxd rax, ebp
bts ax, r10w
add al, spl
neg dl
shl ax, cl
shl ah, 0x0A
add dl, 0x21
btc ax, r8w
shr ax, cl
movsx eax, sp
neg dl
movzx eax, r8w
dec dl
sar al, cl
cdqe
not dl
not ax
add al, r9b
xor r8b, dl
cbw
cdqe
mov [rsp+rdx*1], rsi
sar ax, 0x5D
test rbp, rax
stc
mov eax, [r10]
stc
test r10b, bl
add r10, 0x04
stc
xor eax, r8d
rol eax, 0x01
jmp +0xA9BD8
dec eax
cmc
bswap eax
cmc
neg eax
cmp di, bp
cmc
push r8
and r8d, 0x1F826CDC
sar r8b, 0x70
movzx r8w, r13b
xor [rsp], eax
xor r8b, 0xAB
btc r8d, 0x8F
cmovle r8d, r10d
pop r8
cmc
movsxd rax, eax
clc
add rbx, rax
jmp -0x94358
jmp rbx
movzx r11d, byte ptr [r10]
shld ecx, r8d, 0xB8
rcl cx, cl
shl ax, cl
add r10, 0x01
dec cx
cwde
xor r11b, r8b
and eax, ebp
add cl, sil
shr eax, cl
neg r11b
add r11b, 0x3E
movzx rax, ax
lahf
xor r11b, 0xC9
inc cx
setnz ch
cmovb eax, esp
inc r11b
adc eax, ecx
xor r8b, r11b
shl cx, 0x0F
or al, spl
mov rax, [rsp+r11*1]
bsr cx, r15w
adc ecx, r8d
shr ch, 0xB4
sub rdi, 0x08
not ch
rol cx, cl
shl ecx, 0xD7
mov [rdi], rax
mov ecx, [r10]
stc
add r10, 0x04
test ecx, 0x6DD153C3
xor ecx, r8d
jmp -0x3239E
neg ecx
cmc
sub ecx, 0x50D714E5
clc
ror ecx, 0x01
jmp -0x3BF04
dec ecx
clc
stc
rol ecx, 0x02
jmp -0x22104
dec ecx
jmp +0x1421C
rol ecx, 0x01
jmp +0xFA8
dec ecx
clc
push r8
xor [rsp], ecx
shl r8b, 0xB4
xor r8w, cx
add r8b, spl
pop r8
cmp r10w, r9w
movsxd rcx, ecx
jmp +0x9D07B
add rbx, rcx
jmp +0x93D3
jmp +0xF81E
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
sar cx, 0x75
cwde
add r10, 0x01
xor r11b, r8b
test r12b, 0x9F
neg r11b
xchg cl, cl
shl ax, cl
movsxd rax, r10d
add r11b, 0x3E
cdqe
neg ch
xor r11b, 0xC9
inc r11b
movsxd rcx, esi
mov ch, 0x16
rcr rcx, cl
xor r8b, r11b
and cl, bh
btc cx, si
mov rax, [rsp+r11*1]
sub rdi, 0x08
stc
jmp +0x26208
mov [rdi], rax
shl rcx, 0x7C
sar cl, 0xBD
neg ch
mov ecx, [r10]
clc
add r10, 0x04
test r9, rsi
xor ecx, r8d
stc
neg ecx
test r10d, 0x410A5956
sub ecx, 0x50D714E5
stc
clc
ror ecx, 0x01
jmp +0x3745F
dec ecx
cmc
rol ecx, 0x02
jmp +0x330B5
dec ecx
rol ecx, 0x01
dec ecx
push r8
shr r8d, cl
movsx r8d, di
xor [rsp], ecx
clc
pop r8
movsxd rcx, ecx
clc
cmp r10b, 0x86
add rbx, rcx
jmp +0x2114
jmp +0x42197
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
bt ax, si
dec cx
lahf
add r10, 0x01
bswap rax
xor r11b, r8b
jmp -0x84A13
neg r11b
shld cx, si, 0x9E
bsr eax, ebp
rcr cl, 0x99
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
shr cl, 0x51
btc rcx, 0x18
test r13b, 0xDB
xor r8b, r11b
xchg ah, al
mov rax, [rsp+r11*1]
not cl
and cx, r8w
movsxd rcx, eax
sub rdi, 0x08
or cx, 0x5BCA
mov [rdi], rax
rol rcx, cl
shrd ecx, ebp, 0xA0
shld cx, r9w, 0x7D
mov ecx, [r10]
jmp +0x2FCBD
add r10, 0x04
clc
cmp r12b, r8b
jmp -0xFFA3
xor ecx, r8d
stc
test r11b, 0x30
neg ecx
clc
cmc
test rbp, 0x70C629E2
sub ecx, 0x50D714E5
clc
ror ecx, 0x01
jmp +0x9DB4D
dec ecx
rol ecx, 0x02
jmp -0xE56EB
dec ecx
cmc
stc
rol ecx, 0x01
jmp +0x546EE
dec ecx
cmp bx, 0x3F00
push r8
bt r8d, 0xE4
xor [rsp], ecx
ror r8, 0xE8
adc r8b, al
bsf r8, rbp
pop r8
movsxd rcx, ecx
test rbp, 0x71290159
add rbx, rcx
jmp +0x1BE13
jmp +0x60AF7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
movsx ecx, si
cdqe
add r10, 0x01
xor r11b, r8b
sbb rax, r8
rol eax, 0xB5
xadd rcx, rax
neg r11b
or al, spl
shr ax, 0xB7
add r11b, 0x3E
rcl cx, cl
xor r11b, 0xC9
movsxd rax, r14d
inc r11b
xor r8b, r11b
lahf
mov rax, [rsp+r11*1]
sub rdi, 0x08
bsr ecx, ecx
btc ecx, esi
mov [rdi], rax
xadd ch, cl
bts cx, cx
mov ecx, [r10]
test cl, r9b
cmp dh, 0xB6
test r12b, 0xDC
add r10, 0x04
cmp rbx, rbx
test r8, 0x7389064F
xor ecx, r8d
neg ecx
test edx, 0x34820148
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp -0x1286D
dec ecx
stc
cmc
rol ecx, 0x02
jmp +0x11E2C
dec ecx
clc
stc
rol ecx, 0x01
jmp -0x1BF42
dec ecx
push r8
neg r8
mov r8w, 0xFD
xor [rsp], ecx
pop r8
cmc
movsxd rcx, ecx
add rbx, rcx
jmp +0xACE18
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
dec cl
add r10, 0x01
and cx, 0x3627
shl rcx, 0x35
xor r11b, r8b
neg r11b
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
sar cx, cl
rcr cl, 0x12
lahf
xor r8b, r11b
bswap rcx
mov rax, [rsp+r11*1]
mov cl, 0x4E
rol cx, cl
sub rdi, 0x08
mov [rdi], rax
cmovnl cx, r13w
mov ecx, [r10]
test dil, r11b
add r10, 0x04
xor ecx, r8d
cmc
neg ecx
cmp esi, r8d
cmp ax, 0x1974
sub ecx, 0x50D714E5
stc
ror ecx, 0x01
dec ecx
rol ecx, 0x02
jmp +0x57B6
dec ecx
stc
jmp -0xC1845
rol ecx, 0x01
jmp +0x5335A
dec ecx
test rsp, rax
stc
push r8
xor [rsp], ecx
adc r8w, r15w
jmp +0x26D6A
pop r8
cmp spl, 0x8F
jmp -0x7BFB
movsxd rcx, ecx
test sil, bpl
cmc
add rbx, rcx
jmp +0x1512
jmp +0x38883
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
rcr al, cl
movsx ax, r8b
bts cx, dx
add r10, 0x01
adc ah, al
xor r11b, r8b
shl rcx, 0xB8
neg r11b
test r14d, 0x19164107
add r11b, 0x3E
ror rcx, 0x46
movzx cx, r13b
xor r11b, 0xC9
nop
cwde
mov cl, 0x1F
inc r11b
rcl cl, 0x35
ror ax, cl
movsx rcx, sp
xor r8b, r11b
cdqe
adc cl, 0xBB
mov rax, [rsp+r11*1]
sub rdi, 0x08
btc rcx, 0xD5
mov [rdi], rax
mov ecx, [r10]
test r13b, r15b
add r10, 0x04
clc
xor ecx, r8d
test rsp, r10
neg ecx
jmp -0x78E2D
sub ecx, 0x50D714E5
jmp +0x8833C
ror ecx, 0x01
dec ecx
rol ecx, 0x02
jmp -0x3823D
dec ecx
rol ecx, 0x01
jmp -0x32491
dec ecx
test r9d, 0x55A62CD5
push r8
xor [rsp], ecx
bt r8w, r8w
adc r8b, 0x46
rol r8, cl
pop r8
movsxd rcx, ecx
clc
stc
test si, r8w
add rbx, rcx
jmp -0x5F8D
jmp +0x9C6BE
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
btr eax, esi
stc
add r10, 0x01
not al
cbw
rcr ch, cl
xor r11b, r8b
neg r11b
stc
add r11b, 0x3E
xor r11b, 0xC9
jmp +0xE0398
inc r11b
xor r8b, r11b
mov rax, [rsp+r11*1]
sub rdi, 0x08
mov [rdi], rax
adc cl, r10b
mov ecx, [r10]
add r10, 0x04
clc
xor ecx, r8d
test r10, 0x1D007430
jmp -0x6FD15
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp +0x19EAC
dec ecx
rol ecx, 0x02
jmp -0x49101
dec ecx
rol ecx, 0x01
jmp -0x3BFD2
dec ecx
test r13w, r9w
push r8
sub r8b, 0xA4
xor [rsp], ecx
xadd r8b, r8b
mov r8w, 0x2277
rcl r8d, cl
pop r8
movsxd rcx, ecx
cmc
cmp r14b, bpl
add rbx, rcx
jmp +0xCE94A
jmp -0x35F1
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
add r10, 0x01
bt ecx, r15d
movsxd rax, ebp
btr ax, 0x2A
xor r11b, r8b
adc ecx, 0x1344781D
neg r11b
bt ax, 0x21
not cx
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
movsx ecx, r14w
jmp -0x86DD
xor r8b, r11b
bsf ax, ax
mov rax, [rsp+r11*1]
rol cx, cl
mov ecx, 0x8772461
sub rdi, 0x08
add cx, sp
mov [rdi], rax
mov ecx, [r10]
clc
add r10, 0x04
xor ecx, r8d
neg ecx
cmp si, 0xD7A
cmc
sub ecx, 0x50D714E5
ror ecx, 0x01
dec ecx
jmp -0x44435
rol ecx, 0x02
jmp +0xAC982
dec ecx
clc
rol ecx, 0x01
jmp -0x866C8
dec ecx
cmc
stc
push r8
xor [rsp], ecx
movsx r8w, spl
xchg r8d, r8d
add r8b, 0x0D
pop r8
cmp r11b, 0xE9
movsxd rcx, ecx
add rbx, rcx
jmp +0x6B84C
jmp +0x33CDF
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
add r10, 0x01
or ah, dl
xor r11b, r8b
neg r11b
shl ch, 0x88
bswap ax
add r11b, 0x3E
shrd ax, sp, 0xF4
bt ax, cx
xor ch, 0x68
xor r11b, 0xC9
inc r11b
xor r8b, r11b
mov rax, [rsp+r11*1]
movzx cx, al
shl ecx, 0xEC
sub rdi, 0x08
mov [rdi], rax
mov ecx, [r10]
cmc
test sil, bl
jmp +0x2FB42
add r10, 0x04
cmp r9b, 0xC9
test spl, 0x47
xor ecx, r8d
clc
test r8b, spl
neg ecx
cmc
sub ecx, 0x50D714E5
jmp -0x1DEA7
ror ecx, 0x01
dec ecx
clc
stc
rol ecx, 0x02
jmp +0x14CAA
dec ecx
clc
rol ecx, 0x01
jmp -0x47588
dec ecx
cmp si, 0x5E93
push r8
or r8b, 0x95
sub r8w, 0x6585
xor [rsp], ecx
xor r8b, 0x92
mov r8b, 0x1E
pop r8
test di, bx
cmp r11b, r14b
clc
movsxd rcx, ecx
test r10b, 0xC7
test r12b, r13b
add rbx, rcx
jmp +0x723CC
jmp +0x30EA7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
shr ah, cl
sbb cl, r13b
movsx rcx, r14w
add r10, 0x01
sbb cl, 0x56
xor r11b, r8b
and cl, 0x99
sub cx, cx
neg r11b
shl ecx, cl
xadd ax, ax
sbb cx, cx
add r11b, 0x3E
lahf
movzx ax, dil
bsf ax, r9w
xor r11b, 0xC9
movsx eax, r10w
movsx rax, di
inc r11b
movsx rax, bx
cwde
xor r8b, r11b
not ax
mov rax, [rsp+r11*1]
mov cl, sil
btc cx, r10w
mov rcx, 0x3EA52C29
sub rdi, 0x08
btr ecx, r12d
adc cl, spl
movsx ecx, r13w
mov [rdi], rax
bts rcx, r10
mov ecx, [r10]
cmc
add r10, 0x04
jmp -0x4C9BB
xor ecx, r8d
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp +0x378B6
dec ecx
rol ecx, 0x02
jmp +0x95EBB
dec ecx
stc
cmc
rol ecx, 0x01
jmp -0x7CEC4
dec ecx
test bl, r14b
test r9d, 0x42633C06
push r8
rol r8d, 0x83
sub r8b, 0xD0
btc r8w, r14w
xor [rsp], ecx
rcr r8, cl
pop r8
movsxd rcx, ecx
add rbx, rcx
jmp -0x5C6C4
jmp +0xC79E1
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
bsf ax, r13w
cmp r11b, 0xB3
add r10, 0x01
sub rax, 0x7085651
cdqe
and al, spl
xor r11b, r8b
rcl cl, 0xE2
neg cl
adc al, bpl
neg r11b
not rcx
movzx rax, r13w
shl ecx, 0x8B
add r11b, 0x3E
xor ecx, 0x742A55B6
neg ah
xor r11b, 0xC9
xchg rcx, rcx
inc r11b
rcl eax, 0x8D
sbb al, 0xF5
cdqe
xor r8b, r11b
cdqe
mov rax, [rsp+r11*1]
sub rdi, 0x08
mov [rdi], rax
mov ecx, [r10]
cmc
test r10b, 0x9F
clc
add r10, 0x04
test r14w, 0x9D1
xor ecx, r8d
neg ecx
sub ecx, 0x50D714E5
ror ecx, 0x01
dec ecx
rol ecx, 0x02
dec ecx
stc
clc
cmc
rol ecx, 0x01
jmp -0xC4206
dec ecx
stc
cmp di, r10w
cmp si, 0x788F
push r8
sar r8b, cl
add r8b, 0x6E
sbb r8b, r12b
xor [rsp], ecx
xor r8b, dl
pop r8
cmc
cmp esi, r15d
movsxd rcx, ecx
cmp rdx, r11
add rbx, rcx
jmp +0xAC48
jmp +0xC0DC8
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
shld ecx, r8d, 0xB8
rcl cx, cl
shl ax, cl
add r10, 0x01
dec cx
cwde
xor r11b, r8b
and eax, ebp
add cl, sil
shr eax, cl
neg r11b
add r11b, 0x3E
movzx rax, ax
lahf
xor r11b, 0xC9
inc cx
setnz ch
cmovb eax, esp
inc r11b
adc eax, ecx
xor r8b, r11b
shl cx, 0x0F
or al, spl
mov rax, [rsp+r11*1]
bsr cx, r15w
adc ecx, r8d
shr ch, 0xB4
sub rdi, 0x08
not ch
rol cx, cl
shl ecx, 0xD7
mov [rdi], rax
mov ecx, [r10]
stc
add r10, 0x04
test ecx, 0x6DD153C3
xor ecx, r8d
jmp -0x3239E
neg ecx
cmc
sub ecx, 0x50D714E5
clc
ror ecx, 0x01
jmp -0x3BF04
dec ecx
clc
stc
rol ecx, 0x02
jmp -0x22104
dec ecx
jmp +0x1421C
rol ecx, 0x01
jmp +0xFA8
dec ecx
clc
push r8
xor [rsp], ecx
shl r8b, 0xB4
xor r8w, cx
add r8b, spl
pop r8
cmp r10w, r9w
movsxd rcx, ecx
jmp +0x9D07B
add rbx, rcx
jmp +0x93D3
jmp +0xF81E
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
sar cx, 0x75
cwde
add r10, 0x01
xor r11b, r8b
test r12b, 0x9F
neg r11b
xchg cl, cl
shl ax, cl
movsxd rax, r10d
add r11b, 0x3E
cdqe
neg ch
xor r11b, 0xC9
inc r11b
movsxd rcx, esi
mov ch, 0x16
rcr rcx, cl
xor r8b, r11b
and cl, bh
btc cx, si
mov rax, [rsp+r11*1]
sub rdi, 0x08
stc
jmp +0x26208
mov [rdi], rax
shl rcx, 0x7C
sar cl, 0xBD
neg ch
mov ecx, [r10]
clc
add r10, 0x04
test r9, rsi
xor ecx, r8d
stc
neg ecx
test r10d, 0x410A5956
sub ecx, 0x50D714E5
stc
clc
ror ecx, 0x01
jmp +0x3745F
dec ecx
cmc
rol ecx, 0x02
jmp +0x330B5
dec ecx
rol ecx, 0x01
dec ecx
push r8
shr r8d, cl
movsx r8d, di
xor [rsp], ecx
clc
pop r8
movsxd rcx, ecx
clc
cmp r10b, 0x86
add rbx, rcx
jmp +0x2114
jmp +0x42197
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
bt ax, si
dec cx
lahf
add r10, 0x01
bswap rax
xor r11b, r8b
jmp -0x84A13
neg r11b
shld cx, si, 0x9E
bsr eax, ebp
rcr cl, 0x99
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
shr cl, 0x51
btc rcx, 0x18
test r13b, 0xDB
xor r8b, r11b
xchg ah, al
mov rax, [rsp+r11*1]
not cl
and cx, r8w
movsxd rcx, eax
sub rdi, 0x08
or cx, 0x5BCA
mov [rdi], rax
rol rcx, cl
shrd ecx, ebp, 0xA0
shld cx, r9w, 0x7D
mov ecx, [r10]
jmp +0x2FCBD
add r10, 0x04
clc
cmp r12b, r8b
jmp -0xFFA3
xor ecx, r8d
stc
test r11b, 0x30
neg ecx
clc
cmc
test rbp, 0x70C629E2
sub ecx, 0x50D714E5
clc
ror ecx, 0x01
jmp +0x9DB4D
dec ecx
rol ecx, 0x02
jmp -0xE56EB
dec ecx
cmc
stc
rol ecx, 0x01
jmp +0x546EE
dec ecx
cmp bx, 0x3F00
push r8
bt r8d, 0xE4
xor [rsp], ecx
ror r8, 0xE8
adc r8b, al
bsf r8, rbp
pop r8
movsxd rcx, ecx
test rbp, 0x71290159
add rbx, rcx
jmp +0x1BE13
jmp +0x60AF7
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
movsx ecx, si
cdqe
add r10, 0x01
xor r11b, r8b
sbb rax, r8
rol eax, 0xB5
xadd rcx, rax
neg r11b
or al, spl
shr ax, 0xB7
add r11b, 0x3E
rcl cx, cl
xor r11b, 0xC9
movsxd rax, r14d
inc r11b
xor r8b, r11b
lahf
mov rax, [rsp+r11*1]
sub rdi, 0x08
bsr ecx, ecx
btc ecx, esi
mov [rdi], rax
xadd ch, cl
bts cx, cx
mov ecx, [r10]
test cl, r9b
cmp dh, 0xB6
test r12b, 0xDC
add r10, 0x04
cmp rbx, rbx
test r8, 0x7389064F
xor ecx, r8d
neg ecx
test edx, 0x34820148
sub ecx, 0x50D714E5
ror ecx, 0x01
jmp -0x1286D
dec ecx
stc
cmc
rol ecx, 0x02
jmp +0x11E2C
dec ecx
clc
stc
rol ecx, 0x01
jmp -0x1BF42
dec ecx
push r8
neg r8
mov r8w, 0xFD
xor [rsp], ecx
pop r8
cmc
movsxd rcx, ecx
add rbx, rcx
jmp +0xACE18
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
movzx r11d, byte ptr [r10]
dec cl
add r10, 0x01
and cx, 0x3627
shl rcx, 0x35
xor r11b, r8b
neg r11b
add r11b, 0x3E
xor r11b, 0xC9
inc r11b
sar cx, cl
rcr cl, 0x12
lahf
xor r8b, r11b
bswap rcx
mov rax, [rsp+r11*1]
mov cl, 0x4E
rol cx, cl
sub rdi, 0x08
mov [rdi], rax
cmovnl cx, r13w
mov ecx, [r10]
test dil, r11b
add r10, 0x04
xor ecx, r8d
cmc
neg ecx
cmp esi, r8d
cmp ax, 0x1974
sub ecx, 0x50D714E5
stc
ror ecx, 0x01
dec ecx
rol ecx, 0x02
jmp +0x57B6
dec ecx
stc
jmp -0xC1845
rol ecx, 0x01
jmp +0x5335A
dec ecx
test rsp, rax
stc
push r8
xor [rsp], ecx
adc r8w, r15w
jmp +0x26D6A
pop r8
cmp spl, 0x8F
jmp -0x7BFB
movsxd rcx, ecx
test sil, bpl
cmc
add rbx, rcx
jmp +0x1512
jmp +0x38883
lea rsi, [rsp+0x140]
cmp rdi, rsi
jmp -0x3D6FC
jnbe -0xE161
push rbx
ret
mov rsp, rdi
cmp bpl, 0xEC
pop r9
bsf di, dx
pop r11
ror bl, cl
pop rdi
add r12d, ebx
movzx r15d, sp
pop r8
shl al, 0xDC
pop r10
xchg dl, bl
pop r14
and ax, di
pop rcx
sub bpl, cl
rol bpl, cl
movzx rbp, r8w
pop rax
ror r12b, 0xFE
jmp +0x82E1D
pop r13
popfq
cmovnle dx, r14w
pop rbp
movsxd rsi, r13d
bswap rdx
pop r15
movsx bx, r11b
cdq
pop rbx
cqo
mov dl, 0x59
cdq
pop rdx
pop rsi
movzx r12d, r15w
pop r12
jmp +0x1457B
ret
mov [rbp+0x44], eax
