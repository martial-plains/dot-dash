use std::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use crate::morse::characters::{Characters, get_characters};

#[uniffi::export]
pub trait IEncoderDecoder: Send + Sync {
    /// Encodes the given text into Morse code using the struct’s options.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to encode.
    ///
    /// # Returns
    ///
    /// A `String` containing the encoded Morse code.
    fn encode(&self, text: &str) -> String;
    /// Decodes the given Morse code string into text using the struct’s options.
    ///
    /// # Arguments
    ///
    /// * `morse` - The Morse code string to decode.
    ///
    /// # Returns
    ///
    /// A `String` containing the decoded text.
    fn decode(&self, morse: &str) -> String;
}

/// A type alias for a map that associates Morse code characters with their string representations.
///
/// This alias defines a `BTreeMap` where:
/// - The key is a `MorseCharacterSet`, which represents a specific set or category of Morse code characters.
/// - The value is another `BTreeMap`, where:
///   - The key is a `char` representing a single character.
///   - The value is a `String` providing the string representation or description of the Morse code character.
///
/// This structure supports efficient storage and retrieval of Morse code characters and their associated string descriptions.
/// The use of `BTreeMap` ensures that the data is kept in sorted order, enabling efficient lookups.
/// Enumerates the different character sets used in Morse code.
///
/// Each variant represents a specific alphabet or character set that can be encoded or decoded.
/// The variants include common scripts as well as less common ones.
///
/// /// For more information about each character set, you can refer to the following links:
///
/// - [Latin Alphabet](https://en.wikipedia.org/wiki/Morse_code)
/// - [Extended Latin Characters](https://ham.stackexchange.com/questions/1379/international-characters-in-morse-code)
/// - [Cyrillic Alphabet](https://en.wikipedia.org/wiki/Russian_Morse_code)
/// - [Greek Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Hebrew Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Arabic Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Persian Alphabet](https://en.wikipedia.org/wiki/Morse_code_for_non-Latin_alphabets)
/// - [Japanese Characters](https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7#%E5%92%8C%E6%96%87%E3%83%A2%E3%83%BC%E3%83%AB%E3%82%B9%E7%AC%A6%E5%8F%B7)
/// - [Korean Characters](https://en.wikipedia.org/wiki/SKATS)
/// - [Thai Characters](https://th.wikipedia.org/wiki/รหัสมอร์ส)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Enum)]
pub enum MorseCharacterSet {
    /// Represents the Latin alphabet.
    Latin,
    /// Represents numerical digits.
    Numbers,
    /// Represents punctuation marks.
    Punctuation,
    /// Represents extended Latin characters.
    LatinExtended,
    /// Represents the Cyrillic alphabet.
    Cyrillic,
    /// Represents the Greek alphabet.
    Greek,
    /// Represents the Hebrew alphabet.
    Hebrew,
    /// Represents the Arabic alphabet.
    Arabic,
    /// Represents the Persian alphabet.
    Persian,
    /// Represents Japanese characters.
    Japanese,
    /// Represents Korean characters.
    Korean,
    /// Represents Thai characters.
    Thai,
}

#[uniffi::trait_interface]
pub trait InvalidCharCallback: Send + Sync {
    // A function used to get represented an invalid Morse code character.
    fn invalid_char_callback(&self, character: char) -> char;
}

/// Contains options for encoding and decoding Morse code.
///
/// This struct allows customization of Morse code encoding and decoding by specifying the characters used
/// for dashes, dots, spaces, separators, and invalid characters, as well as a priority character set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, uniffi::Object)]
pub struct Options {
    /// Character used to represent a dash in Morse code.
    pub dash: char,
    /// Character used to represent a dot in Morse code.
    pub dot: char,
    /// Character used to represent a space between words in Morse code.
    pub space: char,
    /// Character used to separate Morse code characters.
    pub separator: char,
    /// List of `MorseCharacterSet` in user-defined order for encoding/decoding.
    pub character_set_order: Vec<MorseCharacterSet>,
}

impl InvalidCharCallback for Options {
    fn invalid_char_callback(&self, character: char) -> char {
        character
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            dash: '-',
            dot: '.',
            space: '/',
            separator: ' ',
            character_set_order: vec![
                MorseCharacterSet::Latin,
                MorseCharacterSet::Numbers,
                MorseCharacterSet::Punctuation,
                MorseCharacterSet::LatinExtended,
                MorseCharacterSet::Cyrillic,
                MorseCharacterSet::Greek,
                MorseCharacterSet::Hebrew,
                MorseCharacterSet::Arabic,
                MorseCharacterSet::Persian,
                MorseCharacterSet::Japanese,
                MorseCharacterSet::Korean,
                MorseCharacterSet::Thai,
            ],
        }
    }
}

