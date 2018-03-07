extern crate tokio;
extern crate tokio_io;
extern crate futures;

use tokio::executor::current_thread;
use tokio::net::TcpListener;
use tokio_io::io;
use futures::{Future, Stream};

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "hello world\n")
            .then(|res| {
                println!("wrote message; success={:?}", res.is_ok());
                Ok(())
            });

        current_thread::spawn(connection);

        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });

    current_thread::run(|_| {
        current_thread::spawn(server);

        println!("Server running on localhost:6142");
    })
}
