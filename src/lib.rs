/// Translation map (well, kind of a map, actually is an array of tuples).
/// Maps ASCII chars from 'a' to 'z' to corresponding Shyriiwook "phonemes"
/// Non ASCII chars will not be translated, but added verbatim to the result.
const CHAR_MAP: &[(char, &str)] = &[
    ('a', "ra"),
    ('b', "rh"),
    ('c', "oa"),
    ('d', "wa"),
    ('e', "wo"),
    ('f', "ww"),
    ('g', "rr"),
    ('h', "ac"),
    ('i', "ah"),
    ('j', "sh"),
    ('k', "or"),
    ('l', "an"),
    ('m', "sc"),
    ('n', "wh"),
    ('o', "oo"),
    ('p', "ak"),
    ('q', "rq"),
    ('r', "rc"),
    ('s', "c"),
    ('t', "ao"),
    ('u', "hu"),
    ('v', "ho"),
    ('w', "oh"),
    ('x', "k"),
    ('y', "ro"),
    ('z', "uf"),
];

/// Translates a string from human language to Shyriiwook, the native language of Wookiees
pub fn translate(str: &str) -> String {
    let mut s = String::new();
    for c in str.chars() {
        let translated_string = find_translation(c);
        let properly_capitalized_string = match translated_string {
            Some(translation) => {
                if c.is_uppercase() {
                    capitalize(&translation)
                } else {
                    translation
                }
            }
            None => c.to_string(),
        };
        s.push_str(&properly_capitalized_string);
    }
    s
}

fn capitalize(str: &str) -> String {
    let first_char = &str.chars().nth(0);
    match first_char {
        Some(c) => format!("{}{}", c.to_uppercase(), &str[1..]),
        _ => String::from(""),
    }
}

/// Finds the corresponding string in the CHAR_MAP map
/// Since CHAR_MAP contains characters from 'a' to 'z'
/// we can index the right translation using ascii codes,
/// that is, (char as u32) - 97
fn find_translation(needle: char) -> Option<String> {
    let index = needle.to_ascii_lowercase() as usize;
    if index >= 97 && index < CHAR_MAP.len() + 97 {
        Some(CHAR_MAP[index - 97].1.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_a() {
        let translated = translate("a");
        assert_eq!(translated, "ra");
    }

    #[test]
    fn translates_z() {
        let translated = translate("Z");
        assert_eq!(translated, "Uf");
    }

    #[test]
    fn translates_non_ascii() {
        let translated = translate("È un’altro perché?");
        assert_eq!(translated, "È huwh’raanaorcoo akworcoaacé?");
    }

    #[test]
    fn translates_simple() {
        let translated = translate("hello world!");
        assert_eq!(translated, "acwoananoo ohoorcanwa!");
    }

    #[test]
    fn translates_mixed_case() {
        let translated = translate("Hello World!");
        assert_eq!(translated, "Acwoananoo Ohoorcanwa!");
    }

    #[test]
    fn translate_with_escape() {
        let translated = translate("hello \n\t world!");
        assert_eq!(translated, "acwoananoo \n\t ohoorcanwa!");
    }

    #[test]
    fn translate_json() {
        let translated = translate("{ hello: \n { world: 'world!' }");
        assert_eq!(translated, "{ acwoananoo: \n { ohoorcanwa: 'ohoorcanwa!' }");
    }

    #[test]
    fn capitalize_string() {
        assert_eq!(capitalize("abc"), "Abc");
    }

    #[test]
    fn capitalize_char() {
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn capitalize_empty() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn capitalize_already_capitalized() {
        assert_eq!(capitalize("Abc"), "Abc");
    }
}
