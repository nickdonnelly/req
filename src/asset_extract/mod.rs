use std;
use std::str;
use regex::Regex;

/// Denotes the type of extractable resource
#[derive(Debug)]
pub enum ExtractionType {
    RemoteFetch,    // fetch from a url
    HTMLTag(String) // get from an html tag like script or style
}

#[derive(Debug)]
pub struct Extraction {
    pub resource_type: ExtractionType,
    pub extracted_from: Option<String>
}

impl Extraction {
    pub fn new(t: ExtractionType, from: Option<String>) -> Self
    {
        Extraction {
            resource_type: t,
            extracted_from: from 
        }
    }

    pub fn uri(&self) -> Option<String>
    {
        self.extracted_from.clone()
    }
}



/// Parses the document in the response for scripts and external resources.
// TODO: Also extract resources directly from tags. This will require a
// rudimentary XML parser.
pub fn extract_resource_list(response: &[u8]) 
    -> Result<Vec<Extraction>, ExtractionError>
{
    // Grabs src="link" and href="link" and put link into a "url" group
    let extraction_regex = Regex::new(r#"((src)|(href))="(?P<url>[^"]*)"{1}"#).unwrap();

    let body_s = match str::from_utf8(response) {
        Ok(v) => v,
        Err(_) => {
            return Err(ExtractionError::new("Unable to decode body."));
        }
    };

    let mut matches: Vec<Extraction> = Vec::new();

    for capture in extraction_regex.captures_iter(body_s) {
        let f = String::from(&capture["url"]);
        let e = Extraction::new(ExtractionType::RemoteFetch,
                                Some(f));
        matches.push(e);
    }

    Ok(matches)
}

#[derive(Debug)]
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

impl std::fmt::Display for Extraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        match &self.extracted_from {
            &Some(ref v) =>
                write!(f, "Extraction from {}", v),
            &None => 
                write!(f, "Extraction from unknown source.")
        }
    }
}


#[cfg(test)]
mod tests {
    use std;
    use super::*;


    #[test]
    pub fn parse_html()
    {
        let html_doc = include_str!("test_data/doc.html");
        let extractions = extract_resource_list(html_doc.as_bytes()).unwrap();
        let mut extraction_strings: Vec<String> = Vec::new();
        let res: Vec<String> = vec![
            "http://test.com/style.css".to_string(),
            "test.html".to_string(),
            "test.img".to_string()
        ];

        for extraction in extractions {
            extraction_strings.push(extraction.extracted_from.unwrap());
        }

        assert_eq!(res, extraction_strings);
    }
}
