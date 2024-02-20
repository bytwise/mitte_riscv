use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;

use crate::rv32zbs;


pub trait Emit: EmitSlice {
    forward! {
        emit_bclri(rd: Register, rs: Register, shamt: u8) => bclri;
        emit_bexti(rd: Register, rs: Register, shamt: u8) => bexti;
        emit_binvi(rd: Register, rs: Register, shamt: u8) => binvi;
        emit_bseti(rd: Register, rs: Register, shamt: u8) => bseti;
    }

    forward! {
        emit_bclr(rd: Register, rs1: Register, rs2: Register) => rv32zbs::bclr;
        emit_bext(rd: Register, rs1: Register, rs2: Register) => rv32zbs::bext;
        emit_binv(rd: Register, rs1: Register, rs2: Register) => rv32zbs::binv;
        emit_bset(rd: Register, rs1: Register, rs2: Register) => rv32zbs::bset;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn bclri(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b010010), i6(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}

#[inline]
pub fn bexti(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b010010), i6(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn binvi(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b011010), i6(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}

#[inline]
pub fn bseti(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b001010), i6(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}
