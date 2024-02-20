use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;


pub trait Emit: EmitSlice {
    forward! {
        emit_bclr(rd: Register, rs1: Register, rs2: Register) => bclr;
        emit_bclri(rd: Register, rs: Register, shamt: u8) => bclri;
        emit_bext(rd: Register, rs1: Register, rs2: Register) => bext;
        emit_bexti(rd: Register, rs: Register, shamt: u8) => bexti;
        emit_binv(rd: Register, rs1: Register, rs2: Register) => binv;
        emit_binvi(rd: Register, rs: Register, shamt: u8) => binvi;
        emit_bset(rd: Register, rs1: Register, rs2: Register) => bset;
        emit_bseti(rd: Register, rs: Register, shamt: u8) => bseti;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn bclr(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0b0100100, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn bclri(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0100100), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}

#[inline]
pub fn bext(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 0b0100100, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn bexti(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0100100), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn binv(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0b0110100, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn binvi(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0110100), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}

#[inline]
pub fn bset(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0b0010100, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn bseti(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0010100), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}
