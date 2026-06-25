//! Internationalization support using Fluent.

use std::sync::LazyLock;

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
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

/// Format a localized message from a Fluent bundle.
pub(crate) fn format_ftl(
    bundle: &FluentBundle<FluentResource>,
    msg_id: &str,
    args: Option<&FluentArgs<'_>>,
) -> String {
    let mut errors = Vec::new();
    let msg = bundle
        .get_message(msg_id)
        .unwrap_or_else(|| panic!("missing fluent message '{msg_id}'"));
    let pattern = msg
        .value()
        .unwrap_or_else(|| panic!("fluent message '{msg_id}' has no value"));
    bundle
        .format_pattern(pattern, args, &mut errors)
        .into_owned()
}

#[cfg(test)]
mod tests;
