use std;
use super::{ ReqHeader, ReqResponse };

/// Extracts all assets from the given ReqResponse to the specified directory.
/// Note that this function is blocking and may take some time to complete.
// TODO: Verify above documentation.
pub fn extract(
    directory: String, 
    response: ReqResponse) 
    -> Result<(), ExtractionError>
{
    let html_content_type = ReqHeader::new("Content-Type", "text/html");

    if !response.headers.contains(&html_content_type) {
        return Err(ExtractionError::new("Bad content type: must be text/html"));
    }

    Ok(())
}

pub struct ExtractionError {
    description: String
}

impl ExtractionError {
    pub fn new(description: &str) -> ExtractionError
    {
        ExtractionError {
            description: String::from(description)
        }
    }
}

impl std::fmt::Display for ExtractionError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "Extraction Error: {}", &self.description)
    }
}


#[cfg(test)]
mod tests {

}
