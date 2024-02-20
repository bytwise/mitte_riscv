use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;


pub trait Emit: EmitSlice {
    forward! {
        emit_clmul(rd: Register, rs1: Register, rs2: Register) => clmul;
        emit_clmulh(rd: Register, rs1: Register, rs2: Register) => clmulh;
        emit_clmulr(rd: Register, rs1: Register, rs2: Register) => clmulr;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn clmul(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn clmulh(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b011, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn clmulr(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b010, funct7: 0b0000101, rd, rs1, rs2 }.encode()
}
