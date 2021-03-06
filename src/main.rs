use rand::Rng;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct SampleSentence {
    kanji: String,
    english: String,
}

#[derive(Debug, Clone)]
struct Word {
    kanji: String,
    hiragana: String,
    english: String,
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
            if word.hiragana != "" && word.english != "" {
                res = format!(
                    "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
                    res, word.kanji, word.hiragana, word.english
                );
            } else if word.hiragana != "" {
                res = format!(
                    "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
                    res, word.kanji, word.hiragana
                );
            } else if word.english != "" {
                res = format!(
                    "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
                    res, word.kanji, word.english
                );
            } else {
                res = format!("{}\\mbox{{{}}}\n, \\ \\ \\\n", res, word.kanji);
            }
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

fn generate(words: &Vec<Word>, header: String, n: i32) -> Result<(), Box<dyn Error>> {
    {
        let filename = format!("gen_kanji_{}.tex", n);
        let mut file = File::create(filename).unwrap();

        file.write_all(b"\\documentclass{article}\n")?;
        file.write_all(b"\\usepackage[a4paper, margin=0.9in]{geometry}\n")?;
        file.write_all(b"\\usepackage[1]{pagesel}\n")?;
        file.write_all(b"\\usepackage{setspace}\n")?;
        file.write_all(b"\\usepackage{fancyhdr}\n")?;
        file.write_all(b"\\hyphenpenalty 10000")?;
        file.write_all(b"\\exhyphenpenalty 10000")?;

        file.write_all(b"\\usepackage[utf8]{inputenc}\n")?;
        file.write_all(b"\\usepackage{CJKutf8, pinyin}\n")?;
        file.write_all(b"\\usepackage[overlap, CJK]{ruby}\n")?;
        file.write_all(b"\\pagenumbering{gobble}\n")?;
        file.write_all(b"\\begin{document}\n")?;
        file.write_all(b"\\begin{CJK*}{UTF8}{min}\n")?;
        file.write_all(b"\\pagestyle{fancy}\n")?;
        file.write_all(b"\\fancyhf{}\n")?;
        file.write_all(b"\\chead{")?;
        file.write_all(header.as_bytes())?;
        file.write_all(b"}\n")?;

        file.write_all(print_words(&words, true).as_bytes())?;

        file.write_all(b"\\end{CJK*}\n")?;
        file.write_all(b"\\end{document}\n")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    // 音楽 = おんがく = music
    // 音楽 = おんがく
    // 音楽

    let mut words: Vec<Word> = Vec::new();
    let mut header = "".to_owned();
    let mut n = 1;

    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if line.chars().next().unwrap() == '#' {
            if header != "" {
                generate(&words, header, n)?;
                words = Vec::new();
                n += 1;
            }
            header = line[1..].to_owned();
        } else {
            let parts: Vec<String> = line
                .split('=')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect();
            match parts.len() {
                1 => {
                    words.push(Word {
                        kanji: parts[0].clone(),
                        hiragana: "".to_owned(),
                        english: "".to_owned(),
                        samples: vec![],
                    });
                }
                2 => {
                    words.push(Word {
                        kanji: parts[0].clone(),
                        hiragana: "".to_owned(),
                        english: parts[1].clone(),
                        samples: vec![],
                    });
                }
                3 => {
                    words.push(Word {
                        kanji: parts[0].clone(),
                        hiragana: parts[1].clone(),
                        english: parts[2].clone(),
                        samples: vec![],
                    });
                }
                _ => {
                    println!("Unexpected line format: {}", line);
                }
            }
        }
    }

    if words.len() > 0 {
        generate(&words, header, n)?;
    }

    Ok(())
}
