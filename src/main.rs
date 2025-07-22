use remotro::{
    balatro::CurrentScreen,
    Remotro
};
mod play;

#[tokio::main]
async fn main() {
    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    println!("Hosted on 127.0.0.1:34143");

    loop {
        println!("Waiting for connection");
        // Wait for a Game to connect
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
        //Game Logic
        loop {
            match balatro.screen().await {
                Ok(screen) => match screen {
                    CurrentScreen::Menu(_menu) => {
                        println!("Menu");
                    },
                    CurrentScreen::SelectBlind(_blinds) => {
                        println!("BlindSelect");
                    },
                    CurrentScreen::Play(play) => {
                        println!("Playing");
                        println!("{}", play::score_hand(play));
                    },
                    CurrentScreen::Shop(_shop) => {
                        println!("Shop");
                    },
                    _ => {}
                }
                Err(e) => { 
                    println!("{e}");
                    break;
                }
            }
        }
    }
}
