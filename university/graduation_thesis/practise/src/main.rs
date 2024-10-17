use std::fs::File;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let n = 8;//ノード数

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

    if encoding == 1 {
        let variables = n;

        let v_of_v = pairwise_encoding(v);
        let clauses = v_of_v.len();    

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            writeln!(file, "{} {} {}", v_of_v[i][0], v_of_v[i][1], 0).expect("cannot write.");
        }
    }
    else if encoding == 2 {
        let ladder_variable = n-1;

        let variables = n + ladder_variable;
        let v_of_v = ladder_encoding(v, n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            writeln!(file, "{} {} {}", v_of_v[i as usize][0], v_of_v[i as usize][1], 0).expect("cannot write.");
        }
    }
    else if encoding == 3 {
        let variables = n + log2(n);
        let v_of_v = binary_encoding(v, n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            for j in 0..v_of_v[i as usize].len() {
                write!(file, "{} ", v_of_v[i as usize][j]).expect("cannot write.");
            }
            writeln!(file, "{}", 0).expect("cannot write.");
        }
    }
    else if encoding == 4 {
        let variables = n + log2(n);
        let v_of_v = relaxed_ladder_ecncoding(v, n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            for j in 0..v_of_v[i as usize].len() {
                write!(file, "{} ", v_of_v[i as usize][j]).expect("cannot write.");
            }
            writeln!(file, "{}", 0).expect("cannot write.");
        }
    }
    else if encoding == 5 {
        let variables = n + frac(n,3);
        let v_of_v = commander_encoding(v,n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..clauses {
            for j in 0..v_of_v[i as usize].len() {
                write!(file, "{} ", v_of_v[i as usize][j]).expect("cannot write.");
            }
            writeln!(file, "{}", 0).expect("cannot write.");
        }
    }
    else if encoding == 6 {
        let variables = n + root(n) + frac(n,root(n));
        let v_of_v = product_encoding(v,n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..v_of_v.len() {
            writeln!(file, "{} {} {}", v_of_v[i][0], v_of_v[i][1], 0).expect("cannot write.");
        }
    }
    else if encoding == 7 {
        let variables = n + log2(n);
        let v_of_v = bimander_encoding(v,n);
        let clauses = v_of_v.len();

        writeln!(file, "p cnf {} {}", variables, clauses).expect("cannot write.");
        for i in 0..v_of_v.len() {
            writeln!(file, "{} {} {}", v_of_v[i][0], v_of_v[i][1], 0).expect("cannot write.");
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

fn ladder_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
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
            result.push(vec![-i,-(n+i)]);
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

fn relaxed_ladder_ecncoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    //ladder valid clauses
    let ladder_variable = n-1;
    for i in 1..ladder_variable {
        result.push(vec![-(n+i+1), n+i])
    }

    //channelling clauses
    for i in 1..n+1 {
        if i == 1 {
            result.push(vec![-i,-(n+i)]);
        }
        else if i == n {
            result.push(vec![-i,n+i-1]);
        }
        else {
            result.push(vec![-i,n+i-1]);
            result.push(vec![-i,-(n+i)]);
        }
    }

    return result;
}

fn commander_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();
    let mut temp_alo: Vec<i32> = Vec::new();
    let mut temp_co_amo: Vec<i32> = Vec::new();

    let commander_valriable = frac(input.len() as i32, 3);

    for i in 0..input.len() {
        temp.push(input[i]);

        if temp.len() == 3 {
            group.push(temp.clone());
            temp.clear();
        }
    }

    if !temp.is_empty() {
        group.push(temp);
    }

    for i in 0..group.len() {
        temp_alo.push(-(n+(i as i32)+1));
        for j in 0..group[i].len() {
            result.push(vec![n+(i as i32)+1,-group[i][j]]);
            temp_alo.push(group[i][j]);
        }

        result.push(temp_alo.clone());
        temp_alo.clear();

        for j in 0..group[i].len() {
            for k in j+1..group[i].len() {
                result.push(vec![-group[i][j],-group[i][k]]);
            }
        }
    }

    for i in 0..commander_valriable {
        temp_co_amo.push(-(n+(i as i32)+1));
    }

    result.push(temp_co_amo.clone());
    return result;
}

fn product_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let p:i32 = root(n);
    let q:i32 = frac(n,p);
    result.push(Vec::new());
    result.push(Vec::new());

    for i in 1..p+1 {
        result[0].push(-(n+i));
    }

    for i in 1..q+1 {
        result[1].push(-(n+p+i));
    }

    for k in 0..input.len() {
        for i in 0..p {
            for j in 0..q {
                if input[k] == (i+1-1)*q+j+1 {
                    let a:i32 = k as i32;
                    result.push(vec![(-(a+1)).try_into().unwrap(), n+i+1]);
                    result.push(vec![(-(a+1)).try_into().unwrap(), n+p+j+1]);
                }
            }
        }
    }

    return result;
}

fn bimander_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();
    let mut temp_co_amo: Vec<i32> = Vec::new();

    //グループ化
    for i in 0..input.len() {
        temp.push(input[i]);

        if temp.len() == 3 {
            group.push(temp.clone());
            temp.clear();
        }
    }
    if !temp.is_empty() {
        group.push(temp);
    }

    let bimander_variable = log2(group.len() as i32);

    //符号化
    for i in 0..bimander_variable {
        for j in 0..group.len() {
            for k in 0..group[j as usize].len(){
                if i+1 == j.try_into().unwrap() {
                    result.push(vec![-group[j][k], n+i+1]);
                }
                else {
                    result.push(vec![-group[j][k], -(n+i+1)]);
                }
            }
        }
    }

    for i in 0..bimander_variable {
        temp_co_amo.push(-(n+(i as i32)+1));
    }

    result.push(temp_co_amo.clone());

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

fn root(num: i32) -> i32 {
    let mut n:i32 = 0;

    loop {
        if n*n >= num {
            break;
        }
        n +=1;
    }
    return n;
}

fn frac(a: i32, b: i32) -> i32 {
    let mut n:i32 = 0;

    loop {
        if n*b >= a{
            break;
        }
        n +=1;
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

