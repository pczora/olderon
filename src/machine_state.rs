pub enum OpCode {
    PSH(i32),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP(usize),
    JMPEQ(usize),
    HLT
}

/// Represents the state of the virtual machine
pub struct MachineState {
    pub stack: Vec<i32>,
    pub pc: usize,
    pub running: bool,
    pub program: Vec<OpCode>
}

impl MachineState {
    pub fn new(program: Vec<OpCode>) -> MachineState {
        MachineState {
            stack: Vec::new(),
            pc: 0,
            running: false,
            program: program
        }
    }

    pub fn fetch(&self) -> &OpCode {
        &self.program[self.pc]
    }
}
