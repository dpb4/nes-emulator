#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    Implicit,        //
    Accumulator,     // AC
    Immediate,       // IM
    ZeroPage,        // Z
    ZeroPageX,       // ZX
    ZeroPageY,       // ZY
    Relative,        // R
    Absolute,        // A
    AbsoluteX,       // AX
    AbsoluteY,       // AY
    Indirect,        // I
    IndexedIndirect, // IX
    IndirectIndexed, // IY
}

#[derive(Debug, Clone, Copy, PartialEq, strum_macros::Display)]
pub enum InstructionName {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    BRK,
    BIT,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

use AddressingMode::*;
use InstructionName as IN;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub name: InstructionName,
    pub opcode: u8,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    pub const fn new(
        name: InstructionName,
        opcode: u8,
        mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
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
pub const ADC_IM: Instruction = Instruction::new(IN::ADC, 0x69, Immediate, 2, 2);
pub const ADC_Z: Instruction = Instruction::new(IN::ADC, 0x65, ZeroPage, 2, 3);
pub const ADC_ZX: Instruction = Instruction::new(IN::ADC, 0x75, ZeroPageX, 2, 4);
pub const ADC_A: Instruction = Instruction::new(IN::ADC, 0x6d, Absolute, 3, 4);
pub const ADC_AX: Instruction = Instruction::new(IN::ADC, 0x7d, AbsoluteX, 3, 4); // ex
pub const ADC_AY: Instruction = Instruction::new(IN::ADC, 0x79, AbsoluteY, 3, 4); // ex
pub const ADC_IX: Instruction = Instruction::new(IN::ADC, 0x61, IndirectIndexed, 2, 6);
pub const ADC_IY: Instruction = Instruction::new(IN::ADC, 0x71, IndexedIndirect, 2, 5); // ex

// AND: arithmetic and
pub const AND_IM: Instruction = Instruction::new(IN::AND, 0x29, Immediate, 2, 2);
pub const AND_Z: Instruction = Instruction::new(IN::AND, 0x25, ZeroPage, 2, 3);
pub const AND_ZX: Instruction = Instruction::new(IN::AND, 0x35, ZeroPageX, 2, 4);
pub const AND_A: Instruction = Instruction::new(IN::AND, 0x2d, Absolute, 3, 4);
pub const AND_AX: Instruction = Instruction::new(IN::AND, 0x3d, AbsoluteX, 3, 4); // ex
pub const AND_AY: Instruction = Instruction::new(IN::AND, 0x39, AbsoluteY, 3, 4); // ex
pub const AND_IX: Instruction = Instruction::new(IN::AND, 0x21, IndirectIndexed, 2, 6);
pub const AND_IY: Instruction = Instruction::new(IN::AND, 0x31, IndexedIndirect, 2, 5); // ex

// ASL: arithmetic shift left
pub const ASL_AC: Instruction = Instruction::new(IN::ASL, 0x0a, Accumulator, 1, 2);
pub const ASL_Z: Instruction = Instruction::new(IN::ASL, 0x06, ZeroPage, 2, 3);
pub const ASL_ZX: Instruction = Instruction::new(IN::ASL, 0x16, ZeroPageX, 2, 4);
pub const ASL_A: Instruction = Instruction::new(IN::ASL, 0x0e, Absolute, 3, 4);
pub const ASL_AX: Instruction = Instruction::new(IN::ASL, 0x1e, AbsoluteX, 3, 4);

// BCC: branch if carry clear
pub const BCC: Instruction = Instruction::new(IN::BCC, 0x90, Relative, 2, 2); // ex

// BCS: branch if carry set
pub const BCS: Instruction = Instruction::new(IN::BCS, 0xb0, Relative, 2, 2); // ex

// BEQ: branch if equal
pub const BEQ: Instruction = Instruction::new(IN::BEQ, 0xf0, Relative, 2, 2); // ex

// BMI: branch if minus
pub const BMI: Instruction = Instruction::new(IN::BMI, 0x30, Relative, 2, 2); // ex

// BNE: branch if not equal
pub const BNE: Instruction = Instruction::new(IN::BNE, 0xd0, Relative, 2, 2); // ex

// BPL: branch if positive
pub const BPL: Instruction = Instruction::new(IN::BPL, 0xd0, Relative, 2, 2); // ex

// BVC: branch if overflow clear
pub const BVC: Instruction = Instruction::new(IN::BVC, 0x50, Relative, 2, 2); // ex

// BVS: branch if overflow set
pub const BVS: Instruction = Instruction::new(IN::BVS, 0x70, Relative, 2, 2); // ex

// BRK: force interrupt
pub const BRK: Instruction = Instruction::new(IN::BRK, 0x00, Implicit, 2, 7);

// BIT: bit test
pub const BIT_Z: Instruction = Instruction::new(IN::BIT, 0x24, ZeroPage, 2, 3);
pub const BIT_A: Instruction = Instruction::new(IN::BIT, 0x2c, Absolute, 3, 4);

// CLC: clear carry flag
pub const CLC: Instruction = Instruction::new(IN::CLC, 0x18, Implicit, 1, 2);

// CLD: clear decimal mode
pub const CLD: Instruction = Instruction::new(IN::CLD, 0xd8, Implicit, 1, 2);

// CLI: clear interrupt disable
pub const CLI: Instruction = Instruction::new(IN::CLI, 0x58, Implicit, 1, 2);

// CLV: clear overflow flag
pub const CLV: Instruction = Instruction::new(IN::CLV, 0xb8, Implicit, 1, 2);

// CMP: compare
pub const CMP_IM: Instruction = Instruction::new(IN::CMP, 0xc9, Immediate, 2, 2);
pub const CMP_Z: Instruction = Instruction::new(IN::CMP, 0xc5, Immediate, 2, 3);
pub const CMP_ZX: Instruction = Instruction::new(IN::CMP, 0xd5, ZeroPageX, 2, 4);
pub const CMP_A: Instruction = Instruction::new(IN::CMP, 0xcd, Absolute, 3, 4);
pub const CMP_AX: Instruction = Instruction::new(IN::CMP, 0xdd, AbsoluteX, 3, 4); // ex
pub const CMP_AY: Instruction = Instruction::new(IN::CMP, 0xd9, AbsoluteY, 3, 4); // ex
pub const CMP_IX: Instruction = Instruction::new(IN::CMP, 0xc1, IndirectIndexed, 2, 6);
pub const CMP_IY: Instruction = Instruction::new(IN::CMP, 0xd1, IndexedIndirect, 2, 5); // ex

// CPX: compare x register
pub const CPX_IM: Instruction = Instruction::new(IN::CPX, 0xe0, Immediate, 2, 2);
pub const CPX_Z: Instruction = Instruction::new(IN::CPX, 0xe4, ZeroPage, 2, 3);
pub const CPX_A: Instruction = Instruction::new(IN::CPX, 0xec, Absolute, 3, 4);

// CPY: compare y register
pub const CPY_IM: Instruction = Instruction::new(IN::CPY, 0xc0, Immediate, 2, 2);
pub const CPY_Z: Instruction = Instruction::new(IN::CPY, 0xc4, ZeroPage, 2, 3);
pub const CPY_A: Instruction = Instruction::new(IN::CPY, 0xcc, Absolute, 3, 4);

// DEC: decrement memory
pub const DEC_Z: Instruction = Instruction::new(IN::DEC, 0xc6, ZeroPage, 2, 5);
pub const DEC_ZX: Instruction = Instruction::new(IN::DEC, 0xd6, ZeroPageX, 2, 6);
pub const DEC_A: Instruction = Instruction::new(IN::DEC, 0xce, Absolute, 3, 6);
pub const DEC_AX: Instruction = Instruction::new(IN::DEC, 0xde, AbsoluteX, 3, 7);

// DEX: decrement x register
pub const DEX: Instruction = Instruction::new(IN::DEX, 0xca, Implicit, 1, 2);

// DEY: decrement Y register
pub const DEY: Instruction = Instruction::new(IN::DEY, 0x88, Implicit, 1, 2);

// EOR: exclusive or
pub const EOR_IM: Instruction = Instruction::new(IN::EOR, 0x49, Immediate, 2, 2);
pub const EOR_Z: Instruction = Instruction::new(IN::EOR, 0x45, ZeroPage, 2, 3);
pub const EOR_ZX: Instruction = Instruction::new(IN::EOR, 0x55, ZeroPageX, 2, 4);
pub const EOR_A: Instruction = Instruction::new(IN::EOR, 0x4d, Absolute, 3, 4);
pub const EOR_AX: Instruction = Instruction::new(IN::EOR, 0x5d, AbsoluteX, 3, 4); // ex
pub const EOR_AY: Instruction = Instruction::new(IN::EOR, 0x59, AbsoluteY, 3, 4); // ex
pub const EOR_IX: Instruction = Instruction::new(IN::EOR, 0x41, IndirectIndexed, 2, 6);
pub const EOR_IY: Instruction = Instruction::new(IN::EOR, 0x51, IndexedIndirect, 2, 5); // ex

// INC: increment memory
pub const INC_Z: Instruction = Instruction::new(IN::INC, 0xe6, ZeroPage, 2, 5);
pub const INC_ZX: Instruction = Instruction::new(IN::INC, 0xf6, ZeroPageX, 2, 6);
pub const INC_A: Instruction = Instruction::new(IN::INC, 0xee, Absolute, 3, 6);
pub const INC_AX: Instruction = Instruction::new(IN::INC, 0xfe, AbsoluteX, 3, 7);

// INX: increment x register
pub const INX: Instruction = Instruction::new(IN::INX, 0xe8, Implicit, 1, 2);

// INY: increment Y register
pub const INY: Instruction = Instruction::new(IN::INY, 0xc8, Implicit, 1, 2);

// JMP: jump
pub const JMP_A: Instruction = Instruction::new(IN::JMP, 0x4c, Absolute, 3, 3);
pub const JMP_I: Instruction = Instruction::new(IN::JMP, 0x6c, Indirect, 3, 5);

// JSR: jump to subroutine
pub const JSR_A: Instruction = Instruction::new(IN::JSR, 0x20, Absolute, 3, 6);

// LDA: load accumulator
pub const LDA_IM: Instruction = Instruction::new(IN::LDA, 0xa9, Immediate, 2, 2);
pub const LDA_Z: Instruction = Instruction::new(IN::LDA, 0xa5, ZeroPage, 2, 3);
pub const LDA_ZX: Instruction = Instruction::new(IN::LDA, 0xb5, ZeroPageX, 2, 4);
pub const LDA_A: Instruction = Instruction::new(IN::LDA, 0xad, Absolute, 3, 4);
pub const LDA_AX: Instruction = Instruction::new(IN::LDA, 0xbd, AbsoluteX, 3, 4); // ex
pub const LDA_AY: Instruction = Instruction::new(IN::LDA, 0xb9, AbsoluteY, 3, 4); // ex
pub const LDA_IX: Instruction = Instruction::new(IN::LDA, 0xa1, IndirectIndexed, 2, 6);
pub const LDA_IY: Instruction = Instruction::new(IN::LDA, 0xb1, IndexedIndirect, 2, 5); // ex

// LDX: load x register
pub const LDX_IM: Instruction = Instruction::new(IN::LDX, 0xa2, Immediate, 2, 2);
pub const LDX_Z: Instruction = Instruction::new(IN::LDX, 0xa6, ZeroPage, 2, 3);
pub const LDX_ZY: Instruction = Instruction::new(IN::LDX, 0xb6, ZeroPageY, 2, 4);
pub const LDX_A: Instruction = Instruction::new(IN::LDX, 0xae, Absolute, 3, 4);
pub const LDX_AY: Instruction = Instruction::new(IN::LDX, 0xbe, AbsoluteY, 3, 4); // ex

// LDY: load y register
pub const LDY_IM: Instruction = Instruction::new(IN::LDY, 0xa0, Immediate, 2, 2);
pub const LDY_Z: Instruction = Instruction::new(IN::LDY, 0xa4, ZeroPage, 2, 3);
pub const LDY_ZX: Instruction = Instruction::new(IN::LDY, 0xb4, ZeroPageX, 2, 4);
pub const LDY_A: Instruction = Instruction::new(IN::LDY, 0xac, Absolute, 3, 4);
pub const LDY_AX: Instruction = Instruction::new(IN::LDY, 0xbc, AbsoluteX, 3, 4); // ex

// LSR: logical shift right
pub const LSR_AC: Instruction = Instruction::new(IN::LSR, 0x4a, Accumulator, 1, 2);
pub const LSR_Z: Instruction = Instruction::new(IN::LSR, 0x46, ZeroPage, 2, 5);
pub const LSR_ZX: Instruction = Instruction::new(IN::LSR, 0x56, ZeroPageX, 2, 6);
pub const LSR_A: Instruction = Instruction::new(IN::LSR, 0x4e, Absolute, 3, 6);
pub const LSR_AX: Instruction = Instruction::new(IN::LSR, 0x5e, AbsoluteX, 3, 7);

// NOP: no operation
pub const NOP: Instruction = Instruction::new(IN::NOP, 0xea, Implicit, 1, 2);

// ORA: logical or
pub const ORA_IM: Instruction = Instruction::new(IN::ORA, 0x09, Immediate, 2, 2);
pub const ORA_Z: Instruction = Instruction::new(IN::ORA, 0x05, ZeroPage, 2, 3);
pub const ORA_ZX: Instruction = Instruction::new(IN::ORA, 0x15, ZeroPageX, 2, 4);
pub const ORA_A: Instruction = Instruction::new(IN::ORA, 0x0d, Absolute, 3, 4);
pub const ORA_AX: Instruction = Instruction::new(IN::ORA, 0x1d, AbsoluteX, 3, 4); // ex
pub const ORA_AY: Instruction = Instruction::new(IN::ORA, 0x19, AbsoluteY, 3, 4); // ex
pub const ORA_IX: Instruction = Instruction::new(IN::ORA, 0x01, IndirectIndexed, 2, 6);
pub const ORA_IY: Instruction = Instruction::new(IN::ORA, 0x11, IndexedIndirect, 2, 5); // ex

// PHA: push accumulator
pub const PHA: Instruction = Instruction::new(IN::PHA, 0x48, Implicit, 1, 3);

// PHP: push processor status
pub const PHP: Instruction = Instruction::new(IN::PHP, 0x08, Implicit, 1, 3);

// PLA: pull accumulator
pub const PLA: Instruction = Instruction::new(IN::PLA, 0x68, Implicit, 1, 4);

// PLP: pull processor status
pub const PLP: Instruction = Instruction::new(IN::PLP, 0x28, Implicit, 1, 4);

// ROL: rotate left
pub const ROL_AC: Instruction = Instruction::new(IN::ROL, 0x2a, Accumulator, 1, 2);
pub const ROL_Z: Instruction = Instruction::new(IN::ROL, 0x26, ZeroPage, 2, 5);
pub const ROL_ZX: Instruction = Instruction::new(IN::ROL, 0x36, ZeroPageX, 2, 6);
pub const ROL_A: Instruction = Instruction::new(IN::ROL, 0x2e, Absolute, 3, 6);
pub const ROL_AX: Instruction = Instruction::new(IN::ROL, 0x3e, AbsoluteX, 3, 7);

// ROR: rotate right
pub const ROR_AC: Instruction = Instruction::new(IN::ROR, 0x6a, Accumulator, 1, 2);
pub const ROR_Z: Instruction = Instruction::new(IN::ROR, 0x66, ZeroPage, 2, 5);
pub const ROR_ZX: Instruction = Instruction::new(IN::ROR, 0x76, ZeroPageX, 2, 6);
pub const ROR_A: Instruction = Instruction::new(IN::ROR, 0x6e, Absolute, 3, 6);
pub const ROR_AX: Instruction = Instruction::new(IN::ROR, 0x7e, AbsoluteX, 3, 7);

// RTI: return from interrupt
pub const RTI: Instruction = Instruction::new(IN::RTI, 0x40, Implicit, 1, 6);

// RTS return from subroutine
pub const RTS: Instruction = Instruction::new(IN::RTS, 0x60, Implicit, 1, 6);

// SBC: subtract with carry
pub const SBC_IM: Instruction = Instruction::new(IN::SBC, 0xe9, Immediate, 2, 2);
pub const SBC_Z: Instruction = Instruction::new(IN::SBC, 0xe5, ZeroPage, 2, 3);
pub const SBC_ZX: Instruction = Instruction::new(IN::SBC, 0xf5, ZeroPageX, 2, 4);
pub const SBC_A: Instruction = Instruction::new(IN::SBC, 0xed, Absolute, 3, 4);
pub const SBC_AX: Instruction = Instruction::new(IN::SBC, 0xfd, AbsoluteX, 3, 4); // ex
pub const SBC_AY: Instruction = Instruction::new(IN::SBC, 0xf9, AbsoluteY, 3, 4); // ex
pub const SBC_IX: Instruction = Instruction::new(IN::SBC, 0xe1, IndirectIndexed, 2, 6);
pub const SBC_IY: Instruction = Instruction::new(IN::SBC, 0xf1, IndexedIndirect, 2, 5); // ex

// SEC: set carry flag
pub const SEC: Instruction = Instruction::new(IN::SEC, 0x38, Implicit, 1, 2);

// SED: set decimal flag
pub const SED: Instruction = Instruction::new(IN::SED, 0xf8, Implicit, 1, 2);

// SEI: set interrupt disable
pub const SEI: Instruction = Instruction::new(IN::SEI, 0x78, Implicit, 1, 2);

// STA: store accumulator
pub const STA_Z: Instruction = Instruction::new(IN::STA, 0x85, ZeroPage, 2, 3);
pub const STA_ZX: Instruction = Instruction::new(IN::STA, 0x95, ZeroPageX, 2, 4);
pub const STA_A: Instruction = Instruction::new(IN::STA, 0x8d, Absolute, 3, 4);
pub const STA_AX: Instruction = Instruction::new(IN::STA, 0x9d, AbsoluteX, 3, 5);
pub const STA_AY: Instruction = Instruction::new(IN::STA, 0x99, AbsoluteY, 3, 5);
pub const STA_IX: Instruction = Instruction::new(IN::STA, 0x81, IndirectIndexed, 2, 6);
pub const STA_IY: Instruction = Instruction::new(IN::STA, 0x91, IndexedIndirect, 2, 6);

// STX: store x register
pub const STX_Z: Instruction = Instruction::new(IN::STX, 0x86, ZeroPage, 2, 3);
pub const STX_ZY: Instruction = Instruction::new(IN::STX, 0x96, ZeroPageY, 2, 4);
pub const STX_A: Instruction = Instruction::new(IN::STX, 0x8e, Absolute, 3, 4);

// STY: store y register
pub const STY_Z: Instruction = Instruction::new(IN::STY, 0x84, ZeroPage, 2, 3);
pub const STY_ZX: Instruction = Instruction::new(IN::STY, 0x94, ZeroPageX, 2, 4);
pub const STY_A: Instruction = Instruction::new(IN::STY, 0x8c, Absolute, 3, 4);

// TAX: transfer accumulator to x
pub const TAX: Instruction = Instruction::new(IN::TAX, 0xaa, Implicit, 1, 2);

// TAY: transfer accumulator to y
pub const TAY: Instruction = Instruction::new(IN::TAY, 0xa8, Implicit, 1, 2);

// TAX: transfer stack pointer to x
pub const TSX: Instruction = Instruction::new(IN::TSX, 0xba, Implicit, 1, 2);

// TXA: transfer x to accumulator
pub const TXA: Instruction = Instruction::new(IN::TXA, 0x8a, Implicit, 1, 2);

// TXS: transfer x to stack pointer
pub const TXS: Instruction = Instruction::new(IN::TXS, 0x9a, Implicit, 1, 2);

// TYA: transfer y to accumulator
pub const TYA: Instruction = Instruction::new(IN::TYA, 0x98, Implicit, 1, 2);

pub const fn get_instruction(opcode: u8) -> &'static Instruction {
    match opcode {
        0x69 => &ADC_IM,
        0x65 => &ADC_Z,
        0x75 => &ADC_ZX,
        0x6d => &ADC_A,
        0x7d => &ADC_AX,
        0x79 => &ADC_AY,
        0x61 => &ADC_IX,
        0x71 => &ADC_IY,
        0x29 => &AND_IM,
        0x25 => &AND_Z,
        0x35 => &AND_ZX,
        0x2d => &AND_A,
        0x3d => &AND_AX,
        0x39 => &AND_AY,
        0x21 => &AND_IX,
        0x31 => &AND_IY,
        0x0a => &ASL_AC,
        0x06 => &ASL_Z,
        0x16 => &ASL_ZX,
        0x0e => &ASL_A,
        0x1e => &ASL_AX,
        0x90 => &BCC,
        0xb0 => &BCS,
        0xf0 => &BEQ,
        0x30 => &BMI,
        0xd0 => &BNE,
        0x10 => &BPL,
        0x50 => &BVC,
        0x70 => &BVS,
        0x00 => &BRK,
        0x24 => &BIT_Z,
        0x2c => &BIT_A,
        0x18 => &CLC,
        0xd8 => &CLD,
        0x58 => &CLI,
        0xb8 => &CLV,
        0xc9 => &CMP_IM,
        0xc5 => &CMP_Z,
        0xd5 => &CMP_ZX,
        0xcd => &CMP_A,
        0xdd => &CMP_AX,
        0xd9 => &CMP_AY,
        0xc1 => &CMP_IX,
        0xd1 => &CMP_IY,
        0xe0 => &CPX_IM,
        0xe4 => &CPX_Z,
        0xec => &CPX_A,
        0xc0 => &CPY_IM,
        0xc4 => &CPY_Z,
        0xcc => &CPY_A,
        0xc6 => &DEC_Z,
        0xd6 => &DEC_ZX,
        0xce => &DEC_A,
        0xde => &DEC_AX,
        0xca => &DEX,
        0x88 => &DEY,
        0x49 => &EOR_IM,
        0x45 => &EOR_Z,
        0x55 => &EOR_ZX,
        0x4d => &EOR_A,
        0x5d => &EOR_AX,
        0x59 => &EOR_AY,
        0x41 => &EOR_IX,
        0x51 => &EOR_IY,
        0xe6 => &INC_Z,
        0xf6 => &INC_ZX,
        0xee => &INC_A,
        0xfe => &INC_AX,
        0xe8 => &INX,
        0xc8 => &INY,
        0x4c => &JMP_A,
        0x6c => &JMP_I,
        0x20 => &JSR_A,
        0xa9 => &LDA_IM,
        0xa5 => &LDA_Z,
        0xb5 => &LDA_ZX,
        0xad => &LDA_A,
        0xbd => &LDA_AX,
        0xb9 => &LDA_AY,
        0xa1 => &LDA_IX,
        0xb1 => &LDA_IY,
        0xa2 => &LDX_IM,
        0xa6 => &LDX_Z,
        0xb6 => &LDX_ZY,
        0xae => &LDX_A,
        0xbe => &LDX_AY,
        0xa0 => &LDY_IM,
        0xa4 => &LDY_Z,
        0xb4 => &LDY_ZX,
        0xac => &LDY_A,
        0xbc => &LDY_AX,
        0x4a => &LSR_AC,
        0x46 => &LSR_Z,
        0x56 => &LSR_ZX,
        0x4e => &LSR_A,
        0x5e => &LSR_AX,
        0xea => &NOP,
        0x09 => &ORA_IM,
        0x05 => &ORA_Z,
        0x15 => &ORA_ZX,
        0x0d => &ORA_A,
        0x1d => &ORA_AX,
        0x19 => &ORA_AY,
        0x01 => &ORA_IX,
        0x11 => &ORA_IY,
        0x48 => &PHA,
        0x08 => &PHP,
        0x68 => &PLA,
        0x28 => &PLP,
        0x2a => &ROL_AC,
        0x26 => &ROL_Z,
        0x36 => &ROL_ZX,
        0x2e => &ROL_A,
        0x3e => &ROL_AX,
        0x6a => &ROR_AC,
        0x66 => &ROR_Z,
        0x76 => &ROR_ZX,
        0x6e => &ROR_A,
        0x7e => &ROR_AX,
        0x40 => &RTI,
        0x60 => &RTS,
        0xe9 => &SBC_IM,
        0xe5 => &SBC_Z,
        0xf5 => &SBC_ZX,
        0xed => &SBC_A,
        0xfd => &SBC_AX,
        0xf9 => &SBC_AY,
        0xe1 => &SBC_IX,
        0xf1 => &SBC_IY,
        0x38 => &SEC,
        0xf8 => &SED,
        0x78 => &SEI,
        0x85 => &STA_Z,
        0x95 => &STA_ZX,
        0x8d => &STA_A,
        0x9d => &STA_AX,
        0x99 => &STA_AY,
        0x81 => &STA_IX,
        0x91 => &STA_IY,
        0x86 => &STX_Z,
        0x96 => &STX_ZY,
        0x8e => &STX_A,
        0x84 => &STY_Z,
        0x94 => &STY_ZX,
        0x8c => &STY_A,
        0xaa => &TAX,
        0xa8 => &TAY,
        0xba => &TSX,
        0x8a => &TXA,
        0x9a => &TXS,
        0x98 => &TYA,
        _ => panic!("bad opcode"),
    }
}
