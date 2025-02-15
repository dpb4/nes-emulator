pub enum AddressingMode {
    // Implied by the instruction itself
    // Ex: `CLC`, `RTS`
    Implicit, //
    // Operates on the accumulator
    // Ex: `LSR A`, `ROR A`
    Accumulator, // AC
    // 8 bit constant specified within the instruction
    // Ex: `LDA #10`, `LDX #LO, LABEL`
    Immediate, // IM
    // 8 bit address is specified within the instruction
    // This allows it to address the first 256 bytes of memory (0x00 to 0xFF)
    // Ex: `LDA $00`, `ASL ANSWER`
    // (Same asm regular absolute, but assembler chooses instruction accordingly)
    ZeroPage, // Z
    // 8 bit address specified within instruction is **wrapping add**ed with the `X` register
    // Because a **wrapping** add is performed, only addresses 0x00 to 0xFF can be addressed
    // Ex: `LDA $80,X` when X=0x0F would load from 0x8F
    // Wrapping Ex: `LDA $80,X` when X=0xFF would load from 0x7F not 0x017F
    ZeroPageX, // ZX
    // 8 bit address specified within instruction is **wrapping add**ed with the `Y` register
    // Because a **wrapping** add is performed, only addresses 0x00 to 0xFF can be addressed
    // Note: This is equivalent to `ZeroPageX` but for the `Y` register
    // and is only used by `LDX` and `STX` instructions
    // Ex: See `ZeroPageX`
    ZeroPageY, // ZY
    // 8 bit **signed** relative offset is included in instruction itself (-128 to 127)
    // which is added to PC if condition is true. Since PC is also incremented
    // by 2 (size of instruction) before instruction is executed, effective branch from
    // start of the branch instruction is (-126 to 129) bytes
    // Ex: `BEQ LABEL`, `BNE *+4 (-2 bytes for instruction, skips next 2-byte instruction)`
    Relative, // R
    // 16-bit **little endian** value is included in the instruction itself
    // being **little endian**, the `0x1234` in `JMP $1234` would be stored as 0x34 0x12
    // Ex: `JMP $1234`, `JSR LABEL`
    Absolute, // A
    // 16-bit **little endian** value is included in the instruction itself
    // This value is added with the `X` register, and the `CARRY` flag
    // Ex: `LDA $8000,x`, `STA $9000,x`
    AbsoluteX, // AX
    // 16-bit **little endian** value is included in the instruction itself
    // This value is added with the `Y` register, and the `CARRY` flag
    // This is the same as the `AbsoluteX` mode, but with Y instead.
    // Ex: `LDA $8000,y`, `STA $9000,y`
    AbsoluteY, // AY
    // 16-bit **little endian** value is included in the instruction itself
    // This value is the memory address of a **little endian** instruction.
    // The value at this memory address is the actual value.
    // Ex: `JMP ($1234)` and address 1234 contains AB, 1235 contains CD
    // would compile to `6C 34 12`, would load value from `0xCDAB`
    Indirect, // IX
    // Also known as Indirect X
    // 8-bit memory address included in instruction itself
    // This value is **wrapping** added to the X register
    // And this value is used to load a **little endian** pointer to the
    // address of the actual value.
    // (Essentially, X is an index to the 8 bit zero page address that
    // contains an array of pointers)
    IndexedIndirect, // IY
    // Also known as Indirect Y
    // 8-bit zero-page memory address included in instruction itself
    // This zero-page 16-bit **little-endian** VALUE (after memory access) is
    // added to register `Y` to get actual target address
    IndirectIndexed, // II
}
use addressingMode::*;

pub struct Instruction {
    pub name: &str,
    pub opcode: u8,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn new(name: &str, opcode: u8, mode: AddressingMode, bytes: u8, cycles: u8) -> Self {
        Instruction {
            name,
            opcode,
            mode,
            bytes,
            cycles,
        }
    }
}

