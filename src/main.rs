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

    res = format!("{}\\normalsize\n", res);
    res = format!("{}\\begin{{spacing}}{{2.1}}\n", res);

    for word in words {
        res = format!(
            "{}\\mbox{{{} = {} = {},}} \\ \\ \\ ",
            res, word.kanji, word.hiragana, word.english
        );
    }

    res = format!("{}\\end{{spacing}}\n", res);

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
                    english: "The cat sleeps. ",
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
                    english: "I'm busy. ",
                },
                SampleSentence {
                    kanji: "僕は寝たい。",
                    english: "I want to sleep. ",
                },
            ],
        },
        Word {
            kanji: "注意",
            hiragana: "ちゅう.い",
            english: "​",
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
            samples: vec![
                SampleSentence {
                    kanji: "足元に気をつけて。",
                    english: "Mind your step.",
                },
            ],
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
                    english: "Please give me some water. ",
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
                    english: "I am an English teacher. ",
                },
                SampleSentence {
                    kanji: "彼は英語がうまい。",
                    english: "He is such a good English speaker.",
                },
                SampleSentence {
                    kanji: "私は英語が話せる。",
                    english: "I can speak English. ",
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
                    kanji: "その猫は黒です。",
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
