use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;

use crate::rv32zba;


pub trait Emit: EmitSlice {
    forward! {
        emit_add_uw(rd: Register, rs1: Register, rs2: Register) => add_uw;
        emit_sh1add_uw(rd: Register, rs1: Register, rs2: Register) => sh1add_uw;
        emit_sh2add_uw(rd: Register, rs1: Register, rs2: Register) => sh2add_uw;
        emit_sh3add_uw(rd: Register, rs1: Register, rs2: Register) => sh3add_uw;
        emit_slli_uw(rd: Register, rs: Register, shamt: u8) => slli_uw;
    }

    forward! {
        emit_sh1add(rd: Register, rs1: Register, rs2: Register) => rv32zba::sh1add;
        emit_sh2add(rd: Register, rs1: Register, rs2: Register) => rv32zba::sh2add;
        emit_sh3add(rd: Register, rs1: Register, rs2: Register) => rv32zba::sh3add;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn add_uw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b000, funct7: 0b0000100, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sh1add_uw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b010, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sh2add_uw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b100, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sh3add_uw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b110, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn slli_uw(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b10), i6(shamt as u32)) as i16;
    IType { opcode: OpImm32, funct3: 0b001, rd, rs, imm12 }.encode()
}
