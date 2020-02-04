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

    res = format!("{}\\begin{{sloppypar}}\n", res);

    res = format!("{}\\large\n", res);
    res = format!("{}\\begin{{spacing}}{{1.4}}\n", res);

    for word in words {
        if word.kanji != "" {
            res = format!(
                "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
                res, word.kanji, word.hiragana, word.english
            );
        } else {
            res = format!(
                "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
                res, word.hiragana, word.english
            );
        }
    }

    res = format!("{}\\end{{spacing}}\n", res);

    res = format!("{}\\LARGE\n", res);
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
                if h_slice[slice_index].kanji != "" {
                    res = format!("{}\\mbox{{{}}}\\ \\ \\ \n", res, h_slice[slice_index].kanji);
                } else {
                    res = format!(
                        "{}\\mbox{{{}}}\\ \\ \\ \n",
                        res, h_slice[slice_index].hiragana
                    );
                }
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

    res = format!("{}\\end{{spacing}}\n", res);

    res = format!("{}\\end{{sloppypar}}\n", res);

    return res;
}

fn print_examples(words: &Vec<Word>) -> String {
    let mut res: String = String::new();
    let mut rng = rand::thread_rng();

    res = format!("{}\\large\n", res);
    res = format!("{}\\begin{{spacing}}{{2.5}}\n", res);
    res = format!("{}\\begin{{center}}\n", res);

    res = format!("{}\\setlength\\LTleft{{0pt}}\n", res);
    res = format!("{}\\setlength\\LTright{{0pt}}\n", res);
    res = format!(
        "{}\\begin{{longtabu}}{{@{{\\extracolsep{{\\fill}}}} l l }}\n",
        res
    );

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
        Word {
            kanji: "起きます",
            hiragana: "お.きる＝＞おきます",
            english: "to wake up",
            samples: vec![],
        },
        Word {
            kanji: "寝ます",
            hiragana: "寝る＝＞ねます",
            english: "to sleep",
            samples: vec![],
        },
        Word {
            kanji: "休みます",
            hiragana: "休む＝＞やすみます",
            english: "to rest",
            samples: vec![],
        },
        Word {
            kanji: "勉強します",
            hiragana: "べんきょう",
            english: "to study",
            samples: vec![],
        },
        Word {
            kanji: "終わります",
            hiragana: "おわります",
            english: "to finish",
            samples: vec![],
        },
        Word {
            kanji: "映画館",
            hiragana: "えいがかん",
            english: "movie theatre",
            samples: vec![],
        },
        Word {
            kanji: "銀行",
            hiragana: "ぎんこう",
            english: "bank",
            samples: vec![],
        },
        Word {
            kanji: "会議",
            hiragana: "かいぎ",
            english: "meeting",
            samples: vec![],
        },
        Word {
            kanji: "郵便局",
            hiragana: "ゆうびんきょく",
            english: "post office",
            samples: vec![],
        },
        Word {
            kanji: "図書館",
            hiragana: "としょかん",
            english: "library",
            samples: vec![],
        },
        Word {
            kanji: "美術館",
            hiragana: "びじゅつかん",
            english: "art museum",
            samples: vec![],
        },
        Word {
            kanji: "朝",
            hiragana: "あさ",
            english: "morning",
            samples: vec![],
        },
        Word {
            kanji: "昼",
            hiragana: "ひる",
            english: "midday",
            samples: vec![],
        },
        Word {
            kanji: "夜",
            hiragana: "よる",
            english: "evening",
            samples: vec![],
        },
        Word {
            kanji: "晩",
            hiragana: "ばん",
            english: "evening",
            samples: vec![],
        },
        Word {
            kanji: "午前",
            hiragana: "ごぜん",
            english: "a.m.",
            samples: vec![],
        },
        Word {
            kanji: "午後",
            hiragana: "ごご",
            english: "p.m.",
            samples: vec![],
        },
        /*
        Word {
            kanji: "",
            hiragana: "",
            english: "",
            samples: vec![
                SampleSentence {
                    kanji: "",
                    english: "",
                },
                SampleSentence {
                    kanji: "",
                    english: "",
                },
                SampleSentence {
                    kanji: "",
                    english: "",
                },
            ],
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
