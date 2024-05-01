use std::net::IpAddr;
use tokio::net::{TcpStream};
use tokio::sync::mpsc::{self};
use tokio::runtime::Runtime;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// IP address of the port scan.
    #[arg(conflicts_with("cidr"), required_unless_present("cidr"))]
    addr: Option<IpAddr>,

    #[arg(long)]
    cidr: Option<cidr::IpCidr>,

    /// Start of the range.
    #[arg(short = 's', long, default_value_t = 1)]
    port_start: u16,

    /// End of the range of ports to scan (inclusive).
    #[arg(short = 'e', long, default_value_t = 1024)]
    port_end: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    assert!(args.port_start != 0);
    assert!(args.port_start <= args.port_end);

    let rt = Runtime::new()?;

    let (tx, mut rx) = mpsc::channel(10);

    rt.block_on(async {
        let n_tasks_per_network = (args.port_end - args.port_start) as usize;
        let mut tasks: Vec<_> = Vec::with_capacity(n_tasks_per_network);

        let (mut from_single, mut from_cidr);

        let addrs: &mut dyn Iterator<Item = IpAddr> = if let Some(addr) = args.addr {
            from_single = vec![addr].into_iter();
            &mut from_single
        } else if let Some(network) = args.cidr {
            from_cidr = network.iter().map(|net| net.address());
            &mut from_cidr
        } else {
            unreachable!()
        };

        for addr in addrs {
            println!("? {addr}:{}-{}", args.port_start, args.port_end);
            for port in args.port_start..=args.port_end {
                let tx = tx.clone();
                let task = tokio::spawn(async move {
                    if let Err(err) = scan(addr, port, tx).await {
                        eprintln!("error: {err}")
                    };
                });

                tasks.push(task);
            }
        }

        for task in tasks {
            task.await.unwrap();
        }
    });

    drop(tx);

    while let Ok((addr, port)) = rx.try_recv() {
        println!("= {addr}:{port}")
    }

    Ok(())
}

async fn scan(addr: IpAddr, port: u16, results_tx: mpsc::Sender<(IpAddr, u16)>) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    if let Ok(_ping) = TcpStream::connect((addr, port)).await {
        results_tx.send((addr, port)).await?;
    }

    Ok(())
}

