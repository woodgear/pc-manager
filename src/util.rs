#[macro_export]
macro_rules! to_array {
    ($vec:expr,$len:expr)=>{
        if $vec.len()<$len {
            Err(format!("convert vec to array err vec len should more then {}",$len))
        }else {
            let mut a=[Default::default();$len];
            for (index,x) in $vec.into_iter().take($len).enumerate() {
                a[index]=x;
            }
            Ok(a)
        }
    }
}
fn e2s<T, E>(r: Result<T, E>) -> Result<T, String>
where
    E: ToString,
{
    r.map_err(|e| e.to_string())
}

pub mod convert {
    use util::e2s;
    pub fn to_u32(data: &str) -> Result<u32, String> {
        use std::str::FromStr;
        e2s(u32::from_str(data))
    }
    pub fn to_bool(data: &str) -> bool {
        data.to_lowercase() == "true"
    }
    pub fn to_option_bool(data: &str) -> Option<bool> {
        if data.is_empty() {
            None
        } else {
            let res: bool = to_bool(data);
            Some(res)
        }
    }
    pub fn to_option_string(data: &str) -> Option<String> {
        if data.is_empty() {
            None
        } else {
            Some(data.to_string())
        }
    }
    
}
#[cfg(test)]
mod test_convert{
    use super::convert;
    #[test]
    fn test_to_u32_should_err() {
        let data="";
        let res=convert::to_u32(data);
        assert_eq!(res.is_err(),true);
    }

    #[test]
    fn test_to_u32_should_ok() {
        let data="10";
        let res=convert::to_u32(data);
        assert_eq!(res,Ok(10));
    }

    #[test]
    fn test_to_bool() {
        let data="true";
        let res=convert::to_bool(data);
        assert_eq!(res,true);
    }

    #[test]
    fn test_to_option_bool() {
        let data="";
        let res=convert::to_option_bool(data);
        assert_eq!(res,None);
    }

    #[test]
    fn test_to_option_string() {
        let data="str";
        let res=convert::to_option_string(data);
        assert_eq!(res,Some("str".to_owned()));
    }
}

pub fn wmic(cmd: &str) -> Result<String, String> {
    use std::process::Command;
    let output = Command::new("wmic")
        .args(cmd.split(' '))
        .output()
        .map_err(|e| format!("eval wmic err {}", e.to_string()))?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

