use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CSSearchableIndex::new("doom-fish.corespotlight.default-handler-demo")?;
    let handler = DefaultIndexExtensionRequestHandler::new()?;

    index.set_default_extension_request_handler(Some(&handler))?;
    handler.simulate_reindex_all(&index)?;
    handler.simulate_reindex_identifiers(&index, ["alpha", "beta"])?;
    handler.simulate_did_throttle(&index)?;
    handler.simulate_did_finish_throttle(&index)?;

    println!("reindex all count: {}", handler.reindex_all_count());
    println!("reindex identifiers count: {}", handler.reindex_identifiers_count());
    println!("last identifiers: {:?}", handler.last_identifiers()?);
    Ok(())
}
