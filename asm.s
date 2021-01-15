.cfi_sections .debug_frame

// GPIOA.IDR: 0x48000010
// GPIOA.BSRR: 0x48000018
// TCK: 5
// TDI: 7
// TDO: 6
// TMS: 4

.section .text._write_tdi_bytes_lsb_mode0_6mhz
.global _write_tdi_bytes_lsb_mode0_6mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bytes_lsb_mode0_6mhz:
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
    .size _write_tdi_bytes_lsb_mode0_6mhz, . - _write_tdi_bytes_lsb_mode0_6mhz



.section .text._write_tdi_bytes_lsb_mode0_3mhz
.global _write_tdi_bytes_lsb_mode0_3mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bytes_lsb_mode0_3mhz:
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
    STR     R3, [R2] // TCK0 + TDIx
    LSRS    R6, R6, #0x1
    MOV     R3, R4
    NOP
    NOP
    NOP
    SUBS    R7, 1
    STR     R5, [R2] // TCK1
    NOP
    BPL     2b

    ADDS    R0, R0, 1
    CMP     R0, R1
    BNE     1b

    LSLS    R5, R5, #0x10
    STR     R5, [R2] // TCK0

3:
    POP     {R4-R7,PC}
    .cfi_endproc
    .size _write_tdi_bytes_lsb_mode0_3mhz, . - _write_tdi_bytes_lsb_mode0_3mhz



.section .text._write_tdi_bytes_lsb_mode0_delay
.global _write_tdi_bytes_lsb_mode0_delay
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bytes_lsb_mode0_delay:
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
    STR     R3, [R2] // TCK0 + TDIx
    LSRS    R6, R6, #0x1

    MOV     R3, R12
    ADDS    R3, 1
5:  SUBS    R3, 1
    BPL     5b

    NOP
    STR     R5, [R2] // TCK1

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
    STR     R5, [R2] // TCK0

3:
    POP     {R4-R7,PC}
    .cfi_endproc
    .size _write_tdi_bytes_lsb_mode0_delay, . - _write_tdi_bytes_lsb_mode0_delay



// fn _write_tdi_bits_lsb_mode0_6mhz(byte: u8, nbits: u8, bsrr: *mut u32);
// nbits is always 1..8
.section .text._write_tdi_bits_lsb_mode0_6mhz
.global _write_tdi_bits_lsb_mode0_6mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bits_lsb_mode0_6mhz:
    PUSH    {R4-R5,LR}

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    // Calculate 12*(8-nbits)-2 == -12*nbits+94
    MOVS    R3, #12
    MULS    R1, R3, R1
    NEGS    R1, R1
    ADDS    R1, #94

    LSLS    R0, R0, #0x8
    MOV     R3, R4 // reorder

    // Jump to the beginning of the nth bit block
    ADD     PC, R1

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4
    NOP
    STR     R5, [R2]

    NOP
    LSLS    R5, R5, #0x10
    STR     R5, [R2]

    POP     {R4-R5,PC}
    .cfi_endproc
    .size _write_tdi_bits_lsb_mode0_6mhz, . - _write_tdi_bits_lsb_mode0_6mhz



// fn _write_tdi_bits_lsb_mode0_3mhz(byte: u8, nbits: u8, bsrr: *mut u32);
// nbits is always 1..8
.section .text._write_tdi_bits_lsb_mode0_3mhz
.global _write_tdi_bits_lsb_mode0_3mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bits_lsb_mode0_3mhz:
    PUSH    {R4-R5,LR}

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    LSLS    R0, R0, #0x8
    MOV     R3, R4 // reorder
    SUBS    R1, #1

