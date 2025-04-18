/// NOTE:
/// ```bash
/// $ cargo build
/// $ ./maelstrom test -w unique-ids --bin ./target/debug/unique_ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-stderr
/// ````
use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde_json::Map;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "generate" {
            let mut generate = req.body.clone().with_type("generate_ok");

            let mut extra = Map::new();

            extra.insert(
                "id".to_string(),
                serde_json::Value::String(Uuid::new_v4().to_string()),
            );

            generate.extra = extra;

            return runtime.reply(req, generate).await;
        }

        done(runtime, req)
    }
}
