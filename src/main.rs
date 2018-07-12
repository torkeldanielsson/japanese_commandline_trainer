extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

struct Kana {
    c: String,
    romaji: String,
}

fn print_kanas(kanagroup: Vec<&Kana>) -> String {

    let mut res: String = "".to_owned();

    let mut kanas = Vec::new();
    for k in kanagroup {
        kanas.push(k);
    }

    let mut rng = rand::thread_rng();

    res = format!("{}\\normalsize\n", res);

    for h in &kanas {
        res = format!("{}{} = {}, \\ \\ \\ ", res, &h.c, &h.romaji);
    }
    res = format!("{}\\\\\n\\par\n", res);

    res = format!("{}\\huge\n", res);

    let mut slice_index = 0;
    let mut lines = 0;

    let mut line_length = 10;
    if kanas.len() < line_length {
        line_length = kanas.len();
    }

    let h_slice: &mut [&Kana] = kanas.as_mut_slice();
    rng.shuffle(h_slice);

    while lines < 11 {

        let mut chars_in_line = 0;

        while chars_in_line < line_length {
            res = format!("{}{}\\ \\ \\ \\ \\ \\ ", res, h_slice[slice_index].c);

            slice_index += 1;
            chars_in_line += 1;

            if slice_index >= h_slice.len() {
                rng.shuffle(h_slice);
                slice_index = 0;
            }
        }

        res = format!("{}\\\\\n\\par\n", res);

        lines += 1;
    }

    res = format!("{}\\newpage\n", res);

    return res;
}

