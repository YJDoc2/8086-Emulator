use lazy_static::lazy_static;
use std::{collections::HashSet, iter::FromIterator};

lazy_static! {
    pub static ref INSTRUCTIONS: HashSet<&'static str> = {
        let str = r#"mov,push,pop,xchg,xlat,in,out,lea,lds,les,lahf,sahf,pushf,popf,
        add,adc,inc,aaa,daa,sub,sbb,dec,neg,cmp,aas,das,mul,imul,aam,div,idiv,aad,cbw,cwd,
        not,and,or,xor,test,shl,sal,shr,sar,rol,ror,rcl,rcr,
        rep,repe,repz,repne,repnz,movs,movsb,movsw,cmps,scas,loads,stos,
        call,ret,jmp,ja,jnbe,jae,jnb,jb,jnae,jbe,jna,jc,je,jz,jg,jnle,jge,jnl,
        jl,jnge,jle,jng,jnc,jne,jnz,jno,jnp,jpo,jns,jo,jp,jpe,js,
        loop,loope,loopz,loopne,loopnz,jcxz,int,into,iret,
        stc,clc,cmc,std,cld,sti,cli,hlt,wait,esc,lock,nop"#;
        HashSet::from_iter(str.split(",").map(|s| s.trim()))
    };
}
