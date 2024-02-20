use mitte_core::EmitSlice;

use crate::encoding::*;
use crate::types::*;
use crate::fixup::FixupKind;

use crate::macros::forward;


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

    forward! {
        emit_add(rd: Register, rs1: Register, rs2: Register) => add;
        emit_addi(rd: Register, rs: Register, imm12: i16) => addi;
        emit_and(rd: Register, rs1: Register, rs2: Register) => and;
        emit_andi(rd: Register, rs: Register, imm12: i16) => andi;
        emit_auipc(rd: Register, imm20: i32) => auipc;
        emit_beq(rs1: Register, rs2: Register, offset: i16) => beq;
        emit_beqz(rs: Register, offset: i16) => beqz;
        emit_bge(rs1: Register, rs2: Register, offset: i16) => bge;
        emit_bgeu(rs1: Register, rs2: Register, offset: i16) => bgeu;
        emit_bgez(rs: Register, offset: i16) => bgez;
        emit_bgt(rs1: Register, rs2: Register, offset: i16) => bgt;
        emit_bgtu(rs1: Register, rs2: Register, offset: i16) => bgtu;
        emit_bgtz(rs: Register, offset: i16) => bgtz;
        emit_ble(rs1: Register, rs2: Register, offset: i16) => ble;
        emit_bleu(rs1: Register, rs2: Register, offset: i16) => bleu;
        emit_blez(rs: Register, offset: i16) => blez;
        emit_blt(rs1: Register, rs2: Register, offset: i16) => blt;
        emit_bltu(rs1: Register, rs2: Register, offset: i16) => bltu;
        emit_bltz(rs: Register, offset: i16) => bltz;
        emit_bne(rs1: Register, rs2: Register, offset: i16) => bne;
        emit_bnez(rs: Register, offset: i16) => bnez;
        emit_ebreak() => ebreak;
        emit_ecall() => ecall;
        emit_j(offset: i32) => j;
        emit_jal(rd: Register, offset: i32) => jal;
        emit_jalr(rd: Register, base: Register, offset: i16) => jalr;
        emit_lb(rd: Register, base: Register, offset: i16) => lb;
        emit_lbu(rd: Register, base: Register, offset: i16) => lbu;
        emit_lh(rd: Register, base: Register, offset: i16) => lh;
        emit_lhu(rd: Register, base: Register, offset: i16) => lhu;
        emit_lui(rd: Register, imm20: u32) => lui;
        emit_lw(rd: Register, base: Register, offset: i16) => lw;
        emit_mv(rd: Register, rs: Register) => mv;
        emit_nop() => nop;
        emit_not(rd: Register, rs: Register) => not;
        emit_neg(rd: Register, rs: Register) => neg;
        emit_or(rd: Register, rs1: Register, rs2: Register) => or;
        emit_ori(rd: Register, rs: Register, imm12: i16) => ori;
        emit_ret() => ret;
        emit_seqz(rd: Register, rs: Register) => seqz;
        emit_sgtz(rd: Register, rs: Register) => sgtz;
        emit_sll(rd: Register, rs1: Register, rs2: Register) => sll;
        emit_slli(rd: Register, rs: Register, shamt: u8) => slli;
        emit_slt(rd: Register, rs1: Register, rs2: Register) => slt;
        emit_slti(rd: Register, rs: Register, imm12: i16) => slti;
        emit_sltiu(rd: Register, rs: Register, imm12: i16) => sltiu;
        emit_sltu(rd: Register, rs1: Register, rs2: Register) => sltu;
        emit_sltz(rd: Register, rs: Register) => sltz;
        emit_snez(rd: Register, rs: Register) => snez;
        emit_sra(rd: Register, rs1: Register, rs2: Register) => sra;
        emit_srai(rd: Register, rs: Register, shamt: u8) => srai;
        emit_srl(rd: Register, rs1: Register, rs2: Register) => srl;
        emit_srli(rd: Register, rs: Register, shamt: u8) => srli;
        emit_sb(rs: Register, base: Register, offset: i16) => sb;
        emit_sh(rs: Register, base: Register, offset: i16) => sh;
        emit_sub(rd: Register, rs1: Register, rs2: Register) => sub;
        emit_sw(rs: Register, base: Register, offset: i16) => sw;
        emit_xor(rd: Register, rs1: Register, rs2: Register) => xor;
        emit_xori(rd: Register, rs: Register, imm12: i16) => xori;
        emit_zext_b(rd: Register, rs: Register) => zext_b;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}


#[inline]
pub fn lui(rd: Register, imm20: u32) -> u32 {
    assert!(imm20 < (1 << 20));
    encode!(i20(imm20), i5(rd as u32), i7(LUI))
}

#[inline]
pub fn auipc(rd: Register, imm20: i32) -> u32 {
    assert!(is_signed_nbit_integer(20, imm20));
    encode!(i20(imm20 as u32), i5(rd as u32), i7(AUIPC))
}

#[inline]
pub fn jal(rd: Register, offset: i32) -> u32 {
    assert!(is_signed_nbit_integer(21, offset));
    encode!(
        i1((offset >> 20) as u32),
        i10((offset >> 1) as u32),
        i1((offset >> 11) as u32),
        i8((offset >> 12) as u32),
        i5(rd as u32),
        i7(JAL)
    )
}

#[inline]
pub fn jalr(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Jalr, funct3: 0b000, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn beq(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b000, rs1, rs2, offset }.encode()
}

#[inline]
pub fn bne(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b001, rs1, rs2, offset }.encode()
}

#[inline]
pub fn blt(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b100, rs1, rs2, offset }.encode()
}

#[inline]
pub fn bge(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b101, rs1, rs2, offset }.encode()
}

#[inline]
pub fn bltu(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b110, rs1, rs2, offset }.encode()
}

