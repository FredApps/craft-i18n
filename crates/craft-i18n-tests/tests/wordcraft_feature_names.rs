//! Descriptive names cover every locale currently offered by WordCraft. Legacy rows remain
//! available to earlier app pins; advancing WordCraft's pin is a separate app change.

const LOCALES: &[&str] = &[
    "ar", "bg", "cs", "da", "de", "es", "et", "fr", "hr", "hu", "it", "ja", "ko", "nl", "pl",
    "pt-br", "ro", "ru", "sl", "sr", "sr-latn", "sv", "tr", "uk", "vi", "zh-hans", "zh-hant",
];

const SOURCES: &[(&str, &str)] = &[
    ("", "Diagram"),
    ("", "Insert Diagram"),
    ("", "Diagram Design"),
    ("", "Diagram Styles"),
    ("", "Decorative Text"),
    ("", "Decorative Text Styles"),
    ("", "select a diagram first"),
    (
        "",
        "this diagram was made in another program: WordCraft shows it but can't edit it",
    ),
    ("", "this diagram can't be edited"),
    ("@msg", "a diagram holds at most {MAX_ITEMS} items"),
    ("@msg", "unknown decorative text style `{id}`"),
    ("", "select a shape or decorative text first"),
];

#[test]
fn wordcraft_descriptive_feature_names_cover_offered_locales() {
    for locale in LOCALES {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../locales/{locale}/wordcraft.tsv"));
        let text = std::fs::read_to_string(path).unwrap();
        for (context, source) in SOURCES {
            let prefix = format!("{context}\t{source}\t");
            let translations: Vec<_> = text
                .lines()
                .filter_map(|line| line.strip_prefix(&prefix))
                .collect();
            assert_eq!(translations.len(), 1, "{locale}: {source}");
            assert!(!translations[0].is_empty(), "{locale}: {source}");
            assert!(!translations[0].contains("SmartArt"), "{locale}: {source}");
            assert!(!translations[0].contains("WordArt"), "{locale}: {source}");
        }
        for old_source in [
            "SmartArt",
            "SmartArt Design",
            "SmartArt Styles",
            "Insert SmartArt",
            "WordArt",
            "WordArt Styles",
        ] {
            assert!(
                text.lines()
                    .any(|line| line.starts_with(&format!("\t{old_source}\t"))),
                "legacy {locale}: {old_source}"
            );
        }
    }
}

// Shared glossary with GridCraft's naming change.
// These values make deliberate wording changes visible in both apps' reviews. The catalog
// checker remains self-contained; it must not read another repository at test time.
const GRIDCRAFT_SHARED_GLOSSARY: &[(&str, &str, &str)] = &[
    ("cs", "Diagram", "Ozdobný text"),
    ("de", "Diagramm", "Dekorativer Text"),
    ("es", "Diagrama", "Texto decorativo"),
    ("fr", "Diagramme", "Texte décoratif"),
    ("it", "Diagramma", "Testo decorativo"),
    ("ja", "図解", "装飾文字"),
    ("ko", "다이어그램", "장식 텍스트"),
    ("nl", "Diagram", "Decoratieve tekst"),
    ("pl", "Diagram", "Tekst ozdobny"),
    ("pt-br", "Diagrama", "Texto decorativo"),
    ("ro", "Diagramă", "Text decorativ"),
    ("ru", "Схема", "Декоративный текст"),
    ("sr-latn", "Dijagram", "Ukrasni tekst"),
    ("tr", "Diyagram", "Dekoratif Metin"),
    ("uk", "Діаграма", "Декоративний текст"),
    ("vi", "Sơ đồ", "Chữ trang trí"),
    ("zh-hans", "图示", "装饰文字"),
];

#[test]
fn wordcraft_shared_feature_terms_match_the_suite_glossary() {
    for (locale, diagram, decorative_text) in GRIDCRAFT_SHARED_GLOSSARY {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../locales/{locale}/wordcraft.tsv"));
        let text = std::fs::read_to_string(path).unwrap();
        for (source, expected) in [("Diagram", diagram), ("Decorative Text", decorative_text)] {
            let prefix = format!("\t{source}\t");
            let actual = text.lines().find_map(|line| line.strip_prefix(&prefix));
            assert_eq!(actual, Some(*expected), "{locale}: {source}");
        }
    }
}
