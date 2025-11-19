use std::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
};

use crate::{
    morse::types::{MorseCharacterSet, Options},
    utils::btmap,
};

pub type Characters = BTreeMap<MorseCharacterSet, BTreeMap<char, String>>;

/// Generates a complete set of Morse code characters for various languages and symbols.
///
/// This function creates and returns a `Characters` mapping that includes Morse code representations
/// for Latin, numbers, punctuation, Latin extended characters, Cyrillic, Greek, Hebrew, Arabic,
/// Persian, Japanese, Korean, and Thai characters. Each set of characters is stored in a `BTreeMap`
/// and organized by the type of Morse character set.
pub fn base_characters() -> Characters {
    let characters = btmap! {
        MorseCharacterSet::Latin => latin_chars(),
        MorseCharacterSet::Numbers => numbers_chars(),
        MorseCharacterSet::Punctuation => punctuation_chars(),
        MorseCharacterSet::LatinExtended => latin_extended_chars(),
        MorseCharacterSet::Cyrillic => cyrillic_chars(),
        MorseCharacterSet::Greek => greek_chars(),
        MorseCharacterSet::Hebrew =>  hebrew_chars(),
        MorseCharacterSet::Arabic => arabic_chars(),
        MorseCharacterSet::Persian => persian_chars(),
        MorseCharacterSet::Japanese => japanese_chars(),
        MorseCharacterSet::Korean => korean_chars(),
        MorseCharacterSet::Thai => thai_chars()
    };

    characters
        .into_iter()
        .map(|(charset, char_map)| {
            (
                charset,
                char_map
                    .into_iter()
                    .map(|(char, morse_code)| (char, morse_code.to_string()))
                    .collect(),
            )
        })
        .collect::<Characters>()
}

/// Returns a `BTreeMap` of Morse code representations for Latin characters.
///
/// This function provides the Morse code mappings for Latin alphabet characters, where each key is a Latin character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Latin characters as keys and their Morse code representations as values.
fn latin_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'A' => "01",
        'B' => "1000",
        'C' => "1010",
        'D' => "100",
        'E' => "0",
        'F' => "0010",
        'G' => "110",
        'H' => "0000",
        'I' => "00",
        'J' => "0111",
        'K' => "101",
        'L' => "0100",
        'M' => "11",
        'N' => "10",
        'O' => "111",
        'P' => "0110",
        'Q' => "1101",
        'R' => "010",
        'S' => "000",
        'T' => "1",
        'U' => "001",
        'V' => "0001",
        'W' => "011",
        'X' => "1001",
        'Y' => "1011",
        'Z' => "1100"
    }
}

/// Returns a `BTreeMap` of Morse code representations for digits (0-9).
///
/// This function provides the Morse code mappings for numeric digits, where each key is a digit and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with digits as keys and their Morse code representations as values.
fn numbers_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        '0' => "11111",
        '1' => "01111",
        '2' => "00111",
        '3' => "00011",
        '4' => "00001",
        '5' => "00000",
        '6' => "10000",
        '7' => "11000",
        '8' => "11100",
        '9' => "11110"
    }
}

/// Returns a `BTreeMap` of Morse code representations for punctuation marks.
///
/// This function provides the Morse code mappings for various punctuation marks, where each key is a punctuation mark
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with punctuation marks as keys and their Morse code representations as values.
fn punctuation_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        '.' => "010101",
        ',' => "110011",
        '?' => "001100",
        '\'' => "011110",
        '!' => "101011",
        '/' => "10010",
        '(' => "10110",
        ')' => "101101",
        '&' => "01000",
        ':' => "111000",
        ';' => "101010",
        '=' => "10001",
        '+' => "01010",
        '-' => "100001",
        '_' => "001101",
        '"' => "010010",
        '$' => "0001001",
        '@' => "011010",
        '¿' => "00101",
        '¡' => "110001"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Latin Extended characters.
