use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct SampleSentence {
    kanji: &'static str,
    english: &'static str,
}

#[derive(Debug, Clone)]
struct Word {
    kanji: &'static str,
    hiragana: &'static str,
    english: &'static str,
    samples: Vec<SampleSentence>,
}

fn print_words(words: &Vec<Word>, kanjis: bool) -> String {
    let mut res: String = String::new();
    let mut rng = rand::thread_rng();

    res = format!("{}\\normalsize\n", res);
    res = format!("{}\\begin{{spacing}}{{1.4}}\n", res);

    for word in words {
        res = format!(
            "{}{} = {} = {}, \\ \\ \\ ",
            res, word.kanji, word.hiragana, word.english
        );
    }

    res = format!("{}\\end{{spacing}}\n", res);
    res = format!("{}\\large\n", res);
    res = format!("{}\\begin{{spacing}}{{2.1}}\n", res);
    res = format!("{}\\begin{{sloppypar}}\n", res);

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
                res = format!("{}\\mbox{{{}}}\\ \\ \\ \n", res, h_slice[slice_index].kanji);
            } else {
                res = format!(
                    "{}\\mbox{{{}}}\\ \\ \\ \n",
                    res, h_slice[slice_index].english
                );
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

    res = format!("{}\\end{{sloppypar}}\n", res);
    res = format!("{}\\end{{spacing}}\n", res);

    return res;
}

fn print_examples(words: &Vec<Word>) -> String {
    let mut res: String = String::new();
    let mut rng = rand::thread_rng();

    res = format!("{}\\large\n", res);
    res = format!("{}\\begin{{spacing}}{{2.5}}\n", res);
    res = format!("{}\\begin{{center}}\n", res);
    res = format!("{}\\begin{{longtabu}}{{ l l }}\n", res);

    let mut samples: Vec<&SampleSentence> = Vec::new();

    for word in words {
        for ws in &word.samples {
            samples.push(&ws);
        }
    }

    rng.shuffle(&mut samples);

    for sample in &samples {
        res = format!("{}{} & {} \\\\\n", res, sample.kanji, sample.english);
    }

    res = format!("{}\\end{{longtabu}}\n", res);
    res = format!("{}\\end{{center}}\n", res);
    res = format!("{}\\end{{spacing}}\n", res);

    return res;
}