fn main() {

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(b"\\documentclass{article}\n").expect("1");
    file.write_all(b"\\usepackage[a4paper]{geometry}\n").expect("2");
    file.write_all(b"\\pagenumbering{gobble}\n").expect("3");
    file.write_all(b"\\begin{document}\n").expect("4");

    let kanagroups = vec![
        /*
        vec![
            Kana { c: "映画".to_owned(), romaji: "えいが = film".to_owned() },
            Kana { c: "音楽".to_owned(), romaji: "おんがく = musik".to_owned() },
            Kana { c: "雑誌".to_owned(), romaji: "ざっし = tidning (magazine)".to_owned() },
            Kana { c: "朝御飯".to_owned(), romaji: "あさごはん = frukost".to_owned() },
            Kana { c: "昼御飯".to_owned(), romaji: "ひるごはん = lunch".to_owned() },
            Kana { c: "晩御飯".to_owned(), romaji: "ばんごはん = middag".to_owned() },
            Kana { c: "お酒".to_owned(), romaji: "おさけ = sprit (alkohol)".to_owned() },
            Kana { c: "お茶".to_owned(), romaji: "おちゃ = grönt te".to_owned() },
            Kana { c: "水".to_owned(), romaji: "みず = vatten".to_owned() },
            Kana { c: "学校".to_owned(), romaji: "がっこう = skola".to_owned() },
            Kana { c: "朝".to_owned(), romaji: "あさ = morgon".to_owned() },
            Kana { c: "今日".to_owned(), romaji: "きょう = idag".to_owned() },
            Kana { c: "今晩".to_owned(), romaji: "こんばん = ikväll".to_owned() },
            Kana { c: "週末".to_owned(), romaji: "しゅうまつ = helg".to_owned() },
            Kana { c: "土曜日".to_owned(), romaji: "どようび = lördag".to_owned() },
            Kana { c: "日曜日".to_owned(), romaji: "にちようび = söndag".to_owned() },
            Kana { c: "毎日".to_owned(), romaji: "まいにち = varje dag".to_owned() },
            Kana { c: "毎晩".to_owned(), romaji: "まいばん = varje kväll".to_owned() },
        ],
        */

        vec![
            Kana { c: "ア".to_owned(), romaji: "a".to_owned() },
            Kana { c: "イ".to_owned(), romaji: "i".to_owned() },
            Kana { c: "ウ".to_owned(), romaji: "u".to_owned() },
            Kana { c: "エ".to_owned(), romaji: "e".to_owned() },
            Kana { c: "オ".to_owned(), romaji: "o".to_owned() },
        ],

        vec![
            Kana { c: "カ".to_owned(), romaji: "ka".to_owned() },
            Kana { c: "キ".to_owned(), romaji: "ki".to_owned() },
            Kana { c: "ク".to_owned(), romaji: "ku".to_owned() },
            Kana { c: "ケ".to_owned(), romaji: "ke".to_owned() },
            Kana { c: "コ".to_owned(), romaji: "ko".to_owned() },
        ],

        vec![
            Kana { c: "サ".to_owned(), romaji: "sa".to_owned() },
            Kana { c: "シ".to_owned(), romaji: "shi".to_owned() },
            Kana { c: "ス".to_owned(), romaji: "su".to_owned() },
            Kana { c: "セ".to_owned(), romaji: "se".to_owned() },
            Kana { c: "ソ".to_owned(), romaji: "so".to_owned() },
        ],

        vec![
            Kana { c: "タ".to_owned(), romaji: "ta".to_owned() },
            Kana { c: "チ".to_owned(), romaji: "chi".to_owned() },
            Kana { c: "ツ".to_owned(), romaji: "tsu".to_owned() },
            Kana { c: "テ".to_owned(), romaji: "te".to_owned() },
            Kana { c: "ト".to_owned(), romaji: "to".to_owned() },
        ],

        vec![
            Kana { c: "ナ".to_owned(), romaji: "na".to_owned() },
            Kana { c: "ニ".to_owned(), romaji: "ni".to_owned() },
            Kana { c: "ヌ".to_owned(), romaji: "nu".to_owned() },
            Kana { c: "ネ".to_owned(), romaji: "ne".to_owned() },
            Kana { c: "ノ".to_owned(), romaji: "no".to_owned() },
        ],

        vec![
            Kana { c: "ハ".to_owned(), romaji: "ha".to_owned() },
            Kana { c: "ヒ".to_owned(), romaji: "hi".to_owned() },
            Kana { c: "フ".to_owned(), romaji: "fu".to_owned() },
            Kana { c: "ヘ".to_owned(), romaji: "he".to_owned() },
            Kana { c: "ホ".to_owned(), romaji: "ho".to_owned() },
        ],

        vec![
            Kana { c: "マ".to_owned(), romaji: "ma".to_owned() },
            Kana { c: "ミ".to_owned(), romaji: "mi".to_owned() },
            Kana { c: "ム".to_owned(), romaji: "mu".to_owned() },
            Kana { c: "メ".to_owned(), romaji: "me".to_owned() },
            Kana { c: "モ".to_owned(), romaji: "mo".to_owned() },
        ],

        vec![
            Kana { c: "ヤ".to_owned(), romaji: "ya".to_owned() },
            Kana { c: "ユ".to_owned(), romaji: "yu".to_owned() },
            Kana { c: "ヨ".to_owned(), romaji: "yo".to_owned() },
        ],

        vec![
            Kana { c: "ラ".to_owned(), romaji: "ra".to_owned() },
            Kana { c: "リ".to_owned(), romaji: "ri".to_owned() },
            Kana { c: "ル".to_owned(), romaji: "ru".to_owned() },
            Kana { c: "レ".to_owned(), romaji: "re".to_owned() },
            Kana { c: "ロ".to_owned(), romaji: "ro".to_owned() },
        ],

        vec![
            Kana { c: "ワ".to_owned(), romaji: "wa".to_owned() },
            Kana { c: "ヲ".to_owned(), romaji: "å(wo)".to_owned() },
            Kana { c: "ン".to_owned(), romaji: "n".to_owned() },
        ],

/*
        vec![
            Kana { c: "あ".to_owned(), romaji: "a".to_owned() },
            Kana { c: "い".to_owned(), romaji: "i".to_owned() },
            Kana { c: "う".to_owned(), romaji: "u".to_owned() },
            Kana { c: "え".to_owned(), romaji: "e".to_owned() },
            Kana { c: "お".to_owned(), romaji: "o".to_owned() },
        ],

        vec![
            Kana { c: "か".to_owned(), romaji: "ka".to_owned() },
            Kana { c: "き".to_owned(), romaji: "ki".to_owned() },
            Kana { c: "く".to_owned(), romaji: "ku".to_owned() },
            Kana { c: "け".to_owned(), romaji: "ke".to_owned() },
            Kana { c: "こ".to_owned(), romaji: "ko".to_owned() },
        ],

        vec![
            Kana { c: "さ".to_owned(), romaji: "sa".to_owned() },
            Kana { c: "し".to_owned(), romaji: "shi".to_owned() },
            Kana { c: "す".to_owned(), romaji: "su".to_owned() },
            Kana { c: "せ".to_owned(), romaji: "se".to_owned() },
            Kana { c: "そ".to_owned(), romaji: "so".to_owned() },
        ],

        vec![
            Kana { c: "た".to_owned(), romaji: "ta".to_owned() },
            Kana { c: "ち".to_owned(), romaji: "chi".to_owned() },
            Kana { c: "つ".to_owned(), romaji: "tsu".to_owned() },
            Kana { c: "て".to_owned(), romaji: "te".to_owned() },
            Kana { c: "と".to_owned(), romaji: "to".to_owned() },
        ],

        vec![
            Kana { c: "な".to_owned(), romaji: "na".to_owned() },
            Kana { c: "に".to_owned(), romaji: "ni".to_owned() },
            Kana { c: "ぬ".to_owned(), romaji: "nu".to_owned() },
            Kana { c: "ね".to_owned(), romaji: "ne".to_owned() },
            Kana { c: "の".to_owned(), romaji: "no".to_owned() },
        ],

        vec![
            Kana { c: "は".to_owned(), romaji: "ha".to_owned() },
            Kana { c: "ひ".to_owned(), romaji: "hi".to_owned() },
            Kana { c: "ふ".to_owned(), romaji: "fu".to_owned() },
            Kana { c: "へ".to_owned(), romaji: "he".to_owned() },
            Kana { c: "ほ".to_owned(), romaji: "ho".to_owned() },
        ],

        vec![
            Kana { c: "ま".to_owned(), romaji: "ma".to_owned() },
            Kana { c: "み".to_owned(), romaji: "mi".to_owned() },
            Kana { c: "む".to_owned(), romaji: "mu".to_owned() },
            Kana { c: "め".to_owned(), romaji: "me".to_owned() },
            Kana { c: "も".to_owned(), romaji: "mo".to_owned() },
        ],

        vec![
            Kana { c: "や".to_owned(), romaji: "ya".to_owned() },
            Kana { c: "ゆ".to_owned(), romaji: "yu".to_owned() },
            Kana { c: "よ".to_owned(), romaji: "yo".to_owned() },
        ],

        vec![
            Kana { c: "ら".to_owned(), romaji: "ra".to_owned() },
            Kana { c: "り".to_owned(), romaji: "ri".to_owned() },
            Kana { c: "る".to_owned(), romaji: "ru".to_owned() },
            Kana { c: "れ".to_owned(), romaji: "re".to_owned() },
            Kana { c: "ろ".to_owned(), romaji: "ro".to_owned() },
        ],

        vec![
            Kana { c: "わ".to_owned(), romaji: "wa".to_owned() },
            Kana { c: "を".to_owned(), romaji: "å(wo)".to_owned() },
            Kana { c: "ん".to_owned(), romaji: "n".to_owned() },
        ],
*/
        ];

/*
    for kanagroup in &kanagroups {

        let mut kanas = Vec::new();
        for k in kanagroup {
            kanas.push(k);
        }

        print_kanas(kanas);
    }
*/

    for group_size in vec![1, 2, 5, 10] {

        let mut counter = 0;
        let mut next_print = group_size;

        let mut kanas_to_print = Vec::new();

        while counter < kanagroups.len() {

            for k in &kanagroups[counter] {
                kanas_to_print.push(k);
            }

            counter += 1;

            if counter >= next_print {
                file.write_all(print_kanas(kanas_to_print).as_bytes()).expect("6");
                kanas_to_print = Vec::new();
                next_print += group_size;
            }
        }

        if kanas_to_print.len() > 0 {
            file.write_all(print_kanas(kanas_to_print).as_bytes()).expect("7");
        }
    }

    /*
    let mut numbers = vec!["一", "二", "三", "四", "五", "六", "七", "八", "九", "十"];

    let mut number_slice = numbers.as_mut_slice();

    for _ in 0..10 {
        rng.shuffle(number_slice);
        for n in number_slice.to_owned() {
            print!("{}   ", n);
        }
    }
    */


    file.write_all(b"\\end{document}\n").expect("7");
}
