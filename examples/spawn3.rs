// 1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、 出力するプログラム

use std::sync::mpsc;
use std::thread;

fn main() {
  // tx: 転送
  // rx: 受信
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}
