

fn main() {
    println!("Hello, world!");
    let pms =prime_factors( 600851475143 );

    for x in pms{
        print!("{} ", x);
    }
}

fn prime_factors(n:i64) -> Vec<i64> {
    let mut vec :Vec<i64> = Vec::new();
    let mut num :i64 = n;
    while num % 2 == 0 {
        vec.push(2);
        num = num / 2;
    }

    for i in (3..(num as f64).sqrt() as i64 +1).step_by(2) {
        while num % i == 0{
            vec.push(i);
            num = num/i;
        }
    }

    if num > 2 {
        vec.push(num);
    }
    return vec;
}