/// A struct to manage Morse code operations including encoding and decoding.
///
/// This struct holds options and character mappings required for encoding and decoding Morse code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, uniffi::Object)]
pub struct MorseCode {
    /// Configuration options for encoding and decoding Morse code.
    ///
    /// This field specifies how Morse code should be represented, including the symbols used for dots, dashes,
    /// spaces, separators, and handling invalid characters. It also defines the priority character set to use.
    options: Options,

    /// A map of Morse code characters and their string representations.
    ///
    /// This field contains a `BTreeMap` where each key is a `MorseCharacterSet` indicating a specific set or category
    /// of Morse code characters, and each value is another `BTreeMap` mapping individual Morse code characters to their
    /// string representations or descriptions. This structure supports efficient storage and retrieval of Morse code data.
    characters: Characters,
}

impl Default for MorseCode {
    fn default() -> Self {
        let options = Options::default();
        Self {
            options: options.clone(),
            characters: get_characters(&options),
        }
    }
}

impl MorseCode {
    /// Creates a new `MorseCode` instance with the given options.
    ///
    /// # Arguments
    ///
    /// * `options` - The options to use for encoding and decoding Morse code.
    ///
    /// # Returns
    ///
    /// A `MorseCode` instance configured with the provided options.
    #[must_use]
    #[uniffi::constructor]
    pub fn new(options: Options) -> Self {
        let characters = get_characters(&options);
        MorseCode {
            options,
            characters,
        }
    }
}

impl IEncoderDecoder for MorseCode {
    fn encode(&self, text: &str) -> String {
        let mut result = String::new();

        let processed_text = text
            .replace(char::is_whitespace, &self.options.separator.to_string())
            .trim()
            .to_uppercase();

        for character in processed_text.chars() {
            let mut found = false;
            for set in self.characters.values() {
                if let Some(encoded) = set.get(&character) {
                    result.push_str(encoded);
                    found = true;
                    break;
                }
            }
            if !found {
                self.options.invalid_char_callback(character);
                result.push(self.options.invalid_char_callback(character));
            }
            result.push(self.options.separator);
        }

        result = result
            .replace('0', &self.options.dot.to_string())
            .replace('1', &self.options.dash.to_string());

        if !result.is_empty() && result.ends_with(&self.options.separator.to_string()) {
            result.pop();
        }

        result
    }

    fn decode(&self, morse: &str) -> String {
        let swapped = swap_characters(self.options.clone());

        morse
            .replace(char::is_whitespace, &self.options.separator.to_string()) // Replace whitespace with separator
            .trim() // Trim leading and trailing whitespace
            .split(self.options.separator) // Split by the separator
            .map(|characters| {
                swapped
                    .get(characters)
                    .copied()
                    .map_or_else(|| characters.to_string(), |c| c.to_string())
            })
            .collect::<String>() // Collect into a single String
    }
}

/// Returns a `BTreeMap` of Morse code representations swapped with their character mappings.
///
/// This function generates a mapping where Morse code representations are keys and the corresponding characters are values.
/// This is useful for reverse lookup of Morse code representations.
///
/// # Parameters
/// - `options`: A configuration object containing custom symbols for dots and dashes.
///
/// # Returns
/// A `BTreeMap` where each key is a Morse code representation and each value is the corresponding character.
fn swap_characters(options: Options) -> BTreeMap<String, char> {
    let mut swapped = BTreeMap::new();
    let mapped_characters = get_mapped_characters(options);

    for chars in mapped_characters.into_values() {
        for (key, value) in chars {
            swapped.entry(value).or_insert(key);
        }
    }

    swapped
}

