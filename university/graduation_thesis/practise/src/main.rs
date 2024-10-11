use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let v = vec![1, 2, 3];

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

    let n = 3;//ノード数

    if encoding == 1 {
        let variables = n;

        let v_of_v = pairwise_encoding(v);
        let clauses = v_of_v.len();    

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            writeln!(file, "{} {} {}", v_of_v[i][0], v_of_v[i][1], 0);
        }
    }
    else if encoding == 2 {
        let ladder_variable = n-1;

        let variables = n + ladder_variable;
        let v_of_v = binary_encoding(v, n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            writeln!(file, "{} {} {}", v_of_v[i as usize][0], v_of_v[i as usize][1], 0);
        }
    }
    else if encoding == 3 {
        let variables = n + log2(n);
        let v_of_v = binary_encoding(v, n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            for j in 0..v_of_v[i as usize].len() {
                write!(file, "{} ", v_of_v[i as usize][j]);
            }
            writeln!(file, "{}", 0);            
        }
    }
    clasp();
}

fn pairwise_encoding(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            result.push(vec![-input[i], -input[j]]);
        }
    }
    return result;
}

fn ladder_ecncoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    //ladder valid clauses
    let ladder_variable = n-1;
    for i in 1..ladder_variable {
        result.push(vec![-(n+i+1), n+i])
    }

    //channelling clauses
    for i in 1..n+1 {
        if i == 1 {
            result.push(vec![n+i, i]);
            result.push(vec![-i,-(n+i)]);
        }
        else if i == n {
            result.push(vec![-(n+i-1), i]);
            result.push(vec![-i,n+i-1]);
        }
        else {
            result.push(vec![-(n+i-1), n+i,i]);
            result.push(vec![-i,n+i-1]);
            result.push(vec![-i,n-(n+i)]);
        }
    }

    return result;
}

fn binary_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..input.len() {
        for j in 1..n+1 {
            if input[i] == j {
                for k in 1..log2(n)+1 {
                    if j == (k+1) {
                        result.push(vec![-j, n + k]);
                    }
                    else {
                        result.push(vec![-j, -(n + k)]);
                    }
                }
            }
        }
    }

    return result;
}

fn log2(num: i32) -> i32 {
    let mut n: i32 = 0;
    loop {
        if n * 2 >= num {
            break;
        }
        n += 1;
    }
    return n;
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