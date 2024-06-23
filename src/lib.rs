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

pub fn translate(str: &str) -> String {
    let mut s = String::new();
    for c in str.chars() {
        let translated_string = find_translation(CHAR_MAP, c);
        let normalized_string = match translated_string {
            Some(translation) => {
                if c.is_uppercase() {
                    capitalize(&translation)
                } else {
                    translation
                }
            }
            None => c.to_string(),
        };
        s.push_str(&normalized_string);
        // do something with `c`
    }
    s
}

fn capitalize(str: &str) -> String {
    format!(
        "{}{}",
        &str.chars().nth(0).unwrap().to_uppercase(),
        &str[1..]
    )
    // String::from(str.chars().nth(0).unwrap())
    //     .to_ascii_uppercase()
    //     .push_str(str.chars())
}

pub fn find_translation<'a>(map: &'a [(char, &'a str)], needle: char) -> Option<String> {
    let found_char = map
        .iter()
        .find(|c| (c.0 == needle) || (c.0.to_ascii_uppercase() == needle));

    match found_char {
        Some(c) => Some(c.1.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