1:
    LSRS    R0, R0, #0x1
    BICS    R3, R3, R0
    STR     R3, [R2]
    MOV     R3, R4 // reorder
    NOP
    NOP
    NOP
    NOP
    SUBS    R1, #1
    STR     R5, [R2]
    BPL     1b

    NOP
    NOP
    NOP
    NOP
    LSLS    R5, R5, #0x10
    STR     R5, [R2]

    POP     {R4-R5,PC}
    .cfi_endproc
    .size _write_tdi_bits_lsb_mode0_3mhz, . - _write_tdi_bits_lsb_mode0_3mhz



// fn _write_tdi_bits_lsb_mode0_delay(byte: u8, nbits: u8, bsrr: *mut u32, delay: u32);
// nbits is always 1..8
.section .text._write_tdi_bits_lsb_mode0_delay
.global _write_tdi_bits_lsb_mode0_delay
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_write_tdi_bits_lsb_mode0_delay:
    PUSH    {R4-R6,LR}

    MOV     R12, R3

    // Load bit constants for BSRR GPIO register
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    LSLS    R0, R0, #0x7
    MOV     R3, R4 // reorder
    SUBS    R1, #1

1:
    BICS    R3, R3, R0
    STR     R3, [R2] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4 // reorder

    MOV     R6, R12
    ADDS    R6, 1
2:  SUBS    R6, 1
    BPL     2b

    STR     R5, [R2] // TCK1

    MOV     R6, R12
2:  SUBS    R6, 1
    BPL     2b

    SUBS    R1, #1
    BPL     1b

    NOP
    NOP
    NOP
    NOP
    LSLS    R5, R5, #0x10
    STR     R5, [R2]

    POP     {R4-R6,PC}
    .cfi_endproc
    .size _write_tdi_bits_lsb_mode0_delay, . - _write_tdi_bits_lsb_mode0_delay



