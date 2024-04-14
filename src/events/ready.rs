use serenity::{client::Context, model::gateway::Ready};

use crate::Handler;

impl Handler {
    pub async fn on_ready(&self, _ctx: Context, ready: Ready) {
        let id = ready.application.id.get();
        let name = &ready.user.name;
        println!("Connected to user {name} [id:{id}]");
    }
}
