pub enum ServerResult {
    RString(String, String),
    RArr(Vec<String>),
}

pub fn parse_io(response: &str) -> Option<ServerResult> {
    println!("{}", response);
    let vec: Vec<&str> = response.split("\r\n").collect();
    let msg = vec[0][1..].to_string();
    match &vec[0][0..1] {
        "$" => Some(ServerResult::RString(msg, vec[1].to_string())),
        "*" => {
            let len = vec[0][1..].parse::<usize>().unwrap();
            let mut v: Vec<String> = Vec::new();
            for i in 0..len {
                v.push(vec[i + 1].to_string());
            }
            Some(ServerResult::RArr(v))
        }
        "+" => Some(ServerResult::RString(msg, vec[1].to_string())),
        "-" => Some(ServerResult::RString(format!("(error) {}", msg), "".to_string())),
        ":" => Some(ServerResult::RString(msg, "".to_string())),
        _ => None,
    }
}