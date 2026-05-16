use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use corespotlight::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CSSearchableIndex::new("doom-fish.corespotlight.delegate-demo")?;
    let seen_identifiers = Arc::new(Mutex::new(Vec::<String>::new()));
    let reindex_all_count = Arc::new(AtomicUsize::new(0));

    let delegate = CSSearchableIndexDelegate::new(
        CSSearchableIndexDelegateCallbacks::new(
            {
                let reindex_all_count = Arc::clone(&reindex_all_count);
                move |_| {
                    reindex_all_count.fetch_add(1, Ordering::SeqCst);
                }
            },
            {
                let seen_identifiers = Arc::clone(&seen_identifiers);
                move |_, identifiers| {
                    seen_identifiers.lock().unwrap().extend(identifiers);
                }
            },
        )
        .data_for_item(|_, item_identifier, type_identifier| {
            Ok(Some(format!("{item_identifier}:{type_identifier}").into_bytes()))
        }),
    )?;

    index.set_delegate(Some(&delegate))?;
    delegate.simulate_reindex_all(&index)?;
    delegate.simulate_reindex_identifiers(&index, ["one", "two"])?;
    let payload = delegate.simulate_data_request(&index, "one", "public.plain-text")?;

    println!("reindex-all count: {}", reindex_all_count.load(Ordering::SeqCst));
    println!("identifiers: {:?}", seen_identifiers.lock().unwrap());
    println!("payload: {}", String::from_utf8(payload)?);
    Ok(())
}
