use crate:: calculation;

pub fn pairwise_encoding(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            result.push(vec![-input[i], -input[j]]);
        }
    }
    return result;
}

pub fn ladder_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
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

pub fn binary_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..input.len() {
        for j in 1..n+1 {
            if input[i] == j {
                for k in 1..calculation::log2(n)+1 {
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

pub fn relaxed_ladder_ecncoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
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

pub fn commander_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();
    let mut temp_alo: Vec<i32> = Vec::new();
    let mut temp_co_amo: Vec<i32> = Vec::new();

    let commander_valriable = calculation::frac(input.len() as i32, 3);

    for i in 0..input.len() {
        temp.push(input[i]);

        if temp.len() == calculation::frac(input.len() as i32, commander_valriable).try_into().unwrap() {
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

    if commander_valriable >= 2 {
        for i in 0..commander_valriable {
            temp_co_amo.push(-(n+(i as i32)+1));
        }
    
        result.push(temp_co_amo.clone());
    }
    
    return result;
}

pub fn product_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let p:i32 = calculation::root(n);
    let q:i32 = calculation::frac(n,p);
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

pub fn bimander_encoding(input: Vec<i32>, n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut group: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();
    let mut temp_co_amo: Vec<i32> = Vec::new();

    //グループ化
    let m = calculation::root(input.len().try_into().unwrap());//１つのグループの要素数

    for i in 0..input.len() {
        temp.push(input[i]);

        if temp.len() == m.try_into().unwrap() {
            group.push(temp.clone());
            temp.clear();
        }
    }
    if !temp.is_empty() {
        group.push(temp);
    }

    let bimander_variable = calculation::log2(group.len() as i32);

    for i in 0..group.len() {
        if group[i].len() >= 2{
            for j in 0..group[i].len()-1 {
                for k in j+1..group[i].len(){
                    result.push(vec![-group[i][j],-group[i][k]])
                }
            }
        }
    }

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

    if bimander_variable >= 2 {
        for i in 0..bimander_variable {
            temp_co_amo.push(-(n+(i as i32)+1));
        }    
        result.push(temp_co_amo.clone());
    }

    return result;
}