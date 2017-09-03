#[derive(new, Debug, Eq, PartialEq)]
pub struct NetWorkAdapter {
    caption: String,
    guid: Option<String>,
    index: u32,
    name: String,
    net_enabled: Option<bool>,
}
pub use self::v2::*;
mod v1 {
    use super::*;

    #[allow(dead_code)]
    pub fn network_adapters() -> Result<Vec<NetWorkAdapter>, String> {
        use std::process::Command;
        use std::str::FromStr;

        let cmd = "path win32_networkadapter get caption,guid,index,name,NetEnabled /format:csv";
        let output = Command::new("wmic")
            .args(cmd.split(' '))
            .output()
            .expect("failed to execute wmic");
        let out_str = String::from_utf8(output.stdout).unwrap();
        let out: Vec<NetWorkAdapter> = out_str
            .lines()
            .skip(2)
            .map(|line| {
                let array: Vec<String> = line.split(",")
                    .map(|item| String::from(String::from(item).trim()))
                    .collect();

                let caption = array.get(1).unwrap();
                let guid = array.get(2).and_then(|x| if !x.is_empty() {
                    Some(x.to_owned())
                } else {
                    None
                });
                let index = array.get(3).unwrap();
                let index: u32 = u32::from_str(index).unwrap();
                let name = array.get(4).unwrap();
                let net_enabled: Option<bool> =
                    array.get(5).and_then(|x: &String| match x.as_ref() {
                        "TRUE" => Some(true),
                        "FALSE" => Some(false),
                        _ => None,
                    });
                let ret: Result<NetWorkAdapter, String> = Ok(NetWorkAdapter {
                    caption: caption.to_owned(),
                    guid: guid.to_owned(),
                    index: index,
                    net_enabled: net_enabled,
                    name: name.to_owned(),
                });
                return ret;
            })
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        Ok(out)
    }
}
#[macro_use]
mod v2 {
    use super::*;
    use util::wmic;
    use util::convert;

    //get all win32_networkadapter
    pub fn network_adapters() -> Result<Vec<NetWorkAdapter>, String> {
        let cmd = "path win32_networkadapter get caption,name,index,guid,NetEnabled /format:csv";
        let out: Vec<NetWorkAdapter> = wmic(cmd)?
            .lines()
            .skip(2)//skip empty line and column name line
            .map(|line| {
                let args:Vec<&str>=line.split(",")
                    .skip(1) //skip node name
                    .map(|item| item.trim())
                    .collect();                
                let args=to_array!(args,5)?;
                let ret: Result<NetWorkAdapter, String> = NetWorkAdapter::try_from(args);
                ret
            })
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        Ok(out)
    }

    impl NetWorkAdapter {
        pub fn try_from(args: [&str; 5]) -> Result<NetWorkAdapter, String> {
            let caption = args[0].to_string();
            let guid = convert::to_option_string(args[1]);
            let index = convert::to_u32(args[2])?;
            let name = args[3].to_string();
            let net_enabled = convert::to_option_bool(args[4]);
            Ok(NetWorkAdapter::new(caption, guid, index, name, net_enabled))
        }
        pub fn enable(&self) -> Result<String, String> {
            wmic(&format!(
                "path win32_networkadapter where index={} call enable",
                self.index
            ))
        }

        pub fn disable(&self) -> Result<String, String> {
            wmic(&format!(
                "path win32_networkadapter where index={} call disable",
                self.index
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_network_adapters() {
        let v1_res: Result<Vec<NetWorkAdapter>, String> = v1::network_adapters();
        let v2_res: Result<Vec<NetWorkAdapter>, String> = v2::network_adapters();
        let v1_list = v1_res.unwrap();
        let v2_list = v2_res.unwrap();
        for i in 0..v1_list.len() {
            assert_eq!(v1_list[i], v2_list[i]);
        }
    }
}
