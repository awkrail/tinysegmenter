/**
TinySegmenter 0.1 -- Super compact Japanese tokenizer in Javascript
(c) 2008 Taku Kudo <taku@chasen.org>
TinySegmenter is freely distributable under the terms of a new BSD licence.
For details, see http://lilyx.net/pages/tinysegmenter_licence.txt
**/

/**
TinySegmenter in Rust is written and distributed by Taichi Nishimura
**/

use std::env;
use std::collections::HashMap;
use std::char;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

struct CharNgram<'a> {
    char_info: CharInfo<'a>,
    type_info: TypeInfo<'a>,
    pinfo: PInfo,
}

struct CharInfo<'a> {
    w1: &'a String,
    w2: &'a String,
    w3: &'a String,
    w4: &'a String,
    w5: &'a String,
    w6: &'a String,
    w2w3: [&'a String; 2],
    w3w4: [&'a String; 2],
    w4w5: [&'a String; 2],
    w5w6: [&'a String; 2],
    w1w2w3: [&'a String; 3],
    w2w3w4: [&'a String; 3],
    w3w4w5: [&'a String; 3],
    w4w5w6: [&'a String; 3],
}

struct TypeInfo<'a> {
    c1: &'a char,
    c2: &'a char,
    c3: &'a char,
    c4: &'a char,
    c5: &'a char,
    c6: &'a char,
    c2c3: [&'a char; 2],
    c3c4: [&'a char; 2],
    c4c5: [&'a char; 2],
    c5c6: [&'a char; 2],
    c1c2c3: [&'a char; 3],
    c2c3c4: [&'a char; 3],
    c3c4c5: [&'a char; 3],
    c4c5c6: [&'a char; 3],
}

struct PInfo {
    p1: char,
    p2: char,
    p3: char,
    p: char
}

fn construct_words(text: &String) -> Vec<char> {
    let mut chars : Vec<char> = Vec::new();
    for c in text.chars() {
        chars.push(c);
    }
    return chars
}

fn construct_hashmap() -> HashMap<char, char> {
    let char_patterns: Vec<((char, char), char)> = vec![
        (('一', '龠'), 'H'),
        (('ぁ', 'ん'), 'I'),
        (('ァ', 'ヴ'),  'K'),
        (('ｱ', 'ﾝ'), 'K'),
        (('a', 'z'), 'A'),
        (('A', 'Z'), 'A'),
        (('ａ', 'ｚ'), 'A'),
        (('Ａ', 'Ｚ'), 'A'),
        (('0', '9'), 'N'),
        (('０', '９'), 'N')
    ];

    let mut char_map = HashMap::new();
    for ((c_st, c_ed), tag_type) in char_patterns {
        let start : u32 = c_st.into();
        let end : u32 = c_ed.into();
        for c in start..end {
            let chr : char = char::from_u32(c).unwrap();
            char_map.insert(chr, tag_type);
        }
    }
    
    let other_chars = vec![
        ("一二三四五六七八九十百千万億兆", 'M'),
        ("々〆ヵヶ", 'H'),
        ("ーｰ\u{ff9e}", 'K')
    ];
    for (chars, tag_type) in other_chars {
        for c in chars.chars() {
            char_map.insert(c, tag_type);
        }
    }

    char_map
}

fn string(text: &str) -> String {
    String::from(text)
}

fn construct_scorer_hashmap() -> HashMap<String, HashMap<String, i32>> {
    let mut scorer_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    // Dumped from python obj to this format
    scorer_map.insert(string("BC1"), hashmap![string("HH") => 6,string("II") => 2461,string("KH") => 406,string("OH") => -1378]);
    scorer_map.insert(string("BC2"), hashmap![string("AA") => -3267,string("AI") => 2744,string("AN") => -878,string("HH") => -4070,string("HM") => -1711,string("HN") => 4012,string("HO") => 3761,string("IA") => 1327,string("IH") => -1184,string("II") => -1332,string("IK") => 1721,string("IO") => 5492,string("KI") => 3831,string("KK") => -8741,string("MH") => -3132,string("MK") => 3334,string("OO") => -2920]);
    scorer_map.insert(string("BC3"), hashmap![string("HH") => 996,string("HI") => 626,string("HK") => -721,string("HN") => -1307,string("HO") => -836,string("IH") => -301,string("KK") => 2762,string("MK") => 1079,string("MM") => 4034,string("OA") => -1652,string("OH") => 266]);
    scorer_map.insert(string("BP1"), hashmap![string("BB") => 295,string("OB") => 304,string("OO") => -125,string("UB") => 352]);
    scorer_map.insert(string("BP2"), hashmap![string("BO") => 60,string("OO") => -1762]);
    scorer_map.insert(string("BQ1"), hashmap![string("BHH") => 1150,string("BHM") => 1521,string("BII") => -1158,string("BIM") => 886,string("BMH") => 1208,string("BNH") => 449,string("BOH") => -91,string("BOO") => -2597,string("OHI") => 451,string("OIH") => -296,string("OKA") => 1851,string("OKH") => -1020,string("OKK") => 904,string("OOO") => 2965]);
    scorer_map.insert(string("BQ2"), hashmap![string("BHH") => 118,string("BHI") => -1159,string("BHM") => 466,string("BIH") => -919,string("BKK") => -1720,string("BKO") => 864,string("OHH") => -1139,string("OHM") => -181,string("OIH") => 153,string("UHI") => -1146]);
    scorer_map.insert(string("BW1"), hashmap![string(",と") => 660,string(",同") => 727,string("B1あ") => 1404,string("B1同") => 542,string("、と") => 660,string("、同") => 727,string("」と") => 1682,string("あっ") => 1505,string("いう") => 1743,string("いっ") => -2055,string("いる") => 672,string("うし") => -4817,string("うん") => 665,string("から") => 3472,string("がら") => 600,string("こう") => -790,string("こと") => 2083,string("こん") => -1262,string("さら") => -4143,string("さん") => 4573,string("した") => 2641,string("して") => 1104,string("すで") => -3399,string("そこ") => 1977,string("それ") => -871,string("たち") => 1122,string("ため") => 601,string("った") => 3463,string("つい") => -802,string("てい") => 805,string("てき") => 1249,string("でき") => 1127,string("です") => 3445,string("では") => 844,string("とい") => -4915,string("とみ") => 1922,string("どこ") => 3887,string("ない") => 5713,string("なっ") => 3015,string("など") => 7379,string("なん") => -1113,string("にし") => 2468,string("には") => 1498,string("にも") => 1671,string("に対") => -912,string("の一") => -501,string("の中") => 741,string("ませ") => 2448,string("まで") => 1711,string("まま") => 2600,string("まる") => -2155,string("やむ") => -1947,string("よっ") => -2565,string("れた") => 2369,string("れで") => -913,string("をし") => 1860,string("を見") => 731,string("亡く") => -1886,string("京都") => 2558,string("取り") => -2784,string("大き") => -2604,string("大阪") => 1497,string("平方") => -2314,string("引き") => -1336,string("日本") => -195,string("本当") => -2423,string("毎日") => -2113,string("目指") => -724,string("Ｂ１あ") => 1404,string("Ｂ１同") => 542,string("｣と") => 1682]);
    scorer_map.insert(string("BW2"), hashmap![string("..") => -11822,string("11") => -669,string("――") => -5730,string("−−") => -13175,string("いう") => -1609,string("うか") => 2490,string("かし") => -1350,string("かも") => -602,string("から") => -7194,string("かれ") => 4612,string("がい") => 853,string("がら") => -3198,string("きた") => 1941,string("くな") => -1597,string("こと") => -8392,string("この") => -4193,string("させ") => 4533,string("され") => 13168,string("さん") => -3977,string("しい") => -1819,string("しか") => -545,string("した") => 5078,string("して") => 972,string("しな") => 939,string("その") => -3744,string("たい") => -1253,string("たた") => -662,string("ただ") => -3857,string("たち") => -786,string("たと") => 1224,string("たは") => -939,string("った") => 4589,string("って") => 1647,string("っと") => -2094,string("てい") => 6144,string("てき") => 3640,string("てく") => 2551,string("ては") => -3110,string("ても") => -3065,string("でい") => 2666,string("でき") => -1528,string("でし") => -3828,string("です") => -4761,string("でも") => -4203,string("とい") => 1890,string("とこ") => -1746,string("とと") => -2279,string("との") => 720,string("とみ") => 5168,string("とも") => -3941,string("ない") => -2488,string("なが") => -1313,string("など") => -6509,string("なの") => 2614,string("なん") => 3099,string("にお") => -1615,string("にし") => 2748,string("にな") => 2454,string("によ") => -7236,string("に対") => -14943,string("に従") => -4688,string("に関") => -11388,string("のか") => 2093,string("ので") => -7059,string("のに") => -6041,string("のの") => -6125,string("はい") => 1073,string("はが") => -1033,string("はず") => -2532,string("ばれ") => 1813,string("まし") => -1316,string("まで") => -6621,string("まれ") => 5409,string("めて") => -3153,string("もい") => 2230,string("もの") => -10713,string("らか") => -944,string("らし") => -1611,string("らに") => -1897,string("りし") => 651,string("りま") => 1620,string("れた") => 4270,string("れて") => 849,string("れば") => 4114,string("ろう") => 6067,string("われ") => 7901,string("を通") => -11877,string("んだ") => 728,string("んな") => -4115,string("一人") => 602,string("一方") => -1375,string("一日") => 970,string("一部") => -1051,string("上が") => -4479,string("会社") => -1116,string("出て") => 2163,string("分の") => -7758,string("同党") => 970,string("同日") => -913,string("大阪") => -2471,string("委員") => -1250,string("少な") => -1050,string("年度") => -8669,string("年間") => -1626,string("府県") => -2363,string("手権") => -1982,string("新聞") => -4066,string("日新") => -722,string("日本") => -7068,string("日米") => 3372,string("曜日") => -601,string("朝鮮") => -2355,string("本人") => -2697,string("東京") => -1543,string("然と") => -1384,string("社会") => -1276,string("立て") => -990,string("第に") => -1612,string("米国") => -4268,string("１１") => -669,string("ｸﾞ") => 1319]);
    scorer_map.insert(string("BW3"), hashmap![string("あた") => -2194,string("あり") => 719,string("ある") => 3846,string("い.") => -1185,string("い。") => -1185,string("いい") => 5308,string("いえ") => 2079,string("いく") => 3029,string("いた") => 2056,string("いっ") => 1883,string("いる") => 5600,string("いわ") => 1527,string("うち") => 1117,string("うと") => 4798,string("えと") => 1454,string("か.") => 2857,string("か。") => 2857,string("かけ") => -743,string("かっ") => -4098,string("かに") => -669,string("から") => 6520,string("かり") => -2670,string("が,") => 1816,string("が、") => 1816,string("がき") => -4855,string("がけ") => -1127,string("がっ") => -913,string("がら") => -4977,string("がり") => -2064,string("きた") => 1645,string("けど") => 1374,string("こと") => 7397,string("この") => 1542,string("ころ") => -2757,string("さい") => -714,string("さを") => 976,string("し,") => 1557,string("し、") => 1557,string("しい") => -3714,string("した") => 3562,string("して") => 1449,string("しな") => 2608,string("しま") => 1200,string("す.") => -1310,string("す。") => -1310,string("する") => 6521,string("ず,") => 3426,string("ず、") => 3426,string("ずに") => 841,string("そう") => 428,string("た.") => 8875,string("た。") => 8875,string("たい") => -594,string("たの") => 812,string("たり") => -1183,string("たる") => -853,string("だ.") => 4098,string("だ。") => 4098,string("だっ") => 1004,string("った") => -4748,string("って") => 300,string("てい") => 6240,string("てお") => 855,string("ても") => 302,string("です") => 1437,string("でに") => -1482,string("では") => 2295,string("とう") => -1387,string("とし") => 2266,string("との") => 541,string("とも") => -3543,string("どう") => 4664,string("ない") => 1796,string("なく") => -903,string("など") => 2135,string("に,") => -1021,string("に、") => -1021,string("にし") => 1771,string("にな") => 1906,string("には") => 2644,string("の,") => -724,string("の、") => -724,string("の子") => -1000,string("は,") => 1337,string("は、") => 1337,string("べき") => 2181,string("まし") => 1113,string("ます") => 6943,string("まっ") => -1549,string("まで") => 6154,string("まれ") => -793,string("らし") => 1479,string("られ") => 6820,string("るる") => 3818,string("れ,") => 854,string("れ、") => 854,string("れた") => 1850,string("れて") => 1375,string("れば") => -3246,string("れる") => 1091,string("われ") => -605,string("んだ") => 606,string("んで") => 798,string("カ月") => 990,string("会議") => 860,string("入り") => 1232,string("大会") => 2217,string("始め") => 1681,string("市") => 965,string("新聞") => -5055,string("日,") => 974,string("日、") => 974,string("社会") => 2024,string("ｶ月") => 990]);
    scorer_map.insert(string("TC1"), hashmap![string("AAA") => 1093,string("HHH") => 1029,string("HHM") => 580,string("HII") => 998,string("HOH") => -390,string("HOM") => -331,string("IHI") => 1169,string("IOH") => -142,string("IOI") => -1015,string("IOM") => 467,string("MMH") => 187,string("OOI") => -1832]);
    scorer_map.insert(string("TC2"), hashmap![string("HHO") => 2088,string("HII") => -1023,string("HMM") => -1154,string("IHI") => -1965,string("KKH") => 703,string("OII") => -2649]);
    scorer_map.insert(string("TC3"), hashmap![string("AAA") => -294,string("HHH") => 346,string("HHI") => -341,string("HII") => -1088,string("HIK") => 731,string("HOH") => -1486,string("IHH") => 128,string("IHI") => -3041,string("IHO") => -1935,string("IIH") => -825,string("IIM") => -1035,string("IOI") => -542,string("KHH") => -1216,string("KKA") => 491,string("KKH") => -1217,string("KOK") => -1009,string("MHH") => -2694,string("MHM") => -457,string("MHO") => 123,string("MMH") => -471,string("NNH") => -1689,string("NNO") => 662,string("OHO") => -3393]);
    scorer_map.insert(string("TC4"), hashmap![string("HHH") => -203,string("HHI") => 1344,string("HHK") => 365,string("HHM") => -122,string("HHN") => 182,string("HHO") => 669,string("HIH") => 804,string("HII") => 679,string("HOH") => 446,string("IHH") => 695,string("IHO") => -2324,string("IIH") => 321,string("III") => 1497,string("IIO") => 656,string("IOO") => 54,string("KAK") => 4845,string("KKA") => 3386,string("KKK") => 3065,string("MHH") => -405,string("MHI") => 201,string("MMH") => -241,string("MMM") => 661,string("MOM") => 841]);
    scorer_map.insert(string("TQ1"), hashmap![string("BHHH") => -227,string("BHHI") => 316,string("BHIH") => -132,string("BIHH") => 60,string("BIII") => 1595,string("BNHH") => -744,string("BOHH") => 225,string("BOOO") => -908,string("OAKK") => 482,string("OHHH") => 281,string("OHIH") => 249,string("OIHI") => 200,string("OIIH") => -68]);
    scorer_map.insert(string("TQ2"), hashmap![string("BIHH") => -1401,string("BIII") => -1033,string("BKAK") => -543,string("BOOO") => -5591]);
    scorer_map.insert(string("TQ3"), hashmap![string("BHHH") => 478,string("BHHM") => -1073,string("BHIH") => 222,string("BHII") => -504,string("BIIH") => -116,string("BIII") => -105,string("BMHI") => -863,string("BMHM") => -464,string("BOMH") => 620,string("OHHH") => 346,string("OHHI") => 1729,string("OHII") => 997,string("OHMH") => 481,string("OIHH") => 623,string("OIIH") => 1344,string("OKAK") => 2792,string("OKHH") => 587,string("OKKA") => 679,string("OOHH") => 110,string("OOII") => -685]);
    scorer_map.insert(string("TQ4"), hashmap![string("BHHH") => -721,string("BHHM") => -3604,string("BHII") => -966,string("BIIH") => -607,string("BIII") => -2181,string("OAAA") => -2763,string("OAKK") => 180,string("OHHH") => -294,string("OHHI") => 2446,string("OHHO") => 480,string("OHIH") => -1573,string("OIHH") => 1935,string("OIHI") => -493,string("OIIH") => 626,string("OIII") => -4007,string("OKAK") => -8156]);
    scorer_map.insert(string("TW1"), hashmap![string("につい") => -4681,string("東京都") => 2026]);
    scorer_map.insert(string("TW2"), hashmap![string("ある程") => -2049,string("いった") => -1256,string("ころが") => -2434,string("しょう") => 3873,string("その後") => -4430,string("だって") => -1049,string("ていた") => 1833,string("として") => -4657,string("ともに") => -4517,string("もので") => 1882,string("一気に") => -792,string("初めて") => -1512,string("同時に") => -8097,string("大きな") => -1255,string("対して") => -2721,string("社会党") => -3216]);
    scorer_map.insert(string("TW3"), hashmap![string("いただ") => -1734,string("してい") => 1314,string("として") => -4314,string("につい") => -5483,string("にとっ") => -5989,string("に当た") => -6247,string("ので,") => -727,string("ので、") => -727,string("のもの") => -600,string("れから") => -3752,string("十二月") => -2287]);
    scorer_map.insert(string("TW4"), hashmap![string("いう.") => 8576,string("いう。") => 8576,string("からな") => -2348,string("してい") => 2958,string("たが,") => 1516,string("たが、") => 1516,string("ている") => 1538,string("という") => 1349,string("ました") => 5543,string("ません") => 1097,string("ようと") => -4258,string("よると") => 5865]);
    scorer_map.insert(string("UC1"), hashmap![string("A") => 484,string("K") => 93,string("M") => 645,string("O") => -505]);
    scorer_map.insert(string("UC2"), hashmap![string("A") => 819,string("H") => 1059,string("I") => 409,string("M") => 3987,string("N") => 5775,string("O") => 646]);
    scorer_map.insert(string("UC3"), hashmap![string("A") => -1370,string("I") => 2311]);
    scorer_map.insert(string("UC4"), hashmap![string("A") => -2643,string("H") => 1809,string("I") => -1032,string("K") => -3450,string("M") => 3565,string("N") => 3876,string("O") => 6646]);
    scorer_map.insert(string("UC5"), hashmap![string("H") => 313,string("I") => -1238,string("K") => -799,string("M") => 539,string("O") => -831]);
    scorer_map.insert(string("UC6"), hashmap![string("H") => -506,string("I") => -253,string("K") => 87,string("M") => 247,string("O") => -387]);
    scorer_map.insert(string("UP1"), hashmap![string("O") => -214]);
    scorer_map.insert(string("UP2"), hashmap![string("B") => 69,string("O") => 935]);
    scorer_map.insert(string("UP3"), hashmap![string("B") => 189]);
    scorer_map.insert(string("UQ1"), hashmap![string("BH") => 21,string("BI") => -12,string("BK") => -99,string("BN") => 142,string("BO") => -56,string("OH") => -95,string("OI") => 477,string("OK") => 410,string("OO") => -2422]);
    scorer_map.insert(string("UQ2"), hashmap![string("BH") => 216,string("BI") => 113,string("OK") => 1759]);
    scorer_map.insert(string("UQ3"), hashmap![string("BA") => -479,string("BH") => 42,string("BI") => 1913,string("BK") => -7198,string("BM") => 3160,string("BN") => 6427,string("BO") => 14761,string("OI") => -827,string("ON") => -3212]);
    scorer_map.insert(string("UW1"), hashmap![string(",") => 156,string("、") => 156,string("「") => -463,string("あ") => -941,string("う") => -127,string("が") => -553,string("き") => 121,string("こ") => 505,string("で") => -201,string("と") => -547,string("ど") => -123,string("に") => -789,string("の") => -185,string("は") => -847,string("も") => -466,string("や") => -470,string("よ") => 182,string("ら") => -292,string("り") => 208,string("れ") => 169,string("を") => -446,string("ん") => -137,string("・") => -135,string("主") => -402,string("京") => -268,string("区") => -912,string("午") => 871,string("国") => -460,string("大") => 561,string("委") => 729,string("市") => -411,string("日") => -141,string("理") => 361,string("生") => -408,string("県") => -386,string("都") => -718,string("｢") => -463,string("･") => -135]);
    scorer_map.insert(string("UW2"), hashmap![string(",") => -829,string("、") => -829,string("〇") => 892,string("「") => -645,string("」") => 3145,string("あ") => -538,string("い") => 505,string("う") => 134,string("お") => -502,string("か") => 1454,string("が") => -856,string("く") => -412,string("こ") => 1141,string("さ") => 878,string("ざ") => 540,string("し") => 1529,string("す") => -675,string("せ") => 300,string("そ") => -1011,string("た") => 188,string("だ") => 1837,string("つ") => -949,string("て") => -291,string("で") => -268,string("と") => -981,string("ど") => 1273,string("な") => 1063,string("に") => -1764,string("の") => 130,string("は") => -409,string("ひ") => -1273,string("べ") => 1261,string("ま") => 600,string("も") => -1263,string("や") => -402,string("よ") => 1639,string("り") => -579,string("る") => -694,string("れ") => 571,string("を") => -2516,string("ん") => 2095,string("ア") => -587,string("カ") => 306,string("キ") => 568,string("ッ") => 831,string("三") => -758,string("不") => -2150,string("世") => -302,string("中") => -968,string("主") => -861,string("事") => 492,string("人") => -123,string("会") => 978,string("保") => 362,string("入") => 548,string("初") => -3025,string("副") => -1566,string("北") => -3414,string("区") => -422,string("大") => -1769,string("天") => -865,string("太") => -483,string("子") => -1519,string("学") => 760,string("実") => 1023,string("小") => -2009,string("市") => -813,string("年") => -1060,string("強") => 1067,string("手") => -1519,string("揺") => -1033,string("政") => 1522,string("文") => -1355,string("新") => -1682,string("日") => -1815,string("明") => -1462,string("最") => -630,string("朝") => -1843,string("本") => -1650,string("東") => -931,string("果") => -665,string("次") => -2378,string("民") => -180,string("気") => -1740,string("理") => 752,string("発") => 529,string("目") => -1584,string("相") => -242,string("県") => -1165,string("立") => -763,string("第") => 810,string("米") => 509,string("自") => -1353,string("行") => 838,string("西") => -744,string("見") => -3874,string("調") => 1010,string("議") => 1198,string("込") => 3041,string("開") => 1758,string("間") => -1257,string("｢") => -645,string("｣") => 3145,string("ｯ") => 831,string("ｱ") => -587,string("ｶ") => 306,string("ｷ") => 568]);
    scorer_map.insert(string("UW3"), hashmap![string(",") => 4889,string("1") => -800,string("−") => -1723,string("、") => 4889,string("々") => -2311,string("〇") => 5827,string("」") => 2670,string("〓") => -3573,string("あ") => -2696,string("い") => 1006,string("う") => 2342,string("え") => 1983,string("お") => -4864,string("か") => -1163,string("が") => 3271,string("く") => 1004,string("け") => 388,string("げ") => 401,string("こ") => -3552,string("ご") => -3116,string("さ") => -1058,string("し") => -395,string("す") => 584,string("せ") => 3685,string("そ") => -5228,string("た") => 842,string("ち") => -521,string("っ") => -1444,string("つ") => -1081,string("て") => 6167,string("で") => 2318,string("と") => 1691,string("ど") => -899,string("な") => -2788,string("に") => 2745,string("の") => 4056,string("は") => 4555,string("ひ") => -2171,string("ふ") => -1798,string("へ") => 1199,string("ほ") => -5516,string("ま") => -4384,string("み") => -120,string("め") => 1205,string("も") => 2323,string("や") => -788,string("よ") => -202,string("ら") => 727,string("り") => 649,string("る") => 5905,string("れ") => 2773,string("わ") => -1207,string("を") => 6620,string("ん") => -518,string("ア") => 551,string("グ") => 1319,string("ス") => 874,string("ッ") => -1350,string("ト") => 521,string("ム") => 1109,string("ル") => 1591,string("ロ") => 2201,string("ン") => 278,string("・") => -3794,string("一") => -1619,string("下") => -1759,string("世") => -2087,string("両") => 3815,string("中") => 653,string("主") => -758,string("予") => -1193,string("二") => 974,string("人") => 2742,string("今") => 792,string("他") => 1889,string("以") => -1368,string("低") => 811,string("何") => 4265,string("作") => -361,string("保") => -2439,string("元") => 4858,string("党") => 3593,string("全") => 1574,string("公") => -3030,string("六") => 755,string("共") => -1880,string("円") => 5807,string("再") => 3095,string("分") => 457,string("初") => 2475,string("別") => 1129,string("前") => 2286,string("副") => 4437,string("力") => 365,string("動") => -949,string("務") => -1872,string("化") => 1327,string("北") => -1038,string("区") => 4646,string("千") => -2309,string("午") => -783,string("協") => -1006,string("口") => 483,string("右") => 1233,string("各") => 3588,string("合") => -241,string("同") => 3906,string("和") => -837,string("員") => 4513,string("国") => 642,string("型") => 1389,string("場") => 1219,string("外") => -241,string("妻") => 2016,string("学") => -1356,string("安") => -423,string("実") => -1008,string("家") => 1078,string("小") => -513,string("少") => -3102,string("州") => 1155,string("市") => 3197,string("平") => -1804,string("年") => 2416,string("広") => -1030,string("府") => 1605,string("度") => 1452,string("建") => -2352,string("当") => -3885,string("得") => 1905,string("思") => -1291,string("性") => 1822,string("戸") => -488,string("指") => -3973,string("政") => -2013,string("教") => -1479,string("数") => 3222,string("文") => -1489,string("新") => 1764,string("日") => 2099,string("旧") => 5792,string("昨") => -661,string("時") => -1248,string("曜") => -951,string("最") => -937,string("月") => 4125,string("期") => 360,string("李") => 3094,string("村") => 364,string("東") => -805,string("核") => 5156,string("森") => 2438,string("業") => 484,string("氏") => 2613,string("民") => -1694,string("決") => -1073,string("法") => 1868,string("海") => -495,string("無") => 979,string("物") => 461,string("特") => -3850,string("生") => -273,string("用") => 914,string("町") => 1215,string("的") => 7313,string("直") => -1835,string("省") => 792,string("県") => 6293,string("知") => -1528,string("私") => 4231,string("税") => 401,string("立") => -960,string("第") => 1201,string("米") => 7767,string("系") => 3066,string("約") => 3663,string("級") => 1384,string("統") => -4229,string("総") => 1163,string("線") => 1255,string("者") => 6457,string("能") => 725,string("自") => -2869,string("英") => 785,string("見") => 1044,string("調") => -562,string("財") => -733,string("費") => 1777,string("車") => 1835,string("軍") => 1375,string("込") => -1504,string("通") => -1136,string("選") => -681,string("郎") => 1026,string("郡") => 4404,string("部") => 1200,string("金") => 2163,string("長") => 421,string("開") => -1432,string("間") => 1302,string("関") => -1282,string("雨") => 2009,string("電") => -1045,string("非") => 2066,string("駅") => 1620,string("１") => -800,string("｣") => 2670,string("･") => -3794,string("ｯ") => -1350,string("ｱ") => 551,string("ｽ") => 874,string("ﾄ") => 521,string("ﾑ") => 1109,string("ﾙ") => 1591,string("ﾛ") => 2201,string("ﾝ") => 278]);
    scorer_map.insert(string("UW4"), hashmap![string(",") => 3930,string(".") => 3508,string("―") => -4841,string("、") => 3930,string("。") => 3508,string("〇") => 4999,string("「") => 1895,string("」") => 3798,string("〓") => -5156,string("あ") => 4752,string("い") => -3435,string("う") => -640,string("え") => -2514,string("お") => 2405,string("か") => 530,string("が") => 6006,string("き") => -4482,string("ぎ") => -3821,string("く") => -3788,string("け") => -4376,string("げ") => -4734,string("こ") => 2255,string("ご") => 1979,string("さ") => 2864,string("し") => -843,string("じ") => -2506,string("す") => -731,string("ず") => 1251,string("せ") => 181,string("そ") => 4091,string("た") => 5034,string("だ") => 5408,string("ち") => -3654,string("っ") => -5882,string("つ") => -1659,string("て") => 3994,string("で") => 7410,string("と") => 4547,string("な") => 5433,string("に") => 6499,string("ぬ") => 1853,string("ね") => 1413,string("の") => 7396,string("は") => 8578,string("ば") => 1940,string("ひ") => 4249,string("び") => -4134,string("ふ") => 1345,string("へ") => 6665,string("べ") => -744,string("ほ") => 1464,string("ま") => 1051,string("み") => -2082,string("む") => -882,string("め") => -5046,string("も") => 4169,string("ゃ") => -2666,string("や") => 2795,string("ょ") => -1544,string("よ") => 3351,string("ら") => -2922,string("り") => -9726,string("る") => -14896,string("れ") => -2613,string("ろ") => -4570,string("わ") => -1783,string("を") => 13150,string("ん") => -2352,string("カ") => 2145,string("コ") => 1789,string("セ") => 1287,string("ッ") => -724,string("ト") => -403,string("メ") => -1635,string("ラ") => -881,string("リ") => -541,string("ル") => -856,string("ン") => -3637,string("・") => -4371,string("ー") => -11870,string("一") => -2069,string("中") => 2210,string("予") => 782,string("事") => -190,string("井") => -1768,string("人") => 1036,string("以") => 544,string("会") => 950,string("体") => -1286,string("作") => 530,string("側") => 4292,string("先") => 601,string("党") => -2006,string("共") => -1212,string("内") => 584,string("円") => 788,string("初") => 1347,string("前") => 1623,string("副") => 3879,string("力") => -302,string("動") => -740,string("務") => -2715,string("化") => 776,string("区") => 4517,string("協") => 1013,string("参") => 1555,string("合") => -1834,string("和") => -681,string("員") => -910,string("器") => -851,string("回") => 1500,string("国") => -619,string("園") => -1200,string("地") => 866,string("場") => -1410,string("塁") => -2094,string("士") => -1413,string("多") => 1067,string("大") => 571,string("子") => -4802,string("学") => -1397,string("定") => -1057,string("寺") => -809,string("小") => 1910,string("屋") => -1328,string("山") => -1500,string("島") => -2056,string("川") => -2667,string("市") => 2771,string("年") => 374,string("庁") => -4556,string("後") => 456,string("性") => 553,string("感") => 916,string("所") => -1566,string("支") => 856,string("改") => 787,string("政") => 2182,string("教") => 704,string("文") => 522,string("方") => -856,string("日") => 1798,string("時") => 1829,string("最") => 845,string("月") => -9066,string("木") => -485,string("来") => -442,string("校") => -360,string("業") => -1043,string("氏") => 5388,string("民") => -2716,string("気") => -910,string("沢") => -939,string("済") => -543,string("物") => -735,string("率") => 672,string("球") => -1267,string("生") => -1286,string("産") => -1101,string("田") => -2900,string("町") => 1826,string("的") => 2586,string("目") => 922,string("省") => -3485,string("県") => 2997,string("空") => -867,string("立") => -2112,string("第") => 788,string("米") => 2937,string("系") => 786,string("約") => 2171,string("経") => 1146,string("統") => -1169,string("総") => 940,string("線") => -994,string("署") => 749,string("者") => 2145,string("能") => -730,string("般") => -852,string("行") => -792,string("規") => 792,string("警") => -1184,string("議") => -244,string("谷") => -1000,string("賞") => 730,string("車") => -1481,string("軍") => 1158,string("輪") => -1433,string("込") => -3370,string("近") => 929,string("道") => -1291,string("選") => 2596,string("郎") => -4866,string("都") => 1192,string("野") => -1100,string("銀") => -2213,string("長") => 357,string("間") => -2344,string("院") => -2297,string("際") => -2604,string("電") => -878,string("領") => -1659,string("題") => -792,string("館") => -1984,string("首") => 1749,string("高") => 2120,string("｢") => 1895,string("｣") => 3798,string("･") => -4371,string("ｯ") => -724,string("ｰ") => -11870,string("ｶ") => 2145,string("ｺ") => 1789,string("ｾ") => 1287,string("ﾄ") => -403,string("ﾒ") => -1635,string("ﾗ") => -881,string("ﾘ") => -541,string("ﾙ") => -856,string("ﾝ") => -3637]);
    scorer_map.insert(string("UW5"), hashmap![string(",") => 465,string(".") => -299,string("1") => -514,string("E2") => -32768,string("]") => -2762,string("、") => 465,string("。") => -299,string("「") => 363,string("あ") => 1655,string("い") => 331,string("う") => -503,string("え") => 1199,string("お") => 527,string("か") => 647,string("が") => -421,string("き") => 1624,string("ぎ") => 1971,string("く") => 312,string("げ") => -983,string("さ") => -1537,string("し") => -1371,string("す") => -852,string("だ") => -1186,string("ち") => 1093,string("っ") => 52,string("つ") => 921,string("て") => -18,string("で") => -850,string("と") => -127,string("ど") => 1682,string("な") => -787,string("に") => -1224,string("の") => -635,string("は") => -578,string("べ") => 1001,string("み") => 502,string("め") => 865,string("ゃ") => 3350,string("ょ") => 854,string("り") => -208,string("る") => 429,string("れ") => 504,string("わ") => 419,string("を") => -1264,string("ん") => 327,string("イ") => 241,string("ル") => 451,string("ン") => -343,string("中") => -871,string("京") => 722,string("会") => -1153,string("党") => -654,string("務") => 3519,string("区") => -901,string("告") => 848,string("員") => 2104,string("大") => -1296,string("学") => -548,string("定") => 1785,string("嵐") => -1304,string("市") => -2991,string("席") => 921,string("年") => 1763,string("思") => 872,string("所") => -814,string("挙") => 1618,string("新") => -1682,string("日") => 218,string("月") => -4353,string("査") => 932,string("格") => 1356,string("機") => -1508,string("氏") => -1347,string("田") => 240,string("町") => -3912,string("的") => -3149,string("相") => 1319,string("省") => -1052,string("県") => -4003,string("研") => -997,string("社") => -278,string("空") => -813,string("統") => 1955,string("者") => -2233,string("表") => 663,string("語") => -1073,string("議") => 1219,string("選") => -1018,string("郎") => -368,string("長") => 786,string("間") => 1191,string("題") => 2368,string("館") => -689,string("１") => -514,string("Ｅ２") => -32768,string("｢") => 363,string("ｲ") => 241,string("ﾙ") => 451,string("ﾝ") => -343]);
    scorer_map.insert(string("UW6"), hashmap![string(",") => 227,string(".") => 808,string("1") => -270,string("E1") => 306,string("、") => 227,string("。") => 808,string("あ") => -307,string("う") => 189,string("か") => 241,string("が") => -73,string("く") => -121,string("こ") => -200,string("じ") => 1782,string("す") => 383,string("た") => -428,string("っ") => 573,string("て") => -1014,string("で") => 101,string("と") => -105,string("な") => -253,string("に") => -149,string("の") => -417,string("は") => -236,string("も") => -206,string("り") => 187,string("る") => -135,string("を") => 195,string("ル") => -673,string("ン") => -496,string("一") => -277,string("中") => 201,string("件") => -800,string("会") => 624,string("前") => 302,string("区") => 1792,string("員") => -1212,string("委") => 798,string("学") => -960,string("市") => 887,string("広") => -695,string("後") => 535,string("業") => -697,string("相") => 753,string("社") => -507,string("福") => 974,string("空") => -822,string("者") => 1811,string("連") => 463,string("郎") => 1082,string("１") => -270,string("Ｅ１") => 306,string("ﾙ") => -673,string("ﾝ") => -496]);
    scorer_map
}

fn get_key_tag(c: &char, char_map: &HashMap<char, char>) -> char {
    match char_map.get(&c) {
        Some(x) => *x,
        None => 'O'
    }
}

fn get_score(key: &String, word: &String, str_score_map: &HashMap<String, HashMap<String, i32>>) -> i32 {
    match str_score_map.get(key) {
        Some(x) => match x.get(word) {
            Some(y) => *y,
            None => 0
        },
        None => -1
    }
}

fn compute_score(ngram_info: &CharNgram, str_score_map: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut score = -332;
    
    // feature
    let p1 = &ngram_info.pinfo.p1.to_string();
    let p2 = &ngram_info.pinfo.p2.to_string();
    let p3 = &ngram_info.pinfo.p3.to_string();
    let p1p2 = format!("{}{}", &p1, &p2);
    let p2p3 = format!("{}{}", &p2, &p3);
    let w1 = &ngram_info.char_info.w1;
    let w2 = &ngram_info.char_info.w2;
    let w3 = &ngram_info.char_info.w3;
    let w4 = &ngram_info.char_info.w4;
    let w5 = &ngram_info.char_info.w5;
    let w6 = &ngram_info.char_info.w6;
    let w2w3 = format!("{}{}", ngram_info.char_info.w2w3[0], ngram_info.char_info.w2w3[1]);
    let w3w4 = format!("{}{}", ngram_info.char_info.w3w4[0], ngram_info.char_info.w3w4[1]);
    let w4w5 = format!("{}{}", ngram_info.char_info.w4w5[0], ngram_info.char_info.w4w5[1]);
    let w1w2w3 = format!("{}{}{}", ngram_info.char_info.w1w2w3[0], ngram_info.char_info.w1w2w3[1], ngram_info.char_info.w1w2w3[2]);
    let w2w3w4 = format!("{}{}{}", ngram_info.char_info.w2w3w4[0], ngram_info.char_info.w2w3w4[1], ngram_info.char_info.w2w3w4[2]);
    let w3w4w5 = format!("{}{}{}", ngram_info.char_info.w3w4w5[0], ngram_info.char_info.w3w4w5[1], ngram_info.char_info.w3w4w5[2]);
    let w4w5w6 = format!("{}{}{}", ngram_info.char_info.w4w5w6[0], ngram_info.char_info.w4w5w6[1], ngram_info.char_info.w4w5w6[2]);
    let c1 = &ngram_info.type_info.c1.to_string();
    let c2 = &ngram_info.type_info.c2.to_string();
    let c3 = &ngram_info.type_info.c3.to_string();
    let c4 = &ngram_info.type_info.c4.to_string();
    let c5 = &ngram_info.type_info.c5.to_string();
    let c6 = &ngram_info.type_info.c6.to_string();
    let c2c3 = format!("{}{}", ngram_info.type_info.c2c3[0], ngram_info.type_info.c2c3[1]);
    let c3c4 = format!("{}{}", ngram_info.type_info.c3c4[0], ngram_info.type_info.c3c4[1]);
    let c4c5 = format!("{}{}", ngram_info.type_info.c4c5[0], ngram_info.type_info.c4c5[1]);
    let c1c2c3 = format!("{}{}{}", ngram_info.type_info.c1c2c3[0], ngram_info.type_info.c1c2c3[1], ngram_info.type_info.c1c2c3[2]);
    let c2c3c4 = format!("{}{}{}", ngram_info.type_info.c2c3c4[0], ngram_info.type_info.c2c3c4[1], ngram_info.type_info.c2c3c4[2]);
    let c3c4c5 = format!("{}{}{}", ngram_info.type_info.c3c4c5[0], ngram_info.type_info.c3c4c5[1], ngram_info.type_info.c3c4c5[2]);
    let c4c5c6 = format!("{}{}{}", ngram_info.type_info.c4c5c6[0], ngram_info.type_info.c4c5c6[1], ngram_info.type_info.c4c5c6[2]);
    let p1c1 = format!("{}{}", &p1, &c1);
    let p2c2 = format!("{}{}", &p2, &c2);
    let p3c3 = format!("{}{}", &p3, &c3);
    let p2c2c3 = format!("{}{}", &p2, &c2c3);
    let p2c3c4 = format!("{}{}", &p2, &c3c4);
    let p3c2c3 = format!("{}{}", &p3, &c2c3);
    let p3c3c4 = format!("{}{}", &p3, &c3c4);
    let p2c1c2c3 = format!("{}{}", &p2, &c1c2c3);
    let p2c2c3c4 = format!("{}{}", &p2, &c2c3c4);
    let p3c1c2c3 = format!("{}{}", &p3, &c1c2c3);
    let p3c2c3c4 = format!("{}{}", &p3, &c2c3c4);

    // feature to score
    score += get_score(&string("UP1"), p1, &str_score_map);
    score += get_score(&string("UP2"), p2, &str_score_map);
    score += get_score(&string("UP3"), p3, &str_score_map);
    score += get_score(&string("BP1"), &p1p2, &str_score_map);
    score += get_score(&string("BP2"), &p2p3, &str_score_map);
    score += get_score(&string("UW1"), w1, &str_score_map);
    score += get_score(&string("UW2"), w2, &str_score_map);
    score += get_score(&string("UW3"), w3, &str_score_map);
    score += get_score(&string("BW1"), &w2w3, &str_score_map);
    score += get_score(&string("BW2"), &w3w4, &str_score_map);
    score += get_score(&string("BW3"), &w4w5, &str_score_map);
    score += get_score(&string("TW1"), &w1w2w3, &str_score_map);
    score += get_score(&string("TW2"), &w2w3w4, &str_score_map);
    score += get_score(&string("TW3"), &w3w4w5, &str_score_map);
    score += get_score(&string("TW4"), &w4w5w6, &str_score_map);
    score += get_score(&string("UC1"), c1, &str_score_map);
    score += get_score(&string("UC2"), c2, &str_score_map);
    score += get_score(&string("UC3"), c3, &str_score_map);
    score += get_score(&string("UC4"), c4, &str_score_map);
    score += get_score(&string("UC5"), c5, &str_score_map);
    score += get_score(&string("UC6"), c6, &str_score_map);
    score += get_score(&string("BC1"), &c2c3, &str_score_map);
    score += get_score(&string("BC2"), &c3c4, &str_score_map);
    score += get_score(&string("BC3"), &c4c5, &str_score_map);
    score += get_score(&string("TC1"), &c1c2c3, &str_score_map);
    score += get_score(&string("TC2"), &c2c3c4, &str_score_map);
    score += get_score(&string("TC3"), &c3c4c5, &str_score_map);
    score += get_score(&string("TC4"), &c4c5c6, &str_score_map);
    score += get_score(&string("UQ1"), &p1c1, &str_score_map);
    score += get_score(&string("UQ2"), &p2c2, &str_score_map);
    score += get_score(&string("UQ3"), &p3c3, &str_score_map);
    score += get_score(&string("BQ1"), &p2c2c3, &str_score_map);
    score += get_score(&string("BQ2"), &p2c3c4, &str_score_map);
    score += get_score(&string("BQ3"), &p3c2c3, &str_score_map);
    score += get_score(&string("BQ4"), &p3c3c4, &str_score_map);
    score += get_score(&string("TQ1"), &p2c1c2c3, &str_score_map);
    score += get_score(&string("TQ2"), &p2c2c3c4, &str_score_map);
    score += get_score(&string("TQ3"), &p3c1c2c3, &str_score_map);
    score += get_score(&string("TQ4"), &p3c2c3c4, &str_score_map);
    score 
}

fn boundary_prediction(segments: Vec<String>, ctype: Vec<char>, str_score_map: HashMap<String, HashMap<String, i32>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    let mut char_info: CharInfo = CharInfo {w1: &segments[0], w2: &segments[1], w3: &segments[2], 
                                            w4: &segments[3], w5: &segments[4], w6: &segments[5],
                                            w2w3: [&segments[1], &segments[2]], w3w4: [&segments[2], &segments[3]],
                                            w4w5: [&segments[3], &segments[4]], w5w6: [&segments[4], &segments[5]],
                                            w1w2w3: [&segments[0], &segments[1], &segments[2]],
                                            w2w3w4: [&segments[1], &segments[2], &segments[3]],
                                            w3w4w5: [&segments[2], &segments[3], &segments[4]],
                                            w4w5w6: [&segments[3], &segments[4], &segments[5]]};
    
    let ctype_info: TypeInfo = TypeInfo {c1: &ctype[0], c2: &ctype[1], c3: &ctype[2],
                                         c4: &ctype[3], c5: &ctype[4], c6: &ctype[5],
                                         c2c3: [&ctype[1], &ctype[2]], c3c4: [&ctype[2], &ctype[3]],
                                         c4c5: [&ctype[3], &ctype[4]], c5c6: [&ctype[4], &ctype[6]],
                                         c1c2c3: [&ctype[0], &ctype[1], &ctype[2]],
                                         c2c3c4: [&ctype[1], &ctype[2], &ctype[3]],
                                         c3c4c5: [&ctype[2], &ctype[3], &ctype[4]],
                                         c4c5c6: [&ctype[3], &ctype[4], &ctype[6]]};
    
    let pinfo: PInfo = PInfo { p1: 'U', p2: 'U', p3: 'U', p: 'O' };
    let mut ngram_info: CharNgram = CharNgram { char_info: char_info, type_info: ctype_info, pinfo: pinfo };
    let mut word = segments[3].clone();

    for i in 4..segments.len()-3 {
        // uni-gram
        ngram_info.char_info.w1 = ngram_info.char_info.w2;
        ngram_info.char_info.w2 = ngram_info.char_info.w3;
        ngram_info.char_info.w3 = ngram_info.char_info.w4;
        ngram_info.char_info.w4 = ngram_info.char_info.w5;
        ngram_info.char_info.w5 = ngram_info.char_info.w6;
        ngram_info.char_info.w6 = &segments[i+2];
        
        ngram_info.type_info.c1 = ngram_info.type_info.c2;
        ngram_info.type_info.c2 = ngram_info.type_info.c3;
        ngram_info.type_info.c3 = ngram_info.type_info.c4;
        ngram_info.type_info.c4 = ngram_info.type_info.c5;
        ngram_info.type_info.c5 = ngram_info.type_info.c6;
        ngram_info.type_info.c6 = &ctype[i+2];
        
        // bi-gram
        ngram_info.char_info.w2w3 = ngram_info.char_info.w3w4;
        ngram_info.char_info.w3w4 = ngram_info.char_info.w4w5;
        ngram_info.char_info.w4w5 = [ngram_info.char_info.w4, ngram_info.char_info.w5];

        ngram_info.type_info.c2c3 = ngram_info.type_info.c3c4;
        ngram_info.type_info.c3c4 = ngram_info.type_info.c4c5;
        ngram_info.type_info.c4c5 = [ngram_info.type_info.c4, ngram_info.type_info.c5];

        // tri-gram
        ngram_info.char_info.w1w2w3 = ngram_info.char_info.w2w3w4;
        ngram_info.char_info.w2w3w4 = ngram_info.char_info.w3w4w5;
        ngram_info.char_info.w3w4w5 = ngram_info.char_info.w4w5w6;
        ngram_info.char_info.w4w5w6 = [ngram_info.char_info.w4w5[0], ngram_info.char_info.w4w5[1], ngram_info.char_info.w6];

        ngram_info.type_info.c1c2c3 = ngram_info.type_info.c2c3c4;
        ngram_info.type_info.c2c3c4 = ngram_info.type_info.c3c4c5;
        ngram_info.type_info.c3c4c5 = ngram_info.type_info.c4c5c6;
        ngram_info.type_info.c4c5c6 = [ngram_info.type_info.c4c5[0], ngram_info.type_info.c4c5[1], ngram_info.type_info.c6];

        // compute score
        let score = compute_score(&ngram_info, &str_score_map);

        // segment or not?
        if score > 0 {
            result.push(word);
            word = string("");
            ngram_info.pinfo.p = 'B';
        }

        ngram_info.pinfo.p1 = ngram_info.pinfo.p2;
        ngram_info.pinfo.p2 = ngram_info.pinfo.p3;
        ngram_info.pinfo.p3 = ngram_info.pinfo.p;
        word.push_str(&segments[i].clone());
    }

    result.push(word);
    result
}

fn tokenize(text: &String) -> String {
    if text.len() == 0 { return String::from(""); }
    
    let char_map = construct_hashmap();
    let str_score_map = construct_scorer_hashmap();

    let chars = construct_words(text);
    let str_chars: Vec<String> = chars.iter().map(|c| String::from(*c)).collect();
    let ctype_text: Vec<char> = chars.into_iter().map(|c| get_key_tag(&c, &char_map)).collect();

    let mut segments: Vec<String> = vec![string("B3"), string("B2"), string("B1")];
    let mut ctype: Vec<char> = vec!['O', 'O', 'O'];

    ctype.extend(ctype_text);
    segments.extend(str_chars);

    ctype.extend(vec!['O', 'O', 'O']);
    segments.extend(vec![string("E1"), string("E2"), string("E3")]);

    let result = boundary_prediction(segments, ctype, str_score_map);
    let segmented_str = result.join(" ");
    segmented_str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    let segmented_str = tokenize(text);
    println!("{}", segmented_str);
}
