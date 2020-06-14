use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use crate::{CommandWriter, parse_io, Result, ServerResult};

pub struct RedisClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl RedisClient {
    pub fn connect(sock_addr: &str) -> Result<RedisClient> {
        println!("connect -> {}", sock_addr);
        let tcp_reader = TcpStream::connect(sock_addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(RedisClient {
            reader: BufReader::new(tcp_reader),
            writer: BufWriter::new(tcp_writer),
        })
    }

    pub fn del(&mut self, key: &str) -> Result<String> {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(2).write_bulk_string("DEL").write_bulk_string(key);
        self.writer.write(cmd.buf.as_bytes())?;
        self.writer.flush().unwrap();
        let mut buffer = [0; 512];
        self.reader.read(&mut buffer).unwrap();
        let response = from_utf8(&buffer).unwrap();
        let parse = parse_io(response).unwrap();
        match parse {
            ServerResult::RString(msg, _) => Ok(msg),
            _ => panic!("error")
        }
    }

    pub fn set(&mut self, key: &str, val: &str) -> Result<String> {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(3).write_bulk_string("SET").write_bulk_string(key).write_bulk_string(val);
        self.writer.write_all(cmd.buf.as_bytes())?;
        self.writer.flush().unwrap();
        let mut buffer = [0; 512];
        self.reader.read(&mut buffer).unwrap();
        let response = from_utf8(&buffer).unwrap();
        let parse = parse_io(response).unwrap();
        match parse {
            ServerResult::RString(msg, _) => Ok(msg),
            _ => panic!("error")
        }
    }

    pub fn get(&mut self, key: &str) -> Result<String> {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(2).write_bulk_string("GET").write_bulk_string(key);
        self.writer.write_all(cmd.buf.as_bytes())?;
        self.writer.flush().unwrap();
        let mut buffer = [0; 512];
        self.reader.read(&mut buffer).unwrap();
        let response = from_utf8(&buffer).unwrap();
        let parse = parse_io(response).unwrap();
        match parse {
            ServerResult::RString(_, val) => Ok(val),
            _ => panic!("error")
        }
    }

    pub fn auth(&mut self, password: &str) -> Result<String> {
        let mut cmd = CommandWriter::new();
        cmd.write_arrs(2).write_bulk_string("AUTH").write_bulk_string(password);
        self.writer.write_all(cmd.buf.as_bytes())?;
        self.writer.flush().unwrap();
        let mut buffer = [0; 512];
        self.reader.read(&mut buffer[..]).unwrap();
        let response = from_utf8(&buffer).unwrap();
        let parse = parse_io(response).unwrap();
        match parse {
            ServerResult::RString(msg, _) => Ok(msg),
            _ => panic!("error")
        }
    }
}