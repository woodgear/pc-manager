pub trait FromOptionReferenceString {
    fn from_option_reference_string(o: Option<&String>) -> Self;
}

impl FromOptionReferenceString for u32 {
    fn from_option_reference_string(o: Option<&String>) -> Self {
        use std::str::FromStr;
        u32::from_str(o.unwrap()).unwrap()
    }
}

impl FromOptionReferenceString for Option<bool> {
    fn from_option_reference_string(o: Option<&String>) -> Self {
        o.and_then(|o| match o.as_ref() {
            "TRUE" => Some(true),
            "FALSE" => Some(false),
            _ => None,
        })
    }
}

impl FromOptionReferenceString for String {
    fn from_option_reference_string(o: Option<&String>) -> Self {
        o.unwrap().to_owned()
    }
}

impl FromOptionReferenceString for Option<String> {
    fn from_option_reference_string(o: Option<&String>) -> Self {
        o.and_then(|x| if x.is_empty() {
            None
        } else {
            Some(x.to_owned())
        })
    }
}

pub fn from_ors<T: FromOptionReferenceString>(o: Option<&String>) -> T {
    FromOptionReferenceString::from_option_reference_string(o)
}
