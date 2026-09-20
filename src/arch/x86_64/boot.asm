MULTIBOOT2_HEADER_MAGIC         equ 0xe85250d6
ARCHITECTURE_I386               equ 0
TAG_TYPE_FRAMEBUFFER            equ 5
TAG_TYPE_END                    equ 0
TAG_OPTIONAL                    equ 1

STACK_SIZE                      equ 0x4000 ; 16KB (0x4000)

; ==========================================
; Multiboot2 header
; ==========================================
section .multiboot_header
align 8
multiboot_header:

    dd MULTIBOOT2_HEADER_MAGIC
    dd ARCHITECTURE_I386
    dd multiboot_header_end - multiboot_header
    dd -(MULTIBOOT2_HEADER_MAGIC + ARCHITECTURE_I386 + (multiboot_header_end - multiboot_header))

align 8
framebuffer_tag_start:
    dw TAG_TYPE_FRAMEBUFFER
    dw TAG_OPTIONAL
    dd framebuffer_tag_end - framebuffer_tag_start
    dd 1024
    dd 768
    dd 32
framebuffer_tag_end:

    ; 3. end_tag
align 8
    dw TAG_TYPE_END
    dw 0
    dd 8
multiboot_header_end:

; ==========================================
; entry point(32bit protect)
; ==========================================
section .text
bits 32
global start
global _start
extern rust_main ; Rust側のメイン関数

start:
_start:
    jmp multiboot_entry

align 8
multiboot_entry:

    mov esp, stack_top

    ; EFLAGS initialization
    push 0
    popfd

    mov edi, eax ; arg1: magic number
    mov esi, ebx ; arg2: Multiboot2 structure pointer

    ; ----------------------------------------------------
    ; 64bit long mode
    ; ----------------------------------------------------
    
    ; PML4 -> PDPT
    mov eax, pdpt_table
    or eax, 0b11 ; Present + Writable
    mov [pml4_table], eax

    ; PDPT -> PD
    mov eax, pd_table
    or eax, 0b11 ; Present + Writable
    mov [pdpt_table], eax

    mov eax, 0b10000011 ; Present + Writable + HugePage(2MB)
    mov [pd_table], eax


    mov eax, pml4_table
    mov cr3, eax


    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr


    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax


    lgdt [gdt64.pointer]

    jmp gdt64.code:start64

; ==========================================
; 64bit mode
; ==========================================
bits 64
start64:

    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; Rust main function call
    call rust_main


loop:
    hlt
    jmp loop

; ==========================================
; data, table
; ==========================================
section .rodata
align 8
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) 
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

; ==========================================
; page table, stack
; ==========================================
section .bss
align 4096
pml4_table:
    resb 4096
pdpt_table:
    resb 4096
pd_table:
    resb 4096

align 16
stack_bottom:
    resb STACK_SIZE
stack_top: