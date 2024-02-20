extern crate mitte_riscv;
extern crate capstone;

use mitte_riscv::*;
use mitte_riscv::Register::*;

use capstone::Capstone;
use capstone::arch::{BuildsCapstone, BuildsCapstoneExtraMode};
use capstone::arch::riscv::{ArchMode, ArchExtraMode};


mod common;

use common::{TestCases, ToLeBytes};


#[track_caller]
fn test_disasm<S>(mnemonic: &str, expected: S, code: &[u8]) where S: AsRef<str> {
    println!("code: {:02x?}", code);
    let capstone = Capstone::new().riscv()
        .mode(ArchMode::RiscV32)
        .extra_mode([ArchExtraMode::RiscVC].iter().copied())
        .build().unwrap();
    let disasm = match capstone.disasm_all(code, 0x0) {
        Ok(disasm) => disasm,
        Err(error) => {
            panic!("failed to disassemble: {:?}", error);
        }
    };

    for (i, e) in disasm.iter().zip(std::iter::once(expected)) {
        println!("disasm: {:?} {:?}", i.mnemonic(), i.op_str());
        assert_eq!(mnemonic, i.mnemonic().unwrap(),
            "{} {:?} != {} {:?}",
            mnemonic, e.as_ref(),
            i.mnemonic().unwrap(), i.op_str().unwrap());
        assert_eq!(Some(e.as_ref()), i.op_str(),
            "{} {:?} != {0} {:?}", mnemonic,
            e.as_ref(), i.op_str().unwrap());
    }
    assert_eq!(disasm.len() as usize, 1);
}


#[track_caller]
fn test0<R, const N: usize>(mnemonic: &str, f: fn() -> R)
    where R: ToLeBytes<Bytes=[u8; N]>
{
    test_disasm(mnemonic, "", &f().to_le_bytes());
}

#[track_caller]
fn test1_format_filter<A1, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1) -> R,
    s: impl Fn(&str) -> String,
    mut filter: impl FnMut(A1) -> bool
)
    where A1: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    for (a1, s1) in A1::test_cases() {
        if filter(a1) {
            test_disasm(mnemonic, s(&s1), &f(a1).to_le_bytes());
        }
    }
}

#[track_caller]
fn test1_filter<A1, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1) -> R,
    filter: impl FnMut(A1) -> bool
)
    where A1: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    test1_format_filter(mnemonic, f,
        |s1| format!("{}", s1),
        filter);
}

#[track_caller]
fn test2_format_filter<A1, A2, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1, A2) -> R,
    s: impl Fn(&str, &str) -> String,
    mut filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            if filter(a1, a2) {
                test_disasm(mnemonic, s(&s1, &s2), &f(a1, a2).to_le_bytes());
            }
        }
    }
}

#[track_caller]
fn test2_filter<A1, A2, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1, A2) -> R,
    filter: impl FnMut(A1, A2) -> bool
)
    where A1: TestCases, A2: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    test2_format_filter(mnemonic, f,
        |s1, s2| format!("{}, {}", s1, s2),
        filter);
}

#[track_caller]
fn test2<A1, A2, R, const N: usize>(mnemonic: &str, f: fn(A1, A2) -> R)
    where A1: TestCases, A2: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    test2_filter(mnemonic, f, |_, _| true);
}

#[track_caller]
fn test3_format_filter<A1, A2, A3, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> R,
    s: impl Fn(&str, &str, &str) -> String,
    mut filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    for (a1, s1) in A1::test_cases() {
        for (a2, s2) in A2::test_cases() {
            for (a3, s3) in A3::test_cases() {
                if filter(a1, a2, a3) {
                    test_disasm(mnemonic, s(&s1, &s2, &s3), &f(a1, a2, a3).to_le_bytes());
                }
            }
        }
    }
}

#[track_caller]
fn test3_filter<A1, A2, A3>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> u32,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_format_filter(mnemonic, f,
        |s1, s2, s3| format!("{}, {}, {}", s1, s2, s3),
        filter)
}

