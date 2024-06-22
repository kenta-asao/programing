use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // claspの実行結果ファイルのパス
    let path = "result.txt";

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 解のカウンタ
    let mut solution_count = 0;
    let mut clauses = Vec::new();

    // 行を読み込みながら処理
    for line in reader.lines() {
        let line = line?;

        // "Answer:" で始まる行はカウンタをインクリメント
        if line.starts_with("Answer:") {
            solution_count += 1;
            continue;
        }

        // "v " で始まる行を処理
        if line.starts_with("v ") {
            // "v " を取り除く
            let values = line[2..].trim_end();
            let literals: Vec<i32> = values.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // 0で終わるリストの場合、0を除く
            if let Some(&0) = literals.last() {
                clauses.push(literals[..literals.len()-1].to_vec());
            } else {
                clauses.push(literals);
            }
        }
    }

    // CNF形式の出力
    let domain = 4;

    println!("x,y");
    
    for n in 0..clauses.len() {
        let mut x = 0;
        let mut y = 0;
        for i in 0..log2(domain) {
            if clauses[n][i as usize]>0 {
                x = x + power(2,i as i32);
            }
            if clauses[n][(i as usize)+count_variables(&clauses)/2]>0 {
                y = y + power(2,i as i32);
            }
        }
        println!("{},{}", x, y);
    }

    Ok(())
}

fn power(num: i32, a: i32) -> i32 {
    //numのa乗を返す
    let mut x = 1;
    for _n in 0..a {
        x = x * num;
    }
    return x;
}

// 変数の数を数える関数
fn count_variables(clauses: &Vec<Vec<i32>>) -> usize {
    let mut variables = std::collections::HashSet::new();
    for clause in clauses {
        for &literal in clause {
            variables.insert(literal.abs());
        }
    }
    variables.len()
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
