use lopdf::{Document, Object};
use std::collections::HashMap;

pub fn parse_pdf(path: &str) -> anyhow::Result<HashMap<String, String>> {
    let doc: Document = Document::load(path)?;

    let mut result: HashMap<String, String> = HashMap::new();

    let catalog: &lopdf::Dictionary = doc.catalog()?;
    let acro_form_obj: &Object = catalog.get(b"AcroForm")?;

    let acro_form_dict: &lopdf::Dictionary = match acro_form_obj {
        Object::Reference(id) => doc.get_dictionary(*id)?,
        Object::Dictionary(dict) => dict,
        _ => return Ok(result),
    };

    let fields: &Vec<Object> = acro_form_dict.get(b"Fields")?.as_array()?;

    for field in fields {
        extract_field(&doc, field, &mut result)?;
    }

    Ok(result)
}

fn extract_field(
    doc: &Document,
    field: &Object,
    result: &mut HashMap<String, String>,
) -> anyhow::Result<()> {
    let field_ref: (u32, u16) = field.as_reference()?;
    let field_dict: &lopdf::Dictionary = doc.get_dictionary(field_ref)?;

    if let Ok(kids) = field_dict.get(b"Kids")
        && let Ok(kids_array) = kids.as_array()
    {
        for kid in kids_array {
            extract_field(doc, kid, result)?;
        }
    }

    let name: String = match field_dict.get(b"T") {
        Ok(Object::String(name, _)) => String::from_utf8_lossy(name).to_string(),
        _ => return Ok(()),
    };

    let value = match field_dict.get(b"V") {
        Ok(Object::String(val, _)) => String::from_utf8_lossy(val).to_string(),
        Ok(Object::Name(val)) => String::from_utf8_lossy(val).to_string(),
        _ => "".to_string(),
    };

    result.insert(name, value);

    Ok(())
}
