use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let path = "graph_coloring.txt"; // 読み込むファイルのパス

    // ファイルを開く
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut num_vertices = 0;
    let mut edges = Vec::new();
    let color = 4;

    let p_edge_re = Regex::new(r"p edge (\d+) (\d+)").unwrap();
    let e_re = Regex::new(r"e (\d+) (\d+)").unwrap();

    // グラフの頂点とエッジを格納する
    let mut num_vertices = 0;
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // 各行を読み込み、パターンにマッチするかを確認
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = p_edge_re.captures(&line) {
            num_vertices = caps[1].parse().unwrap();
            println!("Number of vertices: {}", num_vertices);
        } else if let Some(caps) = e_re.captures(&line) {
            let u: usize = caps[1].parse().unwrap();
            let v: usize = caps[2].parse().unwrap();
            edges.push([u,v]);
        }
        
    }

    direct_encdoing(color,num_vertices,edges);
}

fn direct_encdoing(color:i32, num_vertices:i32, edges:Vec<[usize; 2]>) {
    let path = "encoding.cnf";
    let mut file = File::create(path).expect("file not found.");
    
    let variables = color * num_vertices;
    let clauses = num_vertices + color * (color - 1) * num_vertices / 2 + color;
    writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
    /*at-least-one */
    for n in 0..num_vertices {
        let m = n as i32;
        for j in 1..color + 1 {
            let i = j as i32;
            write! {file, "{} ", m*color+i}.expect("cannot write.");
        }
        writeln!(file, "0").expect("cannot write.");
    }
    /*at-most-one */
    for n in 0..num_vertices {
        let m = n as i32;
        for comb in (m * color + 1..(m + 1) * color + 1).combinations(2) {
            writeln!(file, "-{} -{} 0", comb[0], comb[1]).expect("cannot write.");
        }
    }
}

fn decryption(domain: i32) -> std::io::Result<()> {
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
            let literals: Vec<i32> = values
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            // 0で終わるリストの場合、0を除く
            if let Some(&0) = literals.last() {
                clauses.push(literals[..literals.len() - 1].to_vec());
            } else {
                clauses.push(literals);
            }
        }
    }

    // CNF形式の出力

    println!("解");

    
        for n in 0..clauses.len() {
            let mut x = 0;
            let mut y = 0;
            for i in 0..domain {
                if clauses[n][i as usize] > 0 {
                    x = i;
                }
                if clauses[n][(i as usize) + count_variables(&clauses) / 2] > 0 {
                    y = i;
                }
            }
            println!("{},{}", x, y);
        }
    

    Ok(())
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

fn contains<T: PartialEq>(array: &[T], value: T) -> bool {
    for element in array.iter() {
        if *element == value {
            return true;
        }
    }
    false
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

fn power(num: i32, a: i32) -> i32 {
    //numのa乗を返す
    let mut x = 1;
    for _n in 0..a {
        x = x * num;
    }
    return x;
}
