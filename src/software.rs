use util;

#[derive(Debug, Clone)]
enum Type {
    X86,
    X64,
}

#[derive(Debug, Clone)]
enum Owner {
    Machine,
    User,
}

#[derive(Debug)]
pub struct Software {
    display_name: String,
    display_version: Option<String>,
    publisher: Option<String>,
    uninstall_str: Option<String>,
    install_date: Option<String>,
    install_location: Option<String>,
    software_type: Type,
    software_owner: Owner,
}

trait SoftwareTrait {
    fn find_all() -> Result<Vec<Software>, String>;
    fn uninstall(&self) -> Result<bool, String>;
}

fn to_option<E>(item: Result<String, E>) -> Option<String> {
    item.ok().and_then(|x: String| if x.is_empty() {
        None
    } else {
        Some(x)
    })
}

use winreg::RegKey;
fn get_software_by_reg_key(
    subkey: RegKey,
    software_type: Type,
    software_owner: Owner,
) -> Vec<Software> {
    use winreg::enums::KEY_READ;
    let softwarelist: Vec<Software> = subkey
        .enum_keys()
        .filter_map(|x| x.ok())
        .map(|key| {
            let item = subkey
                .open_subkey_with_flags(key.clone(), KEY_READ)
                .unwrap();
            let display_name: String = item.get_value("DisplayName").unwrap_or(key.to_string());
            let display_version = to_option(item.get_value("DisplayVersion"));
            let publisher = to_option(item.get_value("Publisher"));
            let uninstall_str = to_option(item.get_value("UninstallString"));
            let install_date = to_option(item.get_value("InstallDate"));
            let install_location = to_option(item.get_value("InstallLocation"));

            Software {
                display_name: display_name,
                display_version: display_version,
                publisher: publisher,
                uninstall_str: uninstall_str,
                install_date: install_date,
                install_location: install_location,
                software_type: software_type.clone(),
                software_owner: software_owner.clone(),
            }
        })
        .collect();
    return softwarelist;
}

impl SoftwareTrait for Software {
    fn find_all() -> Result<Vec<Software>, String> {
        use winreg::enums::*;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let lm_x86 = hklm.open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        ).unwrap();
        let lm_x64 = hklm.open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ | KEY_WOW64_64KEY,
        ).unwrap();

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let cu_x86 = hkcu.open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        ).unwrap();
        let cu_x64 = hkcu.open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ | KEY_WOW64_64KEY,
        ).unwrap();


        let mut lm_x86 = get_software_by_reg_key(lm_x86, Type::X86, Owner::Machine);
        let mut lm_x64 = get_software_by_reg_key(lm_x64, Type::X64, Owner::Machine);
        let mut cu_x86 = get_software_by_reg_key(cu_x86, Type::X86, Owner::User);
        let mut cu_x64 = get_software_by_reg_key(cu_x64, Type::X64, Owner::User);
        let mut list = vec![];
        list.append(&mut lm_x86);
        list.append(&mut lm_x64);
        list.append(&mut cu_x86);
        list.append(&mut cu_x64);
        Ok(list)
    }
    fn uninstall(&self) -> Result<bool, String> {
        self.uninstall_str
            .clone()
            .ok_or("not found uninstall string".to_owned())
            .and_then(|cmd| util::call_cmd_slient(cmd))
            .and_then(|_| Ok(true))
    }
}

#[test]
fn test_software_find_all() {
    let res = Software::find_all();
    println!("get all software from reg\n {:#?} ", res.unwrap())
}
