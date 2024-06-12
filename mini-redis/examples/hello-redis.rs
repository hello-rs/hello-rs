use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个 mini-redis 连接, 连接到本地的 Redis 服务
    let mut client = client::connect("127.0.0.1:6379").await?;
    // 设置 key "hello" 的值为 "world"
    client.set("hello", "world".into()).await?;
    // 获取 key "hello" 的值
    let val = client.get("hello").await?;
    // 打印获取的值
    println!("got value from the server; result={:?}", val);
    Ok(())
}
