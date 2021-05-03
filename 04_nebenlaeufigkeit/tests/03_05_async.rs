
#[cfg(test)]
mod async_basics{
    use tokio::task;
    use std::{thread, time};

    #[tokio::test]
    async fn basics_task_needs_polling(){
        println!("A");
        task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_millis(2000));
        println!("C");
    }

    #[tokio::test]
    async fn basics_task_is_polled(){
        println!("A");
        let t = task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_millis(2000));
        _ = t.await;
        println!("C");
    }
}