/// Returns a `Characters` map with Morse code characters mapped to custom symbols based on the given `Options` configuration.
///
/// This function generates a `Characters` map by replacing Morse code symbols (dots and dashes) with custom symbols
/// specified in the `options` configuration.
///
/// # Parameters
/// - `options`: A configuration object containing custom symbols for dots and dashes.
///
/// # Returns
/// A `Characters` map where each key is a `MorseCharacterSet` and each value is a `BTreeMap` of characters and their updated Morse code representations.
fn get_mapped_characters(options: Options) -> Characters {
    let mut mapped = BTreeMap::new();
    let characters = get_characters(&options);

    for (set, chars) in &characters {
        let mut new_set = BTreeMap::new();
        for (key, value) in chars {
            let mapped_value = value
                .replace('0', &options.dot.to_string())
                .replace('1', &options.dash.to_string());
            new_set.insert(*key, mapped_value);
        }
        mapped.insert(*set, new_set);
    }

    mapped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_english_alphabet() {
        assert_eq!(
            MorseCode::default().encode("the quick brown fox jumps over the lazy dog"),
            "- .... . / --.- ..- .. -.-. -.- / -... .-. --- .-- -. / ..-. --- -..- / .--- ..- -- .--. ... / --- ...- . .-. / - .... . / .-.. .- --.. -.-- / -.. --- --."
        );
        assert_eq!(
            MorseCode::new(Options {
                dash: '–',
                dot: '•',
                space: '\\',
                ..Default::default()
            })
            .encode("the quick brown fox jumps over the lazy dog"),
            "– •••• • \\ ––•– ••– •• –•–• –•– \\ –••• •–• ––– •–– –• \\ ••–• ––– –••– \\ •––– ••– –– •––• ••• \\ ––– •••– • •–• \\ – •••• • \\ •–•• •– ––•• –•–– \\ –•• ––– ––•"
        );
    }

    #[test]
    fn decodes_english_alphabet() {
        assert_eq!(MorseCode::default().decode("- .... . / --.- ..- .. -.-. -.- / -... .-. --- .-- -. / ..-. --- -..- / .--- ..- -- .--. ... / --- ...- . .-. / - .... . / .-.. .- --.. -.-- / -.. --- --."), "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
        assert_eq!(MorseCode::new(Options {dash: '–', dot: '•', space: '\\',..Default::default()}).decode("– •••• • \\ ––•– ••– •• –•–• –•– \\ –••• •–• ––– •–– –• \\ ••–• ––– –••– \\ •––– ••– –– •––• ••• \\ ––– •••– • •–• \\ – •••• • \\ •–•• •– ––•• –•–– \\ –•• ––– ––•"), "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
    }

    #[test]
    fn decodes_numbers() {
        assert_eq!(
            MorseCode::default()
                .decode("----- .---- ..--- ...-- ....- ..... -.... --... ---.. ----."),
            "0123456789"
        );
    }

    #[test]
    fn encodes_punctuation() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.encode(".,?'!/("),
            ".-.-.- --..-- ..--.. .----. -.-.-- -..-. -.--."
        );
        assert_eq!(
            morse_code.encode(")&:;=¿¡"),
            "-.--.- .-... ---... -.-.-. -...- ..-.- --...-"
        );
    }

    #[test]
    fn decodes_punctuation() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.decode(".-.-.- --..-- ..--.. .----. -.-.-- -..-. -.--."),
            ".,?'!/("
        );
        assert_eq!(
            morse_code.decode("-.--.- .-... ---... -.-.-. -...- ..-.- --...-"),
            ")&:;=¿¡"
        );
    }

    #[test]
    fn encodes_non_english_alphabet() {
        let morse_code = MorseCode::default();
        assert_eq!(
            morse_code.encode("ÃÁÅÀÂÄ"),
            ".--.- .--.- .--.- .--.- .--.- .-.-"
        );
        assert_eq!(
            morse_code.encode("ĄÆÇĆĈČ"),
            ".-.- .-.- -.-.. -.-.. -.-.. --."
        );
        assert_eq!(
            morse_code.encode("ĘÐÈËĘÉ"),
            "..-.. ..--. .-..- ..-.. ..-.. ..-.."
        );
        assert_eq!(
            morse_code.encode("ÊĞĜĤİÏ"),
            "-..-. --.-. --.-. ---- .-..- -..--"
        );
        assert_eq!(
            morse_code.encode("ÌĴŁŃÑÓ"),
            ".---. .---. .-..- --.-- --.-- ---."
        );
        assert_eq!(
            morse_code.encode("ÒÖÔØŚŞ"),
            "---. ---. ---. ---. ...-... .--.."
        );
        assert_eq!(
            morse_code.encode("ȘŠŜßÞÜ"),
            "---- ---- ...-. ... ... .--.. ..--"
        );
        assert_eq!(morse_code.encode("ÙŬŽŹŻ"), "..-- ..-- --..- --..-. --..-");
    }
}
