.cfi_sections .debug_frame

// TCK: 5
// TDI: 7
// TDO: 6
// TMS: 4

.section .text._write_tdi_bytes_mode0_lsb_12mhz
.global _write_tdi_bytes_mode0_lsb_12mhz
.syntax unified
.thumb_func
.cfi_startproc
_write_tdi_bytes_mode0_lsb_12mhz:
    PUSH    {R4-R6,LR}
    CMP     R1, #0
    BEQ     2f

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    ADDS    R1, R0
    MOV     R3, R4
1:
    LDRB    R6, [R0]
    MVNS    R6, R6
    UXTB    R6, R6

    // MOV     R3, R4 // is executed in advance
    LSLS    R6, R6, #0x7
    BICS    R3, R3, R6
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    NOP
    STR     R3, [R2]

    MOV     R3, R4
    LSRS    R6, R6, #0x1
    STR     R5, [R2]  // reorder
    BICS    R3, R3, R6
    ADDS    R0, R0, 1  // reorder
    STR     R3, [R2]
    CMP     R0, R1  // reorder
    MOV     R3, R4  // reorder
    STR     R5, [R2]

    BNE     1b

    LSLS    R5, R5, #0x10
    STR     R5, [R2]

2:
    POP     {R4-R6,PC}
    .cfi_endproc
    .size _write_tdi_bytes_mode0_lsb_12mhz, . - _write_tdi_bytes_mode0_lsb_12mhz



.section .text._write_tdi_bytes_mode0_lsb_6mhz
.global _write_tdi_bytes_mode0_lsb_6mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bytes_mode0_lsb_6mhz:
    PUSH    {R4-R7,LR}
    CMP     R1, #0
    BEQ     3f

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    ADDS    R1, R0
    MOV     R3, R4
1:
    LDRB    R6, [R0]
    MVNS    R6, R6
    UXTB    R6, R6

    LSLS    R6, R6, #0x7

    MOVS    R7, #7
2:
    BICS    R3, R3, R6
    STR     R3, [R2] // CLK0 + TDIx
    LSRS    R6, R6, #0x1
    MOV     R3, R4
    NOP
    NOP
    NOP
    SUBS    R7, 1
    STR     R5, [R2] // CLK1
    NOP
    BPL     2b

    ADDS    R0, R0, 1
    CMP     R0, R1
    BNE     1b

    LSLS    R5, R5, #0x10
    STR     R5, [R2] // CLK0

3:
    POP     {R4-R7,PC}
    .cfi_endproc
    .size _write_tdi_bytes_mode0_lsb_6mhz, . - _write_tdi_bytes_mode0_lsb_6mhz



.section .text._write_tdi_bytes_mode0_lsb_delay
.global _write_tdi_bytes_mode0_lsb_delay
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bytes_mode0_lsb_delay:
    PUSH    {R4-R7,LR}
    CMP     R1, #0
    BEQ     3f

    MOV     R12, R3

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    ADDS    R1, R0
    MOV     R3, R4
    NOP // alignment
1:
    LDRB    R6, [R0]
    MVNS    R6, R6
    UXTB    R6, R6

    LSLS    R6, R6, #0x7

    MOVS    R7, #7
2:
    BICS    R3, R3, R6
    STR     R3, [R2] // CLK0 + TDIx
    LSRS    R6, R6, #0x1

    MOV     R3, R12
    ADDS    R3, 1
5:  SUBS    R3, 1
    BPL     5b

    NOP
    STR     R5, [R2] // CLK1

    MOV     R3, R12
6:  SUBS    R3, 1
    BPL     6b

    SUBS    R7, 1
    MOV     R3, R4
    BPL     2b

    ADDS    R0, R0, 1
    CMP     R0, R1
    BNE     1b

    LSLS    R5, R5, #0x10
    STR     R5, [R2] // CLK0

3:
    POP     {R4-R7,PC}
    .cfi_endproc
    .size _write_tdi_bytes_mode0_lsb_delay, . - _write_tdi_bytes_mode0_lsb_delay
