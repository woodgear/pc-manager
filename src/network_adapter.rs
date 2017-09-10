use serde::de::Deserializer;

#[derive(new, Debug, Eq, PartialEq, Deserialize)]
pub struct NetWorkAdapter {
    #[serde(rename = "Index")]
    index: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "NetEnabled")]
    #[serde(deserialize_with = "deserialize_from_str")]
    net_enabled: Option<bool>,
}
fn deserialize_from_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::Deserialize;
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Ok(None),
    }
}

pub trait NetWorkAdapterTrait {
    //get all networkadapter
    fn find_all() -> Result<Vec<NetWorkAdapter>, String>;
    fn enable(&self) -> Result<bool, String>;
    fn disable(&self) -> Result<bool, String>;
}

mod v1 {
    use super::*;
    use util::wmic;
    use csv::Reader;

    fn from_string(csv: String) -> Vec<NetWorkAdapter> {
        let mut rdr = Reader::from_reader(csv.as_bytes());
        let iter = rdr.deserialize();
        iter.filter_map(|x| x.ok()).collect()
    }

    fn get_networkadapters_with_csv_format() -> Result<String, String> {
        let cmd = "path win32_networkadapter get index,name,NetEnabled /format:csv";
        wmic(cmd)
    }

    impl NetWorkAdapterTrait for NetWorkAdapter {
        fn find_all() -> Result<Vec<NetWorkAdapter>, String> {
            let csv = get_networkadapters_with_csv_format()?;
            Ok(from_string(csv))
        }

        fn enable(&self) -> Result<bool, String> {
            wmic(&format!(
                "path win32_networkadapter where index={} call enable",
                self.index
            )).and_then(|_| Ok(true))
        }
        fn disable(&self) -> Result<bool, String> {
            wmic(&format!(
                "path win32_networkadapter where index={} call disable",
                self.index
            )).and_then(|_| Ok(true))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_network_adapter_from_string() {
            let csv =
                r#"
        Node,Index,Name,NetEnabled

        KAKA,0,Microsoft Kernel Debug Network Adapter,

        KAKA,14,Qualcomm Atheros AR9485 Wireless Network Adapter,TRUE

        KAKA,16,Qualcomm Atheros AR8171/8175 PCI-E Gigabit Ethernet Controller (NDIS 6.30),FALSE"#;

            assert_eq!(
                from_string(csv.to_string()),
                vec![
                    NetWorkAdapter {
                        index: 0,
                        name: "Microsoft Kernel Debug Network Adapter".to_string(),
                        net_enabled: None,
                    },
                    NetWorkAdapter {
                        index: 14,
                        name: "Qualcomm Atheros AR9485 Wireless Network Adapter".to_string(),
                        net_enabled: Some(true),
                    },
                    NetWorkAdapter {
                        index: 16,
                        name: "Qualcomm Atheros AR8171/8175 PCI-E Gigabit Ethernet Controller (NDIS 6.30)"
                            .to_string(),
                        net_enabled: Some(false),
                    },
                ]
            );
        }
    }

}
