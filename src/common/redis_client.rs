// redis_client.rs
use std::{sync::{Mutex, Once}, fmt::{Debug, self}};

use redis::{Client, Commands, Connection, RedisResult};

static mut REDIS_CLIENT: Option<Mutex<RedisClient>> = None;
static INIT: Once = Once::new();

pub struct RedisClient {
    connection: Connection,
}
// impl Debug for RedisClient {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("RedisClient")
//             .field("connection", &self.connection)
//             .finish()
//     }
// }

impl RedisClient {
    fn new() -> RedisResult<Self> {
        let client = Client::open("redis://127.0.0.1/")?;
        let connection = client.get_connection()?;
        Ok(Self { connection })
    }

    pub fn get_instance() -> &'static Mutex<RedisClient> {
        unsafe {
            INIT.call_once(|| {
                REDIS_CLIENT = Some(Mutex::new(RedisClient::new().unwrap()));
            });

            REDIS_CLIENT
                .as_ref()
                .expect("Failed to initialize Redis client")
        }
    }

    pub fn set_data(&mut self, key: &str, value: i32) -> RedisResult<()> {
        self.connection.set(key, value)?;
        Ok(())
    }

    pub fn get_data(&mut self, key: &str) -> RedisResult<i32> {
        let data: i32 = self.connection.get(key)?;
        Ok(data)
    }

    pub fn delete_data(&mut self, key: &str) -> RedisResult<()> {
        self.connection.del(key)?;
        Ok(())
    }
}
