use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;

use crate::rv32m;


pub trait Emit: EmitSlice {
    forward! {
        emit_divuw(rd: Register, rs1: Register, rs2: Register) => divuw;
        emit_divw(rd: Register, rs1: Register, rs2: Register) => divw;
        emit_mulw(rd: Register, rs1: Register, rs2: Register) => mulw;
        emit_remuw(rd: Register, rs1: Register, rs2: Register) => remuw;
        emit_remw(rd: Register, rs1: Register, rs2: Register) => remw;
    }

    forward! {
        emit_div(rd: Register, rs1: Register, rs2: Register) => rv32m::div;
        emit_divu(rd: Register, rs1: Register, rs2: Register) => rv32m::divu;
        emit_mul(rd: Register, rs1: Register, rs2: Register) => rv32m::mul;
        emit_mulh(rd: Register, rs1: Register, rs2: Register) => rv32m::mulh;
        emit_mulhsu(rd: Register, rs1: Register, rs2: Register) => rv32m::mulhsu;
        emit_mulhu(rd: Register, rs1: Register, rs2: Register) => rv32m::mulhu;
        emit_rem(rd: Register, rs1: Register, rs2: Register) => rv32m::rem;
        emit_remu(rd: Register, rs1: Register, rs2: Register) => rv32m::remu;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn mulw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b000, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn divw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b100, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn divuw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b101, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn remw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b110, funct7: 1, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn remuw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b111, funct7: 1, rd, rs1, rs2 }.encode()
}
