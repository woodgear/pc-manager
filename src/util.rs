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

#[cfg(windows)]
pub fn wmic(cmd: &str) -> Result<String, String> {
    use c_util;
    let cmd =format!("wmic {}",cmd);
    c_util::call_cmd_slient(cmd)
}

pub fn to_string(bin:Vec<u8>)->Result<String,String>{
    use encoding::{DecoderTrap,all,EncodingRef};
    let decodelist=[all::GBK as EncodingRef,all::UTF_8 as EncodingRef];
    let mut res:Result<String,String>=Err("connot convert this bin".to_string());
    for decoder in decodelist.into_iter() {
        res = e2s(decoder.decode(&bin,DecoderTrap::Strict));
        if res.is_ok() {break}
    }
    res
}

#[cfg(test)]
#[test]
fn test_wmic() {
    let res = wmic("lalala");
    assert_eq!(res,Ok("lalala - 找不到别名。".to_string()));
}