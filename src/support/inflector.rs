use regex::Regex;
use std::sync::OnceLock;

pub struct Inflector;

static UNCOUNTABLE: &[&str] = &[
    "aircraft",
    "bellows",
    "bison",
    "deer",
    "equipment",
    "fish",
    "hovercraft",
    "information",
    "jeans",
    "means",
    "measles",
    "money",
    "moose",
    "news",
    "pants",
    "police",
    "rice",
    "series",
    "sheep",
    "spacecraft",
    "species",
    "swine",
    "tights",
    "tongs",
    "trousers",
];

#[derive(Clone)]
struct Rule {
    regex: Regex,
    replacement: &'static str,
}

impl Rule {
    fn new(pattern: &str, replacement: &'static str) -> Self {
        Self {
            regex: Regex::new(pattern).expect("Invalid regex pattern"),
            replacement,
        }
    }
}

fn irregular_rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        vec![
            Rule::new(r"(?i)(alumn|cact|fung|radi|stimul|syllab)i", "${1}us"),
            Rule::new(r"(?i)(alg|antenn|amoeb|larv|vertebr)ae", "${1}a"),
            Rule::new(r"(?i)^(gen)era$", "${1}us"),
            Rule::new(r"(?i)(pe)ople", "${1}rson"),
            Rule::new(r"(?i)^(zombie)s$", "$1"),
            Rule::new(r"(?i)(g)eese", "${1}oose"),
            Rule::new(r"(?i)(criteri)a", "${1}on"),
            Rule::new(r"(?i)^(m)en$", "${1}an"),
            Rule::new(r"(?i)^(echo)es", "$1"),
            Rule::new(r"(?i)^(hero)es", "$1"),
            Rule::new(r"(?i)^(potato)es", "$1"),
            Rule::new(r"(?i)^(tomato)es", "$1"),
            Rule::new(r"(?i)^(t)eeth", "${1}ooth"),
            Rule::new(r"(?i)^(l)ice$", "${1}ouse"),
            Rule::new(r"(?i)^(addend|bacteri|curricul|dat|memorand|quant)a$", "${1}um"),
            Rule::new(r"(?i)^(di)ce", "${1}e"),
            Rule::new(r"(?i)^(f)eet", "${1}oot"),
            Rule::new(r"(?i)^(phenomen)a", "${1}on"),
        ]
    })
}

fn plural_irregular_rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        vec![
            Rule::new(r"(?i)(alumn|cact|fung|radi|stimul|syllab)us", "${1}i"),
            Rule::new(r"(?i)(alg|antenn|amoeb|larv|vertebr)a", "${1}ae"),
            Rule::new(r"(?i)^(gen)us$", "${1}era"),
            Rule::new(r"(?i)(pe)rson$", "${1}ople"),
            Rule::new(r"(?i)^(zombie)s$", "$1"),
            Rule::new(r"(?i)(g)oose$", "${1}eese"),
            Rule::new(r"(?i)(criteri)on", "${1}a"),
            Rule::new(r"(?i)^(men)$", "$1"),
            Rule::new(r"(?i)^(women)", "$1"),
            Rule::new(r"(?i)^(echo)$", "${1}es"),
            Rule::new(r"(?i)^(hero)$", "${1}es"),
            Rule::new(r"(?i)^(potato)", "${1}es"),
            Rule::new(r"(?i)^(tomato)", "${1}es"),
            Rule::new(r"(?i)^(t)ooth$", "${1}eeth"),
            Rule::new(r"(?i)^(l)ouse$", "${1}ice"),
            Rule::new(r"(?i)^(addend|bacteri|curricul|dat|memorand|quant)um$", "${1}a"),
            Rule::new(r"(?i)^(di)e$", "${1}ce"),
            Rule::new(r"(?i)^(f)oot$", "${1}eet"),
            Rule::new(r"(?i)^(phenomen)on", "${1}a"),
        ]
    })
}

