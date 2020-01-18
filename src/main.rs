use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Word {
    kanji: &'static str,
    hiragana: &'static str,
    swedish: &'static str,
}

fn print_words(words: &Vec<Word>, kanjis: bool) -> String {
    let mut res: String = String::new();
    let mut rng = rand::thread_rng();

    res = format!("{}\\normalsize\n", res);

    for word in words {
        res = format!(
            "{}{} = {} = {}, \\ \\ \\ ",
            res, word.kanji, word.hiragana, word.swedish
        );
    }

    res = format!("{}\\\\\n\\par\n", res);

    res = format!("{}\\large\n", res);

    res = format!("{}\\begin{{spacing}}{{2.1}}\n", res);

    let mut slice_index = 0;
    let mut lines = 0;

    let mut line_length = 14;

    if words.len() < line_length {
        line_length = words.len();
    }

    let mut h_slice = words.to_vec();

    while lines < 26 {
        let mut chars_in_line = 0;

        while chars_in_line < line_length {
            if kanjis {
                res = format!("{}\\mbox{{{}}}\\ \\ \\  ", res, h_slice[slice_index].kanji);
            } else {
                res = format!("{}{}\\ \\ \\ \\ \\  ", res, h_slice[slice_index].swedish);
            }

            slice_index += 1;
            chars_in_line += 1;

            if slice_index >= h_slice.len() {
                rng.shuffle(&mut h_slice);
                slice_index = 0;
            }
        }

        lines += 1;
    }

    res = format!("{}\\end{{spacing}}\n", res);

    return res;
}

