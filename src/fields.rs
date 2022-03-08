use crate::add_file_header;
use crate::comm::*;
use crate::structs::{singe_struct_content, struct_content};
use syn::Item;

#[test]
fn fields() -> Result<()> {
    log_setting();
    let items = parse_file(add_file_header(singe_struct_content()))?;
    // debug!("{:#?}", items);
    for ref ite in items.items {
        if let Item::Struct(structs) = ite {
            for field in &structs.fields {
                debug!("{:#?}", field);
            }
        }
    }

    Ok(())
}
