use std::fmt::{Display, Formatter};
pub struct CPU8086 {
    regs: [u16; 8],
    ax: usize,
    bx: usize,
    cx: usize,
    dx: usize,
    bp: usize,
    sp: usize,
    di: usize,
    si: usize,
}

impl CPU8086 {
    pub(crate) fn new() -> Self {
        CPU8086 {
            regs: [0,0,0,0,0,0,0,0],
            ax: 0,
            bx: 1,
            cx: 2,
            dx: 3,
            bp: 4,
            sp: 5,
            di: 6,
            si: 7,
        }
    }

    fn get(&self, reg: Register) -> u16 {
        match reg {
            Register::AX => self.ax(),
            Register::AL => self.al() as u16,
            Register::AH => self.ah() as u16,
            Register::BX => self.bx(),
            Register::BL => self.bl() as u16,
            Register::BH => self.bh() as u16,
            Register::CX => self.cx(),
            Register::CL => self.cl() as u16,
            Register::CH => self.ch() as u16,
            Register::DX => self.dx(),
            Register::DL => self.dl() as u16,
            Register::DH => self.dh() as u16,
            Register::BP => self.bp(),
            Register::SP => self.sp(),
            Register::DI => self.di(),
            Register::SI => self.si(),
        }
    }

    fn set(&mut self, reg: Register, val: u16) {
        match reg {
            Register::AX => self.set_ax(val),
            Register::AL => self.set_al(val as u8),
            Register::AH => self.set_ah(val as u8),
            Register::BX => self.set_bx(val),
            Register::BL => self.set_bl(val as u8),
            Register::BH => self.set_bh(val as u8),
            Register::CX => self.set_cx(val),
            Register::CL => self.set_cl(val as u8),
            Register::CH => self.set_ch(val as u8),
            Register::DX => self.set_dx(val),
            Register::DL => self.set_dl(val as u8),
            Register::DH => self.set_dh(val as u8),
            Register::BP => self.set_bp(val),
            Register::SP => self.set_sp(val),
            Register::DI => self.set_di(val),
            Register::SI => self.set_si(val),
        }
    }

    fn set_bp(&mut self, val: u16) {
        self.regs[self.bp] = val;
    }

    fn bp(&self) -> u16 {
        self.regs[self.bp]
    }

    fn set_sp(&mut self, val: u16) {
        self.regs[self.sp] = val;
    }

    fn sp(&self) -> u16 {
        self.regs[self.sp]
    }

    fn set_di(&mut self, val: u16) {
        self.regs[self.di] = val;
    }

    fn di(&self) -> u16 {
        self.regs[self.di]
    }

    fn set_si(&mut self, val: u16) {
        self.regs[self.si] = val;
    }

    fn si(&self) -> u16 {
        self.regs[self.si]
    }

    // A

    fn set_ax(&mut self, val: u16) {
        let low = val << 8;
        let high = val >> 8;
        self.regs[self.ax] = low ^ high;
    }

    fn ax(&self) -> u16 {
        self.al() as u16 ^ ((self.ah() as u16) << 8)
    }

    fn set_al(&mut self, val: u8) {
        self.regs[self.ax] = ((val as u16) << 8) ^ self.ax();
    }

    fn al(&self) -> u8 {
        (self.regs[self.ax] >> 8) as u8
    }

    fn set_ah(&mut self, val: u8) {
        self.regs[self.ax] = ((self.al() as u16) << 8) ^ val as u16;
    }

    fn ah(&self) -> u8 {
        self.regs[self.ax] as u8
    }

    // B

    fn set_bx(&mut self, val: u16) {
        let low = val << 8;
        let high = val >> 8;
        self.regs[self.bx] = low ^ high;
    }

    fn bx(&self) -> u16 {
        self.bl() as u16 ^ ((self.bh() as u16) << 8)
    }

    fn set_bl(&mut self, val: u8) {
        self.regs[self.bx] = ((val as u16) << 8) ^ self.bx();
    }

    fn bl(&self) -> u8 {
        (self.regs[self.bx] >> 8) as u8
    }

    fn set_bh(&mut self, val: u8) {
        self.regs[self.bx] = ((self.bl() as u16) << 8) ^ val as u16;
    }

    fn bh(&self) -> u8 {
        self.regs[self.bx] as u8
    }

    // C

    fn set_cx(&mut self, val: u16) {
        let low = val << 8;
        let high = val >> 8;
        self.regs[self.cx] = low ^ high;
    }

    fn cx(&self) -> u16 {
        self.cl() as u16 ^ ((self.ch() as u16) << 8)
    }

    fn set_cl(&mut self, val: u8) {
        self.regs[self.cx] = ((val as u16) << 8) ^ self.cx();
    }

    fn cl(&self) -> u8 {
        (self.regs[self.cx] >> 8) as u8
    }