///
/// This function provides the Morse code mappings for Latin Extended characters, where each key is a Latin Extended character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Latin Extended characters as keys and their Morse code representations as values.
fn latin_extended_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'Ã' => "01101", 'Á' => "01101", 'Å' => "01101", 'À' => "01101", 'Â' => "01101",
        'Ä' => "0101", 'Ą' => "0101", 'Æ' => "0101",
        'Ç' => "10100", 'Ć' => "10100", 'Ĉ' => "10100", 'Č' => "110",
        'Ð' => "00110",
        'È' => "01001", 'Ę' => "00100", 'Ë' => "00100", 'É' => "00100", 'Ê' => "10010",
        'Ğ' => "11010", 'Ĝ' => "11010",
        'Ĥ' => "1111", 'İ' => "01001", 'Ï' => "10011", 'Ì' => "01110", 'Ĵ' => "01110",
        'Ł' => "01001",
        'Ń' => "11011", 'Ñ' => "11011",
        'Ó' => "1110", 'Ò' => "1110", 'Ö' => "1110", 'Ô' => "1110", 'Ø' => "1110",
        'Ś' => "0001000", 'Ş' => "01100", 'Ș' => "1111", 'Š' => "1111", 'Ŝ' => "00010",
        'ß' => "000000", 'Þ' => "01100",
        'Ü' => "0011", 'Ù' => "0011", 'Ŭ' => "0011",
        'Ž' => "11001", 'Ź' => "110010", 'Ż' => "11001"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Cyrillic characters.
///
/// This function provides the Morse code mappings for Cyrillic alphabet characters, where each key is a Cyrillic character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Cyrillic characters as keys and their Morse code representations as values.
fn cyrillic_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'А' => "01",
        'Б' => "1000",
        'В' => "011",
        'Г' => "110",
        'Д' => "100",
        'Е' => "0",
        'Ж' => "0001",
        'З' => "1100",
        'И' => "00",
        'Й' => "0111",
        'К' => "101",
        'Л' => "0100",
        'М' => "11",
        'Н' => "10",
        'О' => "111",
        'П' => "0110",
        'Р' => "010",
        'С' => "000",
        'Т' => "1",
        'У' => "001",
        'Ф' => "0010",
        'Х' => "0000",
        'Ц' => "1010",
        'Ч' => "1110",
        'Ш' => "1111",
        'Щ' => "1101",
        'Ъ' => "11011",
        'Ы' => "1011",
        'Ь' => "1001",
        'Э' => "00100",
        'Ю' => "0011",
        'Я' => "0101",
        'Ї' => "01110",
        'Є' => "00100",
        'І' => "00",
        'Ґ' => "110"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Greek characters.
///
/// This function provides the Morse code mappings for Greek alphabet characters, where each key is a Greek character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Greek characters as keys and their Morse code representations as values.
fn greek_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'Α' => "01",
        'Β' => "1000",
        'Γ' => "110",
        'Δ' => "100",
        'Ε' => "0",
        'Ζ' => "1100",
        'Η' => "0000",
        'Θ' => "1010",
        'Ι' => "00",
        'Κ' => "101",
        'Λ' => "0100",
        'Μ' => "11",
        'Ν' => "10",
        'Ξ' => "1001",
        'Ο' => "111",
        'Π' => "0110",
        'Ρ' => "010",
        'Σ' => "000",
        'Τ' => "1",
        'Υ' => "1011",
        'Φ' => "0010",
        'Χ' => "1111",
        'Ψ' => "1101",
        'Ω' => "011"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Hebrew characters.
///
/// This function provides the Morse code mappings for Hebrew script characters, where each key is a Hebrew character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Hebrew characters as keys and their Morse code representations as values.
fn hebrew_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'א' => "01",
        'ב' => "1000",
        'ג' => "110",
        'ד' => "100",
        'ה' => "111",
        'ו' => "0",
        'ז' => "1100",
        'ח' => "0000",
        'ט' => "001",
        'י' => "00",
        'כ' => "101",
        'ל' => "0100",
        'מ' => "11",
        'נ' => "10",
        'ס' => "1010",
        'ע' => "0111",
        'פ' => "0110",
        'צ' => "011",
        'ק' => "1101",
        'ר' => "010",
        'ש' => "000",
        'ת' => "1"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Arabic characters.
