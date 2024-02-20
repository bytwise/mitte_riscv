use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;

use crate::macros::forward;

use crate::rv32zbb;


pub trait Emit: EmitSlice {
    forward! {
        emit_clzw(rd: Register, rs: Register) => clzw;
        emit_cpopw(rd: Register, rs: Register) => cpopw;
        emit_ctzw(rd: Register, rs: Register) => ctzw;
        emit_rev8(rd: Register, rs: Register) => rev8;
        emit_rolw(rd: Register, rs1: Register, rs2: Register) => rolw;
        emit_rori(rd: Register, rs: Register, shamt: u8) => rori;
        emit_roriw(rd: Register, rs: Register, shamt: u8) => roriw;
        emit_rorw(rd: Register, rs1: Register, rs2: Register) => rorw;
        emit_zext_h(rd: Register, rs: Register) => zext_h;
    }

    forward! {
        emit_andn(rd: Register, rs1: Register, rs2: Register) => rv32zbb::andn;
        emit_clz(rd: Register, rs: Register) => rv32zbb::clz;
        emit_cpop(rd: Register, rs: Register) => rv32zbb::cpop;
        emit_ctz(rd: Register, rs: Register) => rv32zbb::ctz;
        emit_max(rd: Register, rs1: Register, rs2: Register) => rv32zbb::max;
        emit_maxu(rd: Register, rs1: Register, rs2: Register) => rv32zbb::maxu;
        emit_min(rd: Register, rs1: Register, rs2: Register) => rv32zbb::min;
        emit_minu(rd: Register, rs1: Register, rs2: Register) => rv32zbb::minu;
        emit_orc_b(rd: Register, rs: Register) => rv32zbb::orc_b;
        emit_orn(rd: Register, rs1: Register, rs2: Register) => rv32zbb::orn;
        emit_rol(rd: Register, rs1: Register, rs2: Register) => rv32zbb::rol;
        emit_ror(rd: Register, rs1: Register, rs2: Register) => rv32zbb::ror;
        emit_sext_b(rd: Register, rs: Register) => rv32zbb::sext_b;
        emit_sext_h(rd: Register, rs: Register) => rv32zbb::sext_h;
        emit_xnor(rd: Register, rs1: Register, rs2: Register) => rv32zbb::xnor;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn clzw(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm32, funct3: 0b001, imm12: 0b0110000_00000, rd, rs }.encode()
}

#[inline]
pub fn cpopw(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm32, funct3: 0b001, imm12: 0b0110000_00010, rd, rs }.encode()
}

#[inline]
pub fn ctzw(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm32, funct3: 0b001, imm12: 0b0110000_00001, rd, rs }.encode()
}

#[inline]
pub fn rev8(rd: Register, rs: Register) -> u32 {
    IType { opcode: OpImm, funct3: 0b101, imm12: 0b0110101_11000, rd, rs }.encode()
}

#[inline]
pub fn rolw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b001, funct7: 0b0110000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn rori(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i6(0b011000), i6(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn roriw(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0110000), i5(shamt as u32)) as i16;
    IType { opcode: OpImm32, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn rorw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op32, funct3: 0b101, funct7: 0b0110000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn zext_h(rd: Register, rs: Register) -> u32 {
    IType { opcode: Op32, funct3: 0b100, imm12: 0b0000100_00000, rd, rs }.encode()
}