    fn set_ch(&mut self, val: u8) {
        self.regs[self.cx] = ((self.cl() as u16) << 8) ^ val as u16;
    }

    fn ch(&self) -> u8 {
        self.regs[self.cx] as u8
    }

    // D

    fn set_dx(&mut self, val: u16) {
        let low = val << 8;
        let high = val >> 8;
        self.regs[self.dx] = low ^ high;
    }

    fn dx(&self) -> u16 {
        self.dl() as u16 ^ ((self.dh() as u16) << 8)
    }

    fn set_dl(&mut self, val: u8) {
        self.regs[self.dx] = ((val as u16) << 8) ^ self.dx();
    }

    fn dl(&self) -> u8 {
        (self.regs[self.dx] >> 8) as u8
    }

    fn set_dh(&mut self, val: u8) {
        self.regs[self.dx] = ((self.dl() as u16) << 8) ^ val as u16;
    }

    fn dh(&self) -> u8 {
        self.regs[self.dx] as u8
    }
}

impl Display for CPU8086 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        for r in [
            Register::AX,
            Register::BX,
            Register::CX,
            Register::DX,
            Register::SP,
            Register::BP,
            Register::SI,
            Register::DI] {

            let value = self.get(r);
            output = format!("{}\n      {}: 0x{:04x} ({})", output, r.to_string().to_lowercase(), value, value);
        }

        write!(f, "{}", output)
    }
}

pub trait CommandImpl {
    fn execute(&mut self, cpu: &mut CPU8086);
}

pub struct Mov<T: Operand, E: Operand> {
    pub(crate) instruction: Instruction<T, E>
}

impl CommandImpl for Mov<RegisterOp, ImmediateOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op1.register, self.instruction.op2.value);
    }
}

impl CommandImpl for Mov<RegisterOp, RegisterOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op2.register, cpu.get(self.instruction.op1.register));
    }
}

pub struct Add<T: Operand, E: Operand> {
    pub(crate) instruction: Instruction<T, E>
}

impl CommandImpl for Add<RegisterOp, ImmediateOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op1.register, 
            cpu.get(self.instruction.op1.register) + self.instruction.op2.value);
    }
}

impl CommandImpl for Add<RegisterOp, RegisterOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op1.register, 
            cpu.get(self.instruction.op1.register) + cpu.get(self.instruction.op2.register));
    }
}

pub struct Sub<T: Operand, E: Operand> {
    pub(crate) instruction: Instruction<T, E>
}

impl CommandImpl for Sub<RegisterOp, ImmediateOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op1.register, 
            cpu.get(self.instruction.op1.register) - self.instruction.op2.value);
    }
}

impl CommandImpl for Sub<RegisterOp, RegisterOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op2.register, 
            cpu.get(self.instruction.op2.register) - cpu.get(self.instruction.op1.register));
    }
}

pub struct Cmp<T: Operand, E: Operand> {
    pub(crate) instruction: Instruction<T, E>
}

impl CommandImpl for Cmp<RegisterOp, ImmediateOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op1.register, 
            cpu.get(self.instruction.op1.register) - self.instruction.op2.value);
    }
}

impl CommandImpl for Cmp<RegisterOp, RegisterOp> {
    fn execute(&mut self, cpu: &mut CPU8086) {
        cpu.set(self.instruction.op2.register, 
            cpu.get(self.instruction.op2.register) - cpu.get(self.instruction.op1.register));
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Command {
    MOV, ADD, SUB, CMP,
    JNZ, JE, JL, JLE, JB,
    JBE, JP, JO, JS, JNE, JNL,
    JG, JNB, JA, JNP, JNO, JNS,
    LOOP, LOOPZ, LOOPNZ, JCXZ
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Command::MOV => "mov",
            Command::ADD => "add",
            Command::SUB => "sub",
            Command::CMP => "cmp",
            Command::JNZ => "jnz",
            Command::JE => "je",
            Command::JL => "jl",
            Command::JLE => "jle",
            Command::JB => "jb",
            Command::JBE => "jbe",
            Command::JP => "jp",
            Command::JO => "jo",
            Command::JS => "js",
            Command::JNE => "jne",
            Command::JNL => "jnl",
            Command::JG => "jg",
            Command::JNB => "jnb",
            Command::JA => "ja",
            Command::JNP => "jnp",
            Command::JNO => "jno",
            Command::JNS => "jns",
            Command::LOOP => "loop",
            Command::LOOPZ => "loopz",
            Command::LOOPNZ => "loopnz",
            Command::JCXZ => "jcxz",
        })
    }
}

pub trait Operand {}

pub struct RegisterOp {
    pub(crate) register: Register,
}

impl Operand for RegisterOp {}

pub struct ImmediateOp {
    pub(crate) value: u16,
}

