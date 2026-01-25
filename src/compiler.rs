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
                
                // Check if variable already exists (reassignment in loops)
                let (reg_id, is_new) = if let Some(&existing_reg) = self.var_map.get(name) {
                    (existing_reg, false)  // Reuse existing register
                } else {
                    let new_reg = self.allocate_register();
                    (new_reg, true)  // Allocate new register
                };
                
                if is_new {
                    self.var_map.insert(name.clone(), reg_id);
                }

                out.push(Instruction::SET as i32);
                out.push(reg_id as i32);

                self.compile_expression(body, out);
                if is_new {
                    self.var_map.remove(name);
                }
            }
            Expr::Binary { left, op, right } => {
                self.compile_expression(left, out);
                self.compile_expression(right, out);

                let instr = match op {
                    BinaryOp::Add => Instruction::ADD,
                    BinaryOp::Sub => Instruction::SUB,
                    BinaryOp::Mul => Instruction::MUL,
                    BinaryOp::Div => Instruction::DIV,
                    BinaryOp::Mod => Instruction::MOD,
                    BinaryOp::Expn => Instruction::EXP,
                    BinaryOp::FloorDiv => Instruction::FLRDIV,
                    BinaryOp::Equal => Instruction::EQ,
                    BinaryOp::NotEqual => Instruction::NEQ,
                    BinaryOp::Less => Instruction::LSS,
                    BinaryOp::Greater => Instruction::GTR,
                    BinaryOp::LessEq => Instruction::LEQ,
                    BinaryOp::GreaterEq => Instruction::GEQ,
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
            Expr::If { condition, then_branch, else_branch } => {
                self.compile_expression(condition, out);
                let jz_pos = out.len(); 
                out.push(Instruction::JMZ as i32);
                out.push(0);
                // magic function -> writes byte code of 'if' expression 
                //  and a placeholder JumpIfZero (Jump if return false) with target as 0 
                // then writes then expression with a JMP to end of 'if' block
                // len of out till now () -> set this as the JumpIfZero target so 
                // if false condition -> run from this place ; 
                // then write bytecode for else block 
                // again len () of out and patch this as the JMP to end addr
                // writing this so that I do not forget in future and also 
                // because this logic tickles my brain

                self.compile_expression(then_branch, out);

                let jmp_pos = out.len();
                out.push(Instruction::JMP as i32);
                out.push(0);

                let else_addr = out.len();
                out[jz_pos +1] = else_addr as i32;

                self.compile_expression(else_branch, out);
                let end_addr = out.len();
                out[jmp_pos +1] = end_addr as i32;
                
            }
            Expr::While { condition, body } => {
                // push initial dummy value (will be replaced by body result)
                out.push(Instruction::PSH as i32);
                out.push(0);
                
                let loop_start = out.len();
                //if loop succeed we send the command flow to be back here; 
                self.compile_expression(condition, out);

                let jmz_pos = out.len();
                //if condition fails we send the command flow to end of while loop ;
                out.push(Instruction::JMZ as i32);
                out.push(0);   

                // This also tickles my brain -> we push a dummy value on stack first. 
                // remove that value before putting the vlue that is being calculated in this 
                // iteration. so in the first iter dummy is removed, and then subsequesntly 
                // all the middle ones are removed, only keeping the final calc at the stack; 
                out.push(Instruction::POP as i32);

                self.compile_expression(body, out);

                out.push(Instruction::JMP as i32);
                out.push(loop_start as i32);

                let loop_end = out.len();
                out[jmz_pos + 1] = loop_end as i32;

            }
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
