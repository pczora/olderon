enum OpCode {
    PSH(i32),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP(usize),
    HLT
}

struct MachineState {
    stack: Vec<i32>,
    pc: usize,
    running: bool,
    program: Vec<OpCode>
}

impl MachineState {
    fn new(program: Vec<OpCode>) -> MachineState {
        MachineState {
            stack: Vec::new(),
            pc: 0,
            running: false,
            program: program
        }
    }

    fn fetch(&self) -> &OpCode {
        &self.program[self.pc]
    }
}

fn main() {
    let program: Vec<OpCode> = vec![OpCode::PSH(4), OpCode::PSH(2), OpCode::DIV, OpCode::POP];
    let mut state: MachineState = MachineState::new(program);

    state.running = true;
    while state.running == true && state.pc < state.program.len() {
        eval(&mut state);
        state.pc += 1;
    }
}

fn eval(state: &mut MachineState) {
    match *state.fetch() {
        OpCode::PSH(val) => push(&mut state.stack, val),
        OpCode::POP => println!("{}", pop(&mut state.stack)),
        OpCode::ADD => add(&mut state.stack),
        OpCode::SUB => sub(&mut state.stack),
        OpCode::MUL => mul(&mut state.stack),
        OpCode::DIV => div(&mut state.stack),
        OpCode::JMP(addr) => jmp(state, addr),
        OpCode::HLT => println!("HLT")
    }
}

fn push(stack: &mut Vec<i32>, val: i32) {
    stack.push(val);
}

fn pop(stack: &mut Vec<i32>) -> i32 {
    match stack.pop() {
        Some(val) => {
            return val;
        },
        None => {
            return -1;
        }
    }
}

fn add(stack: &mut Vec<i32>) {
    let a = pop(stack);
    let b = pop(stack);
    push(stack, b + a);
}

fn sub(stack: &mut Vec<i32>) {
    let a = pop(stack);
    let b = pop(stack);
    push(stack, b - a);
}

fn mul(stack: &mut Vec<i32>) {
    let a = pop(stack);
    let b = pop(stack);
    push(stack, b * a);
}

fn div(stack: &mut Vec<i32>) {
    let a = pop(stack);
    let b = pop(stack);
    push(stack, b / a);
}

fn jmp(state: &mut MachineState, addr: usize) {
    state.pc = addr;
}
