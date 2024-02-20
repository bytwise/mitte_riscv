use mitte_core::EmitSlice;

use crate::types::*;

use crate::macros::forward;

use crate::rv32zbc;


pub trait Emit: EmitSlice {
    forward! {
        emit_clmul(rd: Register, rs1: Register, rs2: Register) => rv32zbc::clmul;
        emit_clmulh(rd: Register, rs1: Register, rs2: Register) => rv32zbc::clmulh;
        emit_clmulr(rd: Register, rs1: Register, rs2: Register) => rv32zbc::clmulr;
    }
}

impl<E> Emit for E where E: EmitSlice + ?Sized {}
