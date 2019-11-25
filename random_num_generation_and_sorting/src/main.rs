mod random_num_generation;
mod random_password_generator;
mod sort_vector;

fn main() {
    random_num_generation::generate_random_nums();

    random_num_generation::generate_random_integer_with_ranges(0, 5);
    random_num_generation::generate_random_float_with_ranges(5.0, 10.5);

    random_password_generator::generate_random_password(30);

    let mut my_vec =  Vec::new();
    my_vec.push(1);
    my_vec.push(4);
    my_vec.push(6);
    my_vec.push(123434);
    my_vec.push(23);
    my_vec.push(5);
    my_vec.push(6);
    my_vec.push(1);
    my_vec.push(3);
    my_vec.push(7);
    my_vec.push(0);
    my_vec.push(-15);


    sort_vector::sort_vector(my_vec);
}
