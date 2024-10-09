use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let n = 5;
    let encoding = 2;
    if encoding == 1 {
        pairwise_encoding(n);
    }
    else {
        ladder_ecncoding(n);
    }
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

fn ladder_ecncoding(n: i32) {
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");
    let ladder_variable = n-1;

    let variables = n + ladder_variable;
    let clauses = n-2 + 3*n;

    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");

    //ladder valid clauses
    for i in 1..ladder_variable {
        writeln! {file, "-{} {} 0", n+i+1, n+i}.expect("cannot write.");
    }

    //channelling clauses
    for i in 1..n+1 {
        if i == 1 {
            writeln! {file, "{} {} 0", n+i, i}.expect("cannot write.");
            writeln! {file, "-{} -{} 0", i, n+i}.expect("cannot write.");
        }
        else if i == n {
            writeln! {file, "-{} {} 0", n+i-1, i}.expect("cannot write.");
            writeln! {file, "-{} {} 0", i, n+i-1}.expect("cannot write.");
        }
        else {
            writeln! {file, "-{} {} {} 0", n+i-1, n+i, i}.expect("cannot write.");
            writeln! {file, "-{} {} 0", i, n+i-1}.expect("cannot write.");
            writeln! {file, "-{} -{} 0", i, n+i}.expect("cannot write.");
        }        
    }

    clasp();
}

//claspの実行 -> result.txtに出力
fn clasp() {
    let output_file = File::create("result.txt").expect("Failed to create file");

    let output = Command::new("clasp")
        .stdout(Stdio::from(output_file))
        .args(&["-n", "0", "encoding.cnf"])
        .output()
        .expect("failed");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
