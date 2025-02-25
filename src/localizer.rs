use std::collections::HashMap;

use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! fls {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn generate_translation_hashmap(vec: Vec<(&'static str, String)>) -> HashMap<&'static str, String> {
    let mut hashmap: HashMap<&'static str, String> = Default::default();
    for (key, value) in vec {
        hashmap.insert(key, value);
    }
    hashmap
}
