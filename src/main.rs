use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 6543)]
    port: u16,

    /// Upstream database host
    #[arg(long, default_value = "127.0.0.1")]
    upstream_host: String,

    /// Upstream database port
    #[arg(long, default_value_t = 5432)]
    upstream_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args = Args::parse();

    info!("Starting DB Proxy on port {}", args.port);
    info!("Forwarding to upstream at {}:{}", args.upstream_host, args.upstream_port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;

    loop {
        let (client_socket, client_addr) = listener.accept().await?;
        info!("Accepted connection from {}", client_addr);

        let upstream_host = args.upstream_host.clone();
        let upstream_port = args.upstream_port;

        tokio::spawn(async move {
            if let Err(e) = process_connection(client_socket, upstream_host, upstream_port).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }
}

async fn process_connection(mut client_socket: tokio::net::TcpStream, upstream_host: String, upstream_port: u16) -> Result<()> {
    let mut upstream_socket = tokio::net::TcpStream::connect(format!("{}:{}", upstream_host, upstream_port)).await?;
    
    let (mut client_read, mut client_write) = client_socket.split();
    let (mut upstream_read, mut upstream_write) = upstream_socket.split();

    // Simple blind forwarding for now
    let client_to_upstream = tokio::io::copy(&mut client_read, &mut upstream_write);
    let upstream_to_client = tokio::io::copy(&mut upstream_read, &mut client_write);

    tokio::select! {
        res = client_to_upstream => {
            info!("Client disconnected: {:?}", res);
        }
        res = upstream_to_client => {
            info!("Upstream disconnected: {:?}", res);
        }
    }

    Ok(())
}
