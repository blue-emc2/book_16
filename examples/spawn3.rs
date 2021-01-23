// 1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、 出力するプログラム

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // tx: 転送
  // rx: 受信
  let (tx, rx) = mpsc::channel();

  println!("spawn begin");

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      println!("send begin");
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  println!("received begin");
  for received in rx {
    println!("Got: {}", received);
  }
  println!("received end");
}