#[track_caller]
fn test3<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test3_filter(mnemonic, f, |_, _, _| true);
}

#[track_caller]
fn test_ldst_filter<A1, A2, A3, R, const N: usize>(
    mnemonic: &str,
    f: fn(A1, A2, A3) -> R,
    filter: impl FnMut(A1, A2, A3) -> bool
)
    where A1: TestCases, A2: TestCases, A3: TestCases, R: ToLeBytes<Bytes=[u8; N]>
{
    test3_format_filter(mnemonic, f,
        |rd, base, offset| format!("{}, {}({})", rd, offset, base),
        filter);
}

#[track_caller]
fn test_ldst<A1, A2, A3>(mnemonic: &str, f: fn(A1, A2, A3) -> u32)
    where A1: TestCases, A2: TestCases, A3: TestCases
{
    test_ldst_filter(mnemonic, f, |_, _, _| true);
}


#[test]
fn test_add() {
    test3("add", rv32i::add);
}

#[test]
fn test_addi() {
    test3_filter("addi", rv32i::addi, |_, _, imm12| imm12 != 0);
}

#[test]
fn test_and() {
    test3("and", rv32i::and);
}

#[test]
fn test_andi() {
    test3("andi", rv32i::andi);
}

#[test]
fn test_auipc() {
    test2("auipc", |rd, offset: u32| rv32i::auipc(rd, ((offset as i32) << 12) >> 12));
}

#[test]
fn test_beq() {
    test3_filter("beq", rv32i::beq, |_, rs2, offset| rs2 != Zero && offset & 1 == 0);
}

#[test]
fn test_bge() {
    test3_filter("bge", rv32i::bge, |rs1, rs2, offset| {
        rs1 != Zero && rs2 != Zero && offset & 1 == 0
    });
}

#[test]
fn test_bgeu() {
    test3_filter("bgeu", rv32i::bgeu, |_, _, offset| offset & 1 == 0);
}

#[test]
fn test_blt() {
    test3_filter("blt", rv32i::blt, |rs1, rs2, offset| {
        rs1 != Zero && rs2 != Zero && offset & 1 == 0
    });
}

#[test]
fn test_bltu() {
    test3_filter("bltu", rv32i::bltu, |_, _, offset| offset & 1 == 0);
}

#[test]
fn test_bne() {
    test3_filter("bne", rv32i::bne, |_, rs2, offset| rs2 != Zero && offset & 1 == 0);
}

#[test]
fn test_c_add() {
    test2_filter("c.add", rv32c::add, |rd, rs| {
        rd != Zero && rs != Zero
    });
}

#[test]
fn test_c_addi() {
    test2_filter("c.addi", rv32c::addi, |rd, imm| {
        rd != Zero && imm != 0 && imm >= -0x20 && imm < 0x20
    });
}

#[test]
fn test_c_addi4spn() {
    test2_format_filter("c.addi4spn", rv32c::addi4spn,
        |rd, imm| format!("{}, sp, {}", rd, imm),
        |_, imm| imm & !3 != 0 && imm & 3 == 0 && imm < 0x100);
}

#[test]
fn test_c_addi16sp() {
    test1_format_filter("c.addi16sp", rv32c::addi16sp,
        |imm| format!("sp, {}", imm),
        |imm| imm & !0xf != 0 && imm & 0xf == 0 && imm >= -0x200 && imm < 0x200);
}

#[test]
fn test_c_and() {
    test2("c.and", rv32c::and);
}

#[test]
fn test_c_andi() {
    test2_filter("c.andi", rv32c::andi, |_, imm| imm >= -0x40 && imm < 0x40);
}

#[test]
fn test_c_beqz() {
    test2_filter("c.beqz", rv32c::beqz, |_, offset| {
        offset & 1 == 0 && offset >= -0x100 && offset < 0x100
    });
}

