use websocket_manager::WebSocketManager;

fn main() {
    let manager = WebSocketManager::new();
    manager.start(8042);
}
