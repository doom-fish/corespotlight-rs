use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CSSearchableIndex::new("doom-fish.corespotlight.request-handler-demo")?;
    let handler = CSIndexExtensionRequestHandler::new(
        CSSearchableIndexDelegateCallbacks::new(
            |_| {},
            |_, identifiers| {
                println!("reindex identifiers: {identifiers:?}");
            },
        )
        .data_for_item(|_, item_identifier, type_identifier| {
            Ok(Some(format!("payload:{item_identifier}:{type_identifier}").into_bytes()))
        })
        .file_url_for_item(|_, item_identifier, type_identifier, in_place| {
            Ok(Some(format!(
                "https://example.com/{item_identifier}/{type_identifier}?in_place={in_place}"
            )))
        }),
    )?;

    index.set_extension_request_handler(Some(&handler))?;
    handler.simulate_reindex_all(&index)?;
    let payload = handler.simulate_data_request(&index, "one", "public.plain-text")?;
    let file_url = handler.simulate_file_url_request(&index, "one", "public.plain-text", true)?;

    println!("payload: {}", String::from_utf8(payload)?);
    println!("file URL: {file_url:?}");
    Ok(())
}
