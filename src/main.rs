mod commands;

use {
    serenity::{
        async_trait,
        client::{Context, EventHandler},
        framework::standard::{
            macros::group,
            StandardFramework,
        },
        model::{
            channel::Message,
            gateway::Ready,
            id::{ChannelId, GuildId, RoleId},
        },
        prelude::TypeMapKey,
        Client,
    },
    std::{collections::HashSet, sync::Arc},
    tokio::sync::Mutex,
    commands::*,
};

#[group]
#[commands(setrole, getrole)]
struct General;

struct Handler {
    autodelete_roles: Arc<Mutex<HashSet<RoleId>>>,
    counter: Arc<Mutex<u32>>,
}

impl Handler {
    fn new(autodelete_roles: Arc<Mutex<HashSet<RoleId>>>, counter: Arc<Mutex<u32>>) -> Self {
        Self {
            autodelete_roles,
            counter,
        }
    }

    async fn send_whats_up_message(&self, ctx: &Context) {
        let channel_id = ChannelId(917057579039989773);

        if let Err(why) = channel_id.say(&ctx.http, "What's up ya'll? :D").await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}

impl TypeMapKey for Handler {
    type Value = Arc<Mutex<HashSet<RoleId>>>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let autodelete_roles = self.autodelete_roles.clone();
        let counter = self.counter.clone();

        tokio::spawn(async move {
            if msg.author.bot {
                return;
            }

            let autodelete_roles = autodelete_roles.lock().await;

            for role_id in &*autodelete_roles {
                if msg
                    .author
                    .has_role(&ctx, GuildId(917057579039989770), *role_id)
                    .await
                    .unwrap_or(false)
                {
                    let mut counter = counter.lock().await;
                    *counter += 1;

                    if let Err(why) = msg.delete(&ctx).await {
                        eprintln!("Error deleting message: {:?}", why);
                    }
                    break; // Break after deleting the message for the first matching role.
                }
            }
        });
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        self.send_whats_up_message(&ctx).await;
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let aid = 1207380915458805800;

    let autodelete_roles: Arc<Mutex<HashSet<RoleId>>> = Arc::new(Mutex::new(HashSet::new()));
    let counter = Arc::new(Mutex::new(0));
    let handler = Handler::new(autodelete_roles.clone(), counter.clone());

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(token.clone())
        .application_id(aid)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Handler>(autodelete_roles);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    let counter_value = counter.lock().await;
    println!("Total messages processed: {}", *counter_value);
}
