use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let n = 3;
    pairwise_encoding(n);
}

//pairwise encoding -> encoding.cnfにDIMACS形式で出力
fn pairwise_encoding(n: i32) {
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    let variables = n;
    let clauses = n * (n - 1) / 2;

    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    for i in 1..n + 1 {
        for j in i + 1..n + 1 {
            writeln! {file, "-{} -{} 0", i,j}.expect("cannot write.");
        }
    }

    clasp();
}

fn clasp() {
    let output_file = File::create("result.txt").expect("Failed to create file");

    let output = Command::new("clasp")
        .stdout(Stdio::from(output_file))
        .args(&["-n", "0", "encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}