fn main() -> Result<(), Box<dyn Error>> {
    let words = vec![
        /*
        Word {
            kanji: "事",
            hiragana: "こと",
            swedish: "(abstrakt) Sak, begrepp",
        },
        Word {
            kanji: "言う",
            hiragana: "い.う",
            swedish: "Säga",
        },
        Word {
            kanji: "思う",
            hiragana: "おも.う",
            swedish: "Tycka",
        },
        Word {
            kanji: "物",
            hiragana: "もの",
            swedish: "Sak, föremål, grej",
        },
        Word {
            kanji: "何",
            hiragana: "なに",
            swedish: "Vad?",
        },
        Word {
            kanji: "私",
            hiragana: "わたし",
            swedish: "Jag",
        },
        Word {
            kanji: "無理",
            hiragana: "むり",
            swedish: "Omöjligt",
        },
        Word {
            kanji: "行く",
            hiragana: "い.く",
            swedish: "Gå",
        },
        Word {
            kanji: "何時",
            hiragana: "なんじ",
            swedish: "Vilken tid?",
        },
        Word {
            kanji: "来る",
            hiragana: "く.る",
            swedish: "Komma / anlända",
        },
        Word {
            kanji: "見る",
            hiragana: "み.る",
            swedish: "Se / titta",
        },
        Word {
            kanji: "今",
            hiragana: "いま",
            swedish: "Nu",
        },
        Word {
            kanji: "良い",
            hiragana: "よ.い",
            swedish: "Bra",
        },
        Word {
            kanji: "所",
            hiragana: "ところ",
            swedish: "(abstrakt) Plats",
        },
        Word {
            kanji: "自転車",
            hiragana: "じてんしゃ",
            swedish: "Cykel",
        },
        */
        Word {
            kanji: "分かる",
            hiragana: "わ.かる",
            swedish: "Förstå",
        },
        Word {
            kanji: "中学校",
            hiragana: "ちゅうがっこう",
            swedish: "Högstadiet",
        },
        Word {
            kanji: "後",
            hiragana: "あと",
            swedish: "Sen / senare",
        },
        Word {
            kanji: "次の方どうぞ",
            hiragana: "つぎのかたどうぞ",
            swedish: "Nästa person tack!",
        },
        Word {
            kanji: "申し訳ない",
            hiragana: "もうしわけない",
            swedish: "Jag är väldigt ledsen..",
        },
        Word {
            kanji: "本",
            hiragana: "ほん",
            swedish: "Bok",
        },
        Word {
            kanji: "当たり前",
            hiragana: "あたりまえ",
            swedish: "Självklart",
        },
        Word {
            kanji: "持つ",
            hiragana: "も.つ",
            swedish: "Att ha (något)",
        },
        Word {
            kanji: "出る",
            hiragana: "で.る",
            swedish: "Lämna, gå ut",
        },
        Word {
            kanji: "考える",
            hiragana: "かんが.える",
            swedish: "Tänka",
        },
        /*
        Word {
            kanji: "入り",
            hiragana: "い.り",
            swedish: "Gå in, stoppa in (något)",
        },
        Word {
            kanji: "作る",
            hiragana: "つく.る",
            swedish: "Göra (något)",
        },
        Word {
            kanji: "聞く",
            hiragana: "き.く",
            swedish: "Fråga / höra",
        },
        Word {
            kanji: "日本",
            hiragana: "にほん",
            swedish: "Japan",
        },
        Word {
            kanji: "場所",
            hiragana: "ば.しょ",
            swedish: "Plats",
        },
        Word {
            kanji: "合う",
            hiragana: "あ.う",
            swedish: "Träffas / mötas",
        },
        Word {
            kanji: "話す",
            hiragana: "はな.す",
            swedish: "Prata",
        },
        Word {
            kanji: "使う",
            hiragana: "つか.う",
            swedish: "Använda",
        },
        Word {
            kanji: "風",
            hiragana: "かぜ",
            swedish: "Vind / Drag / Förkylning",
        },
        Word {
            kanji: "前",
            hiragana: "まえ",
            swedish: "Före",
        },
        Word {
            kanji: "多分",
            hiragana: "た.ぶん",
            swedish: "Kanske",
        },
        Word {
            kanji: "子供",
            hiragana: "こ.ども",
            swedish: "Barn",
        },
        Word {
            kanji: "常",
            hiragana: "じょう",
            swedish: "(abstrakt) Det vanliga, oföränderligt",
        },
        Word {
            kanji: "用",
            hiragana: "よう",
            swedish: "(abstrakt) Anledning, användning, uppgift",
        },
        Word {
            kanji: "漢字",
            hiragana: "かん.じ",
            swedish: "Kanji",
        },
        Word {
            kanji: "非",
            hiragana: "ひ",
            swedish: "(abstrakt) Fel, misstag, icke-",
        },
        Word {
            kanji: "非常",
            hiragana: "ひ.じょう",
            swedish: "Nödsituation / Ovanligt",
        },
        Word {
            kanji: "常用漢字",
            hiragana: "じょう.よう.かん.じ",
            swedish: "Jōyō kanji​ (lista med 2136 kanji som ni måste lära er)",
        },
        */
    ];

    {
        let mut file = File::create("gen_kanji.tex").unwrap();

        file.write_all(b"\\documentclass{article}\n")?;
        file.write_all(b"\\usepackage[a4paper, margin=0.5in]{geometry}\n")?;
        file.write_all(b"\\usepackage{setspace}\n")?;
        file.write_all(b"\\hyphenpenalty 10000")?;
        file.write_all(b"\\exhyphenpenalty 10000")?;

        file.write_all(b"\\usepackage[utf8]{inputenc}\n")?;
        file.write_all(b"\\usepackage{CJKutf8, pinyin}\n")?;
        file.write_all(b"\\usepackage[overlap, CJK]{ruby}\n")?;
        file.write_all(b"\\pagenumbering{gobble}\n")?;
        file.write_all(b"\\begin{document}\n")?;
        file.write_all(b"\\begin{CJK*}{UTF8}{min}\n")?;

        file.write_all(print_words(&words, true).as_bytes())?;

        file.write_all(b"\\end{CJK*}\n")?;
        file.write_all(b"\\end{document}\n")?;
    }

    {
        let mut file = File::create("gen_swedish.tex").unwrap();

        file.write_all(b"\\documentclass{article}\n")?;
        file.write_all(b"\\usepackage[a4paper, margin=0.5in]{geometry}\n")?;
        file.write_all(b"\\usepackage{setspace}\n")?;
        file.write_all(b"\\hyphenpenalty 10000")?;
        file.write_all(b"\\exhyphenpenalty 10000")?;

        file.write_all(b"\\usepackage[utf8]{inputenc}\n")?;
        file.write_all(b"\\usepackage{CJKutf8, pinyin}\n")?;
        file.write_all(b"\\usepackage[overlap, CJK]{ruby}\n")?;
        file.write_all(b"\\pagenumbering{gobble}\n")?;
        file.write_all(b"\\begin{document}\n")?;
        file.write_all(b"\\begin{CJK*}{UTF8}{min}\n")?;

        file.write_all(print_words(&words, false).as_bytes())?;

        file.write_all(b"\\end{CJK*}\n")?;
        file.write_all(b"\\end{document}\n")?;
    }
    Ok(())
}