///
/// This function provides the Morse code mappings for Arabic script characters, where each key is an Arabic character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Arabic characters as keys and their Morse code representations as values.
fn arabic_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'ا' => "01",
        'ب' => "1000",
        'ت' => "1",
        'ث' => "1010",
        'ج' => "0111",
        'ح' => "0000",
        'خ' => "111",
        'د' => "100",
        'ذ' => "1100",
        'ر' => "010",
        'ز' => "1110",
        'س' => "000",
        'ش' => "1111",
        'ص' => "1001",
        'ض' => "0001",
        'ط' => "001",
        'ظ' => "1011",
        'ع' => "0101",
        'غ' => "110",
        'ف' => "0010",
        'ق' => "1101",
        'ك' => "101",
        'ل' => "0100",
        'م' => "11",
        'ن' => "10",
        'ه' => "00100",
        'و' => "011",
        'ي' => "00",
        'ﺀ' => "0"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Persian characters.
///
/// This function provides the Morse code mappings for Persian script characters, where each key is a Persian character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Persian characters as keys and their Morse code representations as values.
fn persian_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'ا' => "01",
        'ب' => "1000",
        'پ' => "0110",
        'ت' => "1",
        'ث' => "1010",
        'ج' => "0111",
        'چ' => "1110",
        'ح' => "0000",
        'خ' => "1001",
        'د' => "100",
        'ذ' => "0001",
        'ر' => "010",
        'ز' => "1100",
        'ژ' => "110",
        'س' => "000",
        'ش' => "1111",
        'ص' => "0101",
        'ض' => "00100",
        'ط' => "001",
        'ظ' => "1011",
        'ع' => "111",
        'غ' => "0011",
        'ف' => "0010",
        'ق' => "111000",
        'ک' => "101",
        'گ' => "1101",
        'ل' => "0100",
        'م' => "11",
        'ن' => "10",
        'و' => "011",
        'ه' => "0",
        'ی' => "00"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Japanese characters.
///
/// This function provides the Morse code mappings for Japanese Katakana characters, where each key is a Japanese character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Japanese characters as keys and their Morse code representations as values.
fn japanese_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'ア' => "11011",
        'カ' => "0100",
        'サ' => "10101",
        'タ' => "10",
        'ナ' => "010",
        'ハ' => "1000",
        'マ' => "1001",
        'ヤ' => "011",
        'ラ' => "000",
        'ワ' => "101",
        'イ' => "01",
        'キ' => "10100",
        'シ' => "11010",
        'チ' => "0010",
        'ニ' => "1010",
        'ヒ' => "11001",
        'ミ' => "00101",
        'リ' => "110",
        'ヰ' => "01001",
        'ウ' => "001",
        'ク' => "0001",
        'ス' => "11101",
        'ツ' => "0110",
        'ヌ' => "0000",
        'フ' => "1100",
        'ム' => "1",
        'ユ' => "10011",
        'ル' => "10110",
        'ン' => "01010",
        'エ' => "10111",
        'ケ' => "1011",
        'セ' => "01110",
        'テ' => "01011",
        'ネ' => "1101",
        'ヘ' => "0",
        'メ' => "10001",
        'レ' => "111",
        'ヱ' => "01100",
        'オ' => "01000",
        'コ' => "1111",
        'ソ' => "1110",
        'ト' => "00100",
        'ノ' => "0011",
        'ホ' => "100",
        'モ' => "10010",
        'ヨ' => "11",
        'ロ' => "0101",
        'ヲ' => "0111",
        '゛' => "00",
        '゜' => "00110",
        '。' => "010100",
        'ー' => "01101",
        '、' => "010101",
        '（' => "101101",
        '）' => "010010"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Korean characters.
