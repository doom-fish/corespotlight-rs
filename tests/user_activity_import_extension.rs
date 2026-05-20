use std::io::Error as IoError;
use std::sync::{Arc, Mutex};

use corespotlight::prelude::*;

#[test]
fn user_activity_round_trips_content_attribute_sets() -> Result<(), Box<dyn std::error::Error>> {
    let activity = NSUserActivity::new("doom-fish.corespotlight.user-activity")?;
    assert_eq!(
        activity.activity_type().as_deref(),
        Some("doom-fish.corespotlight.user-activity")
    );

    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    attributes.set_title(Some("Activity Guide"))?;
    activity.set_content_attribute_set(Some(&attributes))?;

    let returned = activity.content_attribute_set().ok_or_else(|| {
        IoError::other("missing user activity content attribute set")
    })?;
    assert_eq!(returned.title().as_deref(), Some("Activity Guide"));

    activity.set_content_attribute_set(None)?;
    assert!(activity.content_attribute_set().is_none());
    Ok(())
}

#[test]
fn import_extension_simulation_updates_attributes() -> Result<(), Box<dyn std::error::Error>> {
    let seen_urls = Arc::new(Mutex::new(Vec::<String>::new()));
    let import_extension = CSImportExtension::new({
        let seen_urls = Arc::clone(&seen_urls);
        move |attributes, content_url| {
            seen_urls.lock().unwrap().push(content_url.to_string());
            let file_name = content_url.rsplit('/').next().unwrap_or(content_url);
            attributes.set_title(Some(file_name))?;
            attributes.set_display_name(Some("Imported README"))?;
            Ok(())
        }
    })?;

    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    let content_path = format!("{}/README.md", env!("CARGO_MANIFEST_DIR"));
    import_extension.simulate_update(&attributes, &content_path)?;

    assert_eq!(attributes.title().as_deref(), Some("README.md"));
    assert_eq!(
        attributes.display_name().as_deref(),
        Some("Imported README")
    );

    {
        let seen_urls = seen_urls.lock().unwrap();
        assert_eq!(seen_urls.len(), 1);
        assert!(seen_urls[0].starts_with("file://"));
        assert!(seen_urls[0].ends_with("/README.md"));
        drop(seen_urls);
    }
    Ok(())
}
