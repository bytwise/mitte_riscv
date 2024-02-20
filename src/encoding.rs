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


pub const AUIPC: u32     = 0b0010111;
pub const LUI: u32       = 0b0110111;
pub const JAL: u32       = 0b1101111;

#[allow(unused)]
pub enum Opcode {
    Null    = 0,
    Load    = 0b0000011,
    LoadFp  = 0b0000111,
    Custom0 = 0b0001011,
    MiscMem = 0b0001111,
    OpImm   = 0b0010011,
    Auipc   = 0b0010111,
    OpImm32 = 0b0011011,
    Store   = 0b0100011,
    StoreFp = 0b0100111,
    Custom1 = 0b0101011,
    Amo     = 0b0101111,
    Op      = 0b0110011,
    Lui     = 0b0110111,
    Op32    = 0b0111011,
    Madd    = 0b1000011,
    Msub    = 0b1000111,
    Nmsub   = 0b1001011,
    Nmadd   = 0b1001111,
    OpFp    = 0b1010011,
    // Reserved = 0b1010111,
    Custom2 = 0b1011011,
    Branch  = 0b1100011,
    Jalr    = 0b1100111,
    // Reserved = 0b1101011,
    Jal     = 0b1101111,
    System  = 0b1110011,
    // Reserved = 0b1110111,
    Custom3 = 0b1111011,
}

pub use Opcode::*;


pub struct RType {
    pub opcode: Opcode,
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
    pub opcode: Opcode,
    pub funct3: u8,
    pub rd: Register,
    pub rs: Register,
    pub imm12: i16,
}

impl IType {
    #[inline]
    pub fn null() -> IType {
        IType {
            opcode: Opcode::Null,
            funct3: 0,
            rd: Register::Zero,
            rs: Register::Zero,
            imm12: 0,
        }
    }

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
    pub opcode: Opcode,
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
    pub opcode: Opcode,
    pub funct3: u8,
    pub rs1: Register,
    pub rs2: Register,
    pub offset: i16,
}

impl BType {
    #[inline]
    pub fn null() -> BType {
        BType {
            opcode: Opcode::Null,
            funct3: 0,
            rs1: Register::Zero,
            rs2: Register::Zero,
            offset: 0,
        }
    }

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
    pub fn null() -> CbType {
        CbType {
            op: 0,
            rs: CRegister::S0,
            offset: 0,
            funct3: 0,
        }
    }

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
    pub fn null() -> CjType {
        CjType {
            op: 0,
            offset: 0,
            funct3: 0,
        }
    }

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
