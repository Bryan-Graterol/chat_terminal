// std::io::BufReader::<R>::new

use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // Usamos broatcast channel para que nos conectemos con multiples chats
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // Separamos el socket en reader y writer
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // Creamos un loop para que la conexion no se caiga
            loop {
                tokio::select! {
                    _ = reader.read_line(&mut line)=>{
                        tx.send((line.clone(),addr)).unwrap();
                        line.clear();
                    }
                    //enviamos el msj (broatcast)
                    result = rx.recv()=>{
                        let (msj, msj_addr) = result.unwrap();
                        
                        if addr != msj_addr{
                            writer.write_all(msj.as_bytes()).await.unwrap();
                        }
                        
                    }

                }
            }
        });
    }
}
