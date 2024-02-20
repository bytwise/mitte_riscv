use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;


pub trait Emit: EmitSlice {
    forward! {
        emit_andn(rd: Register, rs1: Register, rs2: Register) => andn;
        emit_clz(rd: Register, rs: Register) => clz;
        emit_cpop(rd: Register, rs: Register) => cpop;
        emit_ctz(rd: Register, rs: Register) => ctz;
        emit_max(rd: Register, rs1: Register, rs2: Register) => max;
        emit_maxu(rd: Register, rs1: Register, rs2: Register) => maxu;
        emit_min(rd: Register, rs1: Register, rs2: Register) => min;
        emit_minu(rd: Register, rs1: Register, rs2: Register) => minu;
        emit_orc_b(rd: Register, rs: Register) => orc_b;
        emit_orn(rd: Register, rs1: Register, rs2: Register) => orn;
        emit_rev8(rd: Register, rs: Register) => rev8;
        emit_rol(rd: Register, rs1: Register, rs2: Register) => rol;
        emit_ror(rd: Register, rs1: Register, rs2: Register) => ror;
        emit_rori(rd: Register, rs: Register, shamt: u8) => rori;
        emit_sext_b(rd: Register, rs: Register) => sext_b;
        emit_sext_h(rd: Register, rs: Register) => sext_h;
        emit_xnor(rd: Register, rs1: Register, rs2: Register) => xnor;
        emit_zext_h(rd: Register, rs: Register) => zext_h;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn andn(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b111, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn clz(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b001, imm12: 0b0110000_00000, rd, rs }.encode()
}

#[inline]
pub fn cpop(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b001, imm12: 0b0110000_00010, rd, rs }.encode()
}

#[inline]
pub fn ctz(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b001, imm12: 0b0110000_00001, rd, rs }.encode()
}

#[inline]
pub fn max(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b110, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn maxu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b111, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn min(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b100, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn minu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn orc_b(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b101, imm12: 0b0010100_00111, rd, rs }.encode()
}

#[inline]
pub fn orn(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b110, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn rev8(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b101, imm12: 0b0110100_11000, rd, rs }.encode()
}

#[inline]
pub fn rol(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0b0110000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn ror(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 0b0110000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn rori(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0110000), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn sext_b(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b001, imm12: 0b0110000_00100, rd, rs }.encode()
}

#[inline]
pub fn sext_h(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b001, imm12: 0b0110000_00101, rd, rs }.encode()
}

#[inline]
pub fn xnor(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b100, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn zext_h(rd: Register, rs: Register) -> u32 {
    IType { opcode: Op, funct3: 0b100, imm12: 0b0000100_00000, rd, rs }.encode()
}
