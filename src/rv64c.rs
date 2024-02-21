use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;
use crate::fixup::FixupKind;

use crate::macros::forward;

use crate::rv32c;


pub trait Emit: EmitSlice {
    fn emit_c_beqz_label<Label>(&mut self, rs: CRegister, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::CBranch,
            |e, offset| {
                e.emit_c_beqz(rs, offset as i16)
            },
        )
    }

    fn emit_c_bnez_label<Label>(&mut self, rs: CRegister, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::CBranch,
            |e, offset| {
                e.emit_c_bnez(rs, offset as i16)
            },
        )
    }

    fn emit_c_j_label<Label>(&mut self, label: &mut Label) -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::CJump,
            |e, offset| {
                e.emit_c_j(offset as i16)
            },
        )
    }

    forward! {
        emit_c_addiw(rd: Register, imm: i8) => addiw;
        emit_c_addw(rd: CRegister, rs: CRegister) => addw;
        emit_c_ld(rd: CRegister, base: CRegister, offset: u8) => ld;
        emit_c_ldsp(rs: Register, offset: u8) => ldsp;
        emit_c_sd(rd: CRegister, base: CRegister, offset: u8) => sd;
        emit_c_sdsp(rs: Register, offset: u8) => sdsp;
        emit_c_sext_w(rd: Register) => sext_w;
        emit_c_slli(rd: Register, shamt: u8) => slli;
        emit_c_srai(rd: CRegister, shamt: u8) => srai;
        emit_c_srli(rd: CRegister, shamt: u8) => srli;
        emit_c_subw(rd: CRegister, rs: CRegister) => subw;
    }

    forward! {
        emit_c_add(rd: Register, rs: Register) => rv32c::add;
        emit_c_addi(rd: Register, imm: i8) => rv32c::addi;
        emit_c_addi4spn(rd: CRegister, imm: u16) => rv32c::addi4spn;
        emit_c_addi16sp(imm: i16) => rv32c::addi16sp;
        emit_c_and(rd: CRegister, rs: CRegister) => rv32c::and;
        emit_c_andi(rd: CRegister, imm: i8) => rv32c::andi;
        emit_c_beqz(rs: CRegister, offset: i16) => rv32c::beqz;
        emit_c_bnez(rs: CRegister, offset: i16) => rv32c::bnez;
        emit_c_ebreak() => rv32c::ebreak;
        emit_c_j(offset: i16) => rv32c::j;
        emit_c_jalr(rs: Register) => rv32c::jalr;
        emit_c_jr(rs: Register) => rv32c::jr;
        emit_c_li(rd: Register, imm: i8) => rv32c::li;
        emit_c_lui(rd: Register, imm: i8) => rv32c::lui;
        emit_c_lw(rd: CRegister, base: CRegister, offset: u8) => rv32c::lw;
        emit_c_lwsp(rs: Register, offset: u8) => rv32c::lwsp;
        emit_c_mv(rd: Register, rs: Register) => rv32c::mv;
        emit_c_nop() => rv32c::nop;
        emit_c_or(rd: CRegister, rs: CRegister) => rv32c::or;
        emit_c_sub(rd: CRegister, rs: CRegister) => rv32c::sub;
        emit_c_sw(rd: CRegister, base: CRegister, offset: u8) => rv32c::sw;
        emit_c_swsp(rs: Register, offset: u8) => rv32c::swsp;
        emit_c_unimp() => rv32c::unimp;
        emit_c_xor(rd: CRegister, rs: CRegister) => rv32c::xor;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn ld(rd: CRegister, base: CRegister, offset: u8) -> u16 {
    encode!(
        i3(0b011),
        i3((offset >> 3) as u32),
        i3(base as u32),
        i2((offset >> 6) as u32),
        i3(rd as u32),
        i2(0b00)
    ) as u16
}

#[inline]
pub fn sd(rd: CRegister, base: CRegister, offset: u8) -> u16 {
    encode!(
        i3(0b111),
        i3((offset >> 3) as u32),
        i3(base as u32),
        i2((offset >> 6) as u32),
        i3(rd as u32),
        i2(0b00)
    ) as u16
}

#[inline]
pub fn addiw(rd: Register, imm: i8) -> u16 {
    CiType { op: 0b01, funct3: 0b001, rd, imm }.encode()
}

#[inline]
pub fn srli(rd: CRegister, shamt: u8) -> u16 {
    encode!(
        i3(0b100),
        i1((shamt >> 5) as u32),
        i2(0b00),
        i3(rd as u32),
        i5(shamt as u32),
        i2(0b01)
    ) as u16
}

#[inline]
pub fn srai(rd: CRegister, shamt: u8) -> u16 {
    encode!(
        i3(0b100),
        i1((shamt >> 5) as u32),
        i2(0b01),
        i3(rd as u32),
        i5(shamt as u32),
        i2(0b01)
    ) as u16
}

#[inline]
pub fn subw(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b00, funct6: 0b100111, rd, rs }.encode()
}

#[inline]
pub fn addw(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b01, funct6: 0b100111, rd, rs }.encode()
}

#[inline]
pub fn slli(rd: Register, shamt: u8) -> u16 {
    CiType { op: 0b10, funct3: 0b000, rd, imm: shamt as i8 & 0x3f }.encode()
}

#[inline]
pub fn ldsp(rd: Register, offset: u8) -> u16 {
    encode!(
        i3(0b011),
        i1((offset >> 5) as u32),
        i5(rd as u32),
        i2((offset >> 3) as u32),
        i3((offset >> 6) as u32),
        i2(0b10)
    ) as u16
}

#[inline]
pub fn sdsp(rs: Register, offset: u8) -> u16 {
    encode!(
        i3(0b111),
        i3((offset >> 3) as u32),
        i3((offset >> 6) as u32),
        i5(rs as u32),
        i2(0b10)
    ) as u16
}

#[inline]
pub fn sext_w(rd: Register) -> u16 {
    addiw(rd, 0)
}
