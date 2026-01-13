use crate::vm::Instruction;

pub fn string_to_instruction(token: &str) -> Instruction {
    match token {
        "PSH" => Instruction::PSH,
        "POP" => Instruction::POP,
        "ADD" => Instruction::ADD,
        "SUB" => Instruction::SUB,
        "MUL" => Instruction::MUL,
        "DIV" => Instruction::DIV,
        "SET" => Instruction::SET,
        "HLT" => Instruction::HLT,
        _ => Instruction::UNK,
    }
}