impl Operand for ImmediateOp {}

impl From<Instruction<RegisterOp, RegisterOp>> for Box<dyn CommandImpl> {
    fn from(val: Instruction<RegisterOp, RegisterOp>) -> Self {
        match val.command {
            Command::MOV => Box::new(Mov::<RegisterOp, RegisterOp> {
                instruction: val
            }),
            Command::ADD => Box::new(Add::<RegisterOp, RegisterOp> {
                instruction: val
            }),
            Command::SUB => Box::new(Sub::<RegisterOp, RegisterOp> {
                instruction: val
            }),
            Command::CMP => Box::new(Cmp::<RegisterOp, RegisterOp> {
                instruction: val
            }),
            _ => unimplemented!("Reg/Reg")
        }
    }
}

impl From<Instruction<RegisterOp, ImmediateOp>> for Box<dyn CommandImpl> {
    fn from(val: Instruction<RegisterOp, ImmediateOp>) -> Self {
        match val.command {
            Command::MOV => Box::new(Mov::<RegisterOp, ImmediateOp> {
                instruction: val
            }),
            Command::ADD => Box::new(Add::<RegisterOp, ImmediateOp> {
                instruction: val
            }),
            Command::SUB => Box::new(Sub::<RegisterOp, ImmediateOp> {
                instruction: val
            }),
            Command::CMP => Box::new(Cmp::<RegisterOp, ImmediateOp> {
                instruction: val
            }),
            _ => unimplemented!("Immediate/Reg")
        }
    }
}

pub struct Instruction<T: Operand, E: Operand> {
    pub(crate) command: Command,
    pub(crate) op1: T,
    pub(crate) op2: E,
}

#[derive(Clone, Copy)]
pub enum Register {
    AX, AL, AH,
    BX, BL, BH,
    CX, CL, CH,
    DX, DL, DH,
    BP,
    SP,
    DI,
    SI,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let reg_str = match self {
            Register::AX => "ax",
            Register::AL => "al",
            Register::AH => "ah",
            Register::BX => "bx",
            Register::BL => "bl",
            Register::BH => "bh",
            Register::CX => "cx",
            Register::CL => "cl",
            Register::CH => "ch",
            Register::DX => "dx",
            Register::DL => "dl",
            Register::DH => "dh",
            Register::BP => "bp",
            Register::SP => "sp",
            Register::DI => "di",
            Register::SI => "si",
        };

        write!(f, "{}", reg_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu_sim::CPU8086;

    #[test]
    fn test_cpu_register_access() {
        // A
        let mut cpu = CPU8086::new();
        cpu.set_al(0x0001);
        assert_eq!(cpu.ax(), 0x0001);
        cpu.set_al(0);
        cpu.set_ah(0x01);
        assert_eq!(cpu.ax(), 0x0100);
        cpu.set_ah(0);
        cpu.set_al(0);
        cpu.set_ax(0x01);
        assert_eq!(cpu.ax(), 0x0001);
        // B
        let mut cpu = CPU8086::new();
        cpu.set_bl(0x02);
        assert_eq!(cpu.bx(), 2);
        cpu.set_bl(0);
        cpu.set_bh(0x02);
        assert_eq!(cpu.bx(), 512);
        cpu.set_bh(0);
        cpu.set_bl(0);
        cpu.set_bx(0x02);
        assert_eq!(cpu.bx(), 0x0002);
        // C
        let mut cpu = CPU8086::new();
        cpu.set_cl(0x04);
        assert_eq!(cpu.cx(), 4);
        cpu.set_cl(0);
        cpu.set_ch(0x04);
        assert_eq!(cpu.cx(), 1024);
        cpu.set_ch(0);
        cpu.set_cl(0);
        cpu.set_cx(0x03);
        assert_eq!(cpu.cx(), 0x0003);
        // D
        let mut cpu = CPU8086::new();
        cpu.set_dl(0x08);
        assert_eq!(cpu.dx(), 8);
        cpu.set_dl(0);
        cpu.set_dh(0x08);
        assert_eq!(cpu.dx(), 2048);
        cpu.set_dh(0);
        cpu.set_dl(0);
        cpu.set_dx(0x04);
        assert_eq!(cpu.dx(), 0x0004);
        // BP
        let mut cpu = CPU8086::new();
        cpu.set_bp(0x11);
        assert_eq!(cpu.bp(), 0x11);
        // SP
        let mut cpu = CPU8086::new();
        cpu.set_sp(0x22);
        assert_eq!(cpu.sp(), 0x22);
        // DI
        let mut cpu = CPU8086::new();
        cpu.set_di(0x33);
        assert_eq!(cpu.di(), 0x33);
        // SI
        let mut cpu = CPU8086::new();
        cpu.set_si(0x44);
        assert_eq!(cpu.si(), 0x44);
    }
}
