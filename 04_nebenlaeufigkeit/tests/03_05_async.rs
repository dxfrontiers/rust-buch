
#[cfg(test)]
mod async_basics{
    use tokio::task;
    use std::{thread, time};

    /*
        Without polling for the result of the future, it will never complete
        Hence, "B" will not be printed
     */
    #[tokio::test]
    async fn basics_task_needs_polling(){
        println!("A");
        task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_millis(2000));
        println!("C");
    }

    /*
        Polling (and discarding the result) will lead to the execution of the future
     */
    #[tokio::test]
    async fn basics_task_is_polled(){
        println!("A");
        let t = task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_millis(2000));
        _ = t.await;
        println!("C");
    }

    // TODO: tun
}