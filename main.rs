enum OpCode {
    PSH(i32),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT
}

fn main() {
    let program: Vec<OpCode> = vec![OpCode::PSH(4), OpCode::PSH(2), OpCode::MUL, OpCode::POP];
    let mut stack: Vec<i32> = Vec::new();

    let mut pc = 0;
    let running = true;
    while running == true && pc < program.len() {
        eval(&program[pc], &mut stack);
        pc += 1;
    }
}

fn eval(c: &OpCode, stack: &mut Vec<i32>) {
    match c {
        &OpCode::PSH(val) => push(stack, val),
        &OpCode::POP => println!("{}", pop(stack)),
        &OpCode::ADD => add(stack),
        &OpCode::SUB => sub(stack),
        &OpCode::MUL => mul(stack),
        &OpCode::DIV => div(stack),
        &OpCode::HLT => println!("HLT")
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
