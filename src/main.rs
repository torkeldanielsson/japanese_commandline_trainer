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
        res = format!(
            "{}\\mbox{{{}}}\n = \n\\mbox{{{}}}\n = \n\\mbox{{{}}}\n, \\ \\ \\\n",
            res, word.kanji, word.hiragana, word.english
        );
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
