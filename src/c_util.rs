// power by https://stackoverflow.com/questions/10866944/how-can-i-read-a-child-processs-output
pub fn call_cmd_slient(cmd: String) -> Result<String, String> {
    use std::ffi::CString;

    cpp! {{
        #include <windows.h>
        #include <stdio.h>
        #include <iostream>
    }}

    const MAX_OUT_LEN: u32 = 262144; //max len is 256kb

    let output = unsafe { CString::from_vec_unchecked(vec![0; MAX_OUT_LEN as usize]) };
    let mut output_raw = output.into_raw();

    let cmd_input_raw = CString::new(cmd.into_bytes()).unwrap().into_raw();

    let exit_code = unsafe {
        cpp!([cmd_input_raw as "char *",mut output_raw as "char *",MAX_OUT_LEN as "uint32_t"] -> i32 as "int32_t" {
            PROCESS_INFORMATION processInfo;
            STARTUPINFOA startupInfo;
            SECURITY_ATTRIBUTES saAttr;

            HANDLE stdoutReadHandle = NULL;
            HANDLE stdoutWriteHandle = NULL;

            char cmdline[1024];
            std::string out;

            DWORD exitcode;

            strcpy_s(cmdline, sizeof(cmdline), cmd_input_raw);

            memset(&saAttr, 0, sizeof(saAttr));
            saAttr.nLength = sizeof(SECURITY_ATTRIBUTES);
            saAttr.bInheritHandle = TRUE;
            saAttr.lpSecurityDescriptor = NULL;

            // Create a pipe for the child process's STDOUT.
            if (!CreatePipe(&stdoutReadHandle, &stdoutWriteHandle, &saAttr, 5000))
            {
                printf("CreatePipe: %u\n", GetLastError());
                return -1;
            }

            // Ensure the read handle to the pipe for STDOUT is not inherited.
            if (!SetHandleInformation(stdoutReadHandle, HANDLE_FLAG_INHERIT, 0))
            {
                printf("SetHandleInformation: %u\n", GetLastError());
                return -1;
            }

            memset(&startupInfo, 0, sizeof(startupInfo));
            startupInfo.cb = sizeof(startupInfo);
            startupInfo.hStdError = stdoutWriteHandle;
            startupInfo.hStdOutput = stdoutWriteHandle;
            startupInfo.hStdInput = GetStdHandle(STD_INPUT_HANDLE);
            startupInfo.dwFlags |= STARTF_USESTDHANDLES;
            if (!CreateProcessA(NULL, cmdline, NULL, NULL, TRUE,
                CREATE_NO_WINDOW, NULL, 0, &startupInfo, &processInfo))
            {
                printf("CreateProcessA: %u\n", GetLastError());
                return -1;
            }

            CloseHandle(stdoutWriteHandle);

            DWORD bytes_read;
            char tBuf[257];
            for (;;) {
                if (!ReadFile(stdoutReadHandle, tBuf, 256, &bytes_read, NULL))
                {
                    break;
                }
                if (bytes_read > 0)
                {
                    tBuf[bytes_read] = '\0';
                    out += tBuf;
                }
            }

            if (WaitForSingleObject(processInfo.hProcess, INFINITE) != WAIT_OBJECT_0)
            {
                printf("WaitForSingleObject: %u\n", GetLastError());
                return -1;
            }

            if (!GetExitCodeProcess(processInfo.hProcess, &exitcode))
            {
                printf("GetExitCodeProcess: %u\n", GetLastError());
                return -1;
            }

            CloseHandle( processInfo.hProcess );
            CloseHandle( processInfo.hThread );
            out.copy(output_raw,MAX_OUT_LEN);
            return 0;
        })
    };

    let output = unsafe { CString::from_raw(output_raw) };
    use util;
    util::to_string(output.into_bytes()).and_then(|x| if exit_code == 0 {
        Ok(x.trim().to_string())
    } else {
        Err(x)
    })
}

#[test]
fn test_call_cmd_slient() {
    let out = call_cmd_slient("echo test".to_owned());
    assert_eq!(out, Ok("test".to_string()));
}