// ADC: add with carry
const ADC_IM: Instruction = Instruction::new("ADC", 0x69, Immediate, 2, 2);
const ADC_Z: Instruction = Instruction::new("ADC", 0x65, ZeroPage, 2, 3);
const ADC_ZX: Instruction = Instruction::new("ADC", 0x75, ZeroPageX, 2, 4);
const ADC_A: Instruction = Instruction::new("ADC", 0x6d, Absolute, 3, 4);
const ADC_AX: Instruction = Instruction::new("ADC", 0x7d, AbsoluteX, 3, 4); //
const ADC_AY: Instruction = Instruction::new("ADC", 0x79, AbsoluteY, 3, 4); //
const ADC_IX: Instruction = Instruction::new("ADC", 0x61, Indirect, 2, 6);
const ADC_IY: Instruction = Instruction::new("ADC", 0x71, IndexedIndirect, 2, 5); //

// AND: arithmetic and
const AND_IM: Instruction = Instruction::new("AND", 0x29, Immediate, 2, 2);
const AND_Z: Instruction = Instruction::new("AND", 0x25, ZeroPage, 2, 3);
const AND_ZX: Instruction = Instruction::new("AND", 0x35, ZeroPageX, 2, 4);
const AND_A: Instruction = Instruction::new("AND", 0x2d, Absolute, 3, 4);
const AND_AX: Instruction = Instruction::new("AND", 0x3d, AbsoluteX, 3, 4); //
const AND_AY: Instruction = Instruction::new("AND", 0x39, AbsoluteY, 3, 4); //
const AND_IX: Instruction = Instruction::new("AND", 0x21, Indirect, 2, 6);
const AND_IY: Instruction = Instruction::new("AND", 0x31, IndexedIndirect, 2, 5); //

// ASL: arithmetic shift left
const ASL_AC: Instruction = Instruction::new("ASL", 0x0a, Accumulator, 1, 2);
const ASL_Z: Instruction = Instruction::new("ASL", 0x06, ZeroPage, 2, 3);
const ASL_ZX: Instruction = Instruction::new("ASL", 0x16, ZeroPageX, 2, 4);
const ASL_A: Instruction = Instruction::new("ASL", 0x0e, Absolute, 3, 4);
const ASL_AX: Instruction = Instruction::new("ASL", 0x1e, AbsoluteX, 3, 4);

// BCC: branch if carry clear
const BCC: Instruction = Instruction::new("BCC", 0x90, Relative, 2, 2); //

// BCS: branch if carry set
const BCS: Instruction = Instruction::new("BCS", 0xb0, Relative, 2, 2); //

// BEQ: branch if equal
const BEQ: Instruction = Instruction::new("BEQ", 0xf0, Relative, 2, 2); //

// BMI: branch if minus
const BMI: Instruction = Instruction::new("BMI", 0x30, Relative, 2, 2); //

// BNE: branch if not equal
const BNE: Instruction = Instruction::new("BNE", 0xd0, Relative, 2, 2); //

// BPL: branch if positive
const BPL: Instruction = Instruction::new("BPL", 0xd0, Relative, 2, 2); //

// BVC: branch if overflow clear
const BVC: Instruction = Instruction::new("BVC", 0x50, Relative, 2, 2); //

// BVS: branch if overflow set
const BVS: Instruction = Instruction::new("BVS", 0x70, Relative, 2, 2); //

// BRK: force interrupt
const BRK: Instruction = Instruction::new("BRK", 0x00, Implicit, 1, 7);

// BIT: bit test
const BIT_Z: Instruction = Instruction::new("BIT", 0x24, ZeroPage, 2, 3);
const BIT_A: Instruction = Instruction::new("BIT", 0x2c, Absolute, 3, 4);

// CLC: clear carry flag
const CLC: Instruction = Instruction::new("CLC", 0x18, Implicit, 1, 2);

// CLD: clear decimal mode
const CLD: Instruction = Instruction::new("CLD", 0xd8, Implicit, 1, 2);

// CLI: clear interrupt disable
const CLI: Instruction = Instruction::new("CLI", 0x58, Implicit, 1, 2);

// CLV: clear overflow flag
const CLV: Instruction = Instruction::new("CLV", 0xb8, Implicit, 1, 2);