///
/// This function provides the Morse code mappings for Korean Hangul characters, where each key is a Korean character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Korean characters as keys and their Morse code representations as values.
fn korean_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'ㄱ' => "0100",
        'ㄴ' => "0010",
        'ㄷ' => "1000",
        'ㄹ' => "0001",
        'ㅁ' => "11",
        'ㅂ' => "011",
        'ㅅ' => "110",
        'ㅇ' => "101",
        'ㅈ' => "0110",
        'ㅊ' => "1010",
        'ㅋ' => "1001",
        'ㅌ' => "1100",
        'ㅍ' => "111",
        'ㅎ' => "0111",
        'ㅏ' => "0",
        'ㅑ' => "00",
        'ㅓ' => "1",
        'ㅕ' => "000",
        'ㅗ' => "01",
        'ㅛ' => "10",
        'ㅜ' => "0000",
        'ㅠ' => "010",
        'ㅡ' => "100",
        'ㅣ' => "001"
    }
}

/// Returns a `BTreeMap` of Morse code representations for Thai characters.
///
/// This function provides the Morse code mappings for Thai script characters, where each key is a Thai character
/// and each value is its Morse code representation.
///
/// # Returns
/// A `BTreeMap` with Thai characters as keys and their Morse code representations as values.   
fn thai_chars<'a>() -> BTreeMap<char, &'a str> {
    btmap! {
        'ก' => "110",
        'ข' => "1010",
        'ค' => "101",
        'ง' => "10110",
        'จ' => "10010",
        'ฉ' => "1111",
        'ช' => "1001",
        'ซ' => "1100",
        'ญ' => "0111",
        'ด' => "100",
        'ต' => "1",
        'ถ' => "10100",
        'ท' => "10011",
        'น' => "10",
        'บ' => "1000",
        'ป' => "0110",
        'ผ' => "1101",
        'ฝ' => "10101",
        'พ' => "01100",
        'ฟ' => "0010",
        'ม' => "11",
        'ย' => "1011",
        'ร' => "010",
        'ล' => "0100",
        'ว' => "011",
        'ส' => "000",
        'ห' => "0000",
        'อ' => "10001",
        'ฮ' => "11011",
        'ฤ' => "01011",
        'ะ' => "01000",
        'า' => "01",
        'ิ' => "00100",
        'ี' => "00",
        'ึ' => "00110",
        'ื' => "0011",
        'ุ' => "00101",
        'ู' => "1110",
        'เ' => "0",
        'แ' => "0101",
        'ไ' => "01001",
        'โ' => "111",
        'ำ' => "00010",
        '่' => "001",
        '้' => "0001",
        '๊' => "11000",
        '๋' => "01010",
        'ั' => "01101",
        '็' => "11100",
        '์' => "11001",
        'ๆ' => "10111",
        'ฯ' => "11010"
    }
}

/// Retrieves a `Characters` map based on the given `Options` configuration.
///
/// This function generates a `Characters` map that includes Morse code representations for various character sets,
/// incorporating any options specified, such as a priority character set or custom separators and spaces.
///
/// # Parameters
/// - `options`: A configuration object containing options for character sets and Morse code representation.
///
/// # Returns
/// A `Characters` map where each key is a `MorseCharacterSet` and each value is a `BTreeMap` of characters and their Morse code representations.
pub fn get_characters(options: &Options) -> Characters {
    let base_characters = base_characters();
    let mut characters = base_characters.clone();

    for &set in &options.character_set_order {
        if let Some(set_map) = base_characters.get(&set) {
            match set {
                MorseCharacterSet::Latin => {
                    let mut new_set = set_map.clone();
                    new_set.insert(options.separator, options.space.to_string());
                    characters.insert(MorseCharacterSet::Latin, new_set);
                }
                MorseCharacterSet::Numbers => {
                    characters.insert(MorseCharacterSet::Numbers, set_map.clone());
                }

                _ => {
                    characters.insert(set, set_map.clone());
                }
            }
        }
    }

    characters
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().collect::<BTreeMap<char, String>>()))
        .collect::<Characters>()
}
