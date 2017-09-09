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