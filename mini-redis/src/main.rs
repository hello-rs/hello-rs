use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// 声明类型 Db = 线程内可安全访问的 hashmap

#[tokio::main]
async fn main() {
    // 创建一个 tcp监听该地址
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    loop {
        // 等待连接,返回一个包含流和远程地址的元组
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Accepted addr: {:?}", addr);
        // 为每个连接创建一个新的任务,socket 移动至新任务中处理。
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

// 处理tcp连接
async fn process(socket: TcpStream) {
    // hashmap 存储数据
    let mut db = HashMap::new();
    // 通过 Connection 将字节流转换为 redis 读/写消息帧
    let mut connection = Connection::new(socket);
    // 循环读取消息
    while let Some(req) = connection.read_frame().await.unwrap() {
        // 将消息解析为命令并处理
        let resp = match Command::from_frame(req).unwrap() {
            Command::Set(cmd) => {
                // value 转换为 `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // 写回消息
        connection.write_frame(&resp).await.unwrap();
    }
}
