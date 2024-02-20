use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;
use crate::fixup::FixupKind;

use crate::macros::forward;

use crate::rv32i;


pub trait Emit: EmitSlice {
    fn emit_beq_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_beq(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_beqz_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_beqz(rs, offset as i16)
            },
        )
    }

    fn emit_bge_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bge(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bgeu_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bgeu(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bgez_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bgez(rs, offset as i16)
            },
        )
    }

    fn emit_bgt_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bgt(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bgtu_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bgtu(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bgtz_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bgtz(rs, offset as i16)
            },
        )
    }

    fn emit_ble_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_ble(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bleu_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bleu(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_blez_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_blez(rs, offset as i16)
            },
        )
    }

    fn emit_blt_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_blt(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bltu_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bltu(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bltz_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bltz(rs, offset as i16)
            },
        )
    }

    fn emit_bne_label<Label>(&mut self, rs1: Register, rs2: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bne(rs1, rs2, offset as i16)
            },
        )
    }

    fn emit_bnez_label<Label>(&mut self, rs: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Branch,
            |e, offset| {
                e.emit_bnez(rs, offset as i16)
            },
        )
    }

    fn emit_j_label<Label>(&mut self, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Jump,
            |e, offset| {
                e.emit_j(offset as i32)
            },
        )
    }

    fn emit_jal_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Jump,
            |e, offset| {
                e.emit_jal(rd, offset as i32)
            },
        )
    }

    fn emit_lb_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lb(rd, rd, lower)
            },
        )
    }

    fn emit_lbu_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lbu(rd, rd, lower)
            },
        )
    }

    fn emit_ld_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_ld(rd, rd, lower)
            },
        )
    }

    fn emit_lh_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lh(rd, rd, lower)
            },
        )
    }

    fn emit_lhu_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lhu(rd, rd, lower)
            },
        )
    }

    fn emit_lw_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lw(rd, rd, lower)
            },
        )
    }

    fn emit_lwu_label<Label>(&mut self, rd: Register, label: &mut Label)
        -> Result<(), Self::Error>
    where Self: mitte_core::Emit,
          Label: mitte_core::Label<Self, FixupKind>
    {
        self.emit_branch(
            label,
            FixupKind::Load,
            |e, offset| {
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);
                e.emit_auipc(rd, upper)?;
                e.emit_lwu(rd, rd, lower)
            },
        )
    }

    forward! {
        emit_addiw(rd: Register, rs: Register, imm12: i16) => addiw;
        emit_addw(rd: Register, rs1: Register, rs2: Register) => addw;
        emit_ld(rd: Register, base: Register, offset: i16) => ld;
        emit_lui(rd: Register, imm20: i32) => lui;
        emit_lwu(rd: Register, base: Register, offset: i16) => lwu;
        emit_negw(rd: Register, rs: Register) => negw;
        emit_sd(rs: Register, base: Register, offset: i16) => sd;
        emit_sext_w(rd: Register, rs: Register) => sext_w;
        emit_slli(rd: Register, rs: Register, shamt: u8) => slli;
        emit_slliw(rd: Register, rs: Register, shamt: u8) => slliw;
        emit_sllw(rd: Register, rs1: Register, rs2: Register) => sllw;
        emit_srai(rd: Register, rs: Register, shamt: u8) => srai;
        emit_sraiw(rd: Register, rs: Register, shamt: u8) => sraiw;
        emit_sraw(rd: Register, rs1: Register, rs2: Register) => sraw;
        emit_srli(rd: Register, rs: Register, shamt: u8) => srli;
        emit_srliw(rd: Register, rs: Register, shamt: u8) => srliw;
        emit_srlw(rd: Register, rs1: Register, rs2: Register) => srlw;
        emit_subw(rd: Register, rs1: Register, rs2: Register) => subw;
    }

    forward! {
        emit_add(rd: Register, rs1: Register, rs2: Register) => rv32i::add;
        emit_addi(rd: Register, rs: Register, imm12: i16) => rv32i::addi;
        emit_and(rd: Register, rs1: Register, rs2: Register) => rv32i::and;
        emit_andi(rd: Register, rs: Register, imm12: i16) => rv32i::andi;
        emit_auipc(rd: Register, imm20: i32) => rv32i::auipc;
        emit_beq(rs1: Register, rs2: Register, offset: i16) => rv32i::beq;
        emit_beqz(rs: Register, offset: i16) => rv32i::beqz;
        emit_bge(rs1: Register, rs2: Register, offset: i16) => rv32i::bge;
        emit_bgeu(rs1: Register, rs2: Register, offset: i16) => rv32i::bgeu;
        emit_bgez(rs: Register, offset: i16) => rv32i::bgez;
        emit_bgt(rs1: Register, rs2: Register, offset: i16) => rv32i::bgt;
        emit_bgtu(rs1: Register, rs2: Register, offset: i16) => rv32i::bgtu;
        emit_bgtz(rs: Register, offset: i16) => rv32i::bgtz;
        emit_ble(rs1: Register, rs2: Register, offset: i16) => rv32i::ble;
        emit_bleu(rs1: Register, rs2: Register, offset: i16) => rv32i::bleu;
        emit_blez(rs: Register, offset: i16) => rv32i::blez;
        emit_blt(rs1: Register, rs2: Register, offset: i16) => rv32i::blt;
        emit_bltu(rs1: Register, rs2: Register, offset: i16) => rv32i::bltu;
        emit_bltz(rs: Register, offset: i16) => rv32i::bltz;
        emit_bne(rs1: Register, rs2: Register, offset: i16) => rv32i::bne;
        emit_bnez(rs: Register, offset: i16) => rv32i::bnez;
        emit_ebreak() => rv32i::ebreak;
        emit_ecall() => rv32i::ecall;
        emit_j(offset: i32) => rv32i::j;
        emit_jal(rd: Register, offset: i32) => rv32i::jal;
        emit_jalr(rd: Register, base: Register, offset: i16) => rv32i::jalr;
        emit_lb(rd: Register, base: Register, offset: i16) => rv32i::lb;
        emit_lbu(rd: Register, base: Register, offset: i16) => rv32i::lbu;
        emit_lh(rd: Register, base: Register, offset: i16) => rv32i::lh;
        emit_lhu(rd: Register, base: Register, offset: i16) => rv32i::lhu;
        emit_lw(rd: Register, base: Register, offset: i16) => rv32i::lw;
        emit_mv(rd: Register, rs: Register) => rv32i::mv;
        emit_not(rd: Register, rs: Register) => rv32i::not;
        emit_neg(rd: Register, rs: Register) => rv32i::neg;
        emit_or(rd: Register, rs1: Register, rs2: Register) => rv32i::or;
        emit_ori(rd: Register, rs: Register, imm12: i16) => rv32i::ori;
        emit_ret() => rv32i::ret;
        emit_seqz(rd: Register, rs: Register) => rv32i::seqz;
        emit_sgtz(rd: Register, rs: Register) => rv32i::sgtz;
        emit_sll(rd: Register, rs1: Register, rs2: Register) => rv32i::sll;
        emit_slt(rd: Register, rs1: Register, rs2: Register) => rv32i::slt;
        emit_slti(rd: Register, rs: Register, imm12: i16) => rv32i::slti;
        emit_sltiu(rd: Register, rs: Register, imm12: i16) => rv32i::sltiu;
        emit_sltu(rd: Register, rs1: Register, rs2: Register) => rv32i::sltu;
        emit_sltz(rd: Register, rs: Register) => rv32i::sltz;
        emit_snez(rd: Register, rs: Register) => rv32i::snez;
        emit_sra(rd: Register, rs1: Register, rs2: Register) => rv32i::sra;
        emit_srl(rd: Register, rs1: Register, rs2: Register) => rv32i::srl;
        emit_sb(rs: Register, base: Register, offset: i16) => rv32i::sb;
        emit_sh(rs: Register, base: Register, offset: i16) => rv32i::sh;
        emit_sub(rd: Register, rs1: Register, rs2: Register) => rv32i::sub;
        emit_sw(rs: Register, base: Register, offset: i16) => rv32i::sw;
        emit_xor(rd: Register, rs1: Register, rs2: Register) => rv32i::xor;
        emit_xori(rd: Register, rs: Register, imm12: i16) => rv32i::xori;
        emit_zext_b(rd: Register, rs: Register) => rv32i::zext_b;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn lui(rd: Register, imm20: i32) -> u32 {
    assert!(is_signed_nbit_integer(20, imm20));
    encode!(i20(imm20 as u32), i5(rd as u32), i7(0b0110111))
}

#[inline]
pub fn lwu(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: 0b0000011, funct3: 0b110, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn ld(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: 0b0000011, funct3: 0b011, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn sd(rs: Register, base: Register, offset: i16) -> u32 {
    SType { opcode: 0b0100011, funct3: 0b011, rs, base, imm12: offset }.encode()
}

#[inline]
pub fn slli(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i6(0), i6(shamt as u32), i5(rs as u32), i3(0b001), i5(rd as u32), i7(0b0010011))
}

#[inline]
pub fn srli(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i6(0), i6(shamt as u32), i5(rs as u32), i3(0b101), i5(rd as u32), i7(0b0010011))
}

#[inline]
pub fn srai(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i6(0b010000), i6(shamt as u32), i5(rs as u32), i3(0b101), i5(rd as u32), i7(0b0010011))
}

#[inline]
pub fn addiw(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: 0b0011011, funct3: 0b000, rd, rs, imm12 }.encode()
}

#[inline]
pub fn slliw(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i7(0), i5(shamt as u32), i5(rs as u32), i3(0b001), i5(rd as u32), i7(0b0011011))
}

#[inline]
pub fn srliw(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i7(0), i5(shamt as u32), i5(rs as u32), i3(0b101), i5(rd as u32), i7(0b0011011))
}

#[inline]
pub fn sraiw(rd: Register, rs: Register, shamt: u8) -> u32 {
    encode!(i7(0b0100000), i5(shamt as u32), i5(rs as u32), i3(0b101), i5(rd as u32), i7(0b0011011))
}

#[inline]
pub fn addw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: 0b0111011, funct3: 0b000, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn subw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: 0b0111011, funct3: 0b000, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sllw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: 0b0111011, funct3: 0b001, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn srlw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: 0b0111011, funct3: 0b101, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sraw(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: 0b0111011, funct3: 0b101, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn negw(rd: Register, rs: Register) -> u32 {
    subw(rd, Register::Zero, rs)
}

#[inline]
pub fn sext_w(rd: Register, rs: Register) -> u32 {
    addiw(rd, rs, 0)
}
