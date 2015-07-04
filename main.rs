enum OpCode {
    PSH(i32),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    LOADI(usize, i32)
}

fn main() {
    let program: Vec<OpCode> = vec![OpCode::PSH(4), OpCode::PSH(2), OpCode::DIV, OpCode::POP, OpCode::LOADI(0, 32)];
    let mut stack: Vec<i32> = Vec::new();
    let mut regs: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0]; // TODO: Create constant for number of registers
    let mut pc = 0;
    let running = true;
    while running == true && pc < program.len() {
        eval(&program[pc], &mut stack, &mut regs);
        pc += 1;
        print_regs(regs);
    }
}

fn eval(c: &OpCode, stack: &mut Vec<i32>, regs: &mut [i32; 8]) {
    match c {
        &OpCode::PSH(val) => push(stack, val),
        &OpCode::POP => println!("{}", pop(stack)),
        &OpCode::ADD => add(stack),
        &OpCode::SUB => sub(stack),
        &OpCode::MUL => mul(stack),
        &OpCode::DIV => div(stack),
        &OpCode::HLT => println!("HLT"),
        &OpCode::LOADI(reg, val) => loadi(val, reg, regs)
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

fn loadi(val: i32, reg: usize, regs: &mut [i32; 8]) {
    regs[reg] = val;
}

fn print_regs(regs: [i32; 8]) {
    println!("Registers: ");
    for reg in regs.iter() {
        println!("{}", reg);
    }
    println!("");
}
