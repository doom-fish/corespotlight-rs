use std::io::{Error as IoError, ErrorKind};

use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let attributes = CSSearchableItemAttributeSet::new("public.plain-text")?;
    let import_extension = CSImportExtension::new(|attributes, content_url| {
        let file_name = content_url.rsplit('/').next().unwrap_or(content_url);
        attributes.set_title(Some(file_name))?;
        attributes.set_display_name(Some("Imported from CSImportExtension"))?;
        Ok(())
    })?;

    let content_path = format!("{}/README.md", env!("CARGO_MANIFEST_DIR"));
    import_extension.simulate_update(&attributes, &content_path)?;

    let activity = NSUserActivity::new("doom-fish.corespotlight.import-extension-demo")?;
    activity.set_content_attribute_set(Some(&attributes))?;

    let attached = activity.content_attribute_set().ok_or_else(|| {
        IoError::new(
            ErrorKind::Other,
            "content attribute set was not attached to the user activity",
        )
    })?;

    println!(
        "activity type: {}",
        activity.activity_type().unwrap_or_default()
    );
    println!("imported title: {}", attached.title().unwrap_or_default());
    Ok(())
}