#[test]
fn test_c_bnez() {
    test2_filter("c.bnez", rv32c::bnez, |_, offset| {
        offset & 1 == 0 && offset >= -0x100 && offset < 0x100
    });
}

#[test]
fn test_c_ebreak() {
    test0("c.ebreak", rv32c::ebreak);
}

#[test]
fn test_c_j() {
    test1_filter("c.j", rv32c::j, |offset| {
        offset & 1 == 0
    });
}

#[test]
fn test_c_jalr() {
    test1_filter("c.jalr", rv32c::jalr, |rs| rs != Zero);
}

#[test]
fn test_c_jr() {
    test1_filter("c.jr", rv32c::jr, |rs| rs != Zero);
}

#[test]
fn test_c_li() {
    test2_filter("c.li", rv32c::li, |rd, imm| {
        rd != Zero && imm >= -0x20 && imm < 0x20
    });
}

#[test]
fn test_c_lui() {
    test2_filter("c.lui", rv32c::lui, |rd, imm| {
        rd != Zero && rd != Sp && imm > 0 && imm < 0x20
    });
    test_disasm("c.lui", "t2, 0xfffff", &rv32c::lui(T2, -1).to_le_bytes());
}

#[test]
fn test_c_lw() {
    test_ldst_filter("c.lw", rv32c::lw, |_, _, offset| {
        offset & 3 == 0 && offset < 0x80
    });
}

#[test]
fn test_c_lwsp() {
    test2_format_filter("c.lwsp", rv32c::lwsp,
        |rd, offset| format!("{}, {}(sp)", rd, offset),
        |rd, offset| rd != Zero && offset & 3 == 0);
}

#[test]
fn test_c_mv() {
    test2_filter("c.mv", rv32c::mv, |rd, rs| {
        rd != Zero && rs != Zero
    });
}

#[test]
fn test_c_nop() {
    test0("c.nop", rv32c::nop);
}

#[test]
fn test_c_or() {
    test2("c.or", rv32c::or);
}

#[test]
fn test_c_slli() {
    test2_filter("c.slli", rv32c::slli, |rd, shamt| {
        rd != Zero && shamt > 0 && shamt < 0x20
    });
}

#[test]
fn test_c_srai() {
    test2_filter("c.srai", rv32c::srai, |_, shamt| {
        shamt > 0 && shamt < 0x20
    });
}

#[test]
fn test_c_srli() {
    test2_filter("c.srli", rv32c::srli, |_, shamt| {
        shamt > 0 && shamt < 0x20
    });
}

#[test]
fn test_c_sub() {
    test2("c.sub", rv32c::sub);
}

#[test]
fn test_c_sw() {
    test_ldst_filter("c.sw", rv32c::sw, |_, _, offset| {
        offset & 3 == 0 && offset < 0x80
    });
}

#[test]
fn test_c_swsp() {
    test2_format_filter("c.swsp", rv32c::swsp,
        |rs, offset| format!("{}, {}(sp)", rs, offset),
        |_, offset| offset & 3 == 0);
}

#[test]
fn test_c_xor() {
    test2("c.xor", rv32c::xor);
}

#[test]
fn test_div() {
    test3("div", rv32m::div);
}

#[test]
fn test_divu() {
    test3("divu", rv32m::divu);
}

#[test]
fn test_ebreak() {
    test0("ebreak", rv32i::ebreak);
}

#[test]
fn test_ecall() {
    test0("ecall", rv32i::ecall);
}

#[test]
fn test_j() {
    test1_filter("j", rv32i::j, |offset| {
        offset & 1 == 0
    });
}

#[test]
fn test_jal() {
    test2_filter("jal", rv32i::jal, |rd, offset| {
        rd != Zero && rd != Ra && offset & 1 == 0
    });
}

#[test]
fn test_jalr() {
    test3_filter("jalr", rv32i::jalr, |rd, _, _| {
        rd != Zero && rd != Ra
    });
}

