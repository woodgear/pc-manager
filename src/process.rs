#[derive(new, Debug, Eq, PartialEq, Deserialize)]
pub struct Process {
    #[serde(rename = "CreationDate")]
    creation_date: String,
    #[serde(rename = "ExecutablePath")]
    executable_path: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ProcessId")]
    process_id: String,
}

trait ProcessTrait {
    fn find_all() -> Result<Vec<Process>, String>;
    fn find_by_name(name: &str) -> Result<Vec<Process>, String>;
    fn terminate_by_name(name: &str) -> Result<String, String>;
    fn terminate(&self) -> Result<String, String>;
}

pub use self::v1::*;

mod v1 {
    use super::*;
    use util::wmic;

    fn from_string(csv: String) -> Vec<Process> {
        use csv::Reader;
        let mut rdr = Reader::from_reader(csv.as_bytes());
        let iter = rdr.deserialize();
        iter.filter_map(|x| x.ok()).collect()
    }

    impl ProcessTrait for Process {
        fn find_all() -> Result<Vec<Process>, String> {
            let cmd = "path win32_process get CreationDate,ExecutablePath,Name,ProcessId /format:csv";
            let csv = wmic(cmd)?;
            Ok(from_string(csv))
        }

        fn find_by_name(name: &str) -> Result<Vec<Process>, String> {
            Process::find_all().and_then(|list: Vec<Process>| {
                let list = list.into_iter().filter(|p| p.name.contains(name)).collect();
                Ok(list)
            })
        }

        fn terminate_by_name(name: &str) -> Result<String, String> {
            let cmd = format!("path win32_process where name='{}' call Terminate", name);
            wmic(&cmd)
        }

        fn terminate(&self) -> Result<String, String> {
            let cmd = format!(
                "path win32_process where processid='{}' call Terminate",
                self.process_id
            );
            wmic(&cmd)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_all() {
        let v1_res: Result<Vec<Process>, String> = Process::find_all();
        let v1_list = v1_res.unwrap();
        println!("{:#?}", v1_list);
    }
    #[test]
    fn test_find_by_name() {
        let v1_res: Vec<Process> = Process::find_by_name("notepad.exe").unwrap();
        println!("{:#?}", v1_res);
    }

    #[test]
    fn terminate_by_name() {
        let _ = Process::terminate_by_name("notepad.exe");
    }
}