// fn _transfer_tdi_bits_lsb_mode0_4mhz(byte: u8, nbits: u8) -> u8;
// nbits is always 1..8
.section .text._transfer_tdi_bits_lsb_mode0_4mhz
.global _transfer_tdi_bits_lsb_mode0_4mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_transfer_tdi_bits_lsb_mode0_4mhz:
    PUSH    {R4-R7,LR}

    LDR     R2, =0x48000000 // GPIO base
    MOVS    R6, #0x00000040 // R6 <- TDO mask
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    // Calculate 18*(8-nbits)-2 == -18*nbits+142
    MOVS    R7, #18
    MULS    R7, R1, R7
    NEGS    R7, R7
    ADDS    R7, #142

    LSLS    R0, R0, #0x7
    MOV     R3, R4
    BICS    R3, R3, R0

    // Jump to the beginning of the nth bit block
    ADD     PC, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R5 // changed
    LSLS    R3, R3, #0x10 // changed
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    STR     R3, [R2, #0x18] // TCK0

    MOVS    R3, #32
    SUBS    R3, R3, R1
    LSRS    R0, R0, R3

    POP     {R4-R7,PC}
    .cfi_endproc
    .size _transfer_tdi_bits_lsb_mode0_4mhz, . - _transfer_tdi_bits_lsb_mode0_4mhz



// fn _transfer_tdi_bits_lsb_mode0_2p8mhz(byte: u8, nbits: u8) -> u8;
// nbits is always 1..8
.section .text._transfer_tdi_bits_lsb_mode0_2p8mhz
.global _transfer_tdi_bits_lsb_mode0_2p8mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_transfer_tdi_bits_lsb_mode0_2p8mhz:
    PUSH    {R4-R7,LR}

    MOV     R12, R1 // save nbits

    LDR     R2, =0x48000000 // GPIO base
    MOVS    R6, #0x00000040 // R6 <- TDO mask
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    LSLS    R0, R0, #0x7
    SUBS    R1, #1

    MOV     R3, R4
    BICS    R3, R3, R0
    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    NOP
    NOP
    NOP
    NOP
1:
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7
    //NOP

    BICS    R3, R3, R0
    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4 // reorder

    SUBS    R1, #1
    BPL     1b

    MOVS    R3, #31
    MOV     R1, R12
    SUBS    R3, R3, R1
    LSRS    R0, R0, R3

    LSLS    R5, R5, #0x10
    STR     R5, [R2, #0x18] // TCK0

    POP     {R4-R7,PC}
    .cfi_endproc
    .size _transfer_tdi_bits_lsb_mode0_2p8mhz, . - _transfer_tdi_bits_lsb_mode0_2p8mhz



// fn _transfer_tdi_bits_lsb_mode0_2mhz(byte: u8, nbits: u8) -> u8;
// nbits is always 1..8
.section .text._transfer_tdi_bits_lsb_mode0_2mhz
.global _transfer_tdi_bits_lsb_mode0_2mhz
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_transfer_tdi_bits_lsb_mode0_2mhz:
    PUSH    {R4-R7,LR}

    MOV     R12, R1 // save nbits

    LDR     R2, =0x48000000 // GPIO base
    MOVS    R6, #0x00000040 // R6 <- TDO mask
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    LSLS    R0, R0, #0x7
    SUBS    R1, #1

    MOV     R3, R4
    BICS    R3, R3, R0
1:
    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    SUBS    R1, #1
    BPL     1b

    MOVS    R3, #32
    MOV     R1, R12
    SUBS    R3, R3, R1
    LSRS    R0, R0, R3

    LSLS    R5, R5, #0x10
    STR     R5, [R2, #0x18] // TCK0

    POP     {R4-R7,PC}
    .cfi_endproc
    .size _transfer_tdi_bits_lsb_mode0_2mhz, . - _transfer_tdi_bits_lsb_mode0_2mhz



// fn _transfer_tdi_bits_lsb_mode0_delay(byte: u8, nbits: u8, delay: u32) -> u8;
// nbits is always 1..8
.section .text._transfer_tdi_bits_lsb_mode0_delay
.global _transfer_tdi_bits_lsb_mode0_delay
.syntax unified
.thumb_func
.cfi_startproc
.align 2
_transfer_tdi_bits_lsb_mode0_delay:
    PUSH    {R4-R7,LR}
    MOV     R7, R8
    PUSH    {R7}

    MOV     R8, R2 // save delay
    MOV     R12, R1 // save nbits

    LDR     R2, =0x48000000 // GPIO base
    MOVS    R6, #0x00000040 // R6 <- TDO mask
    LDR     R4, =0x00a00080 // R4 <- [TCK0+TDI0+TDI1]
    MOVS    R5, #0x00000020 // R5 <- [TCK1]

    // Invert byte
    MVNS    R0, R0
    UXTB    R0, R0

    LSLS    R0, R0, #0x7
    SUBS    R1, #1

    MOV     R3, R4
    BICS    R3, R3, R0
1:
    STR     R3, [R2, #0x18] // TCK0 + TDIx
    LSRS    R0, R0, #0x1
    MOV     R3, R4
    BICS    R3, R3, R0

    MOV     R7, R8
    ADDS    R7, 1
2:  SUBS    R7, 1
    BPL     2b
    NOP
    NOP
    NOP

    STR     R5, [R2, #0x18] // TCK1
    LDR     R7, [R2, #0x10] // read TDO
    ANDS    R7, R7, R6
    LSLS    R7, R7, #25
    ORRS    R0, R0, R7

    MOV     R7, R8
2:  SUBS    R7, 1
    BPL     2b

    SUBS    R1, #1
    BPL     1b

    MOVS    R3, #32
    MOV     R1, R12
    SUBS    R3, R3, R1
    LSRS    R0, R0, R3

    LSLS    R5, R5, #0x10
    STR     R5, [R2, #0x18] // TCK0

    POP     {R7}
    MOV     R8, R7
    POP     {R4-R7,PC}
    .cfi_endproc
    .size _transfer_tdi_bits_lsb_mode0_delay, . - _transfer_tdi_bits_lsb_mode0_delay