#[test]
fn test_jr() {
    test1_filter("jr", rv32i::jr, |rs| rs != Ra);
}

#[test]
fn test_lb() {
    test_ldst("lb", rv32i::lb);
}

#[test]
fn test_lbu() {
    test_ldst("lbu", rv32i::lbu);
}

#[test]
fn test_lh() {
    test_ldst("lh", rv32i::lh);
}

#[test]
fn test_lhu() {
    test_ldst("lhu", rv32i::lhu);
}

#[test]
fn test_lui() {
    test2("lui", rv32i::lui);
}

#[test]
fn test_lw() {
    test_ldst("lw", rv32i::lw);
}

#[test]
fn test_mul() {
    test3("mul", rv32m::mul);
}

#[test]
fn test_mulh() {
    test3("mulh", rv32m::mulh);
}

#[test]
fn test_mulhsu() {
    test3("mulhsu", rv32m::mulhsu);
}

#[test]
fn test_mulhu() {
    test3("mulhu", rv32m::mulhu);
}

#[test]
fn test_mv() {
    test2_filter("mv", rv32i::mv, |rd, _| rd != Zero);
}

#[test]
fn test_nop() {
    test0("nop", rv32i::nop);
}

#[test]
fn test_not() {
    test2("not", rv32i::not);
}

#[test]
fn test_neg() {
    test2("neg", rv32i::neg);
}

#[test]
fn test_or() {
    test3("or", rv32i::or);
}

#[test]
fn test_ori() {
    test3("ori", rv32i::ori);
}

#[test]
fn test_rem() {
    test3("rem", rv32m::rem);
}

#[test]
fn test_remu() {
    test3("remu", rv32m::remu);
}

#[test]
fn test_ret() {
    test0("ret", rv32i::ret);
}

#[test]
fn test_seqz() {
    test2("seqz", rv32i::seqz);
}

#[test]
fn test_sgtz() {
    test2_filter("sgtz", rv32i::sgtz, |_, rs| rs != Zero);
}

#[test]
fn test_sll() {
    test3("sll", rv32i::sll);
}

#[test]
fn test_slli() {
    test3_filter("slli", rv32i::slli, |_, _, shamt| shamt > 0 && shamt < 0x20);
}

#[test]
fn test_slt() {
    test3_filter("slt", rv32i::slt, |_, rs1, rs2| rs1 != Zero && rs2 != Zero);
}

#[test]
fn test_slti() {
    test3("slti", rv32i::slti);
}

#[test]
fn test_sltiu() {
    test3_filter("sltiu", rv32i::sltiu, |_, _, imm| {
        imm != 1
    });
}

#[test]
fn test_sltu() {
    test3_filter("sltu", rv32i::sltu, |_, rs1, _| rs1 != Zero);
}

#[test]
fn test_sltz() {
    test2("sltz", rv32i::sltz);
}

#[test]
fn test_snez() {
    test2("snez", rv32i::snez);
}

#[test]
fn test_sra() {
    test3("sra", rv32i::sra);
}

#[test]
fn test_srai() {
    test3_filter("srai", rv32i::srai, |_, _, shamt| shamt > 0 && shamt < 0x20);
}

#[test]
fn test_srl() {
    test3("srl", rv32i::srl);
}

#[test]
fn test_srli() {
    test3_filter("srli", rv32i::srli, |_, _, shamt| shamt > 0 && shamt < 0x20);
}

#[test]
fn test_sb() {
    test_ldst("sb", rv32i::sb);
}

#[test]
fn test_sh() {
    test_ldst("sh", rv32i::sh);
}

#[test]
fn test_sub() {
    test3_filter("sub", rv32i::sub, |_, rs1, _| rs1 != Zero);
}

#[test]
fn test_sw() {
    test_ldst("sw", rv32i::sw);
}

#[test]
fn test_xor() {
    test3("xor", rv32i::xor);
}

#[test]
fn test_xori() {
    test3("xori", rv32i::xori);
}
