use std::fs;
use std::fs::File;

fn run_expression(input: &str) -> Result<i32, String> {
    use expression_solver::lexer::Lexer;
    use expression_solver::parser::Parser;
    use expression_solver::compiler::compile;
    use expression_solver::vm::run_program;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;
    
    let bytecode = compile(&ast);
    
    let mut log_file = File::create("/tmp/test_log.log")
        .map_err(|e| format!("Failed to create log file: {}", e))?;
    
    let result = run_program(bytecode, &mut log_file)
        .map_err(|_| "VM error".to_string())?;
    
    result.ok_or_else(|| "No result on stack".to_string())
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(run_expression("5 + 3").unwrap(), 8);
    assert_eq!(run_expression("10 - 4").unwrap(), 6);
    assert_eq!(run_expression("6 * 7").unwrap(), 42);
    assert_eq!(run_expression("20 / 4").unwrap(), 5);
    assert_eq!(run_expression("(5 + 3) * 2").unwrap(), 16);
}

#[test]
fn test_modulus() {
    assert_eq!(run_expression("10 % 3").unwrap(), 1);
    assert_eq!(run_expression("15 % 5").unwrap(), 0);
    assert_eq!(run_expression("7 % 2").unwrap(), 1);
}

#[test]
fn test_exponentiation() {
    assert_eq!(run_expression("2 ** 3").unwrap(), 8);
    assert_eq!(run_expression("5 ** 2").unwrap(), 25);
    assert_eq!(run_expression("3 ** 4").unwrap(), 81);
}

#[test]
fn test_floor_division() {
    assert_eq!(run_expression("7 // 2").unwrap(), 3);
    assert_eq!(run_expression("10 // 3").unwrap(), 3);
    assert_eq!(run_expression("20 // 6").unwrap(), 3);
}

#[test]
fn test_comparisons() {
    assert_eq!(run_expression("5 == 5").unwrap(), 1);
    assert_eq!(run_expression("5 == 3").unwrap(), 0);
    assert_eq!(run_expression("5 > 3").unwrap(), 1);
    assert_eq!(run_expression("3 < 5").unwrap(), 1);
    assert_eq!(run_expression("5 >= 5").unwrap(), 1);
    assert_eq!(run_expression("5 <= 3").unwrap(), 0);
}

#[test]
fn test_variables() {
    assert_eq!(run_expression("define (x 10 define (y 5 x + y))").unwrap(), 15);
    assert_eq!(run_expression("define (x 5 x * 2)").unwrap(), 10);
}

#[test]
fn test_if_statement() {
    assert_eq!(run_expression("if (5 > 3 100 200)").unwrap(), 100);
    assert_eq!(run_expression("if (3 > 5 100 200)").unwrap(), 200);
    assert_eq!(run_expression("define (x 10 if (x < 5 (x * 2) (x + 5)))").unwrap(), 15);
}

#[test]
fn test_while_loop() {
    let expr = "define (x 5 define (sum 0 define (dummy while (x > 0 define (sum (sum + x) define (x (x - 1) sum))) sum)))";
    assert_eq!(run_expression(expr).unwrap(), 15); // 5+4+3+2+1
}

#[test]
fn test_factorial_program() {
    let input = fs::read_to_string("tests/sample.expr")
        .expect("Failed to read sample.expr");
    
    let result = run_expression(&input).unwrap();
    assert_eq!(result, 120); 
}

#[test]
fn test_odd_even_program() {
    let input = fs::read_to_string("tests/sample2.expr")
        .expect("Failed to read sample2.expr");
    
    let result = run_expression(&input).unwrap();
    assert_eq!(result, 1); // 7 is odd, returns 1
    
    let even_test = "define (n 8 if ((n % 2) == 1 1 0))";
    assert_eq!(run_expression(even_test).unwrap(), 0); // 8 is even
}

#[test]
fn test_fibonacci_program() {
    let input = fs::read_to_string("tests/sample3.expr")
        .expect("Failed to read sample3.expr");
    
    let result = run_expression(&input).unwrap();
    assert_eq!(result, 13); // 7th fibonacci: 1,1,2,3,5,8,13
}
