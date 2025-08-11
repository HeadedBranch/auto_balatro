use remotro::{Remotro, balatro::CurrentScreen};
mod play;

#[tokio::main]
async fn main() {
    let mut remotro = Remotro::host("0.0.0.0", 34143).await.unwrap();
    println!("Hosted on all interfaces at port 34143");

    loop {
        println!("Waiting for connection");
        let mut balatro = match remotro.accept().await {
            Ok(b) => {
                println!("New connection accepted");
                b
            }
            Err(e) => {
                println!("Connection Failed: {e}");
                break;
            }
        };
        loop {
            match balatro.screen().await {
                Ok(screen) => match screen {
                    CurrentScreen::Menu(_menu) => {
                        println!("Menu");
                    }
                    CurrentScreen::SelectBlind(_blinds) => {
                        println!("BlindSelect");
                    }
                    CurrentScreen::Play(play) => {
                        println!("Playing");
                        println!("{}", play::score_hand(play));
                    }
                    CurrentScreen::Shop(_shop) => {
                        println!("Shop");
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("{e}");
                    break;
                }
            }
        }
    }
}
