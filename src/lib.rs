pub mod header_editor;

#[cfg(test)]
mod tests {
    use header_editor::ReqHeader;
    
    #[test]
    fn validate_good_headers(){
        let headers: Vec<String> = vec![
            String::from("Content-Type: text/html"),
            String::from("HeaderName: value"),
            String::from("Header_Title: value123 jlasdfj;sjf aw@#%^@^"),
            String::from("Still-Val-I_d: this is a value"),
        ];
        for header in headers {
            let h = ReqHeader::from_string(&header);

            assert!(h.is_ok());
        }
    }

    #[test]
    #[should_panic]
    fn invalidate_bad_headers(){
        let headers: Vec<String> = vec![
            String::from("Content Type: text/html"),
            String::from("Header~Name: value"),
            String::from("Header_Title: value123 ¢jlasdfj;sjf aw@#%^@^"),
        ];
        for header in headers {
            let h = ReqHeader::from_string(&header);

            assert!(h.is_ok());
        }
    }

    #[test]
    fn it_works() {
        ()
    }
}
