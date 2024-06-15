use std::process::Command;
 
 fn main() {
    let output = Command::new("clasp")
        .args(&["-n", "0", "direct_encoding.cnf", ">", "result.txt"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
 }