use std::io::{stdin, stdout, Write};
use std::process::exit;

use structopt::StructOpt;

use simple_redis_cli::{RedisClient, Result};

#[derive(StructOpt, Debug)]
#[structopt(name = "simple-redis-cli", about = "An simple redis client")]
struct Opt {
    #[structopt(short = "h", long = "host", default_value = "127.0.0.1")]
    host: String,
    #[structopt(short = "p", long = "port", default_value = "6379")]
    port: u32,
    #[structopt(short = "a", long = "auth", default_value = "")]
    auth: String,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let mut client = RedisClient::connect(&format!("{}:{}", opt.host, opt.port))?;
    loop {
        print!("{}:{} > ", opt.host, opt.port);
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_ascii_whitespace();
        let command = parts.next().unwrap();
        let args = parts.collect::<Vec<&str>>();
        match command.to_uppercase().as_str() {
            "AUTH" => {
                if args.len().ne(&1usize) {
                    println!("(error) ERR wrong number of arguments for 'auth' command");
                    continue;
                }
                let pass = args.get(0).clone().unwrap();
                let msg = client.auth(pass).unwrap();
                println!("{}", msg);
            }
            "GET" => {
                if args.len().ne(&1usize) {
                    println!("(error) ERR wrong number of arguments for 'get' command");
                    continue;
                }
                let key = args.get(0).clone().unwrap();
                let val = client.get(key).unwrap();
                println!("{}", val);
            }
            "SET" => {
                if args.len().ne(&2usize) {
                    println!("(error) ERR wrong number of arguments for 'set' command");
                    continue;
                }
                let key = args.get(0).clone().unwrap();
                let val = args.get(1).clone().unwrap();
                let msg = client.set(key, val).unwrap();
                println!("{}", msg);
            }
            "DEL" => {
                if args.is_empty() {
                    println!("(error) ERR wrong number of arguments for 'del' command");
                    continue;
                }
                let key = args.get(0).clone().unwrap();
                let msg = client.del(key).unwrap();
                println!("{}", msg);
            }
            "EXIT" | "Q" => {
                return Ok(());
            }
            _ => {
                println!("(error) ERR invalid command");
            }
        }
    }
}