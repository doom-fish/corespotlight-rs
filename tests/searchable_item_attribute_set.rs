mod common;

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

use common::unique_label;
use corespotlight::prelude::*;

#[test]
fn searchable_items_and_attribute_sets_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some("Guide"))?;
    attributes.set_content_description(Some("Core Spotlight guide"))?;
    attributes.set_display_name(Some("Guide Display"))?;
    attributes.set_keywords(["corespotlight", "rust"])?;
    attributes.set_number(CSSearchableItemAttributeNumberField::Rating, Some(4.5))?;
    attributes.set_url(
        CSSearchableItemAttributeURLField::ContentURL,
        Some("https://example.com/guide"),
    )?;

    let added_date = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    attributes.set_date(CSSearchableItemAttributeDateField::AddedDate, Some(added_date))?;
    attributes.set_date_array(
        CSSearchableItemAttributeDateArrayField::ImportantDates,
        [added_date + Duration::from_secs(60)],
    )?;

    let person = CSPersonData {
        display_name: Some("Ada Lovelace".into()),
        handles: vec!["ada@example.com".into()],
        handle_identifier: "ada@example.com".into(),
        contact_identifier: Some("contact-ada".into()),
    };
    attributes.set_person_array(CSSearchableItemAttributePersonArrayField::Authors, [person.clone()])?;

    let localized = CSLocalizedString::new(&BTreeMap::from([(
        String::from("en"),
        String::from("Localized display"),
    )]))?;
    attributes.set_localized_string(CSSearchableItemAttributeStringField::DisplayName, &localized)?;

    let custom_key = CSCustomAttributeKey::new("com.doomfish.tests.customTopic").ok();
    let custom_value = CustomAttributeValue::Array(vec![
        CustomAttributeValue::String("alpha".into()),
        CustomAttributeValue::Boolean(true),
    ]);
    if let Some(custom_key) = custom_key.as_ref() {
        attributes.set_custom_value(custom_key, custom_value.clone())?;
    }

    assert_eq!(attributes.title().as_deref(), Some("Guide"));
    assert_eq!(attributes.content_description().as_deref(), Some("Core Spotlight guide"));
    assert_eq!(
        attributes.keywords()?,
        vec![String::from("corespotlight"), String::from("rust")]
    );
    assert_eq!(
        attributes.number(CSSearchableItemAttributeNumberField::Rating)?,
        Some(4.5)
    );
    assert_eq!(
        attributes.url(CSSearchableItemAttributeURLField::ContentURL)?,
        Some(String::from("https://example.com/guide"))
    );
    assert_eq!(
        attributes.date(CSSearchableItemAttributeDateField::AddedDate)?,
        Some(added_date)
    );
    assert_eq!(
        attributes
            .date_array(CSSearchableItemAttributeDateArrayField::ImportantDates)?
            .len(),
        1
    );
    assert_eq!(
        attributes.person_array(CSSearchableItemAttributePersonArrayField::Authors)?,
        vec![person]
    );
    assert_eq!(localized.localized_string().as_deref(), Some("Localized display"));
    if let Some(custom_key) = custom_key.as_ref() {
        assert_eq!(attributes.custom_value(custom_key)?, custom_value);
    }

    let identifier = unique_label("item");
    let item = CSSearchableItem::new(Some(&identifier), Some("doom-fish.tests"), &attributes)?;
    assert_eq!(item.unique_identifier().as_deref(), Some(identifier.as_str()));
    assert_eq!(item.domain_identifier().as_deref(), Some("doom-fish.tests"));

    let expiration_date = SystemTime::now() + Duration::from_secs(120);
    item.set_expiration_date(Some(expiration_date))?;
    assert!(item.expiration_date().is_some());
    item.set_is_update(true);
    assert!(item.is_update());

    let listener_options = CSSearchableItemUpdateListenerOptions::SUMMARIZATION
        | CSSearchableItemUpdateListenerOptions::PRIORITY;
    item.set_update_listener_options(listener_options)?;
    let retrieved_options = item.update_listener_options()?;
    assert!(retrieved_options.contains(CSSearchableItemUpdateListenerOptions::SUMMARIZATION));
    assert!(retrieved_options.contains(CSSearchableItemUpdateListenerOptions::PRIORITY));

    let returned_attributes = item.attribute_set()?;
    assert_eq!(returned_attributes.title().as_deref(), Some("Guide"));
    Ok(())
}
