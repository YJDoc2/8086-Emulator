#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[rustfmt::skip]
pub enum Register{
    AL,AH,AX,
    BL,BH,BX,
    CL,CH,CX,
    DL,DH,DX,
    BP,SP,SI,DI,
    ES,CS,DS,SS
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    Word,
    Byte,
}

pub enum IR{
    Allocate{
        size:Size,
        items:Vec<u16>
    }   
}