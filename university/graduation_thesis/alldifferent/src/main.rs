use std::fs::File;
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
    let mut encoding = 1;

    match encoding {
        i32::MIN..= 1 => {
            for i in 0..amo.len() {
                let amo_temp = encoding::pairwise_encoding(amo[i].clone()); // ここで初期化
                for j in 0..amo_temp.len() {
                    result.push(amo_temp[j].clone());
                }
            }
            variables = n * n * domain;
        },
        2 =>  {},
        3..=i32::MAX =>  {},
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
