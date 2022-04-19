// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;

const SESSION_FILE: &str = "amime.session";

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn async_main() -> Result {
    // Initialize the logger
    env_logger::init();

    // Load the configuration
    let config = amime_watch::Config::load()?;

    // Connect the client
    log::info!("connecting bot...");
    let mut client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id: config.telegram.api_id,
        api_hash: config.telegram.api_hash.clone(),
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
    })
    .await?;
    log::info!("bot connected");

    if !client.is_authorized().await? {
        client
            .bot_sign_in(
                &config.bot.token,
                config.telegram.api_id,
                &config.telegram.api_hash,
            )
            .await?;
        client.session().save_to_file(SESSION_FILE)?;
        log::info!("bot authorized");
    }

    // Load and run the plugins
    amime_watch::plugins::Manager::new(config.bot.prefixes)
        .load_plugins()
        .run(client.clone())
        .await?;

    // Save the session
    client.session().save_to_file(SESSION_FILE)?;
    log::info!("session saved");

    Ok(())
}

fn main() -> Result {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
