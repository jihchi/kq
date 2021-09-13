use kdl::{KdlError, KdlNode};

pub fn query_document<I>(_inpnt: I, raw_document: Vec<KdlNode>) -> Result<Vec<KdlNode>, KdlError>
where
    I: AsRef<str>,
{
    Ok(raw_document)
}
