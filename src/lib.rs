use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::Message;

pub struct WebSocketManager {
  servers: Arc<RwLock<HashMap<u16, Arc<Mutex<TcpListener>>>>>,
  // has_listeners: bool,
}

impl WebSocketManager {
  pub fn new() -> WebSocketManager {
    WebSocketManager {
      servers: Arc::new(RwLock::new(HashMap::new())),
      // has_listeners: false,
    }
  }

  pub fn start(&self, port: u16) {
    println!("starting on port {}", port);
    let servers = self.servers.read().expect("Data is fucked");

    if servers.get(&port).is_some() {
      println!("Already running a server on {}", port);
      return;
    }

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let server = Arc::new(Mutex::new(TcpListener::bind(addr).unwrap()));

    // servers.insert(port, server);

    for stream in server.lock().unwrap().incoming() {
      spawn(move || {
        let callback = |req: &Request, mut response: Response| {
          println!("Received a new ws handshake");
          println!("The request's path is: {}", req.uri().path());
          println!("The request's headers are:");
          for (ref header, _value) in req.headers() {
            println!("* {}", header);
          }

          // Let's add an additional header to our response to the client.
          let headers = response.headers_mut();
          headers.append("MyCustomHeader", ":)".parse().unwrap());
          headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

          Ok(response)
        };

        let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

        loop {
          let msg = websocket.read_message().unwrap();
          if msg.is_binary() || msg.is_text() {
            websocket
              .write_message(Message::text(format!("fuck you! {}", msg)))
              .unwrap();
          }
        }
      });
    }
  }
}

impl Default for WebSocketManager {
  fn default() -> Self {
    WebSocketManager::new()
  }
}
