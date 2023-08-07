use megalodon::{entities, error, generator, SNS};
use std::fs::File;
use std::io::Read;
use env_logger;
use tokio::time::{sleep, Duration};
use tokio::signal::ctrl_c;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    api_host: String,
    access_token: String,
    filter_account: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let ctrl_c = ctrl_c();

    tokio::select! {
        // запуск основного кода
        _ = async_main() => {},
        _ = ctrl_c => {
            println!("Received Ctrl+C. Shutting down.");
        },
    }
}

async fn async_main() {
    let mut file = File::open("Config.toml").expect("Unable to open the file");
    let mut config_string = String::new();
    file.read_to_string(&mut config_string).expect("Unable to read the file");

    let config: Config = match toml::from_str(&config_string) {
        Ok(c) => c,
        Err(e) => {
            println!("Error parsing Config.toml: {}", e);
            return;
        }
    };
    // проверка юзеров и начало основного цикла
    match verify_credentials(&config.api_host, &config.access_token).await {
        Ok(_account) => {
            let client = generator(SNS::Mastodon, config.api_host.clone(), Some(config.access_token), None);
            loop {
                // получаем и обрабатываем статусы
                match get_local_status(&client).await {
                    Ok(mut statuses) => {
                        statuses.reverse(); // Reverse the order of statuses(!)
                        //извлекаем id
                        for status in statuses.iter().filter(|s| s.account.username != config.filter_account) {
                            println!("Received message: {}", status.id);
                            // бустим статус
                            match boost_status(&client, &status.id).await {
                                Ok(res) => {
                                    println!("Boosted: {}", res.id);
                                },
                                Err(err) => {
                                    println!("Error boosting {}: {}", status.id, err);
                                }
                            }
                        }
                    },
                    Err(err) => {
                        println!("Error getting local status: {}", err);
                    }
                }
                sleep(Duration::from_secs(30)).await; // Задержка цикла
            }
        }
        Err(e) => {
            println!("Error: Could not verify credentials - {}", e);
        }
    }
}


// проверяем данные пользователя
async fn verify_credentials(
    url: &str,
    access_token: &str,
) -> Result<entities::Account, error::Error> {
    let client = generator(SNS::Mastodon, url.to_string(), Some(access_token.to_string()), None);
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}

// получаем статус
async fn get_local_status(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
) -> Result<Vec<entities::Status>, error::Error> {
    let res = client.get_local_timeline(None).await.map_err(|e| error::Error::from(e))?;
    Ok(res.json())
}

// функция для буста статуса
async fn boost_status(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    id: &str, //передаем id записи
) -> Result<entities::Status, error::Error> {
    let res = client.reblog_status(id.to_string()).await.map_err(|e| error::Error::from(e))?;
    Ok(res.json())
}
