use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;


pub trait Emit: EmitSlice {
    forward! {
        emit_div(rd: Register, rs1: Register, rs2: Register) => div;
        emit_divu(rd: Register, rs1: Register, rs2: Register) => divu;
        emit_mul(rd: Register, rs1: Register, rs2: Register) => mul;
        emit_mulh(rd: Register, rs1: Register, rs2: Register) => mulh;
        emit_mulhsu(rd: Register, rs1: Register, rs2: Register) => mulhsu;
        emit_mulhu(rd: Register, rs1: Register, rs2: Register) => mulhu;
        emit_rem(rd: Register, rs1: Register, rs2: Register) => rem;
        emit_remu(rd: Register, rs1: Register, rs2: Register) => remu;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn mul(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b000, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn mulh(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn mulhsu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b010, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn mulhu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b011, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn div(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b100, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn divu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn rem(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b110, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn remu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b111, funct7: 1, rd, rs1, rs2 }.encode()
}
