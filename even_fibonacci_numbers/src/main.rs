fn main() {
    let fibonacci_sequence = build_fibonacci(4000000);
    let mut even_sum = 0;
    for x in fibonacci_sequence{
        if x % 2==0 {
            even_sum += x;
        }
    }

    print!("{} ", even_sum);
}

fn build_fibonacci(limit: i64) -> Vec<i64>{
    let mut vec: Vec<i64> = Vec::new();
    vec.push(1);
    vec.push(2);

    let mut hit_limit = false;
    while !hit_limit {
        if vec[vec.len() -1] > limit{
            hit_limit = true;
            // remove exceeding value
            vec.pop();
        }else {
            vec.push(vec[vec.len() - 1] + vec[vec.len() - 2]);
        }
    }
    return vec;
}