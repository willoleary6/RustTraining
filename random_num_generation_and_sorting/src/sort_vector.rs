pub fn sort_vector(mut vec: Vec<i32>){


    // using the in vector command will take ownership of contents and will stop you
    // from doing anything with it after, just enumerate through it in the future
    for x in 0..vec.len(){
        print!("{} ", vec[x]);
    }

    println!();
    vec.sort();

    for x in 0..vec.len(){
        print!("{} ", vec[x]);
    }

}