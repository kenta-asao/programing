pub fn alldifferent(x: Vec<Vec<Vec<i32>>>, n:i32, domain:i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut temp:Vec<i32> = Vec::new();

    //横
    let mut row:Vec<Vec<i32>> = Vec::new();
    for k in 0..domain {
        for i in 0..n {
            for j in 0..n {
                temp.push(x[i as usize][j as usize][k as usize]);
            }
            row.push(temp.clone());
            temp.clear();
        }
    }

    //縦
    let mut line:Vec<Vec<i32>> = Vec::new();
    for k in 0..domain {
        for i in 0..n {
            for j in 0..n {
                temp.push(x[j as usize][i as usize][k as usize]);
            }
            line.push(temp.clone());
            temp.clear();
        }
    }

    //斜め
    let mut diagonal:Vec<Vec<i32>> = Vec::new();
    for k in 0..domain {
        for i in 0..n {
            for j in 0..n {
                temp.push(x[j as usize][((i+j)%n) as usize][k as usize]);
            }
            diagonal.push(temp.clone());
            temp.clear();
        }
    }

    //逆斜め
    let mut reverse_diagonal:Vec<Vec<i32>> = Vec::new();
    for k in 0..domain {
        for i in 0..n {
            for j in 0..n {
                temp.push(x[j as usize][((i+j*(n-1))%n) as usize][k as usize]);
            }
            reverse_diagonal.push(temp.clone());
            temp.clear();
        }
    }

    result.extend(row);
    result.extend(line);
    result.extend(diagonal);
    result.extend(reverse_diagonal);

    return result;
}