fn singular_rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        let mut rules = irregular_rules().clone();
        rules.extend(vec![
            Rule::new(r"(?i)(child)ren", "$1"),
            Rule::new(r"(?i)(wo|sea)men$", "${1}man"),
            Rule::new(r"(?i)^(m|l)ice$", "${1}ouse"),
            Rule::new(r"(?i)(bus|canvas|status|alias)(es)?$", "$1"),
            Rule::new(r"(?i)(ss)$", "$1"),
            Rule::new(r"(?i)(database)s$", "$1"),
            Rule::new(r"(?i)([ti])a$", "${1}um"),
            Rule::new(r"(?i)((a)naly|(b)a|(d)iagno|(p)arenthe|(p)rogno|(s)ynop|(t)he)(sis|ses)$", "${1}sis"),
            Rule::new(r"(?i)(analy)(sis|ses)$", "${1}sis"),
            Rule::new(r"(?i)(octop|vir)i$", "${1}us"),
            Rule::new(r"(?i)(hive)s$", "$1"),
            Rule::new(r"(?i)(tive)s$", "$1"),
            Rule::new(r"(?i)(er)ves$", "${1}ve"),
            Rule::new(r"(?i)([lora])ves$", "${1}f"),
            Rule::new(r"(?i)([^f])ves$", "${1}fe"),
            Rule::new(r"(?i)([^aeiouy]|qu)ies$", "${1}y"),
            Rule::new(r"(?i)(m)ovies$", "${1}ovie"),
            Rule::new(r"(?i)(x|ch|ss|sh)es$", "$1"),
            Rule::new(r"(?i)(shoe)s$", "$1"),
            Rule::new(r"(?i)(o)es$", "$1"),
            Rule::new(r"(?i)s$", ""),
        ]);
        rules
    })
}

fn plural_rules() -> &'static Vec<Rule> {
    static RULES: OnceLock<Vec<Rule>> = OnceLock::new();
    RULES.get_or_init(|| {
        let mut rules = plural_irregular_rules().clone();
        rules.extend(vec![
            Rule::new(r"(?i)(child)$", "${1}ren"),
            Rule::new(r"(?i)(m)an$", "${1}en"),
            Rule::new(r"(?i)(m|l)ouse", "${1}ice"),
            Rule::new(r"(?i)(database)s$", "$1"),
            Rule::new(r"(?i)(quiz)$", "${1}zes"),
            Rule::new(r"(?i)^(ox)$", "${1}en"),
            Rule::new(r"(?i)(matr|vert|ind)ix|ex$", "${1}ices"),
            Rule::new(r"(?i)(x|ch|ss|sh)$", "${1}es"),
            Rule::new(r"(?i)([^aeiouy]|qu)y$", "${1}ies"),
            Rule::new(r"(?i)(hive)$", "${1}s"),
            Rule::new(r"(?i)(sc[au]rf)$", "${1}s"),
            Rule::new(r"(?i)(?:([^f])fe|((hoo)|([lra]))f)$", "${2}${1}ves"),
            Rule::new(r"(?i)sis$", "ses"),
            Rule::new(r"(?i)([ti])um$", "${1}a"),
            Rule::new(r"(?i)(buffal|tomat)o$", "${1}oes"),
            Rule::new(r"(?i)(octop|vir)us$", "${1}i"),
            Rule::new(r"(?i)(bus|alias|status|canvas)$", "${1}es"),
            Rule::new(r"(?i)(ax|test)is$", "${1}es"),
            Rule::new(r"(?i)s$", "s"),
            Rule::new(r"(?i)data$", "data"),
            Rule::new(r"(?i)$", "s"),
        ]);
        rules
    })
}

impl Inflector {
    pub fn pluralize(word: &str) -> String {
        if UNCOUNTABLE.contains(&word.to_lowercase().as_str()) {
            return word.to_string();
        }

        for rule in plural_rules() {
            if rule.regex.is_match(word) {
                return rule.regex.replace(word, rule.replacement).to_string();
            }
        }

        format!("{}s", word)
    }

    pub fn singularize(word: &str) -> String {
        if UNCOUNTABLE.contains(&word.to_lowercase().as_str()) {
            return word.to_string();
        }

        for rule in singular_rules() {
            if rule.regex.is_match(word) {
                return rule.regex.replace(word, rule.replacement).to_string();
            }
        }

        if word.ends_with('s') {
            word[..word.len() - 1].to_string()
        } else {
            word.to_string()
        }
    }
}