// CMP: compare
const CMP_IM: Instruction = Instruction::new("CMP", 0xc9, Immediate, 2, 2);
const CMP_Z: Instruction = Instruction::new("CMP", 0xc5, Immediate, 2, 3);
const CMP_ZX: Instruction = Instruction::new("CMP", 0xd5, ZeroPageX, 2, 4);
const CMP_A: Instruction = Instruction::new("CMP", 0xcd, Absolute, 3, 4);
const CMP_AX: Instruction = Instruction::new("CMP", 0xdd, AbsoluteX, 3, 4); //
const CMP_AY: Instruction = Instruction::new("CMP", 0xd9, AbsoluteY, 3, 4); //
const CMP_IX: Instruction = Instruction::new("CMP", 0xc1, Indirect, 2, 6);
const CMP_IY: Instruction = Instruction::new("CMP", 0xd1, IndexedIndirect, 2, 5); //

// CPX: compare x register
const CPX_IM: Instruction = Instruction::new("CPX", 0xe0, Immediate, 2, 2);
const CPX_Z: Instruction = Instruction::new("CPX", 0xe4, ZeroPage, 2, 3);
const CPX_A: Instruction = Instruction::new("CPX", 0xec, Absolute, 3, 4);

// CPY: compare y register
const CPY_IM: Instruction = Instruction::new("CPY", 0xc0, Immediate, 2, 2);
const CPY_Z: Instruction = Instruction::new("CPY", 0xc4, ZeroPage, 2, 3);
const CPY_A: Instruction = Instruction::new("CPY", 0xcc, Absolute, 3, 4);

// LSR: logical shift right
const LSR_AC: Instruction = Instruction::new("LSR", 0x4a, Accumulator, 1, 2);
const LSR_Z: Instruction = Instruction::new("LSR", 0x46, ZeroPage, 2, 5);
const LSR_ZX: Instruction = Instruction::new("LSR", 0x56, ZeroPageX, 2, 6);
const LSR_A: Instruction = Instruction::new("LSR", 0x4e, Absolute, 3, 6);
const LSR_AX: Instruction = Instruction::new("LSR", 0x5e, AbsoluteX, 3, 7);

// NOP: no operation
const NOP: Instruction = Instruction::new("NOP", 0xea, Implied, 1, 2);

// ORA: logical or
const ORA_IM: Instruction = Instruction::new("ORA", 0x09, Immediate, 2, 2);
const ORA_Z: Instruction = Instruction::new("ORA", 0x05, ZeroPage, 2, 3);
const ORA_ZX: Instruction = Instruction::new("ORA", 0x15, ZeroPageX, 2, 4);
const ORA_A: Instruction = Instruction::new("ORA", 0x0d, Absolute, 3, 4);
const ORA_AX: Instruction = Instruction::new("ORA", 0x1d, AbsoluteX, 3, 4); //
const ORA_AY: Instruction = Instruction::new("ORA", 0x19, AbsoluteY, 3, 4); //
const ORA_IX: Instruction = Instruction::new("ORA", 0x01, Indirect, 2, 6);
const ORA_IY: Instruction = Instruction::new("ORA", 0x11, IndexedIndirect, 2, 5); //

// PHA: push accumulator
const PHA: Instruction = Instruction::new("PHA", 0x48, Implied, 1, 3);

// PHP: push processor status
const PHP: Instruction = Instruction::new("PHP", 0x08, Implied, 1, 3);

// PLA: pull accumulator
const PLA: Instruction = Instruction::new("PLA", 0x68, Implied, 1, 4);

// PLP: pull processor status
const PLP: Instruction = Instruction::new("PLP", 0x28, Implied, 1, 4);

// ROL: rotate left
const ROL_AC: Instruction = Instruction::new("ROL", 0x2a, Accumulator, 1, 2);
const ROL_Z: Instruction = Instruction::new("ROL", 0x26, ZeroPage, 2, 5);
const ROL_ZX: Instruction = Instruction::new("ROL", 0x36, ZeroPageX, 2, 6);
const ROL_A: Instruction = Instruction::new("ROL", 0x2e, Absolute, 3, 6);
const ROL_AX: Instruction = Instruction::new("ROL", 0x3e, AbsoluteX, 3, 7);