fn main() -> Result<(), Box<dyn Error>> {
    let words = vec![
        /*
        Word {
            kanji: "事",
            hiragana: "こと",
            english: "(abstrakt) Sak, begrepp",
        },
        Word {
            kanji: "言う",
            hiragana: "い.う",
            english: "Säga",
        },
        Word {
            kanji: "思う",
            hiragana: "おも.う",
            english: "Tycka",
        },
        Word {
            kanji: "物",
            hiragana: "もの",
            english: "Sak, föremål, grej",
        },
        Word {
            kanji: "何",
            hiragana: "なに",
            english: "Vad?",
        },
        Word {
            kanji: "私",
            hiragana: "わたし",
            english: "Jag",
        },
        Word {
            kanji: "無理",
            hiragana: "むり",
            english: "Omöjligt",
        },
        Word {
            kanji: "行く",
            hiragana: "い.く",
            english: "Gå",
        },
        Word {
            kanji: "何時",
            hiragana: "なんじ",
            english: "Vilken tid?",
        },
        Word {
            kanji: "来る",
            hiragana: "く.る",
            english: "Komma / anlända",
        },
        Word {
            kanji: "見る",
            hiragana: "み.る",
            english: "Se / titta",
        },
        Word {
            kanji: "今",
            hiragana: "いま",
            english: "Nu",
        },
        Word {
            kanji: "良い",
            hiragana: "よ.い",
            english: "Bra",
        },
        Word {
            kanji: "所",
            hiragana: "ところ",
            english: "(abstrakt) Plats",
        },
        Word {
            kanji: "自転車",
            hiragana: "じてんしゃ",
            english: "Cykel",
        },
        */

        /*
        Word {
            kanji: "分かる",
            hiragana: "わ.かる",
            english: "Förstå",
        },
        Word {
            kanji: "中学校",
            hiragana: "ちゅうがっこう",
            english: "Högstadiet",
        },
        Word {
            kanji: "後",
            hiragana: "あと",
            english: "Sen / senare",
        },
        Word {
            kanji: "次の方どうぞ",
            hiragana: "つぎのかたどうぞ",
            english: "Nästa person tack!",
        },
        Word {
            kanji: "申し訳ない",
            hiragana: "もうしわけない",
            english: "Jag är väldigt ledsen..",
        },
        Word {
            kanji: "本",
            hiragana: "ほん",
            english: "Bok",
        },
        Word {
            kanji: "当たり前",
            hiragana: "あたりまえ",
            english: "Självklart",
        },
        Word {
            kanji: "持つ",
            hiragana: "も.つ",
            english: "Att ha (något)",
        },
        Word {
            kanji: "出る",
            hiragana: "で.る",
            english: "Lämna, gå ut",
        },
        Word {
            kanji: "考える",
            hiragana: "かんが.える",
            english: "Tänka",
        },
        */
        Word {
            kanji: "入り",
            hiragana: "い.り",
            english: "entering",
            samples: vec![
                SampleSentence {
                    kanji: "入りますか？",
                    english: "Are you going in?",
                },
                SampleSentence {
                    kanji: "気に入りました。",
                    english: "I liked it.",
                },
            ],
        },
        Word {
            kanji: "作る",
            hiragana: "つく.る",
            english: "to make; to produce",
            samples: vec![
                SampleSentence {
                    kanji: "読む本が人を作る。",
                    english: "A man is made by the books he reads.",
                },
                SampleSentence {
                    kanji: "君のために朝食を作るよ。",
                    english: "I'll make breakfast for you.",
                },
                SampleSentence {
                    kanji: "彼はご飯を作るのがうまい。",
                    english: "He's good at making food.",
                },
            ],
        },
        Word {
            kanji: "聞く",
            hiragana: "き.く",
            english: "ask; hear",
            samples: vec![
                SampleSentence {
                    kanji: "何で聞くの？",
                    english: "Why are you listening?",
                },
                SampleSentence {
                    kanji: "ケイトは聞いています。",
                    english: "Kate is listening.",
                },
                SampleSentence {
                    kanji: "どうして私にそれを聞くんですか？",
                    english: "Why do you ask me that?",
                },
            ],
        },
        Word {
            kanji: "日本",
            hiragana: "に.ほん",
            english: "Japan",
            samples: vec![
                SampleSentence {
                    kanji: "日本人ですか。",
                    english: "Are you Japanese?",
                },
                SampleSentence {
                    kanji: "私は日本人です。",
                    english: "I am Japanese.",
                },
                SampleSentence {
                    kanji: "日本の家は小さい。",
                    english: "Japanese houses are small.",
                },
            ],
        },
        Word {
            kanji: "場所",
            hiragana: "ば.しょ",
            english: "place",
            samples: vec![
                SampleSentence {
                    kanji: "住む場所を見つけた。",
                    english: "I've found a place to live.",
                },
                SampleSentence {
                    kanji: "その場所が好きです。",
                    english: "I like that place.",
                },
                SampleSentence {
                    kanji: "学校の場所さえ分かるか？",
                    english: "Do you even know where the school is?",
                },
            ],
        },
        Word {
            kanji: "合う",
            hiragana: "あ.う",
            english: "to meet; to match/fit",
            samples: vec![
                SampleSentence {
                    kanji: "間に合う？",
                    english: "Will you make it in time?",
                },
                SampleSentence {
                    kanji: "明日サラと話し合うよ。",
                    english: "I will discuss it with Sarah tomorrow.",
                },
                SampleSentence {
                    kanji: "この靴で私に合うサイズはありますか。",
                    english: "Do you have these shoes in my size?",
                },
            ],
        },
        Word {
            kanji: "話す",
            hiragana: "はな.す",
            english: "to talk; to speak",
            samples: vec![
                SampleSentence {
                    kanji: "ジャックは英語を話す。",
                    english: "Jack speaks English.",
                },
                SampleSentence {
                    kanji: "メキシコではスペイン語を話す。",
                    english: "In Mexico they speak Spanish.",
                },
                SampleSentence {
                    kanji: "日本語が話せます。",
                    english: "I can speak Japanese.",
                },
            ],
        },
        Word {
            kanji: "使う",
            hiragana: "つか.う",
            english: "to use; to make use of",
            samples: vec![
                SampleSentence {
                    kanji: "どんな時使うの?",
                    english: "When do you use it?",
                },
                SampleSentence {
                    kanji: "どうやって箸を使いますか。",
                    english: "How do I use chopsticks?",
                },
                SampleSentence {
                    kanji: "それは私が使う物です。",
                    english: "It's for my personal use.",
                },
            ],
        },
        Word {
            kanji: "風",
            hiragana: "かぜ",
            english: "wind; draft; cold (illness)",
            samples: vec![
                SampleSentence {
                    kanji: "風が強いな。",
                    english: "The wind is strong.",
                },
                SampleSentence {
                    kanji: "風が少しある。",
                    english: "There's a little wind.",
                },
                SampleSentence {
                    kanji: "今日は風が強い。",
                    english: "There is a strong wind today.",
                },
            ],
        },
        Word {
            kanji: "前",
            hiragana: "まえ",
            english: "before; earlier; previous",
            samples: vec![
                SampleSentence {
                    kanji: "お名前は？",
                    english: "What's your name?",
                },
                SampleSentence {
                    kanji: "2時半前だ。",
                    english: "Two and a half hours ago.",
                },
                SampleSentence {
                    kanji: "１１時１０分前です。",
                    english: "It is ten minutes to eleven.",
                },
            ],
        },
        Word {
            kanji: "多分",
            hiragana: "た.ぶん",
            english: "perhaps; probably",
            samples: vec![
                SampleSentence {
                    kanji: "多分おそすぎる。",
                    english: "Maybe it's too late.",
                },
                SampleSentence {
                    kanji: "メアリーは多分寝てる。",
                    english: "Mary is probably sleeping.",
                },
                SampleSentence {
                    kanji: "アリシアは多分大丈夫。",
                    english: "Alicia is probably ok.",
                },
            ],
        },
        Word {
            kanji: "子供",
            hiragana: "こ.ども",
            english: "child",
            samples: vec![
                SampleSentence {
                    kanji: "子供がいますか。",
                    english: "Do you have children?",
                },
                SampleSentence {
                    kanji: "私はまだ子供です。",
                    english: "I am still a child.",
                },
                SampleSentence {
                    kanji: "５月５日は子供の日です。",
                    english: "May 5 is Children's Day.",
                },
            ],
        },
        Word {
            kanji: "常",
            hiragana: "じょう",
            english: "(abstrakt) Det vanliga / oföränderligt",
            samples: vec![],
        },
        Word {
            kanji: "用",
            hiragana: "よう",
            english: "(abstrakt) Anledning / användning / uppgift",
            samples: vec![
                SampleSentence {
                    kanji: "何の用だ。",
                    english: "What do you want?",
                },
                SampleSentence {
                    kanji: "今日用事あるの？",
                    english: "Is there anything you need to do today?",
                },
                SampleSentence {
                    kanji: "カメラ用の電池を下さい。",
                    english: "May I have some camera batteries, please?",
                },
            ],
        },
        Word {
            kanji: "漢字",
            hiragana: "かん.じ",
            english: "Kanji",
            samples: vec![
                SampleSentence {
                    kanji: "漢字を少し教えてください。",
                    english: "Teach me some kanji, please.",
                },
                SampleSentence {
                    kanji: "私は漢字を勉強しています。",
                    english: "I am studying kanji.",
                },
                SampleSentence {
                    kanji: "この漢字は何と読みますか。",
                    english: "How do you read this kanji?",
                },
            ],
        },
        Word {
            kanji: "非",
            hiragana: "ひ",
            english: "(abstrakt) Fel / misstag, icke-",
            samples: vec![],
        },
        Word {
            kanji: "非常",
            hiragana: "ひ.じょう",
            english: "Nödsituation / Ovanligt",
            samples: vec![SampleSentence {
                kanji: "彼女は非常に忙しい。",
                english: "She is very busy.",
            }],
        },
        Word {
            kanji: "常用漢字",
            hiragana: "じょう.よう.かん.じ",
            english: "Jōyō kanji​ (lista med 2136 kanji)",
            samples: vec![SampleSentence {
                kanji: "常用漢字を学ぶのは良い事です。",
                english: "It's good to learn the Jōyō kanji.​",
            }],
        },
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
        let mut file = File::create("gen_english.tex").unwrap();

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

    {
        let mut file = File::create("gen_samples.tex").unwrap();

        file.write_all(b"\\documentclass{article}\n")?;
        file.write_all(b"\\usepackage[a4paper, margin=0.5in]{geometry}\n")?;
        file.write_all(b"\\usepackage{setspace}\n")?;
        file.write_all(b"\\usepackage{longtable, booktabs}\n")?;
        file.write_all(b"\\usepackage{tabu}\n")?;
        file.write_all(b"\\hyphenpenalty 10000")?;
        file.write_all(b"\\exhyphenpenalty 10000")?;

        file.write_all(b"\\usepackage[utf8]{inputenc}\n")?;
        file.write_all(b"\\usepackage{CJKutf8, pinyin}\n")?;
        file.write_all(b"\\usepackage[overlap, CJK]{ruby}\n")?;
        file.write_all(b"\\pagenumbering{gobble}\n")?;
        file.write_all(b"\\begin{document}\n")?;
        file.write_all(b"\\begin{CJK*}{UTF8}{min}\n")?;

        file.write_all(print_examples(&words).as_bytes())?;

        file.write_all(b"\\end{CJK*}\n")?;
        file.write_all(b"\\end{document}\n")?;
    }

    Ok(())
}
