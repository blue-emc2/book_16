 // 1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、 出力するプログラム

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  // tx: 転送
  // rx: 受信
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);

  println!("spawn begin 1");

  thread::spawn(move || {
    println!("spawned thread begin 1");
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      println!("send begin 1");
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  println!("spawn begin 2");

  thread::spawn(move || {
    println!("spawned thread begin 2");
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      println!("send begin 2");
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
