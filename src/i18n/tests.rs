use super::*;
use icu_locale::langid;

// --- Locale resolution ---

#[test]
fn exact_match() {
    assert_eq!(resolve_locale("en-US"), langid!("en-US"));
    assert_eq!(resolve_locale("zh-CN"), langid!("zh-CN"));
    assert_eq!(resolve_locale("zh-TW"), langid!("zh-TW"));
    assert_eq!(resolve_locale("hi-IN"), langid!("hi-IN"));
    assert_eq!(resolve_locale("es-ES"), langid!("es-ES"));
    assert_eq!(resolve_locale("fr-FR"), langid!("fr-FR"));
    assert_eq!(resolve_locale("ar-SA"), langid!("ar-SA"));
    assert_eq!(resolve_locale("bn-BD"), langid!("bn-BD"));
    assert_eq!(resolve_locale("pt-BR"), langid!("pt-BR"));
    assert_eq!(resolve_locale("ru-RU"), langid!("ru-RU"));
    assert_eq!(resolve_locale("ur-PK"), langid!("ur-PK"));
    assert_eq!(resolve_locale("ja-JP"), langid!("ja-JP"));
    assert_eq!(resolve_locale("de-DE"), langid!("de-DE"));
    assert_eq!(resolve_locale("it-IT"), langid!("it-IT"));
    assert_eq!(resolve_locale("ko-KR"), langid!("ko-KR"));
    assert_eq!(resolve_locale("nl-NL"), langid!("nl-NL"));
    assert_eq!(resolve_locale("sv-SE"), langid!("sv-SE"));
    assert_eq!(resolve_locale("nb-NO"), langid!("nb-NO"));
    assert_eq!(resolve_locale("da-DK"), langid!("da-DK"));
    assert_eq!(resolve_locale("fi-FI"), langid!("fi-FI"));
    assert_eq!(resolve_locale("pl-PL"), langid!("pl-PL"));
    assert_eq!(resolve_locale("pt-PT"), langid!("pt-PT"));
    assert_eq!(resolve_locale("el-GR"), langid!("el-GR"));
    assert_eq!(resolve_locale("cs-CZ"), langid!("cs-CZ"));
    assert_eq!(resolve_locale("hu-HU"), langid!("hu-HU"));
    assert_eq!(resolve_locale("ro-RO"), langid!("ro-RO"));
    assert_eq!(resolve_locale("he-IL"), langid!("he-IL"));
    assert_eq!(resolve_locale("th-TH"), langid!("th-TH"));
    assert_eq!(resolve_locale("tr-TR"), langid!("tr-TR"));
    assert_eq!(resolve_locale("vi-VN"), langid!("vi-VN"));
    assert_eq!(resolve_locale("id-ID"), langid!("id-ID"));
    assert_eq!(resolve_locale("ms-MY"), langid!("ms-MY"));
}

#[test]
fn zh_alias_name() {
    assert_eq!(resolve_locale("zh-Hans-CN"), langid!("zh-CN"));
    assert_eq!(resolve_locale("zh-Hant-TW"), langid!("zh-TW"));
}

#[test]
fn zh_fuzzy_match() {
    assert_eq!(resolve_locale("zh-SG"), langid!("zh-CN"));
    assert_eq!(resolve_locale("zh-Hans-SG"), langid!("zh-CN"));
    assert_eq!(resolve_locale("zh-MY"), langid!("zh-CN"));
    assert_eq!(resolve_locale("zh-Hans-MY"), langid!("zh-CN"));

    assert_eq!(resolve_locale("zh-HK"), langid!("zh-TW"));
    assert_eq!(resolve_locale("zh-Hant-HK"), langid!("zh-TW"));
    assert_eq!(resolve_locale("zh-MO"), langid!("zh-TW"));
    assert_eq!(resolve_locale("zh-Hant-MO"), langid!("zh-TW"));
}

#[test]
fn fallback_to_en_us() {
    assert_eq!(resolve_locale("en-GB"), langid!("en-US"));
    assert_eq!(resolve_locale("en-MY"), langid!("en-US"));
    assert_eq!(resolve_locale("en-SG"), langid!("en-US"));
    assert_eq!(resolve_locale("en-AU"), langid!("en-US"));
}

#[test]
fn fallback_garbage_to_en_us() {
    assert_eq!(resolve_locale("not-a-langid!"), langid!("en-US"));
}

// --- Bundle loading ---

#[test]
fn load_bundles() {
    for locale in &SUPPORTED_LOCALES {
        let locale_str = locale.to_string();
        let bundle = load_bundle(&locale_str);
        assert!(
            bundle.is_ok(),
            "{locale_str} bundle should load: {:?}",
            bundle.err()
        );
    }
}
