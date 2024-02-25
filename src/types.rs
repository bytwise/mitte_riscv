use std::fmt;
use std::convert::TryFrom;
use std::error::Error;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Register {
    Zero, Ra, Sp, Gp,
    Tp, T0, T1, T2,
    S0, S1, A0, A1,
    A2, A3, A4, A5,
    A6, A7, S2, S3,
    S4, S5, S6, S7,
    S8, S9, S10, S11,
    T3, T4, T5, T6,
}

impl Register {
    #[inline]
    pub fn from_index(index: usize) -> Option<Register> {
        use Register::*;
        match index {
            0 => Some(Zero),
            1 => Some(Ra),
            2 => Some(Sp),
            3 => Some(Gp),
            4 => Some(Tp),
            5 => Some(T0),
            6 => Some(T1),
            7 => Some(T2),
            8 => Some(S0),
            9 => Some(S1),
            10 => Some(A0),
            11 => Some(A1),
            12 => Some(A2),
            13 => Some(A3),
            14 => Some(A4),
            15 => Some(A5),
            16 => Some(A6),
            17 => Some(A7),
            18 => Some(S2),
            19 => Some(S3),
            20 => Some(S4),
            21 => Some(S5),
            22 => Some(S6),
            23 => Some(S7),
            24 => Some(S8),
            25 => Some(S9),
            26 => Some(S10),
            27 => Some(S11),
            28 => Some(T3),
            29 => Some(T4),
            30 => Some(T5),
            31 => Some(T6),
            _ => None,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CRegister {
    S0, S1, A0, A1,
    A2, A3, A4, A5,
}

impl CRegister {
    #[inline]
    pub fn from_c_index(index: usize) -> Option<CRegister> {
        use CRegister::*;
        match index {
            0 => Some(S0),
            1 => Some(S1),
            2 => Some(A0),
            3 => Some(A1),
            4 => Some(A2),
            5 => Some(A3),
            6 => Some(A4),
            7 => Some(A5),
            _ => None,
        }
    }
}


impl From<CRegister> for Register {
    #[inline]
    fn from(reg: CRegister) -> Register {
        match reg {
            CRegister::S0 => Register::S0,
            CRegister::S1 => Register::S1,
            CRegister::A0 => Register::A0,
            CRegister::A1 => Register::A1,
            CRegister::A2 => Register::A2,
            CRegister::A3 => Register::A3,
            CRegister::A4 => Register::A4,
            CRegister::A5 => Register::A5,
        }
    }
}

impl TryFrom<Register> for CRegister {
    type Error = CRegisterTryFromError;

    #[inline]
    fn try_from(reg: Register) -> Result<CRegister, CRegisterTryFromError> {
        match reg {
            Register::S0 => Ok(CRegister::S0),
            Register::S1 => Ok(CRegister::S1),
            Register::A0 => Ok(CRegister::A0),
            Register::A1 => Ok(CRegister::A1),
            Register::A2 => Ok(CRegister::A2),
            Register::A3 => Ok(CRegister::A3),
            Register::A4 => Ok(CRegister::A4),
            Register::A5 => Ok(CRegister::A5),
            _ => Err(CRegisterTryFromError(reg)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CRegisterTryFromError(Register);

impl fmt::Display for CRegisterTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not convert {:?} to CRegister", self.0)
    }
}

impl Error for CRegisterTryFromError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_from_index() {
        for i in 0..32 {
            assert_eq!(i, Register::from_index(i).unwrap() as usize);
        }
    }

    #[test]
    fn test_cregister_from_c_index() {
        for i in 0..8 {
            assert_eq!(i, CRegister::from_c_index(i).unwrap() as usize);
        }
    }
}
