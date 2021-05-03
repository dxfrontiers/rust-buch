use showcase::{generate_data, sort_recursive, check_sorted, generate_custom_data};

fn main(){
    //let mut data = generate_data();
    let mut data = generate_custom_data(20);


    let start_time = coarsetime::Instant::now();
    for _ in 0..100 {
        sort_recursive(&mut data);
    }
    let millis = start_time.elapsed().as_millis();
    println!("Sorting recursive took {}",millis);

    check_sorted(&data);
    println!("Data was ok");
}

