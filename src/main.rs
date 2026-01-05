use remotro::{Remotro, balatro::CurrentScreen::*};

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
                    Menu(_menu) => {}
                    SelectBlind(blinds) => {
                        blinds.select().await.expect("message");
                    }
                    Play(/*mut*/ play) => {
                        println!("Playing");
                        //play = play.click(&[0]).await.expect("Something Failed");
                        println!("{}", play::score_hand(&play));
                        //let _ = play.play().await;
                    }
                    Shop(_shop) => {}
                    GameOver(game) => {
                        println!("{:?}",game.outcome());
                        println!("{:?}",game.best_hand());
                        println!("{:?}",game.most_played_hand());
                        println!("{:?}",game.cards_discarded());
                        println!("{:?}",game.cards_played());
                        println!("{:?}",game.times_rerolled());
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
