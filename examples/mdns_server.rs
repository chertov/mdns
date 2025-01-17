use webrtc_mdns as mdns;

use mdns::{config::*, conn::*};

use clap::{App, AppSettings, Arg};
use std::net::SocketAddr;
use std::str::FromStr;
use util::Error;

// For interop with webrtc-rs/mdns_server
// cargo run --color=always --package webrtc-mdns --example mdns_server

// For interop with pion/mdns_client:
// cargo run --color=always --package webrtc-mdns --example mdns_server -- --local-name pion-test.local

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut app = App::new("mDNS Sever")
        .version("0.1.0")
        .author("Rain Liu <yuliu@webrtc.rs>")
        .about("An example of mDNS Sever")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("FULLHELP")
                .help("Prints more detailed help information")
                .long("fullhelp"),
        )
        .arg(
            Arg::with_name("server")
                .required_unless("FULLHELP")
                .takes_value(true)
                .default_value("0.0.0.0:5353")
                .long("server")
                .help("mDNS Server name."),
        )
        .arg(
            Arg::with_name("local-name")
                .long("local-name")
                .takes_value(true)
                .default_value("webrtc-rs-test.local")
                .help("Local name"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    let server = matches.value_of("server").unwrap();
    let local_name = matches.value_of("local-name").unwrap();

    let server = DnsConn::server(
        SocketAddr::from_str(server)?,
        Config {
            local_names: vec![local_name.to_owned()],
            ..Default::default()
        },
    )
    .unwrap();

    println!("Press ctlr-c to stop server");
    tokio::signal::ctrl_c().await.unwrap();
    server.close().await.unwrap();
    Ok(())
}
