
#[cfg(test)]
mod async_basics{
    use tokio::{task, io};
    use std::{thread, time};
    use tokio::runtime::Runtime;
    use std::thread::sleep as std_sleep;
    use std::time::Duration;
    use tokio::runtime::Builder;
    use tokio::time::sleep;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::error::Error;
    /*
        Without polling for the result of the future, it will never complete
        Hence, "B" will not be printed
     */
    #[tokio::test]
    async fn basics_task_needs_polling(){
        println!("A");
        task::spawn(async {println!("B");}); // never appears
        thread::sleep(time::Duration::from_secs(1));
        println!("C");
    }

    /*
        Polling (and discarding the result) will lead to the execution of the future
        We see A,B,C
     */
    #[tokio::test]
    async fn basics_task_is_polled(){
        println!("A");
        let t = task::spawn(async {println!("B");});
        thread::sleep(time::Duration::from_secs(1));
        let _ = t.await;
        println!("C");
    }

    /*
        Important: This function itself is not async
     */
    #[test]
    fn schedule_from_outside(){
        let my_task = async {
            println!("hello ");
            task::spawn(async {println!("world");});
            println!("!");
        };
        let mut rt = Runtime::new().unwrap();
        rt.spawn(my_task);
        // a normal application would either run forever or wait until the runtime becomes idle
        thread::sleep(time::Duration::from_secs(1));
    }

    /*

     */
    #[test]
    fn sample_scenario(){
        let rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();

        rt.spawn(async {
            for i in {0 .. 10}{
                tokio::spawn(handle_workload(i));
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        });
        thread::sleep(time::Duration::from_secs(6));
    }

    async fn handle_workload(wl: u32){
        // println!("tid: {:?}",thread::current().id());
        match wl%2 {
            0 => long_task().await,
            _ => short_task().await
        }
    }

    async fn short_task(){
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("Short done")
    }
    async fn long_task(){
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Long done")
    }



    // #[tokio::test]
    // async fn echo() -> Result<(), Box<dyn Error>>  {
    //     let listener = TcpListener::bind(&"127.0.0.1:8080").await?;
    //     loop {
    //         let (mut socket, _) = listener.accept().await?;
    //         tokio::spawn(async move {
    //             let mut buf = vec![0; 1024];
    //             loop {
    //                 let n = socket.read(&mut buf).await?;
    //                 if n == 0 { break; }
    //
    //                 socket.write_all(&buf[0..n]).await?;
    //             }
    //             io::Result::Ok(0)
    //         });
    //     }
    // }
}