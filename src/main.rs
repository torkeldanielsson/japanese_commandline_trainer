extern crate rand;

use std::io::{stdin,stdout,Write};
use rand::Rng;

struct Hiragana {
    c: String,
    romaji: String,
}

fn main() {

    let mut run = true;

    let mut rng = rand::thread_rng();

    let hiragana = vec![
        Hiragana { c: "あ".to_owned(), romaji: "a".to_owned() },
        Hiragana { c: "い".to_owned(), romaji: "i".to_owned() },
        Hiragana { c: "う".to_owned(), romaji: "u".to_owned() },
        Hiragana { c: "え".to_owned(), romaji: "e".to_owned() },
        Hiragana { c: "お".to_owned(), romaji: "o".to_owned() },
        ];
        /*
        Hiragana { c: "か".to_owned(), romaji: "ka".to_owned() },
        Hiragana { c: "き".to_owned(), romaji: "ki".to_owned() },
        Hiragana { c: "く".to_owned(), romaji: "ku".to_owned() },
        Hiragana { c: "け".to_owned(), romaji: "ke".to_owned() },
        Hiragana { c: "こ".to_owned(), romaji: "ko".to_owned() },
        // ];
        Hiragana { c: "さ".to_owned(), romaji: "sa".to_owned() },
        Hiragana { c: "し".to_owned(), romaji: "shi".to_owned() },
        Hiragana { c: "す".to_owned(), romaji: "su".to_owned() },
        Hiragana { c: "せ".to_owned(), romaji: "se".to_owned() },
        Hiragana { c: "そ".to_owned(), romaji: "so".to_owned() },
        // ];
        Hiragana { c: "た".to_owned(), romaji: "ta".to_owned() },
        Hiragana { c: "ち".to_owned(), romaji: "chi".to_owned() },
        Hiragana { c: "つ".to_owned(), romaji: "tsu".to_owned() },
        Hiragana { c: "て".to_owned(), romaji: "te".to_owned() },
        Hiragana { c: "と".to_owned(), romaji: "to".to_owned() },
        // ];
        Hiragana { c: "な".to_owned(), romaji: "na".to_owned() },
        Hiragana { c: "に".to_owned(), romaji: "ni".to_owned() },
        Hiragana { c: "ぬ".to_owned(), romaji: "nu".to_owned() },
        Hiragana { c: "ね".to_owned(), romaji: "ne".to_owned() },
        Hiragana { c: "の".to_owned(), romaji: "no".to_owned() },
        // ];
        Hiragana { c: "は".to_owned(), romaji: "ha".to_owned() },
        Hiragana { c: "ひ".to_owned(), romaji: "hi".to_owned() },
        Hiragana { c: "ふ".to_owned(), romaji: "fu".to_owned() },
        Hiragana { c: "へ".to_owned(), romaji: "he".to_owned() },
        Hiragana { c: "ほ".to_owned(), romaji: "ho".to_owned() },
        // ];
        Hiragana { c: "ま".to_owned(), romaji: "ma".to_owned() },
        Hiragana { c: "み".to_owned(), romaji: "mi".to_owned() },
        Hiragana { c: "む".to_owned(), romaji: "mu".to_owned() },
        Hiragana { c: "め".to_owned(), romaji: "me".to_owned() },
        Hiragana { c: "も".to_owned(), romaji: "mo".to_owned() },
        // ];
        Hiragana { c: "や".to_owned(), romaji: "ya".to_owned() },
        Hiragana { c: "ゆ".to_owned(), romaji: "yu".to_owned() },
        Hiragana { c: "よ".to_owned(), romaji: "yo".to_owned() },
        // ];
        Hiragana { c: "ら".to_owned(), romaji: "ra".to_owned() },
        Hiragana { c: "り".to_owned(), romaji: "ri".to_owned() },
        Hiragana { c: "る".to_owned(), romaji: "ru".to_owned() },
        Hiragana { c: "れ".to_owned(), romaji: "re".to_owned() },
        Hiragana { c: "ろ".to_owned(), romaji: "ro".to_owned() },
        // ];
        Hiragana { c: "わ".to_owned(), romaji: "wa".to_owned() },
        Hiragana { c: "ゐ".to_owned(), romaji: "wi".to_owned() },
        Hiragana { c: "ゑ".to_owned(), romaji: "we".to_owned() },
        Hiragana { c: "を".to_owned(), romaji: "wo".to_owned() },
        Hiragana { c: "ん".to_owned(), romaji: "n".to_owned() },
        // ];
        */

    while run {

        let i = rng.sample(rand::distributions::Uniform::new(0, hiragana.len()));
        print!("Hiragana: {:?}", hiragana[i].c);
        let _ = stdout().flush();

        let mut s = String::new();
        stdin().read_line(&mut s).expect("Input error");
        
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        if let Some('q') = s.chars().next_back() {
            run = false;
        }

        println!("Romaji: {:?}\n\n", hiragana[i].romaji);
        let _ = stdout().flush();
    }
}
