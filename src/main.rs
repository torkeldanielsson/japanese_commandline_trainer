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

        /*
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
            english: "eternity; permanence; perpetuity",
            samples: vec![],
        },
        Word {
            kanji: "用",
            hiragana: "よう",
            english: "use; purpose​; task",
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
            english: "kanji",
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
            english: "fault; error; mistake; un-; non-",
            samples: vec![],
        },
        Word {
            kanji: "非常",
            hiragana: "ひ.じょう",
            english: "emergency; extraordinary",
            samples: vec![SampleSentence {
                kanji: "彼女は非常に忙しい。",
                english: "She is very busy.",
            }],
        },
        Word {
            kanji: "常用漢字",
            hiragana: "じょう.よう.かん.じ",
            english: "kanji for common use; jōyō kanji",
            samples: vec![SampleSentence {
                kanji: "常用漢字を学ぶのは良い事です。",
                english: "It's good to learn the Jōyō kanji.​",
            }],
        },
        */
        /*
        // 4
        Word {
            kanji: "君",
            hiragana: "きみ",
            english: "you; buddy; pal​",
            samples: vec![
                SampleSentence {
                    kanji: "君が好きだ。",
                    english: "I like you. / I love you.",
                },
                SampleSentence {
                    kanji: "君も行くか。",
                    english: "Are you going, too?",
                },
                SampleSentence {
                    kanji: "君はいつ帰る？",
                    english: "When are you going home?",
                },
            ],
        },
        Word {
            kanji: "好き",
            hiragana: "す.き",
            english: "liked; favourite; loved",
            samples: vec![
                SampleSentence {
                    kanji: "ケーキは好きですか？",
                    english: "Do you like cake?",
                },
                SampleSentence {
                    kanji: "彼が大好き！",
                    english: "I really like him!",
                },
            ],
        },
        Word {
            kanji: "彼",
            hiragana: "かれ",
            english: "he; him",
            samples: vec![
                SampleSentence {
                    kanji: "彼寝てる？",
                    english: "Is he sleeping?",
                },
                SampleSentence {
                    kanji: "彼は寝た。",
                    english: "He went to bed.",
                },
            ],
        },
        Word {
            kanji: "帰る",
            hiragana: "かえ.る",
            english: "to return; to go home; to go back",
            samples: vec![
                SampleSentence {
                    kanji: "今から帰る？",
                    english: "Are you going back now?",
                },
                SampleSentence {
                    kanji: "帰りなさい。",
                    english: "Please go home.",
                },
                SampleSentence {
                    kanji: "7時に帰るよ。",
                    english: "I'll be back at seven o'clock.",
                },
            ],
        },
        Word {
            kanji: "彼女",
            hiragana: "かの.じょ",
            english: "she; her",
            samples: vec![
                SampleSentence {
                    kanji: "彼女は元気？",
                    english: "Is she ok?",
                },
                SampleSentence {
                    kanji: "彼女は学生だ。",
                    english: "She's a student.",
                },
                SampleSentence {
                    kanji: "彼女は忙しい。",
                    english: "She's busy.",
                },
            ],
        },
        Word {
            kanji: "忙しい",
            hiragana: "いそが.しい",
            english: "busy; occupied; restless",
            samples: vec![
                SampleSentence {
                    kanji: "私は忙しい。",
                    english: "I'm busy. / I have much to do.",
                },
                SampleSentence {
                    kanji: "少し忙しい。",
                    english: "I'm a bit busy. / I'm in a bit of a hurry.",
                },
                SampleSentence {
                    kanji: "宿題で忙しい。",
                    english: "I'm busy doing homework.",
                },
            ],
        },
        Word {
            kanji: "学ぶ",
            hiragana: "まなぶ",
            english: "to study (in depth); to learn",
            samples: vec![
                SampleSentence {
                    kanji: "フランス語を学ぶのは難しい。",
                    english: "It's difficult to learn French.",
                },
                SampleSentence {
                    kanji: "学ぶ事がたくさんあります。",
                    english: "There is a lot to learn.",
                },
            ],
        },
        Word {
            kanji: "読む",
            hiragana: "よ.む",
            english: "to read​",
            samples: vec![
                SampleSentence {
                    kanji: "何読んでるの？",
                    english: "What are you reading?",
                },
                SampleSentence {
                    kanji: "私は読みます。",
                    english: "I read.",
                },
                SampleSentence {
                    kanji: "これを読んで。",
                    english: "Read this.",
                },
                SampleSentence {
                    kanji: "彼は読めません。",
                    english: "He can't read it.",
                },
            ],
        },
        Word {
            kanji: "電池",
            hiragana: "でん.ち",
            english: "battery",
            samples: vec![
                SampleSentence {
                    kanji: "カメラ用の電池を下さい。",
                    english: "May I have some camera batteries, please?",
                },
                SampleSentence {
                    kanji: "やばい、電池切れそう",
                    english: "Oh no, my battery’s nearly dead.",
                },
            ],
        },
        Word {
            kanji: "少し",
            hiragana: "すこ.し",
            english: "small quantity; little; few",
            samples: vec![
                SampleSentence {
                    kanji: "少しテレビを見てもいいですか。",
                    english: "Do you mind if I watch a little TV?",
                },
                SampleSentence {
                    kanji: "風が少しある。",
                    english: "There's some wind.",
                },
                SampleSentence {
                    kanji: "今日は少し寒い。",
                    english: "It's a little cold today.",
                },
            ],
        },
        Word {
            kanji: "教える",
            hiragana: "おし.える",
            english: "to teach; to instruct​",
            samples: vec![
                SampleSentence {
                    kanji: "アンは教えます。",
                    english: "Anne teaches",
                },
                SampleSentence {
                    kanji: "私は英語を教えます。",
                    english: "I teach English.",
                },
                SampleSentence {
                    kanji: "彼女は英語を教える事ができる。",
                    english: "She can teach English.",
                },
            ],
        },
        Word {
            kanji: "朝食",
            hiragana: "ちょう.しょく",
            english: "breakfast",
            samples: vec![
                SampleSentence {
                    kanji: "朝食を食べなかった。",
                    english: "I didn't have any breakfast.",
                },
                SampleSentence {
                    kanji: "彼は朝食を食べた。",
                    english: "He ate breakfast.",
                },
                SampleSentence {
                    kanji: "朝食に何をとりましたか。",
                    english: "What did you have for breakfast?",
                },
            ],
        },
        Word {
            kanji: "宿題",
            hiragana: "しゅく.だい",
            english: "homework; assignment",
            samples: vec![
                SampleSentence {
                    kanji: "宿題やった？",
                    english: "Did you do your homework?",
                },
                SampleSentence {
                    kanji: "宿題をやります。",
                    english: "I'll do my homework.",
                },
                SampleSentence {
                    kanji: "彼は今宿題で忙しい。",
                    english: "He's busy with his homework now.",
                },
            ],
        },
        Word {
            kanji: "寒い",
            hiragana: "さむ.い",
            english: "cold (e.g. weather)​",
            samples: vec![
                SampleSentence {
                    kanji: "寒いです。",
                    english: "It's cold.",
                },
                SampleSentence {
                    kanji: "彼女は寒いと言った。",
                    english: "She told me she is cold.",
                },
                SampleSentence {
                    kanji: "私は少し寒いです。",
                    english: "I am a little cold.",
                },
            ],
        },
        */

        /*
        // 5
        Word {
            kanji: "寝る",
            hiragana: "ね.る",
            english: "​to sleep; to lie down​",
            samples: vec![
                SampleSentence {
                    kanji: "犬が寝る。",
                    english: "The dog sleeps.",
                },
                SampleSentence {
                    kanji: "猫が寝る。",
                    english: "A cat sleeps.",
                },
                SampleSentence {
                    kanji: "僕は寝たい。",
                    english: "I want to sleep.",
                },
            ],
        },
        Word {
            kanji: "犬",
            hiragana: "いぬ",
            english: "dog​",
            samples: vec![
                SampleSentence {
                    kanji: "犬を見た。",
                    english: "I saw a dog.",
                },
                SampleSentence {
                    kanji: "犬に注意！",
                    english: "Beware of the dog!",
                },
                SampleSentence {
                    kanji: "私は犬が好き。",
                    english: "I like dogs.",
                },
            ],
        },
        Word {
            kanji: "猫",
            hiragana: "ねこ",
            english: "​cat",
            samples: vec![
                SampleSentence {
                    kanji: "黒猫が好きですか？",
                    english: "Do you like black cats?",
                },
                SampleSentence {
                    kanji: "猫が寝る。",
                    english: "The cat sleeps.",
                },
                SampleSentence {
                    kanji: "私の猫です。",
                    english: "It is my cat.",
                },
            ],
        },
        Word {
            kanji: "僕",
            hiragana: "ぼく",
            english: "​I; me (male term)",
            samples: vec![
                SampleSentence {
                    kanji: "僕だよ！",
                    english: "It's me!",
                },
                SampleSentence {
                    kanji: "僕は忙しい。",
                    english: "I'm busy.",
                },
                SampleSentence {
                    kanji: "僕は本を読みます。",
                    english: "I read books.",
                },
            ],
        },
        Word {
            kanji: "注意",
            hiragana: "ちゅう.い",
            english: "​caution; being careful; warning",
            samples: vec![
                SampleSentence {
                    kanji: "足元にご注意ください。",
                    english: "Watch your step.",
                },
                SampleSentence {
                    kanji: "もうちょっと注意しないと。",
                    english: "I have to pay a little more attention.",
                },
                SampleSentence {
                    kanji: "ひらくドアにご注意ください。",
                    english: "Please be careful of the opening doors.",
                },
            ],
        },
        Word {
            kanji: "足元",
            hiragana: "あし.もと",
            english: "​at one's feet; one's step",
            samples: vec![SampleSentence {
                kanji: "足元に気をつけて。",
                english: "Mind your step.",
            }],
        },
        Word {
            kanji: "気を付けて",
            hiragana: "き.を.つ.けて",
            english: "take care; be careful​​",
            samples: vec![
                SampleSentence {
                    kanji: "気を付けてね。",
                    english: "Take care!",
                },
                SampleSentence {
                    kanji: "これからはもっと気を付けてね。",
                    english: "Be more careful from now on.",
                },
                SampleSentence {
                    kanji: "トム、気をつけて！",
                    english: "Tom, be careful!",
                },
            ],
        },
        Word {
            kanji: "下さい",
            hiragana: "くだ.さい",
            english: "please give me; please do for me​",
            samples: vec![
                SampleSentence {
                    kanji: "来て下さい。",
                    english: "Please come.",
                },
                SampleSentence {
                    kanji: "本を下さい。",
                    english: "Please give me a book.",
                },
                SampleSentence {
                    kanji: "水をください。",
                    english: "Please give me some water.",
                },
            ],
        },
        Word {
            kanji: "水",
            hiragana: "みず",
            english: "water​",
            samples: vec![
                SampleSentence {
                    kanji: "犬は水が好きです。",
                    english: "The dog likes water.",
                },
                SampleSentence {
                    kanji: "猫は水が好きではありません。",
                    english: "The cat does not like water.",
                },
            ],
        },
        Word {
            kanji: "英語",
            hiragana: "えい.ご",
            english: "English (language)​",
            samples: vec![
                SampleSentence {
                    kanji: "英語の先生です。",
                    english: "I am an English teacher.",
                },
                SampleSentence {
                    kanji: "彼は英語がうまい。",
                    english: "He is such a good English speaker.",
                },
                SampleSentence {
                    kanji: "私は英語が話せる。",
                    english: "I can speak English.",
                },
            ],
        },
        Word {
            kanji: "先生",
            hiragana: "せん.せい",
            english: "teacher; instructor; master​​",
            samples: vec![
                SampleSentence {
                    kanji: "彼女は先生です。",
                    english: "She is a teacher.",
                },
                SampleSentence {
                    kanji: "彼は先生ですか。",
                    english: "Is he a teacher?",
                },
                SampleSentence {
                    kanji: "英語の先生です。",
                    english: "I am an English teacher.",
                },
                SampleSentence {
                    kanji: "私は日本語の先生です。",
                    english: "I am a Japanese teacher.",
                },
            ],
        },
        Word {
            kanji: "食べる",
            hiragana: "た.べる",
            english: "​to eat",
            samples: vec![
                SampleSentence {
                    kanji: "アイス食べる？",
                    english: "Do you want some ice cream?",
                },
                SampleSentence {
                    kanji: "食べるときは注意してください。",
                    english: "Please be careful when you eat.",
                },
                SampleSentence {
                    kanji: "電池を食べないでください。",
                    english: "Please don't eat the batteries.",
                },
                SampleSentence {
                    kanji: "犬を食べないでください。",
                    english: "Please don't eat the dog.",
                },
            ],
        },
        Word {
            kanji: "元気",
            hiragana: "げん.き",
            english: "lively; full of spirit; healthy",
            samples: vec![
                SampleSentence {
                    kanji: "元気？",
                    english: "How are you?",
                },
                SampleSentence {
                    kanji: "お元気で！",
                    english: "Good luck!",
                },
                SampleSentence {
                    kanji: "私は元気です。",
                    english: "I'm fine.",
                },
            ],
        },
        Word {
            kanji: "黒 / 黒い",
            hiragana: "くろ / くろい",
            english: "black",
            samples: vec![
                SampleSentence {
                    kanji: "その猫は黒いです。",
                    english: "That cat is black.",
                },
                SampleSentence {
                    kanji: "犬は黒い電池を食べています。",
                    english: "The dog is eating a black battery.",
                },
                SampleSentence {
                    kanji: "自転車が黒である事は良い事だと思います。",
                    english: "I think it is good that the bike is black.",
                },
            ],
        },
        */

        /*
        // 6
        Word {
            kanji: "取る",
            hiragana: "と.る",
            english: "to take; to pick up",
            samples: vec![
                SampleSentence {
                    kanji: "この黒い本を取ってください。",
                    english: "Please take this black book.",
                },
                SampleSentence {
                    kanji: "どっち 取る？",
                    english: "Which one do you take?",
                },
                SampleSentence {
                    kanji: "それ取って。",
                    english: "Give me that.",
                },
            ],
        },
        Word {
            kanji: "知る",
            hiragana: "し.る",
            english: "to know; to be aware of",
            samples: vec![
                SampleSentence {
                    kanji: "すみません、知りません。",
                    english: "I'm sorry, I don't know.",
                },
                SampleSentence {
                    kanji: "私も知りたい！",
                    english: "I also want to know!",
                },
                SampleSentence {
                    kanji: "知らなかった？",
                    english: "Didn't you know that?",
                },
            ],
        },
        Word {
            kanji: "同じ",
            hiragana: "おな.じ",
            english: "same; identical",
            samples: vec![
                SampleSentence {
                    kanji: "同じ物を下さい。",
                    english: "Please give me the same thing.",
                },
                SampleSentence {
                    kanji: "彼は同じ用に話します。",
                    english: "He speaks in the same manner.",
                },
            ],
        },
        Word {
            kanji: "感じ",
            hiragana: "かん.じ",
            english: "feeling; sense; impression​",
            samples: vec![
                SampleSentence {
                    kanji: "何か感じる。",
                    english: "I feel something.",
                },
                SampleSentence {
                    kanji: "彼は感じがいい。",
                    english: "He has a good sense of feeling.",
                },
                SampleSentence {
                    kanji: "今同じ事を感じる。",
                    english: "Now I feel the same.",
                },
            ],
        },
        Word {
            kanji: "番",
            hiragana: "ばん",
            english: "number (in a series)​; (one's) turn",
            samples: vec![
                SampleSentence {
                    kanji: "私が一番。",
                    english: "I'm first.",
                },
                SampleSentence {
                    kanji: "君の番だよ。",
                    english: "It's your move.",
                },
                SampleSentence {
                    kanji: "だれの番だ。",
                    english: "Whose go is it?",
                },
                SampleSentence {
                    kanji: "その時私は非番だった。",
                    english: "I was off duty at the time.",
                },
            ],
        },
        Word {
            kanji: "必要",
            hiragana: "ひつ.よう",
            english: "necessary; needed; essential",
            samples: vec![
                SampleSentence {
                    kanji: "君が必要だ。",
                    english: "I need you.",
                },
                SampleSentence {
                    kanji: "私は水を必要です。",
                    english: "I need water.",
                },
                SampleSentence {
                    kanji: "必要ありません。",
                    english: "I don't need it.",
                },
            ],
        },
        Word {
            kanji: "仕事",
            hiragana: "し.ごと",
            english: "work; job; business",
            samples: vec![
                SampleSentence {
                    kanji: "仕事が必要だ。",
                    english: "I need a job.",
                },
                SampleSentence {
                    kanji: "それが仕事です。",
                    english: "It's my job.",
                },
                SampleSentence {
                    kanji: "仕事に行きなさい。",
                    english: "Go to work.",
                },
            ],
        },
        Word {
            kanji: "皆",
            hiragana: "みんな",
            english: "all; everyone",
            samples: vec![
                SampleSentence {
                    kanji: "皆さんはお元気？",
                    english: "How is everyone?",
                },
                SampleSentence {
                    kanji: "皆さんによろしく。",
                    english: "With kind regards to you all.",
                },
                SampleSentence {
                    kanji: "皆がそう言っている。",
                    english: "Everyone says that.",
                },
            ],
        },
        Word {
            kanji: "書く",
            hiragana: "か.く",
            english: "to write; to draw",
            samples: vec![
                SampleSentence {
                    kanji: "何書いてるの？",
                    english: "What are you writing?",
                },
                SampleSentence {
                    kanji: "書く物をくれ。",
                    english: "Give me something to write on.",
                },
                SampleSentence {
                    kanji: "何か書きなさい。",
                    english: "Write something.",
                },
            ],
        },
        Word {
            kanji: "次",
            hiragana: "つぎ",
            english: "next",
            samples: vec![
                SampleSentence {
                    kanji: "マリアは次です。",
                    english: "Maria is next.",
                },
                SampleSentence {
                    kanji: "次は気を付けてね。",
                    english: "Be more careful next time.",
                },
                SampleSentence {
                    kanji: "次の猫を持ってきてください！",
                    english: "Bring the next cat please!",
                },
            ],
        },
        Word {
            kanji: "結構",
            hiragana: "けっ.こう",
            english:
                "sufficient; fine (in the sense of \"I'm fine\"); (by implication) no thank you​",
            samples: vec![
                SampleSentence {
                    kanji: "結構です。",
                    english: "I'm fine, thank you.",
                },
                SampleSentence {
                    kanji: "それだけで結構です。",
                    english: "That is all.",
                },
                SampleSentence {
                    kanji: "それで結構だと思います。",
                    english: "That would be fine.",
                },
            ],
        },
        Word {
            kanji: "強い",
            hiragana: "つよ.い",
            english: "strong",
            samples: vec![
                SampleSentence {
                    kanji: "風が強くなった。",
                    english: "The wind grew stronger.",
                },
                SampleSentence {
                    kanji: "彼女は気が強い。",
                    english: "She's strong-willed.",
                },
                SampleSentence {
                    kanji: "彼は君より強い。",
                    english: "He's stronger than you.",
                },
            ],
        },
        Word {
            kanji: "勉強",
            hiragana: "べん.きょう",
            english: "study​",
            samples: vec![
                SampleSentence {
                    kanji: "勉強しなさい。",
                    english: "Study!",
                },
                SampleSentence {
                    kanji: "パリで勉強したい。",
                    english: "I'd like to study in Paris.",
                },
                SampleSentence {
                    kanji: "勉強は何が好きですか。",
                    english: "What subjects do you like the best?",
                },
            ],
        },
        Word {
            kanji: "住む",
            hiragana: "すむ",
            english: "to live (of humans); to reside; to inhabit",
            samples: vec![
                SampleSentence {
                    kanji: "どこに住んでいますか？",
                    english: "Where do you live?",
                },
                SampleSentence {
                    kanji: "私はスウェーデンに住んでいます。",
                    english: "I live in Sweden.",
                },
                SampleSentence {
                    kanji: "彼女がどこに住んでいるか知っていますか？",
                    english: "Do you know where she lives?",
                },
            ],
        },
        */
        /*
        Word {
            kanji: "学校",
            hiragana: "がっ.こう",
            english: "school​",
            samples: vec![
                SampleSentence {
                    kanji: "猫は学校に行きません。",
                    english: "Cats do not go to school.",
                },
                SampleSentence {
                    kanji: "私は学校の食べ物が好きです。",
                    english: "I like the food in school.",
                },
            ],
        },
        Word {
            kanji: "ご飯",
            hiragana: "ご.はん",
            english: "cooked rice​; meal",
            samples: vec![],
        },
        Word {
            kanji: "晩ご飯",
            hiragana: "ばん.ご.はん",
            english: "dinner",
            samples: vec![],
        },
        Word {
            kanji: "朝ご飯",
            hiragana: "あさ.ご.はん",
            english: "breakfast",
            samples: vec![],
        },
        Word {
            kanji: "昼ご飯",
            hiragana: "ひる.ご.はん",
            english: "lunch",
            samples: vec![],
        },
        Word {
            kanji: "時間",
            hiragana: "じ.かん",
            english: "time; hours",
            samples: vec![
                SampleSentence {
                    kanji: "時間ある？",
                    english: "Do you have time?",
                },
                SampleSentence {
                    kanji: "寝る時間よ。",
                    english: "It's time to sleep.",
                },
                SampleSentence {
                    kanji: "お昼の時間です",
                    english: "It's time for lunch.",
                },
            ],
        },
        Word {
            kanji: "時々",
            hiragana: "とき.どき",
            english: "sometimes; at times​",
            samples: vec![
                SampleSentence {
                    kanji: "「時々」って何？",
                    english: "What do you mean by \"sometimes\"?",
                },
                SampleSentence {
                    kanji: "時々、猫は朝食に電池を食べます。",
                    english: "Sometimes the cat eats batteries for breakfast.",
                },
                SampleSentence {
                    kanji: "時々犬はご飯を食べます。",
                    english: "Sometimes the dog eats rice.",
                },
            ],
        },
        Word {
            kanji: "半分",
            hiragana: "はん.ぶん",
            english: "half",
            samples: vec![
                SampleSentence {
                    kanji: "私は半分の時間でできます。",
                    english: "I can do it in half the time.",
                },
                SampleSentence {
                    kanji: "昼ご飯の半分しか食べられませんでした。",
                    english: "I only had time to eat half my lunch.",
                },
                SampleSentence {
                    kanji: "私は晩ご飯に半分のお金を使いました。",
                    english: "I spent half my money on dinner.",
                },
            ],
        },
        Word {
            kanji: "大丈夫",
            hiragana: "だい.じょう.ぶ",
            english: "ok",
            samples: vec![
                SampleSentence {
                    kanji: "大丈夫ですか？",
                    english: "Are you ok?",
                },
                SampleSentence {
                    kanji: "彼女は大丈夫だと言った。",
                    english: "She said she would be ok.",
                },
                SampleSentence {
                    kanji: "犬を食べても大丈夫ですか？",
                    english: "Is it ok if I eat the dog?",
                },
            ],
        },
        Word {
            kanji: "学生",
            hiragana: "がく.せい",
            english: "student (esp. a university student)​",
            samples: vec![
                SampleSentence {
                    kanji: "彼女は学生です。",
                    english: "She is a student.",
                },
                SampleSentence {
                    kanji: "学生は昼ご飯を食べています。",
                    english: "The students are eating lunch.",
                },
                SampleSentence {
                    kanji: "学生は皆同じ用に見えます。",
                    english: "The students all look the same.",
                },
            ],
        },
        Word {
            kanji: "お金",
            hiragana: "お.かね",
            english: "money​",
            samples: vec![
                SampleSentence {
                    kanji: "お金が必要ですか？",
                    english: "Do you need money?",
                },
                SampleSentence {
                    kanji: "お金を見つけた。",
                    english: "I found the money.",
                },
                SampleSentence {
                    kanji: "お金があれば、猫に電池を入れます。",
                    english: "When I have money I will feed batteries to my cat.",
                },
            ],
        },
        Word {
            kanji: "親切",
            hiragana: "しん.せつ",
            english: "kind; gentle; friendly",
            samples: vec![
                SampleSentence {
                    kanji: "彼女は彼に親切です。",
                    english: "She's kind to him.",
                },
                SampleSentence {
                    kanji: "彼女は非常に親切だ。",
                    english: "She is very kind.",
                },
                SampleSentence {
                    kanji: "彼は親切な人です。",
                    english: "He's a good person.",
                },
            ],
        },
        Word {
            kanji: "難しい",
            hiragana: "むずか.しい",
            english: "difficult; hard; complicated",
            samples: vec![
                SampleSentence {
                    kanji: "難しすぎる。",
                    english: "It's too hard.",
                },
                SampleSentence {
                    kanji: "これは難しいです。",
                    english: "This is difficult.",
                },
                SampleSentence {
                    kanji: "日本語はそんなに難しくない。",
                    english: "Japanese is not that difficult.",
                },
            ],
        },
        Word {
            kanji: "明日",
            hiragana: "あ.した",
            english: "tomorrow​",
            samples: vec![
                SampleSentence {
                    kanji: "また明日ね！",
                    english: "See you tomorrow!",
                },
                SampleSentence {
                    kanji: "明日は寒くて風が強いでしょう。",
                    english: "It will be cold and windy tomorrow.",
                },
                SampleSentence {
                    kanji: "明日は注意してください、宿題はほとんど無理です。",
                    english: "Be careful tomorrow, the homework is almost impossible.",
                },
            ],
        },
        Word {
            kanji: "靴",
            hiragana: "くつ",
            english: "shoe; shoes",
            samples: vec![
                SampleSentence {
                    kanji: "これはエマの靴ですか？",
                    english: "Is this Emma's shoes?",
                },
                SampleSentence {
                    kanji: "この靴はどう思う？",
                    english: "What do you think of these shoes?",
                },
                SampleSentence {
                    kanji: "靴はありますか？",
                    english: "Do you have your shoes?",
                },
            ],
        },
        Word {
            kanji: "例えば",
            hiragana: "たと.えば",
            english: "for example",
            samples: vec![
                SampleSentence {
                    kanji: "例えば、これは一本のペンです",
                    english: "For example, this is a pen.",
                },
                SampleSentence {
                    kanji: "例えば、英語が好きですか。",
                    english: "For example, do you like English?",
                },
                SampleSentence {
                    kanji: "例：「犬は電池を食べるのが大好き」。",
                    english: "For example: \"The dog loves to eat batteries\".",
                },
            ],
        },
        Word {
            kanji: "問題",
            hiragana: "もんだい",
            english: "problem​; question (e.g. on a test)",
            samples: vec![
                SampleSentence {
                    kanji: "それが問題？",
                    english: "Is that a problem?",
                },
                SampleSentence {
                    kanji: "問題ないよ。",
                    english: "No problem!",
                },
                SampleSentence {
                    kanji: "それは問題です。",
                    english: "That is a problem.",
                },
            ],
        },
        */
        /*
        // 8
        Word {
            kanji: "目",
            hiragana: "め",
            english: "eye; eyesight",
            samples: vec![
                SampleSentence {
                    kanji: "目を使って見てください。",
                    english: "Use your eyes to see.",
                },
                SampleSentence {
                    kanji: "学生は目がないと問題です。",
                    english: "Students have problems if they have no eyes.",
                },
                SampleSentence {
                    kanji: "猫は良い目をあります。",
                    english: "Cats have good eyes.",
                },
            ],
        },
        Word {
            kanji: "眼鏡",
            hiragana: "め.がね",
            english: "glasses; eyeglasses",
            samples: vec![
                SampleSentence {
                    kanji: "彼女は眼鏡をかけています。",
                    english: "She is wearing glasses.",
                },
                SampleSentence {
                    kanji: "犬は眼鏡をかけません。",
                    english: "Dogs do not wear glasses.",
                },
                SampleSentence {
                    kanji: "眼鏡には電池は必要ありません",
                    english: "Glasses do not need batteries.",
                },
            ],
        },
        Word {
            kanji: "青い",
            hiragana: "あおい",
            english: "blue; azure",
            samples: vec![
                SampleSentence {
                    kanji: "彼女の目は青いです。",
                    english: "Her eyes are blue.",
                },
                SampleSentence {
                    kanji: "車は青です。",
                    english: "The car is blue.",
                },
                SampleSentence {
                    kanji: "猫は青くない。",
                    english: "The cat is not blue.",
                },
            ],
        },
        Word {
            kanji: "車",
            hiragana: "くるま",
            english: "car",
            samples: vec![
                SampleSentence {
                    kanji: "車が青い。",
                    english: "The car is blue.",
                },
                SampleSentence {
                    kanji: "その車は黒ではありません。",
                    english: "That car is not black.",
                },
                SampleSentence {
                    kanji: "車には電池があります。",
                    english: "Cars have batteries.",
                },
            ],
        },
        Word {
            kanji: "頃",
            hiragana: "ころ",
            english: "(approximate) time; around; about",
            samples: vec![
                SampleSentence {
                    kanji: "2時半頃出た。",
                    english: "I left around 2:30.",
                },
                SampleSentence {
                    kanji: "彼は二時頃来た。",
                    english: "He came at about two o'clock.",
                },
                SampleSentence {
                    kanji: "4時半頃。",
                    english: "About half past four.",
                },
                SampleSentence {
                    kanji: "8時頃。",
                    english: "About eight o'clock.",
                },
                SampleSentence {
                    kanji: "3時15分頃。",
                    english: "About a quarter past three.",
                },
                SampleSentence {
                    kanji: "5時から5分頃。",
                    english: "About five minutes after five o'clock.",
                },
            ],
        },
        Word {
            kanji: "上",
            hiragana: "うえ",
            english: "above; up; over",
            samples: vec![SampleSentence {
                kanji: "靴は本の上にあります。",
                english: "The shoes are above the book.",
            }],
        },
        Word {
            kanji: "他",
            hiragana: "ほか",
            english: "other (place, thing, person)",
            samples: vec![
                SampleSentence {
                    kanji: "他のものはないの？",
                    english: "Isn't there anything else?",
                },
                SampleSentence {
                    kanji: "他の車はありますか？",
                    english: "Is there another car?",
                },
                SampleSentence {
                    kanji: "他にないのか。",
                    english: "That's all you've got?",
                },
            ],
        },
        Word {
            kanji: "質問",
            hiragana: "しつ.もん",
            english: "question",
            samples: vec![
                SampleSentence {
                    kanji: "質問していい？",
                    english: "Can I ask you a question?",
                },
                SampleSentence {
                    kanji: "何か質問がありますか？",
                    english: "Do you have any questions?",
                },
                SampleSentence {
                    kanji: "常用漢字について質問があります。",
                    english: "I have a question about the Joyo kanji.",
                },
            ],
        },
        Word {
            kanji: "家",
            hiragana: "いえ",
            english: "house; residence; family",
            samples: vec![
                SampleSentence {
                    kanji: "家はどこ？",
                    english: "Where's your house?",
                },
                SampleSentence {
                    kanji: "家に来て。",
                    english: "Come home.",
                },
                SampleSentence {
                    kanji: "家に電話して！",
                    english: "Call home!",
                },
            ],
        },
        Word {
            kanji: "電話",
            hiragana: "でん.わ",
            english: "phone; phone call​",
            samples: vec![
                SampleSentence {
                    kanji: "後で電話して!",
                    english: "Call me later!",
                },
                SampleSentence {
                    kanji: "もう電話しないでください。",
                    english: "Don't call me anymore.",
                },
                SampleSentence {
                    kanji: "彼女は寝ているので、電話しないでください。",
                    english: "You shouldn't call her, she's sleeping.",
                },
            ],
        },
        Word {
            kanji: "違う",
            hiragana: "ちが.う",
            english: "to differ (from)",
            samples: vec![
                SampleSentence {
                    kanji: "違う！",
                    english: "No!",
                },
                SampleSentence {
                    kanji: "どうして？何が違うの？",
                    english: "Why? What's wrong?",
                },
                SampleSentence {
                    kanji: "それは間違った猫です！",
                    english: "That's the wrong cat!",
                },
                SampleSentence {
                    kanji: "私は知っていると思ったが、間違っていた。",
                    english: "I thought I knew but I was wrong.",
                },
            ],
        },
        Word {
            kanji: "受ける",
            hiragana: "う.ける",
            english: "to receive; to get​",
            samples: vec![
                SampleSentence {
                    kanji: "まあ受け取って。",
                    english: "Just take it.",
                },
                SampleSentence {
                    kanji: "彼女は自転車を受け取りました。",
                    english: "She received a bike.",
                },
                SampleSentence {
                    kanji: "子供は犬を受け取りました。",
                    english: "The kid received a dog.",
                },
            ],
        },
        */

        /*
        Word {
            kanji: "言葉",
            hiragana: "こと.ば",
            english: "word; words",
            samples: vec![
                SampleSentence {
                    kanji: "言葉にできない。",
                    english: "I don't know how to say it.",
                },
                SampleSentence {
                    kanji: "言葉を学ぶのは難しいです。",
                    english: "It is difficult to learn words.",
                },
                SampleSentence {
                    kanji: "この言葉は宿題ですか？",
                    english: "Is this word in the homework?",
                },
            ],
        },
        Word {
            kanji: "買う",
            hiragana: "か.う",
            english: "to buy; to purchase",
            samples: vec![
                SampleSentence {
                    kanji: "何買ったの？",
                    english: "What did you buy?",
                },
                SampleSentence {
                    kanji: "彼は車を買った。",
                    english: "He bought a car.",
                },
                SampleSentence {
                    kanji: "ジェニファーは猫を買いました。",
                    english: "Jennifer bought a cat.",
                },
            ],
        },
        Word {
            kanji: "手",
            hiragana: "て",
            english: "hand; arm",
            samples: vec![
                SampleSentence {
                    kanji: "手をあげろ！",
                    english: "Hands up!",
                },
                SampleSentence {
                    kanji: "私の手は青です。",
                    english: "My hands are blue.",
                },
                SampleSentence {
                    kanji: "メアリーは手を使って書いた。",
                    english: "Mary used her hands to write.",
                },
            ],
        },
        Word {
            kanji: "返す",
            hiragana: "かえ.す",
            english: "to return (something); to restore; to put back",
            samples: vec![
                SampleSentence {
                    kanji: "かばんを返せ。",
                    english: "Give me back my bag.",
                },
                SampleSentence {
                    kanji: "サムの猫を返しましたか？",
                    english: "Did you return Sam's cat?",
                },
                SampleSentence {
                    kanji: "先生の本を返しましたか？",
                    english: "Did you return the teacher's book?",
                },
            ],
        },
        Word {
            kanji: "掛ける",
            hiragana: "か.ける",
            english: "to put on (glasses, etc.); to hang up (e.g. a coat, a picture on the wall)",
            samples: vec![SampleSentence {
                kanji: "眼鏡を掛けて下さい。",
                english: "Please put on your glasses.",
            }],
        },
        Word {
            kanji: "終わる",
            hiragana: "お.わる",
            english: "to finish; to end",
            samples: vec![
                SampleSentence {
                    kanji: "宿題終わった？",
                    english: "Have you finished your homework?",
                },
                SampleSentence {
                    kanji: "良い本は終わりましたか？",
                    english: "Is the good book finished?",
                },
                SampleSentence {
                    kanji: "学校の前に本を終わるに注意して下さい。",
                    english: "Remember to finish the book before school.",
                },
            ],
        },
        Word {
            kanji: "意味",
            hiragana: "い.み",
            english: "meaning; significance; sense",
            samples: vec![
                SampleSentence {
                    kanji: "どういう意味？",
                    english: "What do you mean?",
                },
                SampleSentence {
                    kanji: "この言葉の意味は何ですか？",
                    english: "What is the meaning of this word?",
                },
                SampleSentence {
                    kanji: "「エスカレーター」とはどういう意味ですか？",
                    english: "What does \"escalator\" mean?",
                },
            ],
        },
        Word {
            kanji: "味",
            hiragana: "あじ",
            english: "flavor; taste​",
            samples: vec![
                SampleSentence {
                    kanji: "これの味はどうですか？",
                    english: "How is the taste of this?",
                },
                SampleSentence {
                    kanji: "ケーキの味が好きですか？",
                    english: "Do you like the taste of cake?",
                },
                SampleSentence {
                    kanji: "私はあなたが作った昼食の味が好きです。",
                    english: "I like the taste of the lunch you made.",
                },
            ],
        },
        Word {
            kanji: "最近",
            hiragana: "さい.きん",
            english: "recently; lately",
            samples: vec![
                SampleSentence {
                    kanji: "最近どう？",
                    english: "How have you been recently?",
                },
                SampleSentence {
                    kanji: "最近何してるの？",
                    english: "What are you up to these days?",
                },
                SampleSentence {
                    kanji: "最近何か映画見た？",
                    english: "Seen any movies lately?",
                },
            ],
        },
        Word {
            kanji: "最初",
            hiragana: "さい.しょ",
            english: "beginning; outset; first",
            samples: vec![
                SampleSentence {
                    kanji: "彼が最初に来た。",
                    english: "He was the first to come.",
                },
                SampleSentence {
                    kanji: "私が最初に行きます。",
                    english: "I'll go first.",
                },
                SampleSentence {
                    kanji: "最初のころを思い出す。",
                    english: "I remember the first time.",
                },
            ],
        },
        Word {
            kanji: "最後",
            hiragana: "さい.ご",
            english: "last; final; end; conclusion",
            samples: vec![
                SampleSentence {
                    kanji: "彼女が最後に来ました。",
                    english: "She came last.",
                },
                SampleSentence {
                    kanji: "最後の犬を食べました。",
                    english: "We ate the last dog.",
                },
                SampleSentence {
                    kanji: "最後のケーキは良くなかった。",
                    english: "The last cake was not good.",
                },
            ],
        },
        Word {
            kanji: "映画",
            hiragana: "えい.が",
            english: "movie; film",
            samples: vec![
                SampleSentence {
                    kanji: "映画を取りましたか?",
                    english: "Did you pick up a movie?",
                },
                SampleSentence {
                    kanji: "あなたの一番の映画は何ですか？",
                    english: "What is your favourite movie?",
                },
                SampleSentence {
                    kanji: "良い映画はありますか？",
                    english: "Do you have any good movies?",
                },
            ],
        },
        */

        /*
        Word {
            kanji: "新宿",
            hiragana: "しん.じゅく",
            english: "Shinjuku",
            samples: vec![
                SampleSentence {
                    kanji: "明日新宿に行きますか？",
                    english: "Are you going to Shinjuku tomorrow?",
                },
                SampleSentence {
                    kanji: "みんな新宿に来ます。",
                    english: "Everyone comes to Shinjuku.",
                },
                SampleSentence {
                    kanji: "新宿で彼女に会っています。",
                    english: "I'm meeting her in Shinjuku.",
                },
            ],
        },
        Word {
            kanji: "会う",
            hiragana: "あ.う",
            english: "to meet; to encounter; to see​",
            samples: vec![
                SampleSentence {
                    kanji: "また会おう！",
                    english: "See you later!",
                },
                SampleSentence {
                    kanji: "君に会いたい",
                    english: "I want to see you",
                },
                SampleSentence {
                    kanji: "明日3時頃に会いましょう。",
                    english: "Let's meet tomorrow about three o'clock.",
                },
            ],
        },
        Word {
            kanji: "東京",
            hiragana: "とう.きょう",
            english: "Tokyo",
            samples: vec![
                SampleSentence {
                    kanji: "東京にいますか？会いましょう！",
                    english: "Are you in Tokyo? Let's meet!",
                },
                SampleSentence {
                    kanji: "東京のどこに住んでいますか？",
                    english: "Where in Tokyo do you live?",
                },
                SampleSentence {
                    kanji: "東京にはあまり車がありません。",
                    english: "There are not so many cars in Tokyo.",
                },
            ],
        },
        Word {
            kanji: "西",
            hiragana: "にし",
            english: "west",
            samples: vec![
                SampleSentence {
                    kanji: "南大沢は新宿の西にあります。",
                    english: "Minamiosawa is west of Shinjuku.",
                },
                SampleSentence {
                    kanji: "橋本は南大沢の西にあります。",
                    english: "Hashimoto is west of Minamiosawa.",
                },
                SampleSentence {
                    kanji: "スウェーデンはフィンランドの西にあります。",
                    english: "Sweden is west of Finland.",
                },
            ],
        },
        Word {
            kanji: "東",
            hiragana: "ひがし",
            english: "east",
            samples: vec![
                SampleSentence {
                    kanji: "ドイツはフランスの東にあります。",
                    english: "Germany is east of France.",
                },
                SampleSentence {
                    kanji: "東京は東日本にあります。",
                    english: "Tokyo is in eastern Japan.",
                },
                SampleSentence {
                    kanji: "今は橋本の東にいます。",
                    english: "We are in east of Hashimoto now.",
                },
            ],
        },
        Word {
            kanji: "北",
            hiragana: "きた",
            english: "north",
            samples: vec![
                SampleSentence {
                    kanji: "スウェーデンは北ヨーロッパにあります。",
                    english: "Sweden is in northern Europe.",
                },
                SampleSentence {
                    kanji: "私は北日本が好きです。",
                    english: "I like northern Japan.",
                },
                SampleSentence {
                    kanji: "風は北から来ます。",
                    english: "The wind comes from the north.",
                },
            ],
        },
        Word {
            kanji: "南",
            hiragana: "みなみ",
            english: "south",
            samples: vec![
                SampleSentence {
                    kanji: "私は南が好き。",
                    english: "I like the south.",
                },
                SampleSentence {
                    kanji: "私たちはスウェーデンの南に住んでいました。",
                    english: "We lived in the south of Sweden.",
                },
            ],
        },
        Word {
            kanji: "南大沢",
            hiragana: "みなみ.おお.さわ",
            english: "Minami-Ōsawa",
            samples: vec![
                SampleSentence {
                    kanji: "今、私たちは南大沢に住んでいます。",
                    english: "Now we live in Minami-Ōsawa.",
                },
                SampleSentence {
                    kanji: "南大沢はいい場所です。",
                    english: "Minami-Ōsawa is a nice place.",
                },
                SampleSentence {
                    kanji: "これを南大沢で買いました。",
                    english: "I bought this in Minami-Ōsawa.",
                },
            ],
        },
        Word {
            kanji: "多摩",
            hiragana: "た.ま",
            english: "Tama",
            samples: vec![
                SampleSentence {
                    kanji: "多摩センターで映画を見に行きませんか？",
                    english: "Do you want to go see a movie in Tama Center?",
                },
                SampleSentence {
                    kanji: "最後は多摩に行くことです。",
                    english: "The last thing is to go to Tama.",
                },
                SampleSentence {
                    kanji: "多摩センターは南大沢の東にあります。",
                    english: "Tama Center is east of Minamiosawa.",
                },
            ],
        },
        Word {
            kanji: "橋本",
            hiragana: " はし.もと ",
            english: "Hashimoto",
            samples: vec![
                SampleSentence {
                    kanji: "橋本は南大沢の後です。",
                    english: "Hashimoto is after Minamiosawa.",
                },
                SampleSentence {
                    kanji: "橋本に行きます。",
                    english: "I am going to Hashimoto.",
                },
                SampleSentence {
                    kanji: "ああ、これは橋本じゃない！",
                    english: "Oh no, this is not Hashimoto!",
                },
            ],
        },
        Word {
            kanji: "左",
            hiragana: "ひだり",
            english: "left",
            samples: vec![
                SampleSentence {
                    kanji: "黒い本の左にある青い本を取ります。",
                    english: "Take the blue book that is left of the black one.",
                },
                SampleSentence {
                    kanji: "左車を返します。",
                    english: "Return the left car.",
                },
                SampleSentence {
                    kanji: "「足」の左にある言葉はどういう意味ですか？",
                    english: "What does this word to the left of \"foot\" mean?",
                },
            ],
        },
        Word {
            kanji: "右",
            hiragana: "みぎ",
            english: "right",
            samples: vec![
                SampleSentence {
                    kanji: "メアリーの左にいるのは誰ですか？",
                    english: "Who is that to the left of Mary?",
                },
                SampleSentence {
                    kanji: "気をつけて、私はあなたの右にいます。",
                    english: "Watch out, I'm on your right side.",
                },
                SampleSentence {
                    kanji: "右の本をください。",
                    english: "Give me the left book.",
                },
            ],
        },
        Word {
            kanji: "下",
            hiragana: "した",
            english: "under",
            samples: vec![
                SampleSentence {
                    kanji: "最後の本は犬の下にありました！",
                    english: "The last book was under the dog!",
                },
                SampleSentence {
                    kanji: "眼鏡は車の下にありました。",
                    english: "The glasses was under the car.",
                },
            ],
        },
        Word {
            kanji: "新しい",
            hiragana: "あたら.しい",
            english: "new",
            samples: vec![
                SampleSentence {
                    kanji: "これは新しい車です。",
                    english: "This is a new car.",
                },
                SampleSentence {
                    kanji: "新宿で新しい本を買いました。",
                    english: "I bought a new book in Shinjuku.",
                },
                SampleSentence {
                    kanji: "新しい猫を買いました。最後のは電池を食べませんでした。",
                    english: "We bought a new cat. The last one didn't eat batteries.",
                },
            ],
        },
        Word {
            kanji: "止まれ",
            hiragana: "と.まれ",
            english: "stop",
            samples: vec![
                SampleSentence {
                    kanji: "電池を食べないでください！",
                    english: "Please don't eat batteries!",
                },
                SampleSentence {
                    kanji: "そこで止まれ。",
                    english: "Stop there.",
                },
            ],
        },
        Word {
            kanji: "誰",
            hiragana: "だれ",
            english: "who",
            samples: vec![
                SampleSentence {
                    kanji: "誰がいますか？",
                    english: "Who's there?",
                },
                SampleSentence {
                    kanji: "誰がアイスクリームを食べたいですか！",
                    english: "Who wants to eat ice cream!",
                },
                SampleSentence {
                    kanji: "誰が青猫を取ったの？",
                    english: "Who took the blue cat?",
                },
            ],
        },
        */
        Word {
            kanji: "赤い",
            hiragana: "あかい",
            english: "red",
            samples: vec![],
        },
        Word {
            kanji: "明い",
            hiragana: "あかるい",
            english: "bright",
            samples: vec![],
        },
        Word {
            kanji: "秋",
            hiragana: "あき",
            english: "autumn",
            samples: vec![],
        },
        Word {
            kanji: "開ける",
            hiragana: "あける",
            english: "to open",
            samples: vec![],
        },
        Word {
            kanji: "上げる",
            hiragana: "あげる",
            english: "to give",
            samples: vec![],
        },
        Word {
            kanji: "朝",
            hiragana: "あさ",
            english: "morning",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あさって",
            english: "day after tomorrow",
            samples: vec![],
        },
        Word {
            kanji: "足",
            hiragana: "あし",
            english: "foot, leg",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あそこ",
            english: "over there",
            samples: vec![],
        },
        Word {
            kanji: "遊ぶ",
            hiragana: "あそぶ",
            english: "to play, to make a visit",
            samples: vec![],
        },
        Word {
            kanji: "暖かい",
            hiragana: "あたたかい",
            english: "warm",
            samples: vec![],
        },
        Word {
            kanji: "頭",
            hiragana: "あたま",
            english: "head",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あちら",
            english: "there",
            samples: vec![],
        },
        Word {
            kanji: "暑い",
            hiragana: "あつい",
            english: "hot",
            samples: vec![],
        },
        Word {
            kanji: "熱い",
            hiragana: "あつい",
            english: "hot to the touch",
            samples: vec![],
        },
        Word {
            kanji: "厚い",
            hiragana: "あつい",
            english: "kind, deep, thick",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あっち",
            english: "over there",
            samples: vec![],
        },
        Word {
            kanji: "兄",
            hiragana: "あに",
            english: "(humble) older brother",
            samples: vec![],
        },
        Word {
            kanji: "姉",
            hiragana: "あね",
            english: "(humble) older sister",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あの",
            english: "that over there",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あの",
            english: "um...",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "アパート",
            english: "apartment",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あびる",
            english: "to bathe, to shower",
            samples: vec![],
        },
        Word {
            kanji: "危ない",
            hiragana: "あぶない",
            english: "dangerous",
            samples: vec![],
        },
        Word {
            kanji: "甘い",
            hiragana: "あまい",
            english: "sweet",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あまり",
            english: "not very",
            samples: vec![],
        },
        Word {
            kanji: "雨",
            hiragana: "あめ",
            english: "rain",
            samples: vec![],
        },
        Word {
            kanji: "飴",
            hiragana: "あめ",
            english: "candy",
            samples: vec![],
        },
        Word {
            kanji: "洗う",
            hiragana: "あらう",
            english: "to wash",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ある",
            english: "to be,to have (used for inanimate objects)",
            samples: vec![],
        },
        Word {
            kanji: "歩く",
            hiragana: "あるく",
            english: "to walk",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "あれ",
            english: "that",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いい",
            english: "good",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いいえ",
            english: "no",
            samples: vec![],
        },
        /*
        Word {
            kanji: "家",
            hiragana: "いえ",
            english: "house",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いかが",
            english: "how",
            samples: vec![],
        },
        Word {
            kanji: "行く",
            hiragana: "いく",
            english: "to go",
            samples: vec![],
        },
        Word {
            kanji: "いくつ",
            hiragana: "how",
            english: "many?,how old?",
            samples: vec![],
        },
        Word {
            kanji: "いくら",
            hiragana: "how",
            english: "much?",
            samples: vec![],
        },
        Word {
            kanji: "池",
            hiragana: "いけ",
            english: "pond",
            samples: vec![],
        },
        Word {
            kanji: "医者",
            hiragana: "いしゃ",
            english: "medical doctor",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いす",
            english: "chair",
            samples: vec![],
        },
        Word {
            kanji: "忙しい",
            hiragana: "いそがしい",
            english: "busy, irritated",
            samples: vec![],
        },
        Word {
            kanji: "痛い",
            hiragana: "いたい",
            english: "painful",
            samples: vec![],
        },
        Word {
            kanji: "一",
            hiragana: "いち",
            english: "one",
            samples: vec![],
        },
        Word {
            kanji: "一日",
            hiragana: "いちにち",
            english: "first of the month",
            samples: vec![],
        },
        Word {
            kanji: "いちばん",
            hiragana: "best",
            english: ", first",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いつ",
            english: "when",
            samples: vec![],
        },
        Word {
            kanji: "五日",
            hiragana: "いつか",
            english: "five days, fifth day",
            samples: vec![],
        },
        Word {
            kanji: "一緒",
            hiragana: "いっしょ",
            english: "together",
            samples: vec![],
        },
        Word {
            kanji: "五つ",
            hiragana: "いつつ",
            english: "five",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いつも",
            english: "always",
            samples: vec![],
        },
        Word {
            kanji: "犬",
            hiragana: "いぬ",
            english: "dog",
            samples: vec![],
        },
        Word {
            kanji: "今",
            hiragana: "いま",
            english: "now",
            samples: vec![],
        },
        Word {
            kanji: "意味",
            hiragana: "いみ",
            english: "meaning",
            samples: vec![],
        },
        Word {
            kanji: "妹",
            hiragana: "いもうと",
            english: "(humble) younger sister",
            samples: vec![],
        },
        Word {
            kanji: "嫌",
            hiragana: "いや",
            english: "unpleasant",
            samples: vec![],
        },
        Word {
            kanji: "入口",
            hiragana: "いりぐち",
            english: "entrance",
            samples: vec![],
        },
        Word {
            kanji: "居る",
            hiragana: "いる",
            english: "to be, to have (used for people and",
            samples: vec![],
        },
        Word {
            kanji: "入れる",
            hiragana: "いれる",
            english: "to put in",
            samples: vec![],
        },
        Word {
            kanji: "色",
            hiragana: "いろ",
            english: "colour",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "いろいろ",
            english: "various",
            samples: vec![],
        },
        Word {
            kanji: "上",
            hiragana: "うえ",
            english: "on top of",
            samples: vec![],
        },
        Word {
            kanji: "後ろ",
            hiragana: "うしろ",
            english: "behind",
            samples: vec![],
        },
        Word {
            kanji: "薄い",
            hiragana: "うすい",
            english: "thin,weak",
            samples: vec![],
        },
        Word {
            kanji: "歌",
            hiragana: "うた",
            english: "song",
            samples: vec![],
        },
        Word {
            kanji: "歌う",
            hiragana: "うたう",
            english: "to sing",
            samples: vec![],
        },
        Word {
            kanji: "生まれる",
            hiragana: "うまれる",
            english: "to be born",
            samples: vec![],
        },
        Word {
            kanji: "海",
            hiragana: "うみ",
            english: "sea",
            samples: vec![],
        },
        Word {
            kanji: "売る",
            hiragana: "うる",
            english: "to sell",
            samples: vec![],
        },
        Word {
            kanji: "煩い",
            hiragana: "うるさい",
            english: "noisy, annoying",
            samples: vec![],
        },
        Word {
            kanji: "上着",
            hiragana: "うわぎ",
            english: "jacket",
            samples: vec![],
        },
        Word {
            kanji: "絵",
            hiragana: "え",
            english: "picture",
            samples: vec![],
        },
        Word {
            kanji: "映画",
            hiragana: "えいが",
            english: "movie",
            samples: vec![],
        },
        Word {
            kanji: "映画館",
            hiragana: "えいがかん",
            english: "cinema",
            samples: vec![],
        },
        Word {
            kanji: "英語",
            hiragana: "えいご",
            english: "English language",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ええ",
            english: "yes",
            samples: vec![],
        },
        Word {
            kanji: "駅",
            hiragana: "えき",
            english: "station",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "エレベーター",
            english: "elevator",
            samples: vec![],
        },
        Word {
            kanji: "鉛筆",
            hiragana: "えんぴつ",
            english: "pencil",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "おいしい",
            english: "delicious",
            samples: vec![],
        },
        Word {
            kanji: "多い",
            hiragana: "おおい",
            english: "many",
            samples: vec![],
        },
        Word {
            kanji: "大きい",
            hiragana: "おおきい",
            english: "big",
            samples: vec![],
        },
        Word {
            kanji: "大きな",
            hiragana: "おおきな",
            english: "big",
            samples: vec![],
        },
        Word {
            kanji: "大勢",
            hiragana: "おおぜい",
            english: "great number of people",
            samples: vec![],
        },
        Word {
            kanji: "お母さん",
            hiragana: "おかあさん",
            english: "(honorable) mother",
            samples: vec![],
        },
        Word {
            kanji: "お菓子",
            hiragana: "おかし",
            english: "sweets, candy",
            samples: vec![],
        },
        Word {
            kanji: "お金",
            hiragana: "おかね",
            english: "money",
            samples: vec![],
        },
        Word {
            kanji: "起きる",
            hiragana: "おきる",
            english: "to get up",
            samples: vec![],
        },
        Word {
            kanji: "置く",
            hiragana: "おく",
            english: "to put",
            samples: vec![],
        },
        Word {
            kanji: "奥さん",
            hiragana: "おくさん",
            english: "(honorable) wife",
            samples: vec![],
        },
        Word {
            kanji: "お酒",
            hiragana: "おさけ",
            english: "alcohol, rice wine",
            samples: vec![],
        },
        Word {
            kanji: "お皿",
            hiragana: "おさら",
            english: "plate, dish",
            samples: vec![],
        },
        Word {
            kanji: "伯父",
            hiragana: "叔父",
            english: "おじいさん grandfather, male senior citizen",
            samples: vec![],
        },
        Word {
            kanji: "教える",
            hiragana: "おしえる",
            english: "to teach, to tell",
            samples: vec![],
        },
        Word {
            kanji: "伯父",
            hiragana: "叔父",
            english: "おじさん uncle, middle aged gentleman",
            samples: vec![],
        },
        Word {
            kanji: "押す",
            hiragana: "おす",
            english: "to push, to stamp something",
            samples: vec![],
        },
        Word {
            kanji: "遅い",
            hiragana: "おそい",
            english: "late, slow",
            samples: vec![],
        },
        Word {
            kanji: "お茶",
            hiragana: "おちゃ",
            english: "green tea",
            samples: vec![],
        },
        Word {
            kanji: "お手洗い",
            hiragana: "おてあらい",
            english: "bathroom",
            samples: vec![],
        },
        Word {
            kanji: "お父さん",
            hiragana: "おとうさん",
            english: "(honorable) father",
            samples: vec![],
        },
        Word {
            kanji: "弟",
            hiragana: "おとうと",
            english: "younger brother",
            samples: vec![],
        },
        Word {
            kanji: "男",
            hiragana: "おとこ",
            english: "Man",
            samples: vec![],
        },
        Word {
            kanji: "男の子",
            hiragana: "おとこのこ",
            english: "Boy",
            samples: vec![],
        },
        Word {
            kanji: "一昨日",
            hiragana: "おととい",
            english: "day before yesterday",
            samples: vec![],
        },
        Word {
            kanji: "一昨年",
            hiragana: "おととし",
            english: "year before last",
            samples: vec![],
        },
        Word {
            kanji: "大人",
            hiragana: "おとな",
            english: "Adult",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "おなか",
            english: "stomach",
            samples: vec![],
        },
        Word {
            kanji: "同じ",
            hiragana: "おなじ",
            english: "same",
            samples: vec![],
        },
        Word {
            kanji: "お兄さん",
            hiragana: "おにいさん",
            english: "(honorable) older brother",
            samples: vec![],
        },
        Word {
            kanji: "お姉さん",
            hiragana: "おねえさん",
            english: "(honorable) older sister",
            samples: vec![],
        },
        Word {
            kanji: "おばあさん",
            hiragana: "grandmother",
            english: ", female senior-citizen",
            samples: vec![],
        },
        Word {
            kanji: "伯母さん",
            hiragana: "叔母さん",
            english: "おばさん Aunt",
            samples: vec![],
        },
        Word {
            kanji: "お風呂",
            hiragana: "おふろ",
            english: "Bath",
            samples: vec![],
        },
        Word {
            kanji: "お弁当",
            hiragana: "おべんとう",
            english: "boxed lunch",
            samples: vec![],
        },
        Word {
            kanji: "覚える",
            hiragana: "おぼえる",
            english: "to remember",
            samples: vec![],
        },
        Word {
            kanji: "おまわりさん",
            hiragana: "friendly",
            english: "term for policeman",
            samples: vec![],
        },
        Word {
            kanji: "重い",
            hiragana: "おもい",
            english: "Heavy",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "おもしろい",
            english: "Interesting",
            samples: vec![],
        },
        Word {
            kanji: "泳ぐ",
            hiragana: "およぐ",
            english: "to swim",
            samples: vec![],
        },
        Word {
            kanji: "降りる",
            hiragana: "おりる",
            english: "to get off, to descend",
            samples: vec![],
        },
        Word {
            kanji: "終る",
            hiragana: "おわる",
            english: "to finish",
            samples: vec![],
        },
        Word {
            kanji: "音楽",
            hiragana: "おんがく",
            english: "Music",
            samples: vec![],
        },
        Word {
            kanji: "女",
            hiragana: "おんな",
            english: "Woman",
            samples: vec![],
        },
        Word {
            kanji: "女の子",
            hiragana: "おんなのこ",
            english: "Girl",
            samples: vec![],
        },
        Word {
            kanji: "外国",
            hiragana: "がいこく",
            english: "foreign country",
            samples: vec![],
        },
        Word {
            kanji: "外国人",
            hiragana: "がいこくじん",
            english: "Foreigner",
            samples: vec![],
        },
        Word {
            kanji: "会社",
            hiragana: "かいしゃ",
            english: "Company",
            samples: vec![],
        },
        Word {
            kanji: "階段",
            hiragana: "かいだん",
            english: "Stairs",
            samples: vec![],
        },
        Word {
            kanji: "買い物",
            hiragana: "かいもの",
            english: "Shopping",
            samples: vec![],
        },
        Word {
            kanji: "買う",
            hiragana: "かう",
            english: "to buy",
            samples: vec![],
        },
        Word {
            kanji: "返す",
            hiragana: "かえす",
            english: "to return something",
            samples: vec![],
        },
        Word {
            kanji: "帰る",
            hiragana: "かえる",
            english: "to go back",
            samples: vec![],
        },
        Word {
            kanji: "かかる",
            hiragana: "to",
            english: "take time or money",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "かぎ",
            english: "Key",
            samples: vec![],
        },
        Word {
            kanji: "書く",
            hiragana: "かく",
            english: "to write",
            samples: vec![],
        },
        Word {
            kanji: "学生",
            hiragana: "がくせい",
            english: "Student",
            samples: vec![],
        },
        Word {
            kanji: "かける",
            hiragana: "to",
            english: "call by phone",
            samples: vec![],
        },
        Word {
            kanji: "傘",
            hiragana: "かさ",
            english: "Umbrella",
            samples: vec![],
        },
        Word {
            kanji: "貸す",
            hiragana: "かす",
            english: "to lend",
            samples: vec![],
        },
        Word {
            kanji: "風",
            hiragana: "かぜ",
            english: "Wind",
            samples: vec![],
        },
        Word {
            kanji: "風邪",
            hiragana: "かぜ",
            english: "a cold",
            samples: vec![],
        },
        Word {
            kanji: "家族",
            hiragana: "かぞく",
            english: "Family",
            samples: vec![],
        },
        Word {
            kanji: "方",
            hiragana: "かた",
            english: "person, way of doing",
            samples: vec![],
        },
        Word {
            kanji: "学校",
            hiragana: "がっこう",
            english: "School",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "カップ",
            english: "Cup",
            samples: vec![],
        },
        Word {
            kanji: "家庭",
            hiragana: "かてい",
            english: "Household",
            samples: vec![],
        },
        Word {
            kanji: "角",
            hiragana: "かど",
            english: "a corner",
            samples: vec![],
        },
        Word {
            kanji: "かばん",
            hiragana: "bag",
            english: ", basket",
            samples: vec![],
        },
        Word {
            kanji: "花瓶",
            hiragana: "かびん",
            english: "a vase",
            samples: vec![],
        },
        Word {
            kanji: "紙",
            hiragana: "かみ",
            english: "paper",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "カメラ",
            english: "camera",
            samples: vec![],
        },
        Word {
            kanji: "火曜日",
            hiragana: "かようび",
            english: "Tuesday",
            samples: vec![],
        },
        Word {
            kanji: "辛い",
            hiragana: "からい",
            english: "Spicy",
            samples: vec![],
        },
        Word {
            kanji: "体",
            hiragana: "からだ",
            english: "Body",
            samples: vec![],
        },
        Word {
            kanji: "借りる",
            hiragana: "かりる",
            english: "to borrow",
            samples: vec![],
        },
        Word {
            kanji: "軽い",
            hiragana: "かるい",
            english: "Light",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "カレー",
            english: "Curry",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "カレンダー",
            english: "calendar",
            samples: vec![],
        },
        Word {
            kanji: "川",
            hiragana: "河",
            english: "かわ River",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "かわいい",
            english: "Cute",
            samples: vec![],
        },
        Word {
            kanji: "漢字",
            hiragana: "かんじ",
            english: "Chinese character",
            samples: vec![],
        },
        Word {
            kanji: "木",
            hiragana: "き",
            english: "tree, wood",
            samples: vec![],
        },
        Word {
            kanji: "黄色",
            hiragana: "きいろ",
            english: "yellow",
            samples: vec![],
        },
        Word {
            kanji: "黄色い",
            hiragana: "きいろい",
            english: "yellow",
            samples: vec![],
        },
        Word {
            kanji: "消える",
            hiragana: "きえる",
            english: "to disappear",
            samples: vec![],
        },
        Word {
            kanji: "聞く",
            hiragana: "きく",
            english: "to hear, to listen to, to ask",
            samples: vec![],
        },
        Word {
            kanji: "北",
            hiragana: "きた",
            english: "North",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ギター",
            english: "Guitar",
            samples: vec![],
        },
        Word {
            kanji: "汚い",
            hiragana: "きたない",
            english: "Dirty",
            samples: vec![],
        },
        Word {
            kanji: "喫茶店",
            hiragana: "きっさてん",
            english: "coffee lounge",
            samples: vec![],
        },
        Word {
            kanji: "切手",
            hiragana: "きって",
            english: "postage stamp",
            samples: vec![],
        },
        Word {
            kanji: "切符",
            hiragana: "きっぷ",
            english: "Ticket",
            samples: vec![],
        },
        Word {
            kanji: "昨日",
            hiragana: "きのう",
            english: "Yesterday",
            samples: vec![],
        },
        Word {
            kanji: "九",
            hiragana: "きゅう",
            english: "/ く Nine",
            samples: vec![],
        },
        Word {
            kanji: "牛肉",
            hiragana: "ぎゅうにく",
            english: "Beef",
            samples: vec![],
        },
        Word {
            kanji: "牛乳",
            hiragana: "ぎゅうにゅう",
            english: "Milk",
            samples: vec![],
        },
        Word {
            kanji: "今日",
            hiragana: "きょう",
            english: "Today",
            samples: vec![],
        },
        Word {
            kanji: "教室",
            hiragana: "きょうしつ",
            english: "Classroom",
            samples: vec![],
        },
        Word {
            kanji: "兄弟",
            hiragana: "きょうだい",
            english: "(humble) siblings",
            samples: vec![],
        },
        Word {
            kanji: "去年",
            hiragana: "きょねん",
            english: "last year",
            samples: vec![],
        },
        Word {
            kanji: "嫌い",
            hiragana: "きらい",
            english: "Hate",
            samples: vec![],
        },
        Word {
            kanji: "切る",
            hiragana: "きる",
            english: "to cut",
            samples: vec![],
        },
        Word {
            kanji: "着る",
            hiragana: "きる",
            english: "to put on from the shoulders down",
            samples: vec![],
        },
        Word {
            kanji: "きれい",
            hiragana: "pretty",
            english: ", clean",
            samples: vec![],
        },
        Word {
            kanji: "キロ",
            hiragana: "キログラム",
            english: "Kilogram",
            samples: vec![],
        },
        Word {
            kanji: "キロ",
            hiragana: "キロメートル",
            english: "Kilometer",
            samples: vec![],
        },
        Word {
            kanji: "銀行",
            hiragana: "ぎんこう",
            english: "Bank",
            samples: vec![],
        },
        Word {
            kanji: "金曜日",
            hiragana: "きんようび",
            english: "Friday",
            samples: vec![],
        },
        Word {
            kanji: "薬",
            hiragana: "くすり",
            english: "medicine",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ください",
            english: "Please",
            samples: vec![],
        },
        Word {
            kanji: "果物",
            hiragana: "くだもの",
            english: "Fruit",
            samples: vec![],
        },
        Word {
            kanji: "口",
            hiragana: "くち",
            english: "mouth, opening",
            samples: vec![],
        },
        Word {
            kanji: "靴",
            hiragana: "くつ",
            english: "Shoes",
            samples: vec![],
        },
        Word {
            kanji: "靴下",
            hiragana: "くつした",
            english: "Socks",
            samples: vec![],
        },
        Word {
            kanji: "国",
            hiragana: "くに",
            english: "Country",
            samples: vec![],
        },
        Word {
            kanji: "曇り",
            hiragana: "くもり",
            english: "cloudy weather",
            samples: vec![],
        },
        Word {
            kanji: "曇る",
            hiragana: "くもる",
            english: "to become cloudy, to become dim",
            samples: vec![],
        },
        Word {
            kanji: "暗い",
            hiragana: "くらい",
            english: "Gloomy",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "クラス",
            english: "Class",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "グラム",
            english: "Gram",
            samples: vec![],
        },
        Word {
            kanji: "来る",
            hiragana: "くる",
            english: "to come",
            samples: vec![],
        },
        Word {
            kanji: "車",
            hiragana: "くるま",
            english: "car, vehicle",
            samples: vec![],
        },
        Word {
            kanji: "黒",
            hiragana: "くろ",
            english: "Black",
            samples: vec![],
        },
        Word {
            kanji: "黒い",
            hiragana: "くろい",
            english: "black",
            samples: vec![],
        },
        Word {
            kanji: "警官",
            hiragana: "けいかん",
            english: "policeman",
            samples: vec![],
        },
        Word {
            kanji: "今朝",
            hiragana: "けさ",
            english: "this morning",
            samples: vec![],
        },
        Word {
            kanji: "消す",
            hiragana: "けす",
            english: "to erase, to turn off power",
            samples: vec![],
        },
        Word {
            kanji: "結構",
            hiragana: "けっこう",
            english: "splendid, enough",
            samples: vec![],
        },
        Word {
            kanji: "結婚",
            hiragana: "けっこん",
            english: "Marriage",
            samples: vec![],
        },
        Word {
            kanji: "月曜日",
            hiragana: "げつようび",
            english: "Monday",
            samples: vec![],
        },
        Word {
            kanji: "玄関",
            hiragana: "げんかん",
            english: "entry hall",
            samples: vec![],
        },
        Word {
            kanji: "元気",
            hiragana: "げんき",
            english: "health, vitality",
            samples: vec![],
        },
        Word {
            kanji: "五",
            hiragana: "ご",
            english: "Five",
            samples: vec![],
        },
        Word {
            kanji: "公園",
            hiragana: "こうえん",
            english: "Park",
            samples: vec![],
        },
        Word {
            kanji: "交差点",
            hiragana: "こうさてん",
            english: "intersection",
            samples: vec![],
        },
        Word {
            kanji: "紅茶",
            hiragana: "こうちゃ",
            english: "black tea",
            samples: vec![],
        },
        Word {
            kanji: "交番",
            hiragana: "こうばん",
            english: "police box",
            samples: vec![],
        },
        Word {
            kanji: "声",
            hiragana: "こえ",
            english: "Voice",
            samples: vec![],
        },
        Word {
            kanji: "コート",
            hiragana: "coat",
            english: ", tennis court",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "コーヒー",
            english: "Coffee",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ここ",
            english: "Here",
            samples: vec![],
        },
        Word {
            kanji: "午後",
            hiragana: "ごご",
            english: "afternoon",
            samples: vec![],
        },
        Word {
            kanji: "九日",
            hiragana: "ここのか",
            english: "nine days, ninth day",
            samples: vec![],
        },
        Word {
            kanji: "九つ",
            hiragana: "ここのつ",
            english: "Nine",
            samples: vec![],
        },
        Word {
            kanji: "午前",
            hiragana: "ごぜん",
            english: "morning",
            samples: vec![],
        },
        Word {
            kanji: "答える",
            hiragana: "こたえる",
            english: "to answer",
            samples: vec![],
        },
        Word {
            kanji: "こちら",
            hiragana: "this",
            english: "person or way",
            samples: vec![],
        },
        Word {
            kanji: "こっち",
            hiragana: "this",
            english: "person or way",
            samples: vec![],
        },
        Word {
            kanji: "コップ",
            hiragana: "a",
            english: "glass",
            samples: vec![],
        },
        Word {
            kanji: "今年",
            hiragana: "ことし",
            english: "this year",
            samples: vec![],
        },
        Word {
            kanji: "言葉",
            hiragana: "ことば",
            english: "word, language",
            samples: vec![],
        },
        Word {
            kanji: "子供",
            hiragana: "こども",
            english: "Child",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "この",
            english: "This",
            samples: vec![],
        },
        Word {
            kanji: "御飯",
            hiragana: "ごはん",
            english: "cooked rice, meal",
            samples: vec![],
        },
        Word {
            kanji: "コピーする",
            hiragana: "to",
            english: "copy",
            samples: vec![],
        },
        Word {
            kanji: "困る",
            hiragana: "こまる",
            english: "to be worried",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "これ",
            english: "This",
            samples: vec![],
        },
        Word {
            kanji: "今月",
            hiragana: "こんげつ",
            english: "this month",
            samples: vec![],
        },
        Word {
            kanji: "今週",
            hiragana: "こんしゅう",
            english: "this week",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "こんな",
            english: "Such",
            samples: vec![],
        },
        Word {
            kanji: "今晩",
            hiragana: "こんばん",
            english: "this evening",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "さあ",
            english: "well…",
            samples: vec![],
        },
        Word {
            kanji: "財布",
            hiragana: "さいふ",
            english: "Wallet",
            samples: vec![],
        },
        Word {
            kanji: "魚",
            hiragana: "さかな",
            english: "Fish",
            samples: vec![],
        },
        Word {
            kanji: "先",
            hiragana: "さき",
            english: "the future, previous",
            samples: vec![],
        },
        Word {
            kanji: "咲く",
            hiragana: "さく",
            english: "to bloom",
            samples: vec![],
        },
        Word {
            kanji: "作文",
            hiragana: "さくぶん",
            english: "composition, writing",
            samples: vec![],
        },
        Word {
            kanji: "差す",
            hiragana: "さす",
            english: "to stretch out hands, to raise an",
            samples: vec![],
        },
        Word {
            kanji: "雑誌",
            hiragana: "ざっし",
            english: "Magazine",
            samples: vec![],
        },
        Word {
            kanji: "砂糖",
            hiragana: "さとう",
            english: "Sugar",
            samples: vec![],
        },
        Word {
            kanji: "寒い",
            hiragana: "さむい",
            english: "Cold",
            samples: vec![],
        },
        Word {
            kanji: "さ来年",
            hiragana: "さらいねん",
            english: "year after next",
            samples: vec![],
        },
        Word {
            kanji: "三",
            hiragana: "さん",
            english: "Three",
            samples: vec![],
        },
        Word {
            kanji: "散歩",
            hiragana: "さんぽする",
            english: "to stroll",
            samples: vec![],
        },
        Word {
            kanji: "四",
            hiragana: "し",
            english: "/ よん Four",
            samples: vec![],
        },
        Word {
            kanji: "塩",
            hiragana: "しお",
            english: "Salt",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "しかし",
            english: "However",
            samples: vec![],
        },
        Word {
            kanji: "時間",
            hiragana: "じかん",
            english: "Time",
            samples: vec![],
        },
        Word {
            kanji: "仕事",
            hiragana: "しごと",
            english: "Job",
            samples: vec![],
        },
        Word {
            kanji: "辞書",
            hiragana: "じしょ",
            english: "Dictionary",
            samples: vec![],
        },
        Word {
            kanji: "静か",
            hiragana: "しずか",
            english: "Quiet",
            samples: vec![],
        },
        Word {
            kanji: "下",
            hiragana: "した",
            english: "Below",
            samples: vec![],
        },
        Word {
            kanji: "七",
            hiragana: "しち",
            english: "/ なな Seven",
            samples: vec![],
        },
        Word {
            kanji: "質問",
            hiragana: "しつもん",
            english: "Question",
            samples: vec![],
        },
        Word {
            kanji: "自転車",
            hiragana: "じてんしゃ",
            english: "Bicycle",
            samples: vec![],
        },
        Word {
            kanji: "自動車",
            hiragana: "じどうしゃ",
            english: "Automobile",
            samples: vec![],
        },
        Word {
            kanji: "死ぬ",
            hiragana: "しぬ",
            english: "to die",
            samples: vec![],
        },
        Word {
            kanji: "字引",
            hiragana: "じびき",
            english: "Dictionary",
            samples: vec![],
        },
        Word {
            kanji: "自分",
            hiragana: "じぶん",
            english: "Oneself",
            samples: vec![],
        },
        Word {
            kanji: "閉まる",
            hiragana: "しまる",
            english: "to close, to be closed",
            samples: vec![],
        },
        Word {
            kanji: "閉める",
            hiragana: "しめる",
            english: "to close something",
            samples: vec![],
        },
        Word {
            kanji: "締める",
            hiragana: "しめる",
            english: "to tie",
            samples: vec![],
        },
        Word {
            kanji: "じゃ",
            hiragana: "じゃあ",
            english: "well then…",
            samples: vec![],
        },
        Word {
            kanji: "写真",
            hiragana: "しゃしん",
            english: "photograph",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "シャツ",
            english: "Shirt",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "シャワー",
            english: "Shower",
            samples: vec![],
        },
        Word {
            kanji: "十",
            hiragana: "じゅう",
            english: "とお Ten",
            samples: vec![],
        },
        Word {
            kanji: "授業",
            hiragana: "じゅぎょう",
            english: "lesson, class work",
            samples: vec![],
        },
        Word {
            kanji: "宿題",
            hiragana: "しゅくだい",
            english: "homework",
            samples: vec![],
        },
        Word {
            kanji: "上手",
            hiragana: "じょうず",
            english: "Skillful",
            samples: vec![],
        },
        Word {
            kanji: "丈夫",
            hiragana: "じょうぶ",
            english: "strong, durable",
            samples: vec![],
        },
        Word {
            kanji: "しょうゆ",
            hiragana: "soy",
            english: "sauce",
            samples: vec![],
        },
        Word {
            kanji: "食堂",
            hiragana: "しょくどう",
            english: "dining hall",
            samples: vec![],
        },
        Word {
            kanji: "知る",
            hiragana: "しる",
            english: "to know",
            samples: vec![],
        },
        Word {
            kanji: "白",
            hiragana: "しろ",
            english: "White",
            samples: vec![],
        },
        Word {
            kanji: "白い",
            hiragana: "しろい",
            english: "White",
            samples: vec![],
        },
        Word {
            kanji: "新聞",
            hiragana: "しんぶん",
            english: "newspaper",
            samples: vec![],
        },
        Word {
            kanji: "水曜日",
            hiragana: "すいようび",
            english: "Wednesday",
            samples: vec![],
        },
        Word {
            kanji: "吸う",
            hiragana: "すう",
            english: "to smoke, to suck",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "スカート",
            english: "Skirt",
            samples: vec![],
        },
        Word {
            kanji: "好き",
            hiragana: "すき",
            english: "Likeable",
            samples: vec![],
        },
        Word {
            kanji: "少ない",
            hiragana: "すくない",
            english: "a few",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "すぐに",
            english: "Instantly",
            samples: vec![],
        },
        Word {
            kanji: "少し",
            hiragana: "すこし",
            english: "Few",
            samples: vec![],
        },
        Word {
            kanji: "涼しい",
            hiragana: "すずしい",
            english: "Refreshing",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ストーブ",
            english: "Heater",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "スプーン",
            english: "Spoon",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "スポーツ",
            english: "Sport",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ズボン",
            english: "Trousers",
            samples: vec![],
        },
        Word {
            kanji: "住む",
            hiragana: "すむ",
            english: "to live in",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "スリッパ",
            english: "Slippers",
            samples: vec![],
        },
        Word {
            kanji: "する",
            hiragana: "to",
            english: "do",
            samples: vec![],
        },
        Word {
            kanji: "座る",
            hiragana: "すわる",
            english: "to sit",
            samples: vec![],
        },
        Word {
            kanji: "背",
            hiragana: "せ",
            english: "height, stature",
            samples: vec![],
        },
        Word {
            kanji: "生徒",
            hiragana: "せいと",
            english: "Pupil",
            samples: vec![],
        },
        Word {
            kanji: "セーター",
            hiragana: "sweater",
            english: ", jumper",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "せっけん",
            english: "Economy",
            samples: vec![],
        },
        Word {
            kanji: "背広",
            hiragana: "せびろ",
            english: "business suit",
            samples: vec![],
        },
        Word {
            kanji: "狭い",
            hiragana: "せまい",
            english: "Narrow",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ゼロ",
            english: "Zero",
            samples: vec![],
        },
        Word {
            kanji: "千",
            hiragana: "せん",
            english: "Thousand",
            samples: vec![],
        },
        Word {
            kanji: "先月",
            hiragana: "せんげつ",
            english: "last month",
            samples: vec![],
        },
        Word {
            kanji: "先週",
            hiragana: "せんしゅう",
            english: "last week",
            samples: vec![],
        },
        Word {
            kanji: "先生",
            hiragana: "せんせい",
            english: "teacher, doctor",
            samples: vec![],
        },
        Word {
            kanji: "洗濯",
            hiragana: "せんたく",
            english: "Washing",
            samples: vec![],
        },
        Word {
            kanji: "全部",
            hiragana: "ぜんぶ",
            english: "All",
            samples: vec![],
        },
        Word {
            kanji: "掃除",
            hiragana: "そうじする",
            english: "to clean, to sweep",
            samples: vec![],
        },
        Word {
            kanji: "そうして",
            hiragana: "そして",
            english: "And",
            samples: vec![],
        },
        Word {
            kanji: "そこ",
            hiragana: "that",
            english: "place",
            samples: vec![],
        },
        Word {
            kanji: "そちら",
            hiragana: "over",
            english: "there",
            samples: vec![],
        },
        Word {
            kanji: "そっち",
            hiragana: "over",
            english: "there",
            samples: vec![],
        },
        Word {
            kanji: "外",
            hiragana: "そと",
            english: "Outside",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "その",
            english: "That",
            samples: vec![],
        },
        Word {
            kanji: "そば",
            hiragana: "near",
            english: ", beside",
            samples: vec![],
        },
        Word {
            kanji: "空",
            hiragana: "そら",
            english: "Sky",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "それ",
            english: "that",
            samples: vec![],
        },
        Word {
            kanji: "それから",
            hiragana: "after",
            english: "that",
            samples: vec![],
        },
        Word {
            kanji: "それでは",
            hiragana: "in",
            english: "that situation",
            samples: vec![],
        },
        Word {
            kanji: "大学",
            hiragana: "だいがく",
            english: "university",
            samples: vec![],
        },
        Word {
            kanji: "大使館",
            hiragana: "たいしかん",
            english: "embassy",
            samples: vec![],
        },
        Word {
            kanji: "大丈夫",
            hiragana: "だいじょうぶ",
            english: "all right",
            samples: vec![],
        },
        Word {
            kanji: "大好き",
            hiragana: "だいすき",
            english: "to be very likeable",
            samples: vec![],
        },
        Word {
            kanji: "大切",
            hiragana: "たいせつ",
            english: "important",
            samples: vec![],
        },
        Word {
            kanji: "台所",
            hiragana: "だいどころ",
            english: "kitchen",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "たいへん",
            english: "very",
            samples: vec![],
        },
        Word {
            kanji: "たいへん",
            hiragana: "difficult",
            english: "situation",
            samples: vec![],
        },
        Word {
            kanji: "高い",
            hiragana: "たかい",
            english: "tall, expensive",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "たくさん",
            english: "many",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "タクシー",
            english: "taxi",
            samples: vec![],
        },
        Word {
            kanji: "出す",
            hiragana: "だす",
            english: "to put out",
            samples: vec![],
        },
        Word {
            kanji: "立つ",
            hiragana: "たつ",
            english: "to stand",
            samples: vec![],
        },
        Word {
            kanji: "たて",
            hiragana: "length",
            english: ",height",
            samples: vec![],
        },
        Word {
            kanji: "建物",
            hiragana: "たてもの",
            english: "building",
            samples: vec![],
        },
        Word {
            kanji: "楽しい",
            hiragana: "たのしい",
            english: "enjoyable",
            samples: vec![],
        },
        Word {
            kanji: "頼む",
            hiragana: "たのむ",
            english: "to ask",
            samples: vec![],
        },
        Word {
            kanji: "たばこ",
            hiragana: "tobacco",
            english: ",cigarettes",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "たぶん",
            english: "probably",
            samples: vec![],
        },
        Word {
            kanji: "食べ物",
            hiragana: "たべもの",
            english: "food",
            samples: vec![],
        },
        Word {
            kanji: "食べる",
            hiragana: "たべる",
            english: "to eat",
            samples: vec![],
        },
        Word {
            kanji: "卵",
            hiragana: "たまご",
            english: "egg",
            samples: vec![],
        },
        Word {
            kanji: "誰",
            hiragana: "だれ",
            english: "who",
            samples: vec![],
        },
        Word {
            kanji: "誰",
            hiragana: "だれか",
            english: "somebody",
            samples: vec![],
        },
        Word {
            kanji: "誕生日",
            hiragana: "たんじょうび",
            english: "birthday",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "だんだん",
            english: "gradually",
            samples: vec![],
        },
        Word {
            kanji: "小さい",
            hiragana: "ちいさい",
            english: "little",
            samples: vec![],
        },
        Word {
            kanji: "小さな",
            hiragana: "ちいさな",
            english: "little",
            samples: vec![],
        },
        Word {
            kanji: "近い",
            hiragana: "ちかい",
            english: "near",
            samples: vec![],
        },
        Word {
            kanji: "違う",
            hiragana: "ちがう",
            english: "to differ",
            samples: vec![],
        },
        Word {
            kanji: "近く",
            hiragana: "ちかく",
            english: "near",
            samples: vec![],
        },
        Word {
            kanji: "地下鉄",
            hiragana: "ちかてつ",
            english: "underground train",
            samples: vec![],
        },
        Word {
            kanji: "地図",
            hiragana: "ちず",
            english: "map",
            samples: vec![],
        },
        Word {
            kanji: "茶色",
            hiragana: "ちゃいろ",
            english: "brown",
            samples: vec![],
        },
        Word {
            kanji: "ちゃわん",
            hiragana: "rice",
            english: "bowl",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ちょうど",
            english: "exactly",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ちょっと",
            english: "somewhat",
            samples: vec![],
        },
        Word {
            kanji: "一日",
            hiragana: "ついたち",
            english: "first of month",
            samples: vec![],
        },
        Word {
            kanji: "使う",
            hiragana: "つかう",
            english: "to use",
            samples: vec![],
        },
        Word {
            kanji: "疲れる",
            hiragana: "つかれる",
            english: "to get tired",
            samples: vec![],
        },
        Word {
            kanji: "次",
            hiragana: "つぎ",
            english: "next",
            samples: vec![],
        },
        Word {
            kanji: "着く",
            hiragana: "つく",
            english: "to arrive at",
            samples: vec![],
        },
        Word {
            kanji: "机",
            hiragana: "つくえ",
            english: "desk",
            samples: vec![],
        },
        Word {
            kanji: "作る",
            hiragana: "つくる",
            english: "to make",
            samples: vec![],
        },
        Word {
            kanji: "つける",
            hiragana: "to",
            english: "turn on",
            samples: vec![],
        },
        Word {
            kanji: "勤める",
            hiragana: "つとめる",
            english: "to work for someone",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "つまらない",
            english: "boring",
            samples: vec![],
        },
        Word {
            kanji: "冷たい",
            hiragana: "つめたい",
            english: "cold to the touch",
            samples: vec![],
        },
        Word {
            kanji: "強い",
            hiragana: "つよい",
            english: "powerful",
            samples: vec![],
        },
        Word {
            kanji: "手",
            hiragana: "て",
            english: "hand",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "テープ",
            english: "tape",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "テーブル",
            english: "table",
            samples: vec![],
        },
        Word {
            kanji: "テープレコーダー",
            hiragana: "tape",
            english: "recorder",
            samples: vec![],
        },
        Word {
            kanji: "出かける",
            hiragana: "でかける",
            english: "to go out",
            samples: vec![],
        },
        Word {
            kanji: "手紙",
            hiragana: "てがみ",
            english: "letter",
            samples: vec![],
        },
        Word {
            kanji: "できる",
            hiragana: "to",
            english: "be able to",
            samples: vec![],
        },
        Word {
            kanji: "出口",
            hiragana: "でぐち",
            english: "exit",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "テスト",
            english: "test",
            samples: vec![],
        },
        Word {
            kanji: "では",
            hiragana: "with",
            english: "that...",
            samples: vec![],
        },
        Word {
            kanji: "デパート",
            hiragana: "department",
            english: "store",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "でも",
            english: "but",
            samples: vec![],
        },
        Word {
            kanji: "出る",
            hiragana: "でる",
            english: "to appear,to leave",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "テレビ",
            english: "television",
            samples: vec![],
        },
        Word {
            kanji: "天気",
            hiragana: "てんき",
            english: "weather",
            samples: vec![],
        },
        Word {
            kanji: "電気",
            hiragana: "でんき",
            english: "electricity,electric light",
            samples: vec![],
        },
        Word {
            kanji: "電車",
            hiragana: "でんしゃ",
            english: "electric train",
            samples: vec![],
        },
        Word {
            kanji: "電話",
            hiragana: "でんわ",
            english: "telephone",
            samples: vec![],
        },
        Word {
            kanji: "戸",
            hiragana: "と",
            english: "Japanese style door",
            samples: vec![],
        },
        Word {
            kanji: "ドア",
            hiragana: "Western",
            english: "style door",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "トイレ",
            english: "toilet",
            samples: vec![],
        },
        Word {
            kanji: "どう",
            hiragana: "how",
            english: ",in what way",
            samples: vec![],
        },
        Word {
            kanji: "どうして",
            hiragana: "for",
            english: "what reason",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どうぞ",
            english: "please",
            samples: vec![],
        },
        Word {
            kanji: "動物",
            hiragana: "どうぶつ",
            english: "animal",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どうも",
            english: "thanks",
            samples: vec![],
        },
        Word {
            kanji: "遠い",
            hiragana: "とおい",
            english: "far",
            samples: vec![],
        },
        Word {
            kanji: "十日",
            hiragana: "とおか",
            english: "ten days,the tenth day",
            samples: vec![],
        },
        Word {
            kanji: "時々",
            hiragana: "ときどき",
            english: "sometimes",
            samples: vec![],
        },
        Word {
            kanji: "時計",
            hiragana: "とけい",
            english: "watch,clock",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どこ",
            english: "where",
            samples: vec![],
        },
        Word {
            kanji: "所",
            hiragana: "ところ",
            english: "place",
            samples: vec![],
        },
        Word {
            kanji: "年",
            hiragana: "とし",
            english: "year",
            samples: vec![],
        },
        Word {
            kanji: "図書館",
            hiragana: "としょかん",
            english: "library",
            samples: vec![],
        },
        Word {
            kanji: "どちら",
            hiragana: "which",
            english: "of two",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どっち",
            english: "which",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "とても",
            english: "very",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どなた",
            english: "who",
            samples: vec![],
        },
        Word {
            kanji: "隣",
            hiragana: "となり",
            english: "next door to",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "どの",
            english: "which",
            samples: vec![],
        },
        Word {
            kanji: "飛ぶ",
            hiragana: "とぶ",
            english: "to fly,to hop",
            samples: vec![],
        },
        Word {
            kanji: "止まる",
            hiragana: "とまる",
            english: "to come to a halt",
            samples: vec![],
        },
        Word {
            kanji: "友達",
            hiragana: "ともだち",
            english: "friend",
            samples: vec![],
        },
        Word {
            kanji: "土曜日",
            hiragana: "どようび",
            english: "Saturday",
            samples: vec![],
        },
        Word {
            kanji: "鳥",
            hiragana: "とり",
            english: "bird",
            samples: vec![],
        },
        Word {
            kanji: "とり肉",
            hiragana: "とりにく",
            english: "chicken meat",
            samples: vec![],
        },
        Word {
            kanji: "取る",
            hiragana: "とる",
            english: "to take something",
            samples: vec![],
        },
        Word {
            kanji: "撮る",
            hiragana: "とる",
            english: "to take a photo or record a film",
            samples: vec![],
        },
        Word {
            kanji: "どれ",
            hiragana: "which",
            english: "(of three or more)",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ナイフ",
            english: "knife",
            samples: vec![],
        },
        Word {
            kanji: "中",
            hiragana: "なか",
            english: "middle",
            samples: vec![],
        },
        Word {
            kanji: "長い",
            hiragana: "ながい",
            english: "long",
            samples: vec![],
        },
        Word {
            kanji: "鳴く",
            hiragana: "なく",
            english: "animal noise. to chirp, roar or croak etc.",
            samples: vec![],
        },
        Word {
            kanji: "無くす",
            hiragana: "なくす",
            english: "to lose something",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "なぜ",
            english: "why",
            samples: vec![],
        },
        Word {
            kanji: "夏",
            hiragana: "なつ",
            english: "summer",
            samples: vec![],
        },
        Word {
            kanji: "夏休み",
            hiragana: "なつやすみ",
            english: "summer holiday",
            samples: vec![],
        },
        Word {
            kanji: "など",
            hiragana: "et",
            english: "cetera",
            samples: vec![],
        },
        Word {
            kanji: "七つ",
            hiragana: "ななつ",
            english: "seven",
            samples: vec![],
        },
        Word {
            kanji: "七日",
            hiragana: "なのか",
            english: "seven days,the seventh day",
            samples: vec![],
        },
        Word {
            kanji: "名前",
            hiragana: "なまえ",
            english: "name",
            samples: vec![],
        },
        Word {
            kanji: "習う",
            hiragana: "ならう",
            english: "to learn",
            samples: vec![],
        },
        Word {
            kanji: "並ぶ",
            hiragana: "ならぶ",
            english: "to line up,to stand in a line",
            samples: vec![],
        },
        Word {
            kanji: "並べる",
            hiragana: "ならべる",
            english: "to line up,to set up",
            samples: vec![],
        },
        Word {
            kanji: "なる",
            hiragana: "to",
            english: "become",
            samples: vec![],
        },
        Word {
            kanji: "何",
            hiragana: "なん",
            english: "/なに what",
            samples: vec![],
        },
        Word {
            kanji: "二",
            hiragana: "に",
            english: "two",
            samples: vec![],
        },
        Word {
            kanji: "賑やか",
            hiragana: "にぎやか",
            english: "bustling,busy",
            samples: vec![],
        },
        Word {
            kanji: "肉",
            hiragana: "にく",
            english: "meat",
            samples: vec![],
        },
        Word {
            kanji: "西",
            hiragana: "にし",
            english: "west",
            samples: vec![],
        },
        Word {
            kanji: "日曜日",
            hiragana: "にちようび",
            english: "Sunday",
            samples: vec![],
        },
        Word {
            kanji: "荷物",
            hiragana: "にもつ",
            english: "luggage",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ニュース",
            english: "news",
            samples: vec![],
        },
        Word {
            kanji: "庭",
            hiragana: "にわ",
            english: "garden",
            samples: vec![],
        },
        Word {
            kanji: "脱ぐ",
            hiragana: "ぬぐ",
            english: "to take off clothes",
            samples: vec![],
        },
        Word {
            kanji: "温い",
            hiragana: "ぬるい",
            english: "luke warm",
            samples: vec![],
        },
        Word {
            kanji: "ネクタイ",
            hiragana: "tie",
            english: ",necktie",
            samples: vec![],
        },
        Word {
            kanji: "猫",
            hiragana: "ねこ",
            english: "cat",
            samples: vec![],
        },
        Word {
            kanji: "寝る",
            hiragana: "ねる",
            english: "to go to bed,to sleep",
            samples: vec![],
        },
        Word {
            kanji: "ノート",
            hiragana: "notebook",
            english: ",exercise book",
            samples: vec![],
        },
        Word {
            kanji: "登る",
            hiragana: "のぼる",
            english: "to climb",
            samples: vec![],
        },
        Word {
            kanji: "飲み物",
            hiragana: "のみもの",
            english: "a drink",
            samples: vec![],
        },
        Word {
            kanji: "飲む",
            hiragana: "のむ",
            english: "to drink",
            samples: vec![],
        },
        Word {
            kanji: "乗る",
            hiragana: "のる",
            english: "to get on,to ride",
            samples: vec![],
        },
        Word {
            kanji: "歯",
            hiragana: "は",
            english: "tooth",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "パーティー",
            english: "party",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "はい",
            english: "yes",
            samples: vec![],
        },
        Word {
            kanji: "灰皿",
            hiragana: "はいざら",
            english: "ashtray",
            samples: vec![],
        },
        Word {
            kanji: "入る",
            hiragana: "はいる",
            english: "to enter,to contain",
            samples: vec![],
        },
        Word {
            kanji: "葉書",
            hiragana: "はがき",
            english: "postcard",
            samples: vec![],
        },
        Word {
            kanji: "はく",
            hiragana: "to",
            english: "wear,to put on trousers",
            samples: vec![],
        },
        Word {
            kanji: "箱",
            hiragana: "はこ",
            english: "box",
            samples: vec![],
        },
        Word {
            kanji: "橋",
            hiragana: "はし",
            english: "bridge",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "はし",
            english: "chopsticks",
            samples: vec![],
        },
        Word {
            kanji: "始まる",
            hiragana: "はじまる",
            english: "to begin",
            samples: vec![],
        },
        Word {
            kanji: "初め",
            hiragana: "始め",
            english: "はじめ beginning",
            samples: vec![],
        },
        Word {
            kanji: "初めて",
            hiragana: "はじめて",
            english: "for the first time",
            samples: vec![],
        },
        Word {
            kanji: "走る",
            hiragana: "はしる",
            english: "to run",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "バス",
            english: "bus",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "バター",
            english: "butter",
            samples: vec![],
        },
        Word {
            kanji: "二十歳",
            hiragana: "はたち",
            english: "20 years old,20th year",
            samples: vec![],
        },
        Word {
            kanji: "働く",
            hiragana: "はたらく",
            english: "to work",
            samples: vec![],
        },
        Word {
            kanji: "八",
            hiragana: "はち",
            english: "eight",
            samples: vec![],
        },
        Word {
            kanji: "二十日",
            hiragana: "はつか",
            english: "twenty days,twentieth",
            samples: vec![],
        },
        Word {
            kanji: "花",
            hiragana: "はな",
            english: "flower",
            samples: vec![],
        },
        Word {
            kanji: "鼻",
            hiragana: "はな",
            english: "nose",
            samples: vec![],
        },
        Word {
            kanji: "話",
            hiragana: "はなし",
            english: "talk,story",
            samples: vec![],
        },
        Word {
            kanji: "話す",
            hiragana: "はなす",
            english: "to speak",
            samples: vec![],
        },
        Word {
            kanji: "早い",
            hiragana: "はやい",
            english: "early",
            samples: vec![],
        },
        Word {
            kanji: "速い",
            hiragana: "はやい",
            english: "quick",
            samples: vec![],
        },
        Word {
            kanji: "春",
            hiragana: "はる",
            english: "spring",
            samples: vec![],
        },
        Word {
            kanji: "貼る",
            hiragana: "はる",
            english: "to stick",
            samples: vec![],
        },
        Word {
            kanji: "晴れ",
            hiragana: "はれ",
            english: "clear weather",
            samples: vec![],
        },
        Word {
            kanji: "晴れる",
            hiragana: "はれる",
            english: "to be sunny",
            samples: vec![],
        },
        Word {
            kanji: "半",
            hiragana: "はん",
            english: "half",
            samples: vec![],
        },
        Word {
            kanji: "晩",
            hiragana: "ばん",
            english: "evening",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "パン",
            english: "bread",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ハンカチ",
            english: "handkerchief",
            samples: vec![],
        },
        Word {
            kanji: "番号",
            hiragana: "ばんごう",
            english: "number",
            samples: vec![],
        },
        Word {
            kanji: "晩御飯",
            hiragana: "ばんごはん",
            english: "evening meal",
            samples: vec![],
        },
        Word {
            kanji: "半分",
            hiragana: "はんぶん",
            english: "half minute",
            samples: vec![],
        },
        Word {
            kanji: "東",
            hiragana: "ひがし",
            english: "east",
            samples: vec![],
        },
        Word {
            kanji: "引く",
            hiragana: "ひく",
            english: "to pull",
            samples: vec![],
        },
        Word {
            kanji: "弾く",
            hiragana: "ひく",
            english: "to play an instrument with strings,",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "including",
            english: "piano",
            samples: vec![],
        },
        Word {
            kanji: "低い",
            hiragana: "ひくい",
            english: "short,low",
            samples: vec![],
        },
        Word {
            kanji: "飛行機",
            hiragana: "ひこうき",
            english: "aeroplane",
            samples: vec![],
        },
        Word {
            kanji: "左",
            hiragana: "ひだり",
            english: "left hand side",
            samples: vec![],
        },
        Word {
            kanji: "人",
            hiragana: "ひと",
            english: "person",
            samples: vec![],
        },
        Word {
            kanji: "一つ",
            hiragana: "ひとつ",
            english: "one",
            samples: vec![],
        },
        Word {
            kanji: "一月",
            hiragana: "ひとつき",
            english: "one month",
            samples: vec![],
        },
        Word {
            kanji: "一人",
            hiragana: "ひとり",
            english: "one person",
            samples: vec![],
        },
        Word {
            kanji: "暇",
            hiragana: "ひま",
            english: "free time",
            samples: vec![],
        },
        Word {
            kanji: "百",
            hiragana: "ひゃく",
            english: "hundred",
            samples: vec![],
        },
        Word {
            kanji: "病院",
            hiragana: "びょういん",
            english: "hospital",
            samples: vec![],
        },
        Word {
            kanji: "病気",
            hiragana: "びょうき",
            english: "illness",
            samples: vec![],
        },
        Word {
            kanji: "昼",
            hiragana: "ひる",
            english: "noon, daytime",
            samples: vec![],
        },
        Word {
            kanji: "昼御飯",
            hiragana: "ひるごはん",
            english: "midday meal",
            samples: vec![],
        },
        Word {
            kanji: "広い",
            hiragana: "ひろい",
            english: "spacious,wide",
            samples: vec![],
        },
        Word {
            kanji: "フィルム",
            hiragana: "roll",
            english: "of film",
            samples: vec![],
        },
        Word {
            kanji: "封筒",
            hiragana: "ふうとう",
            english: "envelope",
            samples: vec![],
        },
        Word {
            kanji: "プール",
            hiragana: "swimming",
            english: "pool",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "フォーク",
            english: "fork",
            samples: vec![],
        },
        Word {
            kanji: "吹く",
            hiragana: "ふく",
            english: "to blow",
            samples: vec![],
        },
        Word {
            kanji: "服",
            hiragana: "ふく",
            english: "clothes",
            samples: vec![],
        },
        Word {
            kanji: "二つ",
            hiragana: "ふたつ",
            english: "two",
            samples: vec![],
        },
        Word {
            kanji: "豚肉",
            hiragana: "ぶたにく",
            english: "pork",
            samples: vec![],
        },
        Word {
            kanji: "二人",
            hiragana: "ふたり",
            english: "two people",
            samples: vec![],
        },
        Word {
            kanji: "二日",
            hiragana: "ふつか",
            english: "two days, second day of the month",
            samples: vec![],
        },
        Word {
            kanji: "太い",
            hiragana: "ふとい",
            english: "fat",
            samples: vec![],
        },
        Word {
            kanji: "冬",
            hiragana: "ふゆ",
            english: "winter",
            samples: vec![],
        },
        Word {
            kanji: "降る",
            hiragana: "ふる",
            english: "to fall, e.g. rain or snow",
            samples: vec![],
        },
        Word {
            kanji: "古い",
            hiragana: "ふるい",
            english: "old (not used for people)",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ふろ",
            english: "bath",
            samples: vec![],
        },
        Word {
            kanji: "文章",
            hiragana: "ぶんしょう",
            english: "sentence,text",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ページ",
            english: "page",
            samples: vec![],
        },
        Word {
            kanji: "下手",
            hiragana: "へた",
            english: "unskillful",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ベッド",
            english: "bed",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ペット",
            english: "pet",
            samples: vec![],
        },
        Word {
            kanji: "部屋",
            hiragana: "へや",
            english: "room",
            samples: vec![],
        },
        Word {
            kanji: "辺",
            hiragana: "へん",
            english: "area",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ペン",
            english: "pen",
            samples: vec![],
        },
        Word {
            kanji: "勉強",
            hiragana: "べんきょうする",
            english: "to study",
            samples: vec![],
        },
        Word {
            kanji: "便利",
            hiragana: "べんり",
            english: "useful, convenient",
            samples: vec![],
        },
        Word {
            kanji: "帽子",
            hiragana: "ぼうし",
            english: "hat",
            samples: vec![],
        },
        Word {
            kanji: "ボールペン",
            hiragana: "ball",
            english: "-point pen",
            samples: vec![],
        },
        Word {
            kanji: "ほか",
            hiragana: "other",
            english: ", the rest",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ポケット",
            english: "pocket",
            samples: vec![],
        },
        Word {
            kanji: "欲しい",
            hiragana: "ほしい",
            english: "want",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ポスト",
            english: "post",
            samples: vec![],
        },
        Word {
            kanji: "細い",
            hiragana: "ほそい",
            english: "thin",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ボタン",
            english: "button",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ホテル",
            english: "hotel",
            samples: vec![],
        },
        Word {
            kanji: "本",
            hiragana: "ほん",
            english: "book",
            samples: vec![],
        },
        Word {
            kanji: "本棚",
            hiragana: "ほんだな",
            english: "bookshelves",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ほんとう",
            english: "truth",
            samples: vec![],
        },
        Word {
            kanji: "毎朝",
            hiragana: "まいあさ",
            english: "every morning",
            samples: vec![],
        },
        Word {
            kanji: "毎月",
            hiragana: "まいげつ",
            english: "/まいつき every month",
            samples: vec![],
        },
        Word {
            kanji: "毎週",
            hiragana: "まいしゅう",
            english: "every week",
            samples: vec![],
        },
        Word {
            kanji: "毎日",
            hiragana: "まいにち",
            english: "every day",
            samples: vec![],
        },
        Word {
            kanji: "毎年",
            hiragana: "まいねん",
            english: "/まいとし every year",
            samples: vec![],
        },
        Word {
            kanji: "毎晩",
            hiragana: "まいばん",
            english: "every night",
            samples: vec![],
        },
        Word {
            kanji: "前",
            hiragana: "まえ",
            english: "before",
            samples: vec![],
        },
        Word {
            kanji: "曲る",
            hiragana: "まがる",
            english: "to turn,to bend",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "まずい",
            english: "unpleasant",
            samples: vec![],
        },
        Word {
            kanji: "また",
            hiragana: "again",
            english: ",and",
            samples: vec![],
        },
        Word {
            kanji: "まだ",
            hiragana: "yet",
            english: ",still",
            samples: vec![],
        },
        Word {
            kanji: "町",
            hiragana: "まち",
            english: "town,city",
            samples: vec![],
        },
        Word {
            kanji: "待つ",
            hiragana: "まつ",
            english: "to wait",
            samples: vec![],
        },
        Word {
            kanji: "まっすぐ",
            hiragana: "straight",
            english: "ahead,direct",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "マッチ",
            english: "match",
            samples: vec![],
        },
        Word {
            kanji: "窓",
            hiragana: "まど",
            english: "window",
            samples: vec![],
        },
        Word {
            kanji: "丸い",
            hiragana: "円い",
            english: "まるい round,circular",
            samples: vec![],
        },
        Word {
            kanji: "万",
            hiragana: "まん",
            english: "ten thousand",
            samples: vec![],
        },
        Word {
            kanji: "万年筆",
            hiragana: "まんねんひつ",
            english: "fountain pen",
            samples: vec![],
        },
        Word {
            kanji: "磨く",
            hiragana: "みがく",
            english: "to brush teeth, to polish",
            samples: vec![],
        },
        Word {
            kanji: "右",
            hiragana: "みぎ",
            english: "right side",
            samples: vec![],
        },
        Word {
            kanji: "短い",
            hiragana: "みじかい",
            english: "short",
            samples: vec![],
        },
        Word {
            kanji: "水",
            hiragana: "みず",
            english: "water",
            samples: vec![],
        },
        Word {
            kanji: "店",
            hiragana: "みせ",
            english: "shop",
            samples: vec![],
        },
        Word {
            kanji: "見せる",
            hiragana: "みせる",
            english: "to show",
            samples: vec![],
        },
        Word {
            kanji: "道",
            hiragana: "みち",
            english: "street",
            samples: vec![],
        },
        Word {
            kanji: "三日",
            hiragana: "みっか",
            english: "three days, third day of the month",
            samples: vec![],
        },
        Word {
            kanji: "三つ",
            hiragana: "みっつ",
            english: "three",
            samples: vec![],
        },
        Word {
            kanji: "緑",
            hiragana: "みどり",
            english: "green",
            samples: vec![],
        },
        Word {
            kanji: "皆さん",
            hiragana: "みなさん",
            english: "everyone",
            samples: vec![],
        },
        Word {
            kanji: "南",
            hiragana: "みなみ",
            english: "south",
            samples: vec![],
        },
        Word {
            kanji: "耳",
            hiragana: "みみ",
            english: "ear",
            samples: vec![],
        },
        Word {
            kanji: "見る",
            hiragana: "観る",
            english: "みる to see, to watch",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "みんな",
            english: "everyone",
            samples: vec![],
        },
        Word {
            kanji: "六日",
            hiragana: "むいか",
            english: "six days, sixth day of the month",
            samples: vec![],
        },
        Word {
            kanji: "向こう",
            hiragana: "むこう",
            english: "over there",
            samples: vec![],
        },
        Word {
            kanji: "難しい",
            hiragana: "むずかしい",
            english: "difficult",
            samples: vec![],
        },
        Word {
            kanji: "六つ",
            hiragana: "むっつ",
            english: "six",
            samples: vec![],
        },
        Word {
            kanji: "村",
            hiragana: "むら",
            english: "village",
            samples: vec![],
        },
        Word {
            kanji: "目",
            hiragana: "め",
            english: "eye",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "メートル",
            english: "metre",
            samples: vec![],
        },
        Word {
            kanji: "眼鏡",
            hiragana: "めがね",
            english: "glasses",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "もう",
            english: "already",
            samples: vec![],
        },
        Word {
            kanji: "もう一度",
            hiragana: "もういちど",
            english: "again",
            samples: vec![],
        },
        Word {
            kanji: "木曜日",
            hiragana: "もくようび",
            english: "Thursday",
            samples: vec![],
        },
        Word {
            kanji: "持つ",
            hiragana: "もつ",
            english: "to hold",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "もっと",
            english: "more",
            samples: vec![],
        },
        Word {
            kanji: "物",
            hiragana: "もの",
            english: "thing",
            samples: vec![],
        },
        Word {
            kanji: "門",
            hiragana: "もん",
            english: "gate",
            samples: vec![],
        },
        Word {
            kanji: "問題",
            hiragana: "もんだい",
            english: "problem",
            samples: vec![],
        },
        Word {
            kanji: "八百屋",
            hiragana: "やおや",
            english: "greengrocer",
            samples: vec![],
        },
        Word {
            kanji: "野菜",
            hiragana: "やさい",
            english: "vegetable",
            samples: vec![],
        },
        Word {
            kanji: "易しい",
            hiragana: "やさしい",
            english: "easy, simple",
            samples: vec![],
        },
        Word {
            kanji: "安い",
            hiragana: "やすい",
            english: "cheap",
            samples: vec![],
        },
        Word {
            kanji: "休み",
            hiragana: "やすみ",
            english: "rest,holiday",
            samples: vec![],
        },
        Word {
            kanji: "休む",
            hiragana: "やすむ",
            english: "to rest",
            samples: vec![],
        },
        Word {
            kanji: "八つ",
            hiragana: "やっつ",
            english: "eight",
            samples: vec![],
        },
        Word {
            kanji: "山",
            hiragana: "やま",
            english: "mountain",
            samples: vec![],
        },
        Word {
            kanji: "やる",
            hiragana: "to",
            english: "do",
            samples: vec![],
        },
        Word {
            kanji: "夕方",
            hiragana: "ゆうがた",
            english: "evening",
            samples: vec![],
        },
        Word {
            kanji: "夕飯",
            hiragana: "ゆうはん",
            english: "dinner",
            samples: vec![],
        },
        Word {
            kanji: "郵便局",
            hiragana: "ゆうびんきょく",
            english: "post office",
            samples: vec![],
        },
        Word {
            kanji: "昨夜",
            hiragana: "ゆうべ",
            english: "last night",
            samples: vec![],
        },
        Word {
            kanji: "有名",
            hiragana: "ゆうめい",
            english: "famous",
            samples: vec![],
        },
        Word {
            kanji: "雪",
            hiragana: "ゆき",
            english: "snow",
            samples: vec![],
        },
        Word {
            kanji: "行く",
            hiragana: "ゆく",
            english: "to go",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ゆっくりと",
            english: "slowly",
            samples: vec![],
        },
        Word {
            kanji: "八日",
            hiragana: "ようか",
            english: "eight days, eighth day of the month",
            samples: vec![],
        },
        Word {
            kanji: "洋服",
            hiragana: "ようふく",
            english: "western-style clothes",
            samples: vec![],
        },
        Word {
            kanji: "よく",
            hiragana: "often",
            english: ", well",
            samples: vec![],
        },
        Word {
            kanji: "横",
            hiragana: "よこ",
            english: "beside,side,width",
            samples: vec![],
        },
        Word {
            kanji: "四日",
            hiragana: "よっか",
            english: "four days, fouth day of the month",
            samples: vec![],
        },
        Word {
            kanji: "四つ",
            hiragana: "よっつ",
            english: "four",
            samples: vec![],
        },
        Word {
            kanji: "呼ぶ",
            hiragana: "よぶ",
            english: "to call out,to invite",
            samples: vec![],
        },
        Word {
            kanji: "読む",
            hiragana: "よむ",
            english: "to read",
            samples: vec![],
        },
        Word {
            kanji: "夜",
            hiragana: "よる",
            english: "evening,night",
            samples: vec![],
        },
        Word {
            kanji: "弱い",
            hiragana: "よわい",
            english: "weak",
            samples: vec![],
        },
        Word {
            kanji: "来月",
            hiragana: "らいげつ",
            english: "next month",
            samples: vec![],
        },
        Word {
            kanji: "来週",
            hiragana: "らいしゅう",
            english: "next week",
            samples: vec![],
        },
        Word {
            kanji: "来年",
            hiragana: "らいねん",
            english: "next year",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ラジオ",
            english: "radio",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "ラジカセ",
            english: "/",
            samples: vec![],
        },
        Word {
            kanji: "ラジオカセット",
            hiragana: "radio",
            english: "cassette player",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "りっぱ",
            english: "splendid",
            samples: vec![],
        },
        Word {
            kanji: "留学生",
            hiragana: "りゅうがくせい",
            english: "overseas student",
            samples: vec![],
        },
        Word {
            kanji: "両親",
            hiragana: "りょうしん",
            english: "both parents",
            samples: vec![],
        },
        Word {
            kanji: "料理",
            hiragana: "りょうり",
            english: "cuisine",
            samples: vec![],
        },
        Word {
            kanji: "旅行",
            hiragana: "りょこう",
            english: "travel",
            samples: vec![],
        },
        Word {
            kanji: "零",
            hiragana: "れい",
            english: "zero",
            samples: vec![],
        },
        Word {
            kanji: "冷蔵庫",
            hiragana: "れいぞうこ",
            english: "refrigerator",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "レコード",
            english: "record",
            samples: vec![],
        },
        Word {
            kanji: "",
            hiragana: "レストラン",
            english: "restaurant",
            samples: vec![],
        },
        Word {
            kanji: "練習",
            hiragana: "れんしゅうする",
            english: "to practice",
            samples: vec![],
        },
        Word {
            kanji: "廊下",
            hiragana: "ろうか",
            english: "corridor",
            samples: vec![],
        },
        Word {
            kanji: "六",
            hiragana: "ろく",
            english: "six",
            samples: vec![],
        },
        Word {
            kanji: "ワイシャツ",
            hiragana: "business",
            english: "shirt",
            samples: vec![],
        },
        Word {
            kanji: "若い",
            hiragana: "わかい",
            english: "young",
            samples: vec![],
        },
        Word {
            kanji: "分かる",
            hiragana: "わかる",
            english: "to be understood",
            samples: vec![],
        },
        Word {
            kanji: "忘れる",
            hiragana: "わすれる",
            english: "to forget",
            samples: vec![],
        },
        Word {
            kanji: "私",
            hiragana: "わたくし",
            english: "(humble) I,myself",
            samples: vec![],
        },
        Word {
            kanji: "私",
            hiragana: "わたし",
            english: "I,myself",
            samples: vec![],
        },
        Word {
            kanji: "渡す",
            hiragana: "わたす",
            english: "to hand over",
            samples: vec![],
        },
        Word {
            kanji: "渡る",
            hiragana: "わたる",
            english: "to go across",
            samples: vec![],
        },
        Word {
            kanji: "悪い",
            hiragana: "わるい",
            english: "bad",
            samples: vec![],
        },
        */
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
