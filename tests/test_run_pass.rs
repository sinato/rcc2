use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn run(input: &str, expect: &str) {
    // compile
    Command::new("sh")
        .arg("-c")
        .arg(format!("./target/debug/rcc2 \"{}\"", input))
        .status()
        .expect("process failed to execute");

    // run generated IR and get returned status code
    let status = Command::new("sh")
        .arg("-c")
        .arg("llvm-as compiled.ll; lli compiled.bc")
        .status()
        .expect("failed to execute process");

    println!("{:?} => {:?}", status.to_string(), expect);
    assert!(status.to_string() == String::from(format!("exit code: {}", expect)));
}

fn get_code(filename: &str) -> String {
    let filename = String::from("./tests/resources/") + filename;
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("somethig went wrong reading the file");
    contents
}

#[test]
fn test_addition() {
    let code = get_code("test_addition.c");
    run(&code, "3")
}

#[test]
fn test_array() {
    let code = get_code("test_array.c");
    run(&code, "33")
}

#[test]
fn test_array2() {
    let code = get_code("test_array2.c");
    run(&code, "77")
}

#[test]
fn test_array3() {
    let code = get_code("test_array3.c");
    run(&code, "72")
}

#[test]
fn test_division() {
    let code = get_code("test_division.c");
    run(&code, "6")
}

#[test]
fn test_function_call() {
    let code = get_code("test_function_call.c");
    run(&code, "10")
}

#[test]
fn test_function_call2() {
    let code = get_code("test_function_call2.c");
    run(&code, "35")
}

#[test]
fn test_multi_statement() {
    let code = get_code("test_multi_statements.c");
    run(&code, "23")
}

#[test]
fn test_multi_term() {
    let code = get_code("test_multi_term.c");
    run(&code, "7")
}

#[test]
fn test_multiplication() {
    let code = get_code("test_multiplication.c");
    run(&code, "2")
}

#[test]
fn test_pointer() {
    let code = get_code("test_pointer.c");
    run(&code, "24")
}

#[test]
fn test_subtraction() {
    let code = get_code("test_subtraction.c");
    run(&code, "5")
}

#[test]
fn test_variable() {
    let code = get_code("test_variable.c");
    run(&code, "26")
}
