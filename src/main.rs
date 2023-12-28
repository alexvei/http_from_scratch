/* simple HTTP server */
/* author: Giovanni */

/* reasoning: understanding HTTP better */

use std::net::TcpListener;

fn main() {
    /* creating a local tcplistener at port 8477 */
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    /* concat host address and port for final end point*/
    let end_point: String = HOST.to_owned() + ":" + PORT;

    /* create a tcp listener at our end point */
    let listener = TcpListener::bind(end_point).unwrap();

    /* let developer know that the web server is listening */
    println!("web server is listening at port {}", PORT);

    /* communicate with any incoming connections */

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection established!")
    }
}
