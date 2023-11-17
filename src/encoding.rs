use crate::types::{Register, CRegister};


#[inline]
pub fn is_signed_nbit_integer(n: u8, value: impl Into<i64>) -> bool {
    let value = value.into();
    ((-1) << (n - 1)) <= value && value < (1 << (n - 1))
}


macro_rules! encode {
    ($($e:expr),*) => {
        {
            let mut bits = crate::encoding::BitBuffer::new();
            $(
            bits.push($e);
            )*
            bits.bits
        }
    };
}

pub(crate) use encode;


pub struct BitBuffer {
    pub bits: u32,
}

impl BitBuffer {
    #[inline]
    pub fn new() -> BitBuffer {
        BitBuffer { bits: 0 }
    }

    #[inline]
    pub fn push<const N: u8>(&mut self, bits: Bits<N>) {
        self.bits <<= N;
        self.bits |= bits.0;
    }
}


pub struct Bits<const N: u8>(u32);

macro_rules! define_bits {
    ($($f:ident: $n:literal),*) => {
        $(
            #[inline]
            pub fn $f(x: u32) -> Bits<$n> {
                Bits(x & ((1 << $n) - 1))
            }
        )*
    };
}

define_bits! {
    i1: 1, i2: 2, i3: 3, i4: 4, i5: 5, i6: 6, i7: 7, i8: 8, i10: 10, i12: 12, i20: 20
}


#[inline]
pub fn to_i20_i12_imm_pair(imm: i32) -> (i32, i16) {
    (
        (imm >> 12) + ((imm >> 11) & 1),
        ((imm as i16) << 4) >> 4
    )
}


pub struct RType {
    pub opcode: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
}

impl RType {
    #[inline]
    pub fn encode(self) -> u32 {
        encode!(
            i7(self.funct7 as u32),
            i5(self.rs2 as u32),
            i5(self.rs1 as u32),
            i3(self.funct3 as u32),
            i5(self.rd as u32),
            i7(self.opcode as u32)
        )
    }
}

pub struct IType {
    pub opcode: u8,
    pub funct3: u8,
    pub rd: Register,
    pub rs: Register,
    pub imm12: i16,
}

impl IType {
    #[track_caller]
    #[inline]
    pub fn encode(self) -> u32 {
        assert!(is_signed_nbit_integer(12, self.imm12));
        encode!(
            i12(self.imm12 as u32),
            i5(self.rs as u32),
            i3(self.funct3 as u32),
            i5(self.rd as u32),
            i7(self.opcode as u32)
        )
    }
}

pub struct SType {
    pub opcode: u8,
    pub funct3: u8,
    pub rs: Register,
    pub base: Register,
    pub imm12: i16,
}

impl SType {
    #[track_caller]
    #[inline]
    pub fn encode(self) -> u32 {
        assert!(is_signed_nbit_integer(12, self.imm12));
        encode!(
            i7((self.imm12 >> 5) as u32),
            i5(self.rs as u32),
            i5(self.base as u32),
            i3(self.funct3 as u32),
            i5(self.imm12 as u32),
            i7(self.opcode as u32)
        )
    }
}

pub struct BType {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: Register,
    pub rs2: Register,
    pub offset: i16,
}

impl BType {
    #[track_caller]
    #[inline]
    pub fn encode(self) -> u32 {
        assert!(is_signed_nbit_integer(13, self.offset));
        encode!(
            i1((self.offset >> 12) as u32),
            i6((self.offset >> 5) as u32),
            i5(self.rs2 as u32),
            i5(self.rs1 as u32),
            i3(self.funct3 as u32),
            i4((self.offset >> 1) as u32),
            i1((self.offset >> 11) as u32),
            i7(self.opcode as u32)
        )
    }
}

pub struct CrType {
    pub op: u8,
    pub rs: Register,
    pub rd: Register,
    pub funct4: u8,
}

impl CrType {
    #[inline]
    pub fn encode(self) -> u16 {
        encode!(
            i4(self.funct4 as u32),
            i5(self.rd as u32),
            i5(self.rs as u32),
            i2(self.op as u32)
        ) as u16
    }
}

pub struct CiType {
    pub op: u8,
    pub rd: Register,
    pub imm: i8,
    pub funct3: u8,
}

impl CiType {
    #[inline]
    pub fn encode(self) -> u16 {
        encode!(
            i3(self.funct3 as u32),
            i1((self.imm >> 5) as u32),
            i5(self.rd as u32),
            i5(self.imm as u32),
            i2(self.op as u32)
        ) as u16
    }
}

pub struct CaType {
    pub op: u8,
    pub rs: CRegister,
    pub funct2: u8,
    pub rd: CRegister,
    pub funct6: u8,
}

impl CaType {
    #[inline]
    pub fn encode(self) -> u16 {
        encode!(
            i6(self.funct6 as u32),
            i3(self.rd as u32),
            i2(self.funct2 as u32),
            i3(self.rs as u32),
            i2(self.op as u32)
        ) as u16
    }
}

pub struct CbType {
    pub op: u8,
    pub rs: CRegister,
    pub offset: i16,
    pub funct3: u8,
}

impl CbType {
    #[inline]
    pub fn encode(self) -> u16 {
        encode!(
            i3(self.funct3 as u32),
            i1((self.offset >> 8) as u32),
            i2((self.offset >> 3) as u32),
            i3(self.rs as u32),
            i2((self.offset >> 6) as u32),
            i2((self.offset >> 1) as u32),
            i1((self.offset >> 5) as u32),
            i2(self.op as u32)
        ) as u16
    }
}

pub struct CjType {
    pub op: u8,
    pub offset: i16,
    pub funct3: u8,
}

impl CjType {
    #[inline]
    pub fn encode(self) -> u16 {
        encode!(
            i3(self.funct3 as u32),
            i1((self.offset >> 11) as u32),
            i1((self.offset >> 4) as u32),
            i2((self.offset >> 8) as u32),
            i1((self.offset >> 10) as u32),
            i1((self.offset >> 6) as u32),
            i1((self.offset >> 7) as u32),
            i3((self.offset >> 1) as u32),
            i1((self.offset >> 5) as u32),
            i2(self.op as u32)
        ) as u16
    }
}
