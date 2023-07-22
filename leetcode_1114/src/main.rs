use std::sync::Arc; 
use tokio::sync::mpsc; 
use tokio::task;

use leetcode_1114::Foo; 

#[tokio::main]
async fn main() {
    let foo = Arc::new(Foo); 

    let (tx1, mut rx1) = mpsc::channel::<()>(1); 
    let (tx2, mut rx2) = mpsc::channel::<()>(1); 

    let foo1 = foo.clone(); 
    let foo2 = foo.clone(); 
    let foo3 = foo.clone(); 

    let task1 = task::spawn(async move {
        foo1.first().await;
        tx1.send(()).await.expect("failed to send to tx1");  
    }); 

    let task2 = task::spawn(async move {
        rx1.recv().await.expect("failed to receive from rx1"); 
        foo2.second().await; 
        tx2.send(()).await.expect("failed to send to tx2"); 
    }); 

    let task3 = task::spawn(async move {
        rx2.recv().await.expect("failed to receive from rx2"); 
        foo3.third().await; 
    }); 

    task1.await.expect("failed to await task1"); 
    task2.await.expect("failed to await task2");
    task3.await.expect("failed to await task3");
}