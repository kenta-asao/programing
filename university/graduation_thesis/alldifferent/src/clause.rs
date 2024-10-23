pub fn at_most_one(input: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    if input.len() >= 2{
        for i in 0..input.len() {
            for k in i+1..input.len() {
                result.push(vec![-input[i],-input[k]]);
            }
        }
    }

    return result;
}

pub fn at_least_one(input: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..input.len() {
        result.push(input[i]);
    }

    return result;
}