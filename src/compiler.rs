use crate::{
    parser::{BinaryOp, Expr, UnaryOp},
    vm::Instruction,
};
use std::collections::HashMap;

pub struct Compiler {
    var_map: HashMap<String, usize>,
    next_register: usize,
}
impl Compiler {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            next_register: 0,
        }
    }
    fn allocate_register(&mut self) -> usize {
        let reg = self.next_register;
        self.next_register += 1;
        reg
    }

    pub fn compile_expression(&mut self, expr: &Expr, out: &mut Vec<i32>) {
        match expr {
            Expr::Number(n) => {
                out.push(Instruction::PSH as i32);
                out.push(*n);
            }
            Expr::Variable(name) => {
                let reg_id = self
                    .var_map
                    .get(name)
                    .expect(&format!("Undefined Variable : {}", name));
                out.push(Instruction::GET as i32);
                out.push(*reg_id as i32);
            }
            Expr::Define { name, value, body } => {
                self.compile_expression(value, out);
                let reg_id = self.allocate_register();
                self.var_map.insert(name.clone(), reg_id);

                out.push(Instruction::SET as i32);
                out.push(reg_id as i32);

                self.compile_expression(body, out);
                self.var_map.remove(name);
            }
            Expr::Binary { left, op, right } => {
                self.compile_expression(left, out);
                self.compile_expression(right, out);

                let instr = match op {
                    BinaryOp::Add => Instruction::ADD,
                    BinaryOp::Sub => Instruction::SUB,
                    BinaryOp::Mul => Instruction::MUL,
                    BinaryOp::Div => Instruction::DIV,
                };

                out.push(instr as i32);
            }
            Expr::Unary { op, expr } => match op {
                UnaryOp::Neg => {
                    out.push(Instruction::PSH as i32);
                    out.push(0);
                    self.compile_expression(expr, out);
                    out.push(Instruction::SUB as i32);
                }
            },
        }
    }
}

// out -> contains the bytecode for vm (form of Vec<i32>)
pub fn compile(expr: &Expr) -> Vec<i32> {
    let mut compiler = Compiler::new();
    let mut program = Vec::new();
    compiler.compile_expression(expr, &mut program);
    program.push(Instruction::HLT as i32);
    program
}