#[inline]
pub fn bgeu(rs1: Register, rs2: Register, offset: i16) -> u32 {
    BType { opcode: Branch, funct3: 0b111, rs1, rs2, offset }.encode()
}

#[inline]
pub fn lb(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Load, funct3: 0b000, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn lh(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Load, funct3: 0b001, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn lw(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Load, funct3: 0b010, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn lbu(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Load, funct3: 0b100, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn lhu(rd: Register, base: Register, offset: i16) -> u32 {
    IType { opcode: Load, funct3: 0b101, rd, rs: base, imm12: offset }.encode()
}

#[inline]
pub fn sb(rs: Register, base: Register, offset: i16) -> u32 {
    SType { opcode: Store, funct3: 0b000, rs, base, imm12: offset }.encode()
}

#[inline]
pub fn sh(rs: Register, base: Register, offset: i16) -> u32 {
    SType { opcode: Store, funct3: 0b001, rs, base, imm12: offset }.encode()
}

#[inline]
pub fn sw(rs: Register, base: Register, offset: i16) -> u32 {
    SType { opcode: Store, funct3: 0b010, rs, base, imm12: offset }.encode()
}

#[inline]
pub fn addi(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b000, rd, rs, imm12 }.encode()
}

#[inline]
pub fn slti(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b010, rd, rs, imm12 }.encode()
}

#[inline]
pub fn sltiu(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b011, rd, rs, imm12 }.encode()
}

#[inline]
pub fn xori(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b100, rd, rs, imm12 }.encode()
}

#[inline]
pub fn ori(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b110, rd, rs, imm12 }.encode()
}

#[inline]
pub fn andi(rd: Register, rs: Register, imm12: i16) -> u32 {
    IType { opcode: OpImm, funct3: 0b111, rd, rs, imm12 }.encode()
}

#[inline]
pub fn slli(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b001, rd, rs, imm12 }.encode()
}

#[inline]
pub fn srli(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn srai(rd: Register, rs: Register, shamt: u8) -> u32 {
    let imm12 = encode!(i7(0b0100000), i5(shamt as u32)) as i16;
    IType { opcode: OpImm, funct3: 0b101, rd, rs, imm12 }.encode()
}

#[inline]
pub fn add(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b000, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sub(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b000, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sll(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b001, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn slt(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b010, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sltu(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b011, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn xor(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b100, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn srl(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn sra(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b101, funct7: 0b0100000, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn or(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b110, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn and(rd: Register, rs1: Register, rs2: Register) -> u32 {
    RType { opcode: Op, funct3: 0b111, funct7: 0, rd, rs1, rs2 }.encode()
}

#[inline]
pub fn ecall() -> u32 {
    IType { opcode: System, imm12: 0, ..IType::null() }.encode()
}

#[inline]
pub fn ebreak() -> u32 {
    IType { opcode: System, imm12: 1, ..IType::null() }.encode()
}

#[inline]
pub fn nop() -> u32 {
    addi(Register::Zero, Register::Zero, 0)
}

#[inline]
pub fn mv(rd: Register, rs: Register) -> u32 {
    addi(rd, rs, 0)
}

#[inline]
pub fn not(rd: Register, rs: Register) -> u32 {
    xori(rd, rs, -1)
}

#[inline]
pub fn neg(rd: Register, rs: Register) -> u32 {
    sub(rd, Register::Zero, rs)
}

#[inline]
pub fn zext_b(rd: Register, rs: Register) -> u32 {
    andi(rd, rs, 0xff)
}

#[inline]
pub fn seqz(rd: Register, rs: Register) -> u32 {
    sltiu(rd, rs, 1)
}

#[inline]
pub fn snez(rd: Register, rs: Register) -> u32 {
    sltu(rd, Register::Zero, rs)
}

#[inline]
pub fn sltz(rd: Register, rs: Register) -> u32 {
    slt(rd, rs, Register::Zero)
}

#[inline]
pub fn sgtz(rd: Register, rs: Register) -> u32 {
    slt(rd, Register::Zero, rs)
}

#[inline]
pub fn beqz(rs: Register, offset: i16) -> u32 {
    beq(rs, Register::Zero, offset)
}

#[inline]
pub fn bnez(rs: Register, offset: i16) -> u32 {
    bne(rs, Register::Zero, offset)
}

#[inline]
pub fn blez(rs: Register, offset: i16) -> u32 {
    bge(Register::Zero, rs, offset)
}

#[inline]
pub fn bgez(rs: Register, offset: i16) -> u32 {
    bge(rs, Register::Zero, offset)
}

#[inline]
pub fn bltz(rs: Register, offset: i16) -> u32 {
    blt(Register::Zero, rs, offset)
}

#[inline]
pub fn bgtz(rs: Register, offset: i16) -> u32 {
    blt(rs, Register::Zero, offset)
}

#[inline]
pub fn bgt(rs1: Register, rs2: Register, offset: i16) -> u32 {
    blt(rs2, rs1, offset)
}

#[inline]
pub fn ble(rs1: Register, rs2: Register, offset: i16) -> u32 {
    bge(rs2, rs1, offset)
}

#[inline]
pub fn bgtu(rs1: Register, rs2: Register, offset: i16) -> u32 {
    bltu(rs2, rs1, offset)
}

#[inline]
pub fn bleu(rs1: Register, rs2: Register, offset: i16) -> u32 {
    bgeu(rs2, rs1, offset)
}

#[inline]
pub fn j(offset: i32) -> u32 {
    jal(Register::Zero, offset)
}

#[inline]
pub fn jr(rs: Register) -> u32 {
    jalr(Register::Zero, rs, 0)
}

#[inline]
pub fn ret() -> u32 {
    jalr(Register::Zero, Register::Ra, 0)
}
