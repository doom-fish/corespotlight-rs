use std::collections::BTreeMap;
use std::time::SystemTime;

use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some("Foundation Models Guide"))?;
    attributes.set_string(
        CSSearchableItemAttributeStringField::TextContent,
        Some("Large language models, embeddings, and Spotlight."),
    )?;
    attributes.set_keywords(["rust", "spotlight", "apple"])?;
    attributes.set_number(CSSearchableItemAttributeNumberField::Rating, Some(4.5))?;
    attributes.set_url(
        CSSearchableItemAttributeURLField::ContentURL,
        Some("https://example.com/foundation-models"),
    )?;
    attributes.set_date(
        CSSearchableItemAttributeDateField::AddedDate,
        Some(SystemTime::now()),
    )?;
    attributes.set_person_array(
        CSSearchableItemAttributePersonArrayField::Authors,
        [CSPersonData {
            display_name: Some("Ada Lovelace".into()),
            handles: vec!["ada@example.com".into()],
            handle_identifier: "ada@example.com".into(),
            contact_identifier: Some("contact-ada".into()),
        }],
    )?;

    let localized = CSLocalizedString::new(&BTreeMap::from([(
        String::from("en"),
        String::from("Foundation Models Guide"),
    )]))?;
    attributes.set_localized_string(CSSearchableItemAttributeStringField::DisplayName, &localized)?;

    let custom_key = CSCustomAttributeKey::new("com.doomfish.demo.topic").ok();
    if let Some(custom_key) = custom_key.as_ref() {
        attributes.set_custom_value(custom_key, CustomAttributeValue::String("ml".into()))?;
    }

    println!("title: {:?}", attributes.title());
    println!("keywords: {:?}", attributes.keywords()?);
    println!("rating: {:?}", attributes.number(CSSearchableItemAttributeNumberField::Rating)?);
    if let Some(custom_key) = custom_key.as_ref() {
        println!("custom topic: {:?}", attributes.custom_value(custom_key)?);
    } else {
        println!("custom topic: unavailable for the current bundle identifier");
    }
    Ok(())
}
