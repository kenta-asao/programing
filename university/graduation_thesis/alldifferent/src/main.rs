use std::fs::File;
use std::io;
use std::io::Write;

mod clause;
mod encoding;
mod calculation;
mod clasp;
mod decryption;
mod alldifferent;

fn main(){
    let n = 5;
    let domain = 5;
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");

    let mut encoding = String::new();
    println!("実行する符号化を選択してください。");
    println!("1:pairwise encoding");
    println!("2:ladder encoding");
    println!("3:binary encoding");
    println!("4:relaxed ladder encoding");
    println!("5:commander encoding");
    println!("6:product encoding");
    println!("7:bimander encoding");

    io::stdin()
        .read_line(&mut encoding)
        .expect("Failed to read line");
    let encoding: i32 = encoding.trim().parse().expect("Please type a number!");

    let mut x: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();
    let mut temp_temp:Vec<Vec<i32>> = Vec::new();
    let mut result:Vec<Vec<i32>> = Vec::new();

    //変数
    for i in 0..n*n*domain {
        temp.push(i+1);
        if (i+1)%domain == 0 {
            temp_temp.push(temp.clone());
            temp.clear();
        }
        if temp_temp.len() == (n as usize) {
            x.push(temp_temp.clone());
            temp_temp.clear();
        }
    }

    //ALO
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            result.push(clause::at_least_one(&x[i][j]));
        }
    }

    let mut amo_temp:Vec<Vec<i32>> = Vec::new();

    //それぞれの変数のAMO
    for i in 0..n {
        for j in 0..n {
            amo_temp = encoding::pairwise_encoding(x[i as usize][j as usize].clone());
            for k in 0..amo_temp.len() {
                result.push(amo_temp[k].clone());
            }
        }
    }

    //縦横斜めのAMO
    let amo = alldifferent::alldifferent(x,n,domain);
    let mut amo_temp:Vec<Vec<i32>> = Vec::new();
    let mut variables = 0;

    match encoding {
        i32::MIN..= 1 => {
            for i in 0..amo.len() {
                let amo_temp = encoding::pairwise_encoding(amo[i].clone());
                for j in 0..amo_temp.len() {
                    result.push(amo_temp[j].clone());
                }
            }
            variables = n * n * domain;
        },
        2 => {},
        3 => {
            for i in 0..amo.len() {
                let amo_temp = encoding::binary_encoding(amo[i].clone(), n*n*domain);
                for j in 0..amo_temp.len() {
                    result.push(amo_temp[j].clone());
                }
            }
            variables = n * n * domain +calculation::log2(n*n*domain);
        },
        4 => {},
        5 => {},
        6 => {},
        7..=i32::MAX =>  {},
    }

    let clauses = result.len(); 
    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    for i in 0..clauses {
        for j in 0..result[i as usize].len() {
            write!(file, "{} ", result[i as usize][j]).expect("cannot write.");
        }
        writeln!(file, "{}", 0).expect("cannot write.");
    }

    clasp::clasp();

    let _ = decryption::decryption(n, domain);
}
