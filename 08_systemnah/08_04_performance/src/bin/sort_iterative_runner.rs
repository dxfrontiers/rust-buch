use rust_buch_08_04_performance::{sort_iterative, check_sorted, generate_custom_data};

/**
    Sort 100 * 2^20 random elements and check if they have been sorted in the end
    We print the time taken to sort the data 100 times.
    In 99 Times we work on sorted data but thats ok.
*/
fn main(){
    let mut data = generate_custom_data(20);

    let start_time = coarsetime::Instant::now();
    for _ in 0..100 {
        sort_iterative(&mut data);
    }
    let millis = start_time.elapsed().as_millis();
    println!("Sorting iterative took {}",millis);

    check_sorted(&data);
    println!("Data was ok");
}

