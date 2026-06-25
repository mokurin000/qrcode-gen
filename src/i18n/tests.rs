use super::*;
use icu::locale::locale;

// --- Locale resolution ---

#[test]
fn exact_match() {
    assert_eq!(resolve_locale("en-US"), locale!("en-US"));
    assert_eq!(resolve_locale("zh-CN"), locale!("zh-CN"));
    assert_eq!(resolve_locale("zh-TW"), locale!("zh-TW"));
    assert_eq!(resolve_locale("hi-IN"), locale!("hi-IN"));
    assert_eq!(resolve_locale("es-ES"), locale!("es-ES"));
    assert_eq!(resolve_locale("fr-FR"), locale!("fr-FR"));
    assert_eq!(resolve_locale("ar-SA"), locale!("ar-SA"));
    assert_eq!(resolve_locale("bn-BD"), locale!("bn-BD"));
    assert_eq!(resolve_locale("pt-BR"), locale!("pt-BR"));
    assert_eq!(resolve_locale("ru-RU"), locale!("ru-RU"));
    assert_eq!(resolve_locale("ur-PK"), locale!("ur-PK"));
    assert_eq!(resolve_locale("ja-JP"), locale!("ja-JP"));
    assert_eq!(resolve_locale("de-DE"), locale!("de-DE"));
    assert_eq!(resolve_locale("it-IT"), locale!("it-IT"));
    assert_eq!(resolve_locale("ko-KR"), locale!("ko-KR"));
    assert_eq!(resolve_locale("nl-NL"), locale!("nl-NL"));
    assert_eq!(resolve_locale("sv-SE"), locale!("sv-SE"));
    assert_eq!(resolve_locale("nb-NO"), locale!("nb-NO"));
    assert_eq!(resolve_locale("da-DK"), locale!("da-DK"));
    assert_eq!(resolve_locale("fi-FI"), locale!("fi-FI"));
    assert_eq!(resolve_locale("pl-PL"), locale!("pl-PL"));
    assert_eq!(resolve_locale("pt-PT"), locale!("pt-PT"));
    assert_eq!(resolve_locale("el-GR"), locale!("el-GR"));
    assert_eq!(resolve_locale("cs-CZ"), locale!("cs-CZ"));
    assert_eq!(resolve_locale("hu-HU"), locale!("hu-HU"));
    assert_eq!(resolve_locale("ro-RO"), locale!("ro-RO"));
    assert_eq!(resolve_locale("he-IL"), locale!("he-IL"));
    assert_eq!(resolve_locale("th-TH"), locale!("th-TH"));
    assert_eq!(resolve_locale("tr-TR"), locale!("tr-TR"));
    assert_eq!(resolve_locale("vi-VN"), locale!("vi-VN"));
    assert_eq!(resolve_locale("id-ID"), locale!("id-ID"));
    assert_eq!(resolve_locale("ms-MY"), locale!("ms-MY"));
}

#[test]
fn zh_fuzzy_match() {
    assert_eq!(resolve_locale("zh-Hans-CN"), locale!("zh-CN"));
    assert_eq!(resolve_locale("zh-Hant-TW"), locale!("zh-TW"));
}

#[test]
fn fallback_to_en_us() {
    assert_eq!(resolve_locale("en-GB"), locale!("en-US"));
    assert_eq!(resolve_locale("en-AU"), locale!("en-US"));
}

#[test]
fn fallback_garbage_to_en_us() {
    assert_eq!(resolve_locale("not-a-locale!"), locale!("en-US"));
}

// --- Bundle loading ---

#[test]
fn load_bundles() {
    let bundle = load_bundle("en-US");
    assert!(
        bundle.is_ok(),
        "en-US bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("zh-CN");
    assert!(
        bundle.is_ok(),
        "zh-CN bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("zh-TW");
    assert!(
        bundle.is_ok(),
        "zh-TW bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("hi-IN");
    assert!(
        bundle.is_ok(),
        "hi-IN bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("es-ES");
    assert!(
        bundle.is_ok(),
        "es-ES bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("fr-FR");
    assert!(
        bundle.is_ok(),
        "fr-FR bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ar-SA");
    assert!(
        bundle.is_ok(),
        "ar-SA bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("bn-BD");
    assert!(
        bundle.is_ok(),
        "bn-BD bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("pt-BR");
    assert!(
        bundle.is_ok(),
        "pt-BR bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ru-RU");
    assert!(
        bundle.is_ok(),
        "ru-RU bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ur-PK");
    assert!(
        bundle.is_ok(),
        "ur-PK bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ja-JP");
    assert!(
        bundle.is_ok(),
        "ja-JP bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("de-DE");
    assert!(
        bundle.is_ok(),
        "de-DE bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("it-IT");
    assert!(
        bundle.is_ok(),
        "it-IT bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ko-KR");
    assert!(
        bundle.is_ok(),
        "ko-KR bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("nl-NL");
    assert!(
        bundle.is_ok(),
        "nl-NL bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("sv-SE");
    assert!(
        bundle.is_ok(),
        "sv-SE bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("nb-NO");
    assert!(
        bundle.is_ok(),
        "nb-NO bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("da-DK");
    assert!(
        bundle.is_ok(),
        "da-DK bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("fi-FI");
    assert!(
        bundle.is_ok(),
        "fi-FI bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("pl-PL");
    assert!(
        bundle.is_ok(),
        "pl-PL bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("pt-PT");
    assert!(
        bundle.is_ok(),
        "pt-PT bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("el-GR");
    assert!(
        bundle.is_ok(),
        "el-GR bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("cs-CZ");
    assert!(
        bundle.is_ok(),
        "cs-CZ bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("hu-HU");
    assert!(
        bundle.is_ok(),
        "hu-HU bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ro-RO");
    assert!(
        bundle.is_ok(),
        "ro-RO bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("he-IL");
    assert!(
        bundle.is_ok(),
        "he-IL bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("th-TH");
    assert!(
        bundle.is_ok(),
        "th-TH bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("tr-TR");
    assert!(
        bundle.is_ok(),
        "tr-TR bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("vi-VN");
    assert!(
        bundle.is_ok(),
        "vi-VN bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("id-ID");
    assert!(
        bundle.is_ok(),
        "id-ID bundle should load: {:?}",
        bundle.err()
    );
    let bundle = load_bundle("ms-MY");
    assert!(
        bundle.is_ok(),
        "ms-MY bundle should load: {:?}",
        bundle.err()
    );
}
