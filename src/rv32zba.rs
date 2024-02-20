use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;


pub trait Emit: EmitSlice {
    forward! {
        emit_sh1add(rd: Register, rs1: Register, rs2: Register) => sh1add;
        emit_sh2add(rd: Register, rs1: Register, rs2: Register) => sh2add;
        emit_sh3add(rd: Register, rs1: Register, rs2: Register) => sh3add;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn sh1add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b010, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sh2add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b100, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sh3add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b110, funct7: 0b0010000, rd, rs1, rs2 }.encode()
}
