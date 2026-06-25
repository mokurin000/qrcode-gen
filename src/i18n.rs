//! Internationalization support using Fluent.

use std::sync::LazyLock;

use fluent_bundle::{FluentBundle, FluentResource};
use icu::locale::fallback::LocaleFallbacker;
use icu::locale::{DataLocale, Locale, locale};
use unic_langid::LanguageIdentifier;

use crate::Result;

/// The set of supported locales.
static SUPPORTED_LOCALES: LazyLock<Vec<Locale>> = LazyLock::new(|| {
    vec![
        "en-US".parse().expect("static locale"),
        "zh-CN".parse().expect("static locale"),
        "zh-TW".parse().expect("static locale"),
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

        // und: fallback to en-US
        if current == &DataLocale::default() {
            break locale!("en-US");
        }

        match current.to_string().as_str() {
            _ if SUPPORTED_LOCALES
                .iter()
                .any(|locale| locale == &current.into_locale()) =>
            {
                return current.into_locale();
            }
            "en" => return locale!("en-US"),
            "zh" => return locale!("zh-CN"),
            _ => {}
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
        _ => include_str!("../locales/en-US/main.ftl"),
    };

    let langid: LanguageIdentifier = locale_str
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
    fn exact_match_en_us() {
        assert_eq!(resolve_locale("en-US"), locale!("en-US"));
    }

    #[test]
    fn exact_match_zh_cn() {
        assert_eq!(resolve_locale("zh-CN"), locale!("zh-CN"));
        assert_eq!(resolve_locale("zh-Hans-CN"), locale!("zh-CN"));
    }

    #[test]
    fn exact_match_zh_tw() {
        assert_eq!(resolve_locale("zh-TW"), locale!("zh-TW"));
        assert_eq!(resolve_locale("zh-Hant-TW"), locale!("zh-TW"));
    }

    #[test]
    fn fallback_en_gb_to_en_us() {
        assert_eq!(resolve_locale("en-GB"), locale!("en-US"));
    }

    #[test]
    fn fallback_unknown_to_en_us() {
        assert_eq!(resolve_locale("fr-FR"), locale!("en-US"));
    }

    #[test]
    fn fallback_garbage_to_en_us() {
        assert_eq!(resolve_locale("not-a-locale!"), locale!("en-US"));
    }

    // --- Bundle loading ---

    #[test]
    fn load_bundle_en_us() {
        let bundle = load_bundle("en-US");
        assert!(
            bundle.is_ok(),
            "en-US bundle should load: {:?}",
            bundle.err()
        );
    }

    #[test]
    fn load_bundle_zh_cn() {
        let bundle = load_bundle("zh-CN");
        assert!(
            bundle.is_ok(),
            "zh-CN bundle should load: {:?}",
            bundle.err()
        );
    }

    #[test]
    fn load_bundle_zh_tw() {
        let bundle = load_bundle("zh-TW");
        assert!(
            bundle.is_ok(),
            "zh-TW bundle should load: {:?}",
            bundle.err()
        );
    }
}
