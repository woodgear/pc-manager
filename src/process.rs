#[derive(new, Debug, Eq, PartialEq)]
pub struct Process {
    creation_date: String,
    executable_path: String,
    name: String,
    process_id:String,
}

trait WMIProcess {
    fn find_all()->Result<Vec<Process>, String>;
    fn find_by_name(name:&str)->Result<Vec<Process>, String>;
    fn terminate_by_name(name:&str)->Result<String,String>;
    fn terminate(&self)->Result<String,String>;
}

pub use self::v1::*;

mod v1{
    use super::*;
    use util::wmic;
    impl Process {
        pub fn try_from(args: [&str; 4]) -> Result<Process, String> {
            let creation_date = args[0].to_string();
            let executable_path =args[1].to_string() ;
            let name = args[2].to_string();
            let process_id = args[3].to_string();
            Ok(Process::new(creation_date, executable_path, name, process_id))
        }
    }

    impl WMIProcess for Process {
        fn find_all() -> Result<Vec<Process>, String> {
            let cmd = "path win32_process get CreationDate,ExecutablePath,Name,ProcessId /format:csv";
            let out: Vec<Process> = wmic(cmd)?
                .lines()
                .skip(2)//skip empty line and column name line
                .map(|line| {
                    let args:Vec<&str>=line.split(",")
                        .skip(1) //skip node name
                        .map(|item| item.trim())
                        .collect();                
                    let args=to_array!(args,4)?;
                    let ret: Result<Process, String> = Process::try_from(args);
                    ret
                })
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();
            Ok(out)
        }

        fn find_by_name(name:&str)->Result<Vec<Process>, String>{
            Process::find_all().and_then(|list:Vec<Process>|{
                let list= list
                .into_iter()
                .filter(|p|p.name.contains(name))
                .collect();
                Ok(list)
            })
        }
        
        fn terminate_by_name(name:&str)->Result<String,String>{
            let cmd=format!("path win32_process where name='{}' call Terminate",name);
            wmic(&cmd)
        }

        fn terminate(&self)->Result<String,String>{
            let cmd=format!("path win32_process where processid='{}' call Terminate",self.process_id);
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
        println!("{:#?}",v1_list);
    }
    #[test]
    fn test_find_by_name() {
        let v1_res: Vec<Process> = Process::find_by_name("notepad.exe").unwrap();
        println!("{:#?}",v1_res);
    }

    #[test]
    fn terminate_by_name() {
        Process::terminate_by_name("notepad.exe");
    }
}
