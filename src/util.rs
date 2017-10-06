

use subprocess::{Exec,Redirection};

pub fn call_cmd_slient(cmd: String) -> Result<String, String> {
    let out = Exec::shell(cmd)
        .stdout(Redirection::Pipe)
        .capture().map_err(|e|e.to_string())?
        .stdout_str();
    return Ok(out.trim().to_owned());
}

#[test]
fn test_call_cmd_slient() {
    let out = call_cmd_slient("echo test".to_owned());
    assert_eq!(out, Ok("test".to_string()));
}

fn e2s<T, E>(r: Result<T, E>) -> Result<T, String>
where
    E: ToString,
{
    r.map_err(|e| e.to_string())
}

#[cfg(windows)]
pub fn wmic(cmd: &str) -> Result<String, String> {
    let cmd = format!("wmic {}", cmd);
    call_cmd_slient(cmd)
}

fn to_string(bin: Vec<u8>) -> Result<String, String> {
    use encoding::{DecoderTrap, all, EncodingRef};
    let decodelist = [all::GBK as EncodingRef, all::UTF_8 as EncodingRef];
    let mut res: Result<String, String> = Err("connot convert this bin".to_string());
    for decoder in decodelist.into_iter() {
        res = e2s(decoder.decode(&bin, DecoderTrap::Strict));
        if res.is_ok() {
            break;
        }
    }
    res
}

#[cfg(test)]
#[test]
fn test_wmic() {
    let res = wmic("lalala");
    assert_eq!(res, Ok("lalala - 找不到别名。".to_string()));
}
