use std::thread;

fn main(){
    //创建一个新的线程
    let handle = thread::spawn(||{
        //在新线程中执行的代码
        for i in i..=5 {
            println!("Thread :Count {}",i);
        }
    });
    //主线程继续执行的代码
    for i in i..=3 {
        println!("To do {i}");
    }
    //等待线程执行完成
    handle.join().unwrap();
}