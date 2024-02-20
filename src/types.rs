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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CRegister {
    S0, S1, A0, A1,
    A2, A3, A4, A5,
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
