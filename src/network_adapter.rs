
#[derive(Debug, Eq, PartialEq)]
pub struct NetWorkAdapter {
    caption: String,
    name: String,
    index: u32,
    guid: Option<String>,
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

mod v2 {
    use super::*;
    use util::from_ors;

    impl From<Vec<String>> for NetWorkAdapter {
        fn from(list: Vec<String>) -> Self {
            NetWorkAdapter {
                caption: from_ors(list.get(1)),
                guid: from_ors(list.get(2)),
                index: from_ors(list.get(3)),
                name: from_ors(list.get(4)),
                net_enabled: from_ors(list.get(5)),
            }
        }
    }

    impl From<String> for NetWorkAdapter {
        fn from(csv: String) -> Self {
            let list: Vec<String> = csv.split("")
                .map(|item| String::from(String::from(item).trim()))
                .collect();
            NetWorkAdapter::from(list)
        }
    }

    pub fn network_adapters() -> Result<Vec<NetWorkAdapter>, String> {
        use std::process::Command;
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
                let ret: Result<NetWorkAdapter, String> = Ok(NetWorkAdapter::from(array));
                return ret;
            })
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_network_adapters() {
        let v1_res: Result<Vec<NetWorkAdapter>, String> = v1::network_adapters();
        let v2_res: Result<Vec<NetWorkAdapter>, String> = v2::network_adapters();
        assert_eq!(v1_res, v2_res);
    }
}
