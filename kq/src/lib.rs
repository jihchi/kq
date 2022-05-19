use kdl::KdlDocument;
use nom::combinator::all_consuming;
use nom::Finish;

mod parser;

pub fn select(input: &str, document: KdlDocument) -> Result<KdlDocument, String> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(document);
    }

    let (_input, selector) = all_consuming(parser::selector)(input)
        .finish()
        .map_err(|err| err.to_string())?;

    Ok(document)
}

