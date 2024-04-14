use serenity::{client::Context, model::channel::Message};
use std::{env, path::Path};
use tokio::{
    fs::{create_dir, File},
    io::AsyncWriteExt,
};

use crate::Handler;

impl Handler {
    pub async fn on_message(&self, _ctx: Context, msg: Message) {
        if msg.attachments.len() == 0 {
            return;
        }

        let base_dir = env::var("BASE_DIR").unwrap();
        let dir = Path::new(&base_dir)
            .join("data")
            .join(msg.id.get().to_string());

        if let Err(why) = create_dir(&dir).await {
            println!("Error creating dir for message: {why:?}");
            return;
        };

        for attachment in msg.attachments.iter() {
            let content = match attachment.download().await {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment content: {why:?}");
                    continue;
                }
            };

            let mut file = match File::create(&dir.join(&attachment.filename)).await {
                Ok(file) => file,
                Err(why) => {
                    println!("Error opening file for writing: {why:?}");
                    continue;
                }
            };

            if let Err(why) = file.write_all(&content).await {
                println!("Error writing file contents: {why:?}");
            }
        }
    }
}
