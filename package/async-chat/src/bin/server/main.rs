mod connection;
mod group;
mod group_table;

use async_chat::utils::ChatResult;
use async_std::prelude::*;
use connection::serve;
use std::sync::Arc;

fn log_error(result: ChatResult<()>) {
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: Server ADDRESS");

    let chat_group_table = Arc::new(group_table::GroupTable::new());

    async_std::task::block_on(async {
        use async_std::{net, task};

        let listener = net::TcpListener::bind(address).await?;

        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async move {
                log_error(serve(socket, groups).await);
            });
        }

        Ok(())
    })
}
