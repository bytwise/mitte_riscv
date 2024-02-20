use mitte_riscv::*;
use mitte_riscv::Register::*;


pub trait ToLeBytes {
    const N: usize;
    type Bytes;
    fn to_le_bytes(self) -> Self::Bytes;
}

impl ToLeBytes for u16 {
    const N: usize = 2;
    type Bytes = [u8; 2];
    fn to_le_bytes(self) -> [u8; 2] {
        self.to_le_bytes()
    }
}

impl ToLeBytes for u32 {
    const N: usize = 4;
    type Bytes = [u8; 4];
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}


pub trait TestCases: Sized + Copy + 'static {
    fn test_cases() -> Vec<(Self, String)>;
}

impl TestCases for u8 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (1, "1".into()),
            (2, "2".into()),
            (3, "3".into()),
            (4, "4".into()),
            (7, "7".into()),
            (8, "8".into()),
            (0xf, "0xf".into()),
            (0x10, "0x10".into()),
            (0x1f, "0x1f".into()),
            (0x20, "0x20".into()),
            (0x3f, "0x3f".into()),
            (0x40, "0x40".into()),
            (0x7f, "0x7f".into()),
            (0x80, "0x80".into()),
            (0xff, "0xff".into()),
        ]
    }
}

impl TestCases for i8 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (1, "1".into()),
            (0x7e, "0x7e".into()),
            (-1, "-1".into()),
            (-0x80, "-0x80".into()),
        ]
    }
}

impl TestCases for u16 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (1, "1".into()),
            (2, "2".into()),
            (3, "3".into()),
            (4, "4".into()),
            (7, "7".into()),
            (8, "8".into()),
            (0xf, "0xf".into()),
            (0x10, "0x10".into()),
            (0x1f, "0x1f".into()),
            (0x20, "0x20".into()),
            (0x3f, "0x3f".into()),
            (0x40, "0x40".into()),
            (0x7f, "0x7f".into()),
            (0x80, "0x80".into()),
            (0xff, "0xff".into()),
            (0x100, "0x100".into()),
            (0x1ff, "0x1ff".into()),
            (0x200, "0x200".into()),
            (0x3ff, "0x3ff".into()),
            (0x400, "0x400".into()),
            (0x7ff, "0x7ff".into()),
            (0x800, "0x800".into()),
            (0xfff, "0xfff".into()),
            (0x1000, "0x1000".into()),
            (0x1fff, "0x1fff".into()),
            (0x2000, "0x2000".into()),
            (0x3fff, "0x3fff".into()),
            (0x4000, "0x4000".into()),
            (0x7fff, "0x7fff".into()),
            (0x8000, "0x8000".into()),
            (0xffff, "0xffff".into()),
        ]
    }
}

impl TestCases for i16 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (1, "1".into()),
            (2, "2".into()),
            (4, "4".into()),
            (8, "8".into()),
            (0x10, "0x10".into()),
            (0x20, "0x20".into()),
            (0x40, "0x40".into()),
            (0x7e, "0x7e".into()),
            (0x80, "0x80".into()),
            (0xfe, "0xfe".into()),
            (0x1fe, "0x1fe".into()),
            (0x3fe, "0x3fe".into()),
            (0x7fe, "0x7fe".into()),
            (-0x80, "-0x80".into()),
            (-0x100, "-0x100".into()),
            (-0x200, "-0x200".into()),
            (-0x400, "-0x400".into()),
            (-0x800,"-0x800".into()),
        ]
    }
}

impl TestCases for u32 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (1, "1".into()),
            (2, "2".into()),
            (3, "3".into()),
            (4, "4".into()),
            (7, "7".into()),
            (8, "8".into()),
            (0xf, "0xf".into()),
            (0x10, "0x10".into()),
            (0x1f, "0x1f".into()),
            (0x20, "0x20".into()),
            (0x3f, "0x3f".into()),
            (0x40, "0x40".into()),
            (0x7f, "0x7f".into()),
            (0x80, "0x80".into()),
            (0xff, "0xff".into()),
            (0x100, "0x100".into()),
            (0x1ff, "0x1ff".into()),
            (0x200, "0x200".into()),
            (0x3ff, "0x3ff".into()),
            (0x400, "0x400".into()),
            (0x7ff, "0x7ff".into()),
            (0x800, "0x800".into()),
            (0xfff, "0xfff".into()),
            (0x1000, "0x1000".into()),
            (0x1fff, "0x1fff".into()),
            (0x2000, "0x2000".into()),
            (0x3fff, "0x3fff".into()),
            (0x4000, "0x4000".into()),
            (0x7fff, "0x7fff".into()),
            (0x8000, "0x8000".into()),
            (0xffff, "0xffff".into()),
            (0xf_ffff, "0xfffff".into()),
        ]
    }
}

impl TestCases for i32 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "0".into()),
            (0x7e, "0x7e".into()),
            (0xfe, "0xfe".into()),
            (0x1fe, "0x1fe".into()),
            (0x3fe, "0x3fe".into()),
            (0x7fe, "0x7fe".into()),
            (0xffe, "0xffe".into()),
            (0x1ffe, "0x1ffe".into()),
            (0x3ffe, "0x3ffe".into()),
            (0x7ffe, "0x7ffe".into()),
            (0xfffe, "0xfffe".into()),
            (0x1_fffe, "0x1fffe".into()),
            (0x3_fffe, "0x3fffe".into()),
            (0x7_fffe, "0x7fffe".into()),
            (0xf_fffe, "0xffffe".into()),
            (-0x80, "-0x80".into()),
            (-0x100, "-0x100".into()),
            (-0x200, "-0x200".into()),
            (-0x400, "-0x400".into()),
            (-0x800,"-0x800".into()),
            (-0x1000,"-0x1000".into()),
            (-0x2000,"-0x2000".into()),
            (-0x4000,"-0x4000".into()),
            (-0x8000,"-0x8000".into()),
            (-0x1_0000,"-0x10000".into()),
            (-0x2_0000,"-0x20000".into()),
            (-0x4_0000,"-0x40000".into()),
            (-0x8_0000,"-0x80000".into()),
            (-0x10_0000,"-0x100000".into()),
        ]
    }
}

impl TestCases for Register {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Zero, "zero".into()),
            (Ra, "ra".into()),
            (Sp, "sp".into()),
            (Gp, "gp".into()),
            (Tp, "tp".into()),
            (T0, "t0".into()),
            (S0, "s0".into()),
            (A0, "a0".into()),
            (A1, "a1".into()),
            (T6, "t6".into()),
        ]
    }
}

impl TestCases for CRegister {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (CRegister::S0, "s0".into()),
            (CRegister::S1, "s1".into()),
            (CRegister::A0, "a0".into()),
            (CRegister::A1, "a1".into()),
            (CRegister::A2, "a2".into()),
            (CRegister::A3, "a3".into()),
            (CRegister::A4, "a4".into()),
            (CRegister::A5, "a5".into()),
        ]
    }
}
