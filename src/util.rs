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

pub fn wmic(cmd: &str) -> Result<String, String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(cmd.split(' '))
        .output()
        .map_err(|e| format!("eval wmic err {}", e.to_string()))?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
