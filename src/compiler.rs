use crate::{
    parser::{BinaryOp, Expr, UnaryOp},
    vm::Instruction,
};

// out -> contains the bytecode for vm (form of Vec<i32>)
pub fn compile_expression(expr: &Expr, out: &mut Vec<i32>) {
    match expr {
        Expr::Number(n) => {
            out.push(Instruction::PSH as i32);
            out.push(*n);
        }
        Expr::Binary { left, op, right } => {
            compile_expression(left, out);
            compile_expression(right, out);

            let instr = match op {
                BinaryOp::Add => Instruction::ADD,
                BinaryOp::Sub => Instruction::SUB,
                BinaryOp::Mul => Instruction::MUL,
                BinaryOp::Div => Instruction::DIV,
            };
            
            out.push(instr as i32);
        }
        Expr::Unary { op, expr } => {
            match op {
                UnaryOp::Neg => {
                    out.push(Instruction::PSH as i32);
                    out.push(0);
                    compile_expression(expr, out);
                    out.push(Instruction::SUB as i32);
                }
            }
        }
        &Expr::Variable(_) | &Expr::Let { .. } => todo!()
    }
}

pub fn compile(expr: &Expr) -> Vec<i32> {
    let mut program = Vec::new();
    compile_expression(expr, &mut program);
    program.push(Instruction::HLT as i32);
    program
}