use redis::{Client, Connection, Commands, RedisResult};


pub struct QARedis {
    pub client: Client,
    pub conn: Connection,
}

impl QARedis {
    pub fn new(uri: &str) -> Self {
        let client = Client::open(uri).unwrap();
        let conn = client.get_connection().unwrap();
        Self {
            client,
            conn,
        }
    }

    pub fn get(&mut self, key: &str){
        println!("{:?}",self.conn.get(key));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis() {
        let mut rd = QARedis::new("redis://127.0.0.1/");
        println!("{:#?}", rd.get("name"));
    }
}