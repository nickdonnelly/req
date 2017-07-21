extern crate ascii;
//use std::ascii::AsciiExt;
use super::ReqHeader;
use self::ascii::AsciiString;

pub fn is_valid_header(header: &ReqHeader) -> bool {
    if !is_valid_field_name(&header.field){
        return false;
    }else if !is_valid_field_body(&header.value) {
        return false;
    }
    true
}

pub fn is_valid_field_name(field: &String) -> bool {
    // No whitespace allowed in field names
    if field.contains(" ") || field.contains("\n") || field.contains("\r") {
        return false;
    }

    // No non-ascii characters
    let ascii_fieldname = AsciiString::from_ascii(field.clone().into_bytes());
    if ascii_fieldname.is_err() {
        return false;
    }

    true
}

pub fn is_valid_field_body(body: &String) -> bool {
    let ascii_body = AsciiString::from_ascii(body.clone().into_bytes());
    if ascii_body.is_err() {
        return false;
    }

    true
}
