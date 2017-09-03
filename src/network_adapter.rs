#[derive(new, Debug, Eq, PartialEq)]
pub struct NetWorkAdapter {
    caption: String,
    guid: Option<String>,
    index: u32,
    name: String,
    net_enabled: Option<bool>,
}
pub use self::v1::*;

mod v1 {
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
        let v1_list = v1_res.unwrap();
    }
}
