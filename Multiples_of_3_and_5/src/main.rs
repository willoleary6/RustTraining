fn main() {
    // defining the paramenters
    let limit = 1000000000;
    let multiples: [i32; 2] = [3, 5];
    // run calculations
    let results = get_list_of_all_natural_multiples_of_list_below_n(limit, &multiples);


    for x in results{
      println!("{} ", x);
    }
}

fn get_list_of_all_natural_multiples_of_list_below_n(n: i32, multiples: & [i32]) -> Vec<i32>{
    // list to store all numbers
    let mut vec = Vec::new();
    // run through all natural numbers up to n
    for i in 1..n {
        // look at the multiples were looking for
        for j in multiples {
            if i % *j == 0 {
                vec.push(i);
            }

        }
    }
    return vec;
}