// ROR: rotate right
const ROR_AC: Instruction = Instruction::new("ROR", 0x6a, Accumulator, 1, 2);
const ROR_Z: Instruction = Instruction::new("ROR", 0x66, ZeroPage, 2, 5);
const ROR_ZX: Instruction = Instruction::new("ROR", 0x76, ZeroPageX, 2, 6);
const ROR_A: Instruction = Instruction::new("ROR", 0x6e, Absolute, 3, 6);
const ROR_AX: Instruction = Instruction::new("ROR", 0x7e, AbsoluteX, 3, 7);

// RTI: return from interrupt
const RTI: Instruction = Instruction::new("RTI", 0x40, Implied, 1, 6);

// RTS return from subroutine
const RTS: Instruction = Instruction::new("RTS", 0x60, Implied, 1, 6);

// SBC: subtract with carry
const SBC_IM: Instruction = Instruction::new("SBC", 0xe9, Immediate, 2, 2);
const SBC_Z: Instruction = Instruction::new("SBC", 0xe5, ZeroPage, 2, 3);
const SBC_ZX: Instruction = Instruction::new("SBC", 0xf5, ZeroPageX, 2, 4);
const SBC_A: Instruction = Instruction::new("SBC", 0xed, Absolute, 3, 4);
const SBC_AX: Instruction = Instruction::new("SBC", 0xfd, AbsoluteX, 3, 4); //
const SBC_AY: Instruction = Instruction::new("SBC", 0xf9, AbsoluteY, 3, 4); //
const SBC_IX: Instruction = Instruction::new("SBC", 0xe1, Indirect, 2, 6);
const SBC_IY: Instruction = Instruction::new("SBC", 0xf1, IndexedIndirect, 2, 5); //

// SEC: set carry flag
const SEC: Instruction = Instruction::new("SEC", 0x38, Implied, 1, 2);

// SED: set decimal flag
const SED: Instruction = Instruction::new("SED", 0xf8, Implied, 1, 2);

// SEI: set interrupt disable
const SEI: Instruction = Instruction::new("SEI", 0x78, Implied, 1, 2);

// STA: store accumulator
const STA_Z: Instruction = Instruction::new("STA", 0x85, ZeroPage, 2, 3);
const STA_ZX: Instruction = Instruction::new("STA", 0x95, ZeroPageX, 2, 4);
const STA_A: Instruction = Instruction::new("STA", 0x8d, Absolute, 3, 4);
const STA_AX: Instruction = Instruction::new("STA", 0x9d, AbsoluteX, 3, 5);
const STA_AY: Instruction = Instruction::new("STA", 0x99, AbsoluteY, 3, 5);
const STA_IX: Instruction = Instruction::new("STA", 0x81, Indirect, 2, 6);
const STA_IY: Instruction = Instruction::new("STA", 0x91, IndexedIndirect, 2, 6);

// STX: store x register
const STX_Z: Instruction = Instruction::new("STX", 0x86, ZeroPage, 2, 3);
const STX_ZY: Instruction = Instruction::new("STX", 0x96, ZeroPageY, 2, 4);
const STX_A: Instruction = Instruction::new("STX", 0x8e, Absolute, 3, 4);

// STY: store y register
const STY_Z: Instruction = Instruction::new("STY", 0x84, ZeroPage, 2, 3);
const STY_ZX: Instruction = Instruction::new("STY", 0x94, ZeroPageX, 2, 4);
const STY_A: Instruction = Instruction::new("STY", 0x8c, Absolute, 3, 4);

// TAX: transfer accumulator to x
const TAX: Instruction = Instruction::new("TAX", 0xaa, Implied, 1, 2);

// TAY: transfer accumulator to y
const TAY: Instruction = Instruction::new("TAY", 0xa8, Implied, 1, 2);

// TAX: transfer stack pointer to x
const TSX: Instruction = Instruction::new("TSX", 0xba, Implied, 1, 2);

// TXA: transfer x to accumulator
const TXA: Instruction = Instruction::new("TXA", 0x8a, Implied, 1, 2);

// TXS: transfer x to stack pointer
const TXS: Instruction = Instruction::new("TXS", 0x9a, Implied, 1, 2);

// TYA: transfer y to accumulator
const TYA: Instruction = Instruction::new("TYA", 0x98, Implied, 1, 2);
