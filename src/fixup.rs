use std::ops::Range;

use mitte_core::Error;

use crate::encoding::*;


pub enum FixupKind {
    Jump,
    Branch,
    CJump,
    CBranch,
    JumpFar,
    Load,
}

impl FixupKind {
    #[inline]
    fn apply_fixup_32(&self, instruction: u32, offset: i64) -> u32 {
        match *self {
            FixupKind::Jump => {
                assert!(is_signed_nbit_integer(21, offset));
                let mask = encode_jump_offset(-1);
                let offset = encode_jump_offset(offset as i32);
                (instruction & !mask) | offset
            }
            FixupKind::Branch => {
                assert!(is_signed_nbit_integer(13, offset));
                let mask = encode_branch_offset(-1);
                let offset = encode_branch_offset(offset as i16);
                (instruction & !mask) | offset
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    fn apply_fixup_16(&self, instruction: u16, offset: i64) -> u16 {
        match *self {
            FixupKind::CJump => {
                assert!(is_signed_nbit_integer(12, offset));
                let mask = encode_c_jump_offset(-1);
                let offset = encode_c_jump_offset(offset as i16);
                (instruction & !mask) | offset
            }
            FixupKind::CBranch => {
                assert!(is_signed_nbit_integer(9, offset));
                let mask = encode_c_branch_offset(-1);
                let offset = encode_c_branch_offset(offset as i16);
                (instruction & !mask) | offset
            }
            _ => unreachable!(),
        }
    }
}

impl<Emit> mitte_core::FixupKind<Emit> for FixupKind
    where Emit: mitte_core::Emit + ?Sized
{
    #[inline]
    fn apply_fixup(&self, emit: &mut Emit, range: Range<u64>, offset: i64) -> Result<(), Error> {
        match *self {
            FixupKind::Jump |
            FixupKind::Branch => {
                let buffer = emit.get_mut_array::<4>(range.start)?;
                let instruction = u32::from_le_bytes(*buffer);
                buffer.copy_from_slice(&self.apply_fixup_32(instruction, offset).to_le_bytes());
                Ok(())
            }
            FixupKind::CJump |
            FixupKind::CBranch => {
                let buffer = emit.get_mut_array::<2>(range.start)?;
                let instruction = u16::from_le_bytes(*buffer);
                buffer.copy_from_slice(&self.apply_fixup_16(instruction, offset).to_le_bytes());
                Ok(())
            }
            FixupKind::JumpFar |
            FixupKind::Load => {
                assert!(is_signed_nbit_integer(32, offset));
                let (upper, lower) = to_i20_i12_imm_pair(offset as i32);

                let auipc_buffer = emit.get_mut_array::<4>(range.start)?;
                let auipc = u32::from_le_bytes(*auipc_buffer);
                auipc_buffer.copy_from_slice(&apply_auipc_fixup(auipc, upper).to_le_bytes());

                let itype_buffer = emit.get_mut_array::<4>(range.start + 4)?;
                let itype = u32::from_le_bytes(*itype_buffer);
                itype_buffer.copy_from_slice(&apply_itype_fixup(itype, lower).to_le_bytes());
                Ok(())
            }
        }
    }
}


#[inline]
fn apply_auipc_fixup(instruction: u32, offset: i32) -> u32 {
    let mask = encode_auipc_offset(-1);
    let offset = encode_auipc_offset(offset);
    (instruction & !mask) | offset
}

#[inline]
fn apply_itype_fixup(instruction: u32, offset: i16) -> u32 {
    let mask = encode_itype_offset(-1);
    let offset = encode_itype_offset(offset);
    (instruction & !mask) | offset
}


#[inline]
fn encode_jump_offset(offset: i32) -> u32 {
    encode!(
        i1((offset >> 20) as u32),
        i10((offset >> 1) as u32),
        i1((offset >> 11) as u32),
        i8((offset >> 12) as u32),
        i12(0)
    )
}

#[inline]
fn encode_branch_offset(offset: i16) -> u32 {
    BType { offset, ..BType::null() }.encode()
}

#[inline]
fn encode_auipc_offset(offset: i32) -> u32 {
    encode!(i20(offset as u32), i5(0), i7(0))
}

#[inline]
fn encode_itype_offset(offset: i16) -> u32 {
    IType { imm12: offset, ..IType::null() }.encode()
}

#[inline]
fn encode_c_jump_offset(offset: i16) -> u16 {
    CjType { offset, ..CjType::null() }.encode()
}

#[inline]
fn encode_c_branch_offset(offset: i16) -> u16 {
    CbType { offset, ..CbType::null() }.encode()
}
