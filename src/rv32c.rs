use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;
use crate::fixup::FixupKind;

use crate::macros::forward;


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
        emit_c_add(rd: Register, rs: Register) => add;
        emit_c_addi(rd: Register, imm: i8) => addi;
        emit_c_addi4spn(rd: CRegister, imm: u16) => addi4spn;
        emit_c_addi16sp(imm: i16) => addi16sp;
        emit_c_and(rd: CRegister, rs: CRegister) => and;
        emit_c_andi(rd: CRegister, imm: i8) => andi;
        emit_c_beqz(rs: CRegister, offset: i16) => beqz;
        emit_c_bnez(rs: CRegister, offset: i16) => bnez;
        emit_c_ebreak() => ebreak;
        emit_c_j(offset: i16) => j;
        emit_c_jalr(rs: Register) => jalr;
        emit_c_jr(rs: Register) => jr;
        emit_c_li(rd: Register, imm: i8) => li;
        emit_c_lui(rd: Register, imm: i8) => lui;
        emit_c_lw(rd: CRegister, base: CRegister, offset: u8) => lw;
        emit_c_lwsp(rs: Register, offset: u8) => lwsp;
        emit_c_mv(rd: Register, rs: Register) => mv;
        emit_c_nop() => nop;
        emit_c_or(rd: CRegister, rs: CRegister) => or;
        emit_c_slli(rd: Register, shamt: u8) => slli;
        emit_c_srai(rd: CRegister, shamt: u8) => srai;
        emit_c_srli(rd: CRegister, shamt: u8) => srli;
        emit_c_sub(rd: CRegister, rs: CRegister) => sub;
        emit_c_sw(rd: CRegister, base: CRegister, offset: u8) => sw;
        emit_c_swsp(rs: Register, offset: u8) => swsp;
        emit_c_unimp() => unimp;
        emit_c_xor(rd: CRegister, rs: CRegister) => xor;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn unimp() -> u16 {
    0
}

#[inline]
pub fn addi4spn(rd: CRegister, imm: u16) -> u16 {
    encode!(
        i3(0b000),
        i2((imm >> 4) as u32),
        i4((imm >> 6) as u32),
        i1((imm >> 2) as u32),
        i1((imm >> 3) as u32),
        i3(rd as u32),
        i2(0b00)
    ) as u16
}

#[inline]
pub fn lw(rd: CRegister, base: CRegister, offset: u8) -> u16 {
    encode!(
        i3(0b010),
        i3((offset >> 3) as u32),
        i3(base as u32),
        i1((offset >> 2) as u32),
        i1((offset >> 6) as u32),
        i3(rd as u32),
        i2(0b00)
    ) as u16
}

#[inline]
pub fn sw(rd: CRegister, base: CRegister, offset: u8) -> u16 {
    encode!(
        i3(0b110),
        i3((offset >> 3) as u32),
        i3(base as u32),
        i1((offset >> 2) as u32),
        i1((offset >> 6) as u32),
        i3(rd as u32),
        i2(0b00)
    ) as u16
}

#[inline]
pub fn nop() -> u16 {
    encode!(i3(0b000), i1(0), i5(0), i5(0), i2(0b01)) as u16
}

#[inline]
pub fn addi(rd: Register, imm: i8) -> u16 {
    CiType { op: 0b01, funct3: 0b000, rd, imm }.encode()
}

#[inline]
pub fn li(rd: Register, imm: i8) -> u16 {
    CiType { op: 0b01, funct3: 0b010, rd, imm }.encode()
}

#[inline]
pub fn addi16sp(imm: i16) -> u16 {
    encode!(
        i3(0b011),
        i1((imm >> 9) as u32),
        i5(2),
        i1((imm >> 4) as u32),
        i1((imm >> 6) as u32),
        i2((imm >> 7) as u32),
        i1((imm >> 5) as u32),
        i2(0b01)
    ) as u16
}

#[inline]
pub fn lui(rd: Register, imm: i8) -> u16 {
    CiType { op: 0b01, funct3: 0b011, rd, imm }.encode()
}

#[inline]
pub fn srli(rd: CRegister, shamt: u8) -> u16 {
    encode!(
        i3(0b100),
        i1(0),
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
        i1(0),
        i2(0b01),
        i3(rd as u32),
        i5(shamt as u32),
        i2(0b01)
    ) as u16
}

#[inline]
pub fn andi(rd: CRegister, imm: i8) -> u16 {
    encode!(
        i3(0b100),
        i1((imm >> 5) as u32),
        i2(0b10),
        i3(rd as u32),
        i5(imm as u32),
        i2(0b01)
    ) as u16
}

#[inline]
pub fn sub(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b00, funct6: 0b100011, rd, rs }.encode()
}

#[inline]
pub fn xor(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b01, funct6: 0b100011, rd, rs }.encode()
}

#[inline]
pub fn or(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b10, funct6: 0b100011, rd, rs }.encode()
}

#[inline]
pub fn and(rd: CRegister, rs: CRegister) -> u16 {
    CaType { op: 0b01, funct2: 0b11, funct6: 0b100011, rd, rs }.encode()
}

#[inline]
pub fn j(offset: i16) -> u16 {
    CjType { op: 0b01, funct3: 0b101, offset }.encode()
}

#[inline]
pub fn beqz(rs: CRegister, offset: i16) -> u16 {
    CbType { op: 0b01, funct3: 0b110, rs, offset }.encode()
}

#[inline]
pub fn bnez(rs: CRegister, offset: i16) -> u16 {
    CbType { op: 0b01, funct3: 0b111, rs, offset }.encode()
}

#[inline]
pub fn slli(rd: Register, shamt: u8) -> u16 {
    CiType { op: 0b10, funct3: 0b000, rd, imm: shamt as i8 & 0x1f }.encode()
}

#[inline]
pub fn lwsp(rd: Register, offset: u8) -> u16 {
    encode!(
        i3(0b010),
        i1((offset >> 5) as u32),
        i5(rd as u32),
        i3((offset >> 2) as u32),
        i2((offset >> 6) as u32),
        i2(0b10)
    ) as u16
}

#[inline]
pub fn jr(rs: Register) -> u16 {
    CrType { op: 0b10, funct4: 0b1000, rd: rs, rs: Register::Zero }.encode()
}

#[inline]
pub fn mv(rd: Register, rs: Register) -> u16 {
    CrType { op: 0b10, funct4: 0b1000, rd, rs }.encode()
}

#[inline]
pub fn ebreak() -> u16 {
    encode!(i3(0b100), i1(1), i5(0), i5(0), i2(0b10)) as u16
}

#[inline]
pub fn jalr(rs: Register) -> u16 {
    CrType { op: 0b10, funct4: 0b1001, rd: rs, rs: Register::Zero }.encode()
}

#[inline]
pub fn add(rd: Register, rs: Register) -> u16 {
    CrType { op: 0b10, funct4: 0b1001, rd, rs }.encode()
}

#[inline]
pub fn swsp(rs: Register, offset: u8) -> u16 {
    encode!(
        i3(0b110),
        i4((offset >> 2) as u32),
        i2((offset >> 6) as u32),
        i5(rs as u32),
        i2(0b10)
    ) as u16
}
