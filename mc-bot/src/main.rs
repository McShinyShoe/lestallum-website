// src/main.rs
use azalea::{prelude::*, protocol::packets::ProtocolPacket};
use parking_lot::Mutex;
use std::sync::Arc;
use std::env;

#[derive(Default, Clone, Component)]
struct State {
    did_initial_actions: Arc<Mutex<bool>>,
    start_inventory_open: Arc<Mutex<bool>>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let email;
    if let Some(second_arg) = env::args().nth(1) {
        println!("Using Email: {}", &second_arg);
        email=second_arg;
    } else {
        println!("No second argument provided");
        return Ok(());
    }
    let account = Account::microsoft(&email);
    let account= account.await?;
    ClientBuilder::new()
        .set_handler(handle_event)
        .start(account, "mc.thecavern.net:25565")
        .await
        .unwrap();

    Ok(())
}

async fn handle_event(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Spawn => {
            if !*state.did_initial_actions.lock() {
                *state.did_initial_actions.lock() = true;
                
                loop {
                    bot.set_selected_hotbar_slot(4);
                    bot.wait_updates(20).await;
    
                    bot.start_use_item();
                    bot.wait_updates(100).await;
                    
                    bot.wait_updates(600).await;
                    if *state.start_inventory_open.lock() {
                        break;
                    }
                }

            }
        }
        Event::Packet(packet) => {
            if packet.name().contains("open_screen") {
                *state.did_initial_actions.lock() = true;
                bot.wait_updates(100).await;
                let inventory = bot.get_inventory();
                let target_slot: u16 = 11;
                inventory.left_click(target_slot);
            }
        }
        Event::Chat(chat) => {
            let message = chat.message().to_ansi();
            println!("[CHAT] {}", message);
            
            if message.contains("[+] Lestallum") {
                let args: Vec<String> = env::args().skip(2).collect();
                for arg in args {
                    bot.chat(arg);
                    bot.wait_updates(10).await;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
