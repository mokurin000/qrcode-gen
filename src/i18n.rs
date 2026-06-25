//! Internationalization support using Fluent.

use std::sync::LazyLock;

use fluent_bundle::{FluentBundle, FluentResource};
use icu::locale::fallback::LocaleFallbacker;
use icu::locale::{DataLocale, Locale, locale};

use crate::Result;

/// The set of supported locales.
static SUPPORTED_LOCALES: LazyLock<Vec<Locale>> = LazyLock::new(|| {
    vec![
        "en-US".parse().expect("static locale"),
        "zh-CN".parse().expect("static locale"),
        "zh-TW".parse().expect("static locale"),
        "hi-IN".parse().expect("static locale"),
        "es-ES".parse().expect("static locale"),
        "fr-FR".parse().expect("static locale"),
        "ar-SA".parse().expect("static locale"),
        "bn-BD".parse().expect("static locale"),
        "pt-BR".parse().expect("static locale"),
        "ru-RU".parse().expect("static locale"),
        "ur-PK".parse().expect("static locale"),
        "ja-JP".parse().expect("static locale"),
        "de-DE".parse().expect("static locale"),
        "it-IT".parse().expect("static locale"),
        "ko-KR".parse().expect("static locale"),
        "nl-NL".parse().expect("static locale"),
        "sv-SE".parse().expect("static locale"),
        "nb-NO".parse().expect("static locale"),
        "da-DK".parse().expect("static locale"),
        "fi-FI".parse().expect("static locale"),
        "pl-PL".parse().expect("static locale"),
        "pt-PT".parse().expect("static locale"),
        "el-GR".parse().expect("static locale"),
        "cs-CZ".parse().expect("static locale"),
        "hu-HU".parse().expect("static locale"),
        "ro-RO".parse().expect("static locale"),
        "he-IL".parse().expect("static locale"),
        "th-TH".parse().expect("static locale"),
        "tr-TR".parse().expect("static locale"),
        "vi-VN".parse().expect("static locale"),
        "id-ID".parse().expect("static locale"),
        "ms-MY".parse().expect("static locale"),
    ]
});

/// Resolve a system locale string to the best matching supported locale.
pub fn resolve_locale(sys_str: &str) -> Locale {
    let locale: Locale = match sys_str.parse() {
        Ok(l) => l,
        Err(_) => return locale!("en-US"),
    };

    // Direct match check
    if SUPPORTED_LOCALES.contains(&locale) {
        eprintln!("direct match: {locale}");
        return locale;
    }

    // Walk the ICU fallback chain to find the closest supported locale.
    let fallbacker = LocaleFallbacker::new();
    let mut fallbacks = fallbacker
        .for_config(Default::default())
        .fallback_for(locale.clone().into());

    loop {
        let current = fallbacks.get();
        let current_locale = current.into_locale();

        // und: fallback to en-US
        if current == &DataLocale::default() {
            break locale!("en-US");
        }

        if SUPPORTED_LOCALES
            .iter()
            .any(|locale| locale == &current_locale)
        {
            return current_locale;
        }

        fallbacks.step();
    }
}

/// Load a `FluentBundle` for the given locale string (e.g. `"en-US"`).
///
/// Embedds FTL files at compile time via `include_str!`.
pub fn load_bundle(locale_str: &str) -> Result<FluentBundle<FluentResource>> {
    let ftl_content = match locale_str {
        "en-US" => include_str!("../locales/en-US/main.ftl"),
        "zh-CN" => include_str!("../locales/zh-CN/main.ftl"),
        "zh-TW" => include_str!("../locales/zh-TW/main.ftl"),
        "hi-IN" => include_str!("../locales/hi-IN/main.ftl"),
        "es-ES" => include_str!("../locales/es-ES/main.ftl"),
        "fr-FR" => include_str!("../locales/fr-FR/main.ftl"),
        "ar-SA" => include_str!("../locales/ar-SA/main.ftl"),
        "bn-BD" => include_str!("../locales/bn-BD/main.ftl"),
        "pt-BR" => include_str!("../locales/pt-BR/main.ftl"),
        "ru-RU" => include_str!("../locales/ru-RU/main.ftl"),
        "ur-PK" => include_str!("../locales/ur-PK/main.ftl"),
        "ja-JP" => include_str!("../locales/ja-JP/main.ftl"),
        "de-DE" => include_str!("../locales/de-DE/main.ftl"),
        "it-IT" => include_str!("../locales/it-IT/main.ftl"),
        "ko-KR" => include_str!("../locales/ko-KR/main.ftl"),
        "nl-NL" => include_str!("../locales/nl-NL/main.ftl"),
        "sv-SE" => include_str!("../locales/sv-SE/main.ftl"),
        "nb-NO" => include_str!("../locales/nb-NO/main.ftl"),
        "da-DK" => include_str!("../locales/da-DK/main.ftl"),
        "fi-FI" => include_str!("../locales/fi-FI/main.ftl"),
        "pl-PL" => include_str!("../locales/pl-PL/main.ftl"),
        "pt-PT" => include_str!("../locales/pt-PT/main.ftl"),
        "el-GR" => include_str!("../locales/el-GR/main.ftl"),
        "cs-CZ" => include_str!("../locales/cs-CZ/main.ftl"),
        "hu-HU" => include_str!("../locales/hu-HU/main.ftl"),
        "ro-RO" => include_str!("../locales/ro-RO/main.ftl"),
        "he-IL" => include_str!("../locales/he-IL/main.ftl"),
        "th-TH" => include_str!("../locales/th-TH/main.ftl"),
        "tr-TR" => include_str!("../locales/tr-TR/main.ftl"),
        "vi-VN" => include_str!("../locales/vi-VN/main.ftl"),
        "id-ID" => include_str!("../locales/id-ID/main.ftl"),
        "ms-MY" => include_str!("../locales/ms-MY/main.ftl"),
        _ => include_str!("../locales/en-US/main.ftl"),
    };

    let langid = locale_str
        .parse()
        .map_err(|e| color_eyre::eyre::eyre!("invalid language identifier '{locale_str}': {e}"))?;

    let resource = FluentResource::try_new(ftl_content.to_owned())
        .map_err(|(_, e)| color_eyre::eyre::eyre!("invalid FTL for '{locale_str}': {e:?}"))?;

    let mut bundle = FluentBundle::new(vec![langid]);
    bundle
        .add_resource(resource)
        .map_err(|e| color_eyre::eyre::eyre!("failed to add resource for '{locale_str}': {e:?}"))?;

    Ok(bundle)
}

/// Helper to get the `en-US` locale for fallback.
pub fn en_us_locale() -> Locale {
    locale!("en-US")
}

#[cfg(test)]
mod tests {
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
}
