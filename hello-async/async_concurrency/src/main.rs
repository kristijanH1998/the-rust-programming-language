use std::time::Duration;
use trpl::Either;

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    // trpl::block_on(async {
    //     let fut1 = async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     trpl::join(fut1, fut2).await;
    // });

    //-------------------------
    // let a = async {
    //     println!("'a' started.");
    //     slow("a", 30);
    //     trpl::yield_now().await;
    //     slow("a", 10);
    //     trpl::yield_now().await;
    //     slow("a", 20);
    //     trpl::yield_now().await;
    //     println!("'a' finished.");
    // };

    // let b = async {
    //     println!("'b' started.");
    //     slow("b", 75);
    //     trpl::yield_now().await;
    //     slow("b", 10);
    //     trpl::yield_now().await;
    //     slow("b", 15);
    //     trpl::yield_now().await;
    //     slow("b", 350);
    //     trpl::yield_now().await;
    //     println!("'b' finished.");
    // };

    // ---------------------

    let slow = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "Finally finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }

}

