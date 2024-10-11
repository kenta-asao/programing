use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let n = vec![1,3,5]
    amo = pairwise_encoding(n);
}

//pairwise encoding -> encoding.cnfにDIMACS形式で出力
fn pairwise_encoding(n: Vec<T>) {

    let mut amo = vec![];

    for i in 1..n + 1 {
        for j in i + 1..n + 1 {
            amo.push(vec![-n[i], -n[j], 0]);
        }
    }
    return amo;
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