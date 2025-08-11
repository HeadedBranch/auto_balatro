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
                    CurrentScreen::Play(/*mut*/ play) => {
                        println!("Playing");
                        //play = play.click(&[0]).await.expect("Something Failed");
                        println!("{}", play::score_hand(&play));
                        //let _ = play.play().await;
                    }
                    CurrentScreen::Shop(_shop) => {
                        println!("Shop");
                    }
                    CurrentScreen::GameOver(_game) => {
                        println!("Game Over");
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
