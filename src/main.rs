use std::env;
mod machine_state;
use machine_state::{OpCode, MachineState};

fn main() {
    let args: Vec<_> = env::args().collect(); //TODO: parse program from file
    let program: Vec<OpCode> = vec![OpCode::PSH(4), OpCode::PSH(4), OpCode::JMPEQ(5), OpCode::PSH(2), OpCode::POP, OpCode::HLT];
    let mut state: MachineState = MachineState::new(program);

    state.running = true;
    while state.running == true && state.pc < state.program.len() {
        eval(&mut state);
        //print_stack(&state.stack);
        state.pc += 1;
    }
}

fn eval(state: &mut MachineState) {
    match *state.fetch() {
        OpCode::PSH(val) => push(&mut state.stack, val),
        OpCode::POP => println!("POP: {}", pop(&mut state.stack)),
        OpCode::ADD => add(&mut state.stack),
        OpCode::SUB => sub(&mut state.stack),
        OpCode::MUL => mul(&mut state.stack),
        OpCode::DIV => div(&mut state.stack),
        OpCode::JMP(addr) => jmp(state, addr),
        OpCode::JMPEQ(addr) => jmpeq(state, addr),
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
    state.pc = addr - 1; // We need to subtract here because the PC is incremented after evaluating the JMP instruction
}

fn jmpeq(state: &mut MachineState, addr: usize) {
    sub(&mut state.stack);
    if pop(&mut state.stack) == 0 { // If the top two stack elements are equal, their difference is 0
        jmp(state, addr);
    }
}

/// Prints the stack for debugging purposes
fn print_stack(stack: &Vec<i32>) {
    println!("=== STACK ===");
    for element in stack.iter().rev() { // Iterate backwards through the vector (top element is the last element in the vector)
        println!("{}", element);
    }
    println!("");
}
