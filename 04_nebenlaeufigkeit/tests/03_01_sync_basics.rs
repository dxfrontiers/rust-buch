#[cfg(test)]
mod lifetimes {
    use std::sync::{Arc, Mutex};
    use std::{thread, time};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::cell::{Cell, RefCell};
    use std::time::Duration;
    use std::thread::JoinHandle;

    /*
        Most basic usage of results from threads
     */
    #[test]
    fn test_fork_join_basics() {
        let handle = thread::spawn(||{
            7+11
        });
        let result = handle.join().unwrap();
        assert_eq!(result,18);
    }

    /*
        Let the Threads print their IDs, some prints will be interleaved
     */
    #[test]
    fn test_interleaving_print() {

        let mut handles = Vec::new();

        for i in 0..4 {
            handles.push(thread::spawn(move || {
                for _ in 0..10{
                    println!(" Thread {} hier!",i);
                }
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("Threads done");
    }


    #[test]
    fn test_channel_basics_unidirectional(){
        let (task_tx, task_rx) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            println!("Thread läuft");
            let input:String = task_rx.recv().expect("Receive failed");
            println!("Input: {}",input.to_uppercase());
        });
        thread::sleep(time::Duration::from_millis(10));
        println!("Erzeuge Eingabe");
        let work = String::from("Mietzekatze");
        task_tx.send(work).expect("Could not submit work");
        handle.join();
    }


    #[test]
    fn test_channel_basics_bidirectional(){
        let (task_tx, task_rx) = std::sync::mpsc::channel();
        let (result_tx, result_rx) = std::sync::mpsc::channel();

        thread::spawn(move || task_rx.into_iter().for_each(|mut input :String|{
            let upper: String = input.to_uppercase();
            std::mem::replace(&mut input,upper);
            result_tx.send(input).unwrap();
        }));
        let work = String::from("Mietzekatze");
        task_tx.send(work).expect("Could not submit work");
        let result = result_rx.recv().expect("Could not receive result");
        println!("Result {}",result);
    }


    /*
        Which thread gets to live the longest?
     */
    // #[test]
    // fn test_par_compute_with_reference() {
    //     let vec = vec![0u32; 4096];
    //     let mut handles = Vec::new();
    //
    //     for i in 0..16 {
    //         let data = &vec;
    //         handles.push(thread::spawn(move || {
    //             println!("[{:2}]: {}", i, data[i]); // simulate processing
    //         }));
    //     }
    //     handles.into_iter().for_each(|h|h.join().unwrap());
    //     println!("{}", vec[0]);
    // }


    /*
        Showcases how to access data behind an Arc
     */
    #[test]
    fn test_par_compute_with_arc_print() {
        let vec = vec![0u32; 4096];
        let data = Arc::new(vec);
        let mut handles = Vec::new();

        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                println!("[{:2}]: {}", i, data[i]); // simulate processing
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap())
    }

    /*
        Showcases how to access data behind an Arc, compute something and collect results
     */
    #[test]
    fn test_par_compute_with_arc_result() {
        let part = vec![0,1,2,3,4,5,6,7,8,9];
        let parts =
            vec![part;16]
                .into_iter()
                .flatten()
                .collect::<Vec<u32>>();
        let data = Arc::new(parts);
        //let data = // generate Arc<Vec<u32>>Arc::new(parts);

        let mut handles = Vec::new();

        // spawn 16 threads to calculate the sum of each section in parallel
        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                let sum =
                    data[i*10..(i+1)*10]
                        .iter()
                        .sum::<u32>();
                sum
            }));
        }
        let total = handles
            .into_iter()
            .filter_map(|h|h.join().ok())
            .sum::<u32>();
        println!("Result is {}",total);
        assert_eq!(total,16*((10*9)/2)); // verify result, attention with integer division
    }

    /*
        A more concise version of the function above
     */
    type H = Vec<JoinHandle<u32>>;
    #[test]
    fn test_par_compute_with_arc_result_concise() {
        let data = Arc::new(vec![vec![0,1,2,3,4,5,6,7,8,9];16]
            .into_iter().flatten().collect::<Vec<u32>>());
        let total = (0..16)
            .map(|i|(i,data.clone()))
            .map(|(i,data)|  thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                data[i*10..(i+1)*10].iter().sum::<u32>()
            }))
            .filter_map(|h|h.join().ok()).sum::<u32>();
        println!("Result is {}",total);
        assert_eq!(total,16*((10*9)/2)); // verify result, attention with integer division
    }

    #[test]
    fn test_par_compute_with_arc_result_concise_correct() {
        let data = Arc::new(vec![vec![0,1,2,3,4,5,6,7,8,9];16]
            .into_iter().flatten().collect::<Vec<u32>>());
        let total = (0..16)
            .map(|i|(i,data.clone()))
            .map(|(i,data)|  thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                data[i*10..(i+1)*10].iter().sum::<u32>()
            })).collect::<H>().into_iter()
            .filter_map(|h|h.join().ok()).sum::<u32>();
        println!("Result is {}",total);
        assert_eq!(total,16*((10*9)/2)); // verify result, attention with integer division
    }

    #[test]
    fn test_par_compute_with_rayon() {
        use rayon::prelude::*;
        let data = vec![vec![0,1,2,3,4,5,6,7,8,9];16]
            .into_iter().flatten().collect::<Vec<u32>>();
let total = (0..16)
    .into_par_iter()
    .map(|i|  data[i*10..(i+1)*10].iter().sum::<u32>())
    .sum::<u32>();
        println!("Result is {}",total);
        assert_eq!(total,16*((10*9)/2)); // verify result, attention with integer division
    }







    /*
        Showcases the usage of atomics with an Arc
        Not that we do not need a mutex here, as Atomic Types are safely shareable across threads
     */
    #[test]
    fn test_atomic_arc() {
        let ctr = AtomicU64::new(0);
        let data = Arc::new(ctr);

        let mut handles = Vec::new();

        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                data.fetch_add(1, Ordering::SeqCst);
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("Result is {}",data.load(Ordering::SeqCst));
    }

    /*
        Shows that we cannot simply use RefCell here
     */
    #[test]
    fn test_arc_illegal_cell_usage() {
        let ctr = RefCell::new(0);
        let data = Arc::new(ctr);

        let mut handles = Vec::new();

        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                // illegal error[E0277]: `std::cell::RefCell<i32>` cannot be shared between threads safely
                //*data.borrow_mut() += 1;
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        println!("Result is {}",data.borrow());
    }

    /*
        Simple example for the usage of a Mutex
     */
    #[test]
    fn test_mutex() {
        let data = Arc::new(Mutex::new(8));
        let mut handles = Vec::new();

        for i in 0..16 {
            let data = data.clone();
            handles.push(thread::spawn(move || {
                *data.lock().expect("poisioned")+=1;
            }));
        }
        handles.into_iter().for_each(|h|h.join().unwrap());
        let result = {
            data.lock().unwrap().clone()
        };
        println!("Result: {}",result);
        assert_eq!(result,8+16)
    }






    /*
        A few more samples not in the book
     */
    #[test]
    fn test_spawn_basics() {

        let number = 4;
        thread::spawn(move || println!("number:{}",number));

        let bx = Box::new(7);
        thread::spawn(move || println!("bx:{}",bx));

        let vec = vec![1,2,3];
        thread::spawn(move || println!("vec:{:?}",vec));

        // target needs to have a static lifetime
        // bc current method might live shorter than the thread
        const target:u32 = 8;
        //let target = 7;
        let rf = &target;
        thread::spawn( move || println!("rf:{:?}",rf));


        thread::sleep(Duration::from_secs(1));
        println!("ok");

    }
}