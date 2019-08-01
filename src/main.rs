use dns_lookup::{ getaddrinfo, SockType, AddrInfoHints, AddrInfo, LookupErrorKind };
use std::io::{ ErrorKind };
use std::net::{ TcpStream };
use std::time::{ Duration, SystemTime };
use std::process;
use clap::{ Arg, App };

fn main() {

    let matches = App::new("tcping")
        .version("0.1.0")
        .author("guoliim")
        .about("test tcp ping")
        .arg(Arg::with_name("host")
                    .short("h")
                    .long("host")
                    .help("host of server")
                    .takes_value(true))
        .arg(Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .alias("service")
                    .help("service of server")
                    .takes_value(true))
        .arg(Arg::with_name("count")
                    .short("c")
                    .long("count")
                    .help("counts of tcping")
                    .takes_value(true))
        .get_matches();

    let host = match matches.value_of("host") {
        Some(host) => host,
        None => {
            println!("cant't get server host");
            process::exit(1);
        },
    };

    let port = match matches.value_of("port") {
        Some(port) => port,
        None => "80",
    };

    let cnt = match matches.value_of("count") {
        Some(count) => count.parse().unwrap(),
        None => 8,
    };

    let sockets = match getaddrinfo(
        Some(host),
        Some(port),
        Some(AddrInfoHints {
            socktype: SockType::Stream.into(),
            .. AddrInfoHints::default()
        }),
    ) {
        Ok(sockets) => (
            sockets
                .collect::<std::io::Result<Vec<_>>>()
                .unwrap()
        ),
        Err(err) => match err.kind() {
            LookupErrorKind::Again => {
                println!("Temporary failure in name resolution");
                process::exit(1)
            },
            LookupErrorKind::NoName => {
                println!("NAME or SERVICE is unknown");
                process::exit(1)
            },
            LookupErrorKind::Socktype => {
                println!("SocketType not support");
                process::exit(1)
            },
            LookupErrorKind::Service => {
                println!("Service not support");
                process::exit(1)
            },
            _ => {
                println!("There are some err");
                process::exit(1)
            }
        },
    };

    for socket in sockets {
        for _ in 0..cnt {
            handle_tcping(&socket)
        }
    }
}

fn handle_tcping (socket: &AddrInfo) {

    let AddrInfo { sockaddr, .. } = socket;

    let sys_time = SystemTime::now();

    let _stream =
        match TcpStream::connect_timeout(sockaddr, Duration::from_millis(2000)) {
            Ok(stream) => {

                let duration =
                    SystemTime::now()
                        .duration_since(sys_time)
                        .unwrap()
                        .as_millis();

                println!("{} connected to {} {}ms", stream.local_addr().unwrap(), sockaddr, duration);

                Ok(stream)
            },
            Err(err) => match err.kind() {
                ErrorKind::TimedOut => {
                    println!("connected to {} timeout", sockaddr);
                    Err(err)
                },
                _ => {
                    println!("connected to {} failed", sockaddr);
                    Err(err)
                },
            },
        };
}
