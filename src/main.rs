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

    res = format!("{}\\large\n", res);

    res = format!("{}\\linespread{{2}}\n", res);

    let mut slice_index = 0;
    let mut lines = 0;

    let mut line_length = 9;
    if kanas.len() < line_length {
        line_length = kanas.len();
    }

    let h_slice: &mut [&Kana] = kanas.as_mut_slice();
    
    //rng.shuffle(h_slice);

    while lines < 16 {

        let mut chars_in_line = 0;

        while chars_in_line < line_length {
            res = format!("{}{}\\ \\ \\  ", res, h_slice[slice_index].c);

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

    // res = format!("{}\\par\n", res);

    res = format!("{}\\linespread{{1}}\n", res);

    res = format!("{}\\newpage\n", res);

    return res;
}

fn main() {

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(b"\\documentclass{article}\n").expect("1");
    file.write_all(b"\\usepackage[a4paper]{geometry}\n").expect("2");
    file.write_all(b"\\usepackage{setspace}\n").expect("2");
    file.write_all(b"\\pagenumbering{gobble}\n").expect("3");
    file.write_all(b"\\begin{document}\n").expect("4");

    let kanagroups = vec![
/*
        vec![
            Kana { c: "映画".to_owned(), romaji: "えいが = film".to_owned() },
            Kana { c: "音楽".to_owned(), romaji: "おんがく = musik".to_owned() },
            Kana { c: "新聞".to_owned(), romaji: "しんぶん = tidning (dagstidning)".to_owned() },
            Kana { c: "雑誌".to_owned(), romaji: "ざっし = tidning (magazine)".to_owned() },
            Kana { c: "朝御飯".to_owned(), romaji: "あさごはん = frukost".to_owned() },
            Kana { c: "昼御飯".to_owned(), romaji: "ひるごはん = lunch".to_owned() },
            Kana { c: "晩御飯".to_owned(), romaji: "ばんごはん = middag".to_owned() },
            Kana { c: "御飯".to_owned(), romaji: "ごはん = ris/måltid".to_owned() },
            Kana { c: "朝".to_owned(), romaji: "あさ = morgon".to_owned() },
            Kana { c: "今日".to_owned(), romaji: "きょう = idag".to_owned() },
            Kana { c: "今晩".to_owned(), romaji: "こんばん = ikväll".to_owned() },
            Kana { c: "週末".to_owned(), romaji: "しゅうまつ = helg".to_owned() },
            Kana { c: "毎日".to_owned(), romaji: "まいにち = varje dag".to_owned() },
            Kana { c: "毎晩".to_owned(), romaji: "まいばん = varje kväll".to_owned() },
            Kana { c: "アルバイト".to_owned(), romaji: "extra-jobb (t ex efter skolan)".to_owned() },
            Kana { c: "デパート".to_owned(), romaji: "varuhus (typ Åhléns)".to_owned() },
        ],
        
        vec![
            Kana { c: "行く".to_owned(), romaji: "いく = gå".to_owned() },
            Kana { c: "帰える".to_owned(), romaji: "かえる = återvända/komma tillbaka".to_owned() },
            Kana { c: "聞く".to_owned(), romaji: "きく = lyssna/höra".to_owned() },
            Kana { c: "飲む".to_owned(), romaji: "のむ = dricka".to_owned() },
            Kana { c: "話す".to_owned(), romaji: "はなす = prata".to_owned() },
            Kana { c: "読む".to_owned(), romaji: "よむ = läsa".to_owned() },
            Kana { c: "起きる".to_owned(), romaji: "おきる = stiga upp (~vakna)".to_owned() },
            Kana { c: "食べる".to_owned(), romaji: "たべる = äta".to_owned() },
        ],
        
        vec![
            Kana { c: "寝る".to_owned(), romaji: "ねる = sova".to_owned() },
            Kana { c: "見る".to_owned(), romaji: "みる = se/titta".to_owned() },
            Kana { c: "来る".to_owned(), romaji: "くる = komma".to_owned() },
            Kana { c: "する".to_owned(), romaji: "göra".to_owned() },
            Kana { c: "勉強する".to_owned(), romaji: "べんきょうする = plugga/studera".to_owned() },
            Kana { c: "早い".to_owned(), romaji: "はやい = tidig (tidigt)".to_owned() },
            Kana { c: "時々".to_owned(), romaji: "ときどき = ibland (då och då)".to_owned() },
            Kana { c: "テレビ".to_owned(), romaji: "teve".to_owned() },
            Kana { c: "コーヒー".to_owned(), romaji: "kaffe".to_owned() },
        ],
        
        vec![
            Kana { c: "お酒".to_owned(), romaji: "おさけ = sprit (alkohol)".to_owned() },
            Kana { c: "お茶".to_owned(), romaji: "おちゃ = grönt te".to_owned() },
            Kana { c: "水".to_owned(), romaji: "みず = vatten".to_owned() },
            Kana { c: "学校".to_owned(), romaji: "がっこう = skola".to_owned() },
            Kana { c: "スポーツ".to_owned(), romaji: "sport".to_owned() },
            Kana { c: "デート".to_owned(), romaji: "date".to_owned() },
            Kana { c: "うち".to_owned(), romaji: "hemma (hos mig)".to_owned() },
            Kana { c: "ホテル".to_owned(), romaji: "hotell".to_owned() },
            Kana { c: "レストラン".to_owned(), romaji: "resturang".to_owned() },
        ],
        
        vec![
            Kana { c: "猫".to_owned(), romaji: "ねこ = katt".to_owned() },
            Kana { c: "人".to_owned(), romaji: "ひと = person/folk/människor".to_owned() },
            Kana { c: "お寺".to_owned(), romaji: "おてら = tempel".to_owned() },
            Kana { c: "公園".to_owned(), romaji: "こうえん = park".to_owned() },
            Kana { c: "バス停".to_owned(), romaji: "バスてい = busshållplats".to_owned() },
            Kana { c: "病院".to_owned(), romaji: "びょういん = sjukhus".to_owned() },
            Kana { c: "本屋".to_owned(), romaji: "ほんや = bokhandel".to_owned() },
            Kana { c: "町".to_owned(), romaji: "まち = stad".to_owned() },
            Kana { c: "昨日".to_owned(), romaji: "きのう = igår".to_owned() },
            Kana { c: "さっき".to_owned(), romaji: "nyss".to_owned() },
        ],
        
        vec![
            Kana { c: "月曜日".to_owned(), romaji: "げつようび = måndag".to_owned() },
            Kana { c: "火曜日".to_owned(), romaji: "かようび = tisdag".to_owned() },
            Kana { c: "水曜日".to_owned(), romaji: "すいようび = onsdag".to_owned() },
            Kana { c: "木曜日".to_owned(), romaji: "もくようび = torsdag".to_owned() },
            Kana { c: "金曜日".to_owned(), romaji: "きんようび = fredag".to_owned() },
            Kana { c: "土曜日".to_owned(), romaji: "どようび = lördag".to_owned() },
            Kana { c: "日曜日".to_owned(), romaji: "にちようび = söndag".to_owned() },
            Kana { c: "買い物".to_owned(), romaji: "かいもの = shopping (saker man köper)".to_owned() },
        ],
        
        vec![
            Kana { c: "あなた".to_owned(), romaji: "du (använd helst inte!)".to_owned() },
            Kana { c: "犬".to_owned(), romaji: "いぬ = hund".to_owned() },
            Kana { c: "お土産".to_owned(), romaji: "おみやげ = souvenir".to_owned() },
            Kana { c: "子供".to_owned(), romaji: "こども = barn".to_owned() },
            Kana { c: "机".to_owned(), romaji: "つくえ = skrivbord".to_owned() },
            Kana { c: "手紙".to_owned(), romaji: "てがみ = brev".to_owned() },
            Kana { c: "クラス".to_owned(), romaji: "klass".to_owned() },
            Kana { c: "パン".to_owned(), romaji: "bröd".to_owned() },
            Kana { c: "スーパー".to_owned(), romaji: "stormarknad (typ ICA)".to_owned() },
        ],
        
        vec![
            Kana { c: "海".to_owned(), romaji: "うみ = hav".to_owned() },
            Kana { c: "切手".to_owned(), romaji: "きって = frimärke".to_owned() },
            Kana { c: "切符".to_owned(), romaji: "きっぷ = biljett".to_owned() },
            Kana { c: "サーフィング".to_owned(), romaji: "surfing".to_owned() },
            Kana { c: "バス".to_owned(), romaji: "bus".to_owned() },
            Kana { c: "宿題".to_owned(), romaji: "しゅくだい = läxa".to_owned() },
            Kana { c: "食べ物".to_owned(), romaji: "たべもの = mat".to_owned() },
            Kana { c: "誕生日".to_owned(), romaji: "たんじょうび = födelsedag".to_owned() },
            Kana { c: "テスト".to_owned(), romaji: "prov (test)".to_owned() },
            Kana { c: "天気".to_owned(), romaji: "てんき = väder".to_owned() },
            Kana { c: "飲み物".to_owned(), romaji: "のみもの = dryck".to_owned() },
            Kana { c: "葉書".to_owned(), romaji: "はがき = vykort".to_owned() },
            Kana { c: "飛行機".to_owned(), romaji: "ひこうき = flygplan".to_owned() },
            Kana { c: "部屋".to_owned(), romaji: "へや = rum".to_owned() },
            Kana { c: "僕".to_owned(), romaji: "ぼく = jag (manligt, informellt)".to_owned() },
        ],
        
        vec![
            Kana { c: "休み".to_owned(), romaji: "やすみ = lov/ledigt".to_owned() },
            Kana { c: "旅行".to_owned(), romaji: "りょこう = resa".to_owned() },
            Kana { c: "新しい".to_owned(), romaji: "あたらしい = ny".to_owned() },
            Kana { c: "暑い".to_owned(), romaji: "あつい = varmt (om vädret)".to_owned() },
            Kana { c: "熱い".to_owned(), romaji: "あつい = varmt (om en sak)".to_owned() },
            Kana { c: "忙しい".to_owned(), romaji: "いそがしい = upptagen".to_owned() },
            Kana { c: "大きい".to_owned(), romaji: "おおきい = stor".to_owned() },
            Kana { c: "面白い".to_owned(), romaji: "おもしろい = intressant/rolig".to_owned() },
        ],
        
        vec![
            Kana { c: "怖い".to_owned(), romaji: "こわい = läskig".to_owned() },
            Kana { c: "寒い".to_owned(), romaji: "さむい = kallt (om vädret)".to_owned() },
            Kana { c: "楽しい".to_owned(), romaji: "たのしい = roligt".to_owned() },
            Kana { c: "小さい".to_owned(), romaji: "ちいさい = liten".to_owned() },
            Kana { c: "つまらない".to_owned(), romaji: "tråkig".to_owned() },
            Kana { c: "古い".to_owned(), romaji: "ふるい = gammal".to_owned() },
            Kana { c: "難しい".to_owned(), romaji: "むずかしい = svår".to_owned() },
            Kana { c: "やさしい".to_owned(), romaji: "vänlig".to_owned() },
            Kana { c: "安い".to_owned(), romaji: "やすい = billig".to_owned() },
        ],
        
        vec![
            Kana { c: "嫌い".to_owned(), romaji: "きらい = ogilla".to_owned() },
            Kana { c: "きれい".to_owned(), romaji: "fin".to_owned() },
            Kana { c: "元気".to_owned(), romaji: "げんき = pigg/hälsosam".to_owned() },
            Kana { c: "静か".to_owned(), romaji: "しずか = tyst".to_owned() },
            Kana { c: "好き".to_owned(), romaji: "すき = gilla".to_owned() },
            Kana { c: "大嫌い".to_owned(), romaji: "だいきらい = hata".to_owned() },
            Kana { c: "大好き".to_owned(), romaji: "だいすき = älska".to_owned() },
            Kana { c: "にぎやか".to_owned(), romaji: "livlig".to_owned() },
            Kana { c: "ハンサム".to_owned(), romaji: "snygg".to_owned() },
            Kana { c: "暇".to_owned(), romaji: "ひま = inte upptagen/har mycket tid".to_owned() },
        ],
        
        vec![
            Kana { c: "泳ぐ".to_owned(), romaji: "およぐ = simma".to_owned() },
            Kana { c: "聞く".to_owned(), romaji: "きく = fråga (även lyssna)".to_owned() },
            Kana { c: "乗る".to_owned(), romaji: "のる = kliva ombord".to_owned() },
            Kana { c: "やる".to_owned(), romaji: "att göra (något)".to_owned() },
            Kana { c: "出かける".to_owned(), romaji: "でかける = kliva av (t ex en buss)".to_owned() },
            Kana { c: "一緒に".to_owned(), romaji: "いっしょに = tillsammans".to_owned() },
            Kana { c: "それから".to_owned(), romaji: "och sen".to_owned() },
            Kana { c: "大丈夫".to_owned(), romaji: "だいじょうぶ = det är ok/allt är som det ska/inget att oroa sig för".to_owned() },
            Kana { c: "とても".to_owned(), romaji: "väldigt/mycket".to_owned() },
            Kana { c: "どんな".to_owned(), romaji: "vilken sorts ...".to_owned() },
            Kana { c: "〜まで".to_owned(), romaji: "till (en plats/tid) / så långt som till (en plats)".to_owned() },
            Kana { c: "〜枚".to_owned(), romaji: "〜まい = räkneord för platta saker".to_owned() },
        ],
        
        vec![
            Kana { c: "〜時間".to_owned(), romaji: "〜じかん = ~ timmar".to_owned() },
            Kana { c: "一時間".to_owned(), romaji: "いちじかん = en timma".to_owned() },
            Kana { c: "先週".to_owned(), romaji: "せんしゅう = förra veckan".to_owned() },
            Kana { c: "時".to_owned(), romaji: "とき = när (tiden då något hände)".to_owned() },
            Kana { c: "会う".to_owned(), romaji: "あう = träffa (möta någon)".to_owned() },
            Kana { c: "ある".to_owned(), romaji: "finns".to_owned() },
            Kana { c: "買う".to_owned(), romaji: "かう = köpa".to_owned() },
            Kana { c: "書く".to_owned(), romaji: "かく = skriva".to_owned() },
            Kana { c: "写真".to_owned(), romaji: "しゃしん = fotografi".to_owned() },
            Kana { c: "写真を撮る".to_owned(), romaji: "しゃしんをとる = att ta kort/fota".to_owned() },
            Kana { c: "撮る".to_owned(), romaji: "とる = att ta (kort, för fotografi)".to_owned() },
            Kana { c: "待つ".to_owned(), romaji: "まつ = vänta".to_owned() },
            Kana { c: "いる".to_owned(), romaji: "är/befinner sig (används för personer)".to_owned() },
            Kana { c: "ごめんなさい".to_owned(), romaji: "förlåt".to_owned() },
            Kana { c: "一人で".to_owned(), romaji: "ひとりで = ensam".to_owned() },
        ],

        vec![
            Kana { c: "右".to_owned(), romaji: "みぎ = höger".to_owned() },
            Kana { c: "左".to_owned(), romaji: "ひだり = vänster".to_owned() },
            Kana { c: "前".to_owned(), romaji: "まえ = framför".to_owned() },
            Kana { c: "後ろ".to_owned(), romaji: "うしろ = bakom".to_owned() },
            Kana { c: "中".to_owned(), romaji: "なか = i/inuti".to_owned() },
            Kana { c: "上".to_owned(), romaji: "うえ = över".to_owned() },
            Kana { c: "下".to_owned(), romaji: "した = under".to_owned() },
            Kana { c: "そば".to_owned(), romaji: "nära".to_owned() },
            Kana { c: "隣".to_owned(), romaji: "となり = bredvid (granne med)".to_owned() },
            Kana { c: "間".to_owned(), romaji: "あいだ = mellan (AとBの〜)".to_owned() },
            Kana { c: "そこ".to_owned(), romaji: "där".to_owned() },
            Kana { c: "ここ".to_owned(), romaji: "här".to_owned() },
            Kana { c: "だから".to_owned(), romaji: "därför".to_owned() },
            Kana { c: "たくさん".to_owned(), romaji: "många/mycket".to_owned() },
            Kana { c: "どうして".to_owned(), romaji: "varför".to_owned() },
            Kana { c: "もしもし".to_owned(), romaji: "hej (i telefon)".to_owned() },
        ],

        vec![
            Kana { c: "成る".to_owned(), romaji: "なる = att bli".to_owned() },
            Kana { c: "年".to_owned(), romaji: "とし/ネン = år".to_owned() },
            Kana { c: "月".to_owned(), romaji: "つき/ゲツ、ガツ = månad".to_owned() },
            Kana { c: "から".to_owned(), romaji: "från".to_owned() },
            Kana { c: "日".to_owned(), romaji: "ひ/ニチ = dag".to_owned() },
            Kana { c: "事".to_owned(), romaji: "こと = sak".to_owned() },
            Kana { c: "ある(有る)".to_owned(), romaji: "att vara/är".to_owned() },
            Kana { c: "など".to_owned(), romaji: "mm (med mera) / etc (et cetera)".to_owned() },
            Kana { c: "言う".to_owned(), romaji: "いう = att säga (verb)".to_owned() },
            Kana { c: "日本".to_owned(), romaji: "Japan".to_owned() },
            Kana { c: "為".to_owned(), romaji: "ため = eftersom".to_owned() },
            Kana { c: "人".to_owned(), romaji: "ひと = person/folk/människor".to_owned() },
            Kana { c: "まで".to_owned(), romaji: "till".to_owned() },
            Kana { c: "又".to_owned(), romaji: "また = också".to_owned() },
            Kana { c: "行う".to_owned(), romaji: "おこなう = att göra (något)".to_owned() },
            Kana { c: "できる(出来る)".to_owned(), romaji: "fungera/att kunna göra (något)".to_owned() },
            Kana { c: "駅".to_owned(), romaji: "えき = (tåg)station".to_owned() },
            Kana { c: "国".to_owned(), romaji: "くに = land".to_owned() },
        ],

        vec![
            Kana { c: "大学".to_owned(), romaji: "だいがく = universitet".to_owned() },
            Kana { c: "現在".to_owned(), romaji: "げんざい = aktuellt/existerande/nuvarande".to_owned() },
            Kana { c: "後".to_owned(), romaji: "あと = senare/bakom".to_owned() },
            Kana { c: "線".to_owned(), romaji: "せん = (tåg)linje".to_owned() },
            Kana { c: "放送".to_owned(), romaji: "ほうそう = broadcast/utsändning".to_owned() },
            Kana { c: "号".to_owned(), romaji: "ごう = nummer".to_owned() },
            Kana { c: "軍".to_owned(), romaji: "ぐん = armé/trupper".to_owned() },
            Kana { c: "無い".to_owned(), romaji: "ない = ingen/inget".to_owned() },
            Kana { c: "部".to_owned(), romaji: "ぶ = del/avdelning".to_owned() },
            Kana { c: "持つ".to_owned(), romaji: " = att ha/hålla".to_owned() },
            Kana { c: "所".to_owned(), romaji: "もつ = ".to_owned() },
            Kana { c: "名前".to_owned(), romaji: "なまえ = namn".to_owned() },
        ],

        vec![
            Kana { c: "回".to_owned(), romaji: "かい = gånger/tillfälle".to_owned() },
            Kana { c: "世界".to_owned(), romaji: "せかい = världen".to_owned() },
            Kana { c: "時代".to_owned(), romaji: "じだい = period/epok/era".to_owned() },
            Kana { c: "東京".to_owned(), romaji: "とうきょう = Tokyo".to_owned() },
            Kana { c: "呼ぶ".to_owned(), romaji: "よぶ = att kalla på (någon)/samtal".to_owned() },
            Kana { c: "その後".to_owned(), romaji: "そのあと = efter det".to_owned() },
            Kana { c: "会".to_owned(), romaji: "かい = möte/sammankomst".to_owned() },
            Kana { c: "会社".to_owned(), romaji: "かいしゃ = företag".to_owned() },
            Kana { c: "機".to_owned(), romaji: "き = maskin/mekanism".to_owned() },
            Kana { c: "利用".to_owned(), romaji: "りよう = användning".to_owned() },
            Kana { c: "飛行機".to_owned(), romaji: "ひこうき = flygplan".to_owned() },
            Kana { c: "洞窟".to_owned(), romaji: "どうくつ = grotta".to_owned() },
        ],
*/
/*
        // easy news umeboshi pt 1
        vec![
            Kana { c: "梅干し".to_owned(), romaji: "うめぼし = saltade, torkade, plommon".to_owned() },
            Kana { c: "有名".to_owned(), romaji: "ゆうめい = berömd".to_owned() },
            Kana { c: "梅".to_owned(), romaji: "うめ = körsbär".to_owned() },
            Kana { c: "暑い".to_owned(), romaji: "あつい = varmt (väder)".to_owned() },
            Kana { c: "とても".to_owned(), romaji: "väldigt".to_owned() },
            Kana { c: "続いています".to_owned(), romaji: "つづいています = fortsätter".to_owned() },
            Kana { c: "塩分".to_owned(), romaji: "えんぶん = salt".to_owned() },
            Kana { c: "多い".to_owned(), romaji: "おおい = mycket".to_owned() },
            Kana { c: "食べて".to_owned(), romaji: "たべて = ät (grundform är 食べる)".to_owned() },
            Kana { c: "熱中症".to_owned(), romaji: "ねっちゅうしょう = värmeslag".to_owned() },
            Kana { c: "考える".to_owned(), romaji: "かんがえる = tänka".to_owned() },
            Kana { c: "人".to_owned(), romaji: "ひと = människa, person".to_owned() },
            Kana { c: "県".to_owned(), romaji: "けん = prefektur, lite som landskap".to_owned() },
            Kana { c: "市".to_owned(), romaji: "し = stad".to_owned() },
        ],

        // easy news umeboshi pt 2
        vec![
            Kana { c: "作る".to_owned(), romaji: "つくる = att göra, producera".to_owned() },
            Kana { c: "工場".to_owned(), romaji: "こうじょう = fabrik".to_owned() },
            Kana { c: "買いたい".to_owned(), romaji: "かいたい = vill köpa (något), grundform 買う".to_owned() },
            Kana { c: "注文".to_owned(), romaji: "ちゅうもん = beställning".to_owned() },
            Kana { c: "たくさん".to_owned(), romaji: "stor mängd, mycket".to_owned() },
            Kana { c: "足りる".to_owned(), romaji: "たりる = (att något är) tillräckligt".to_owned() },
            Kana { c: "別".to_owned(), romaji: "べつ = en annan, särskild".to_owned() },
            Kana { c: "仕事".to_owned(), romaji: "しごと = arbete".to_owned() },
            Kana { c: "手伝う".to_owned(), romaji: "てつだう = att hjälpa till".to_owned() },
            Kana { c: "午後９時まで".to_owned(), romaji: "ごごくじまで = fram till klockan 21 (午後 = p.m.)".to_owned() },
            Kana { c: "休みの日".to_owned(), romaji: "やすみのひ = helgdag/ledig dag".to_owned() },
            Kana { c: "皆さん".to_owned(), romaji: "みなさん = alla".to_owned() },
            Kana { c: "元気".to_owned(), romaji: "げんき = pigg, livfull, glad".to_owned() },
            Kana { c: "過ごしてください".to_owned(), romaji: "すごしてください = snälla spendera (den)".to_owned() },
            Kana { c: "話していました".to_owned(), romaji: "はなしていました = pratade".to_owned() },
        ],
*/

/*
        // easy news whale pt 1
        vec![
            Kana { c: "神奈川県".to_owned(), romaji: "かながわけん = Kanagawa prefektur".to_owned() },
            Kana { c: "鎌倉市".to_owned(), romaji: "かまくらし = Kamakura stad".to_owned() },
            Kana { c: "海岸".to_owned(), romaji: "かいがん = strand".to_owned() },
            Kana { c: "死んでいる".to_owned(), romaji: "しんでいる = död".to_owned() },
            Kana { c: "クジラ".to_owned(), romaji: "val".to_owned() },
            Kana { c: "見つかりました".to_owned(), romaji: "みつかりました = hittades".to_owned() },
            Kana { c: "国立科学博物館".to_owned(), romaji: "こくりつかがくはくぶつかん = naturhistoriska riksmuséet".to_owned() },
            Kana { c: "専門家".to_owned(), romaji: "せんもんか = specialist / expert".to_owned() },
            Kana { c: "体".to_owned(), romaji: "からだ = kropp".to_owned() },
            Kana { c: "調べる".to_owned(), romaji: "しらべる = att undersöka".to_owned() },
            Kana { c: "シロナガスクジラ".to_owned(), romaji: "(白長須鯨) = blåval".to_owned() },
            Kana { c: "だ".to_owned(), romaji: "informell form av です = är / att vara".to_owned() },
            Kana { c: "わかりました".to_owned(), romaji: "förstod (grundform: 分かる/わかる)".to_owned() },
            Kana { c: "地球".to_owned(), romaji: "ちきゅう = jorden (planeten)".to_owned() },
            Kana { c: "いちばん".to_owned(), romaji: "(一番) = nummer ett, den första".to_owned() },
            Kana { c: "大きい".to_owned(), romaji: "おおきい = stor".to_owned() },
        ],
*/
        // easy news whale pt 1
        vec![
            Kana { c: "動物".to_owned(), romaji: "どうぶつ = djur".to_owned() },
            Kana { c: "雄".to_owned(), romaji: "おす = manligt kön/hanne".to_owned() },
            Kana { c: "長さ".to_owned(), romaji: "ながさ = längd".to_owned() },
            Kana { c: "今年".to_owned(), romaji: "".to_owned() },
            Kana { c: "生まれた".to_owned(), romaji: "".to_owned() },
            Kana { c: "子ども".to_owned(), romaji: "(子供) = こども = barn".to_owned() },
            Kana { c: "考えています".to_owned(), romaji: "かんがえています = tror/tänker".to_owned() },
            Kana { c: "流れてきた".to_owned(), romaji: "ながれてきた = hitspolad (流れる = bortspolad)".to_owned() },
            Kana { c: "初めて".to_owned(), romaji: "はじめて = första gången (även: börja)".to_owned() },
            Kana { c: "だろう".to_owned(), romaji: "ser det ut som/tror man".to_owned() },
            Kana { c: "言っています".to_owned(), romaji: "いっています = säger (grundform: 言う)".to_owned() },
            Kana { c: "細かく".to_owned(), romaji: "こまかく = ".to_owned() },
            Kana { c: "原因".to_owned(), romaji: "げんいん = ursprung".to_owned() },
            Kana { c: "しっかり".to_owned(), romaji: "(確り) = ordentligt".to_owned() },
            Kana { c: "~なければならない".to_owned(), romaji: "~ det är inte ok att inte göra \"~\"".to_owned() },
            Kana { c: "思っています".to_owned(), romaji: "おもっています = tror (grundform 思う)".to_owned() },
        ],

/*
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
*/
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

    for group_size in vec![2, /* 2, 5, 10*/] {

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
