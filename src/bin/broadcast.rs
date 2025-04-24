/// NOTE:
/// ```bash
/// $ cargo build
/// $ ./maelstrom test -w broadcast --bin ./target/debug/broadcast --node-count 1 --time-limit 10 --log-stderr
/// ````
use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde_json::{json, Map};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {
    seen: Arc<Mutex<HashSet<i64>>>,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {
        match request.get_type() {
            "broadcast" => {
                if let Some(message) = request.body.extra.get("message") {
                    let s = self.seen.clone();

                    let mut value = s.lock().unwrap();

                    value.insert(message.as_i64().unwrap());
                }

                return runtime.reply_ok(request).await;
            }
            "read" => {
                let mut response = request.body.clone().with_type("read_ok");

                let mut extra = Map::new();
                let s = self.seen.clone();

                let messages = serde_json::Value::Array(
                    s.lock()
                        .unwrap()
                        .iter()
                        .copied()
                        .map(|v| json!(v))
                        .collect(),
                );

                extra.insert("messages".to_string(), messages);

                response.extra = extra;

                return runtime.reply(request, response).await;
            }
            "topology" => runtime.reply_ok(request).await,
            _ => done(runtime, request),
        }
    }
}
