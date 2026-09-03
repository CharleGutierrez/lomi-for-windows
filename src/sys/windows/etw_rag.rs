use windows::Win32::System::EventLog::*;
use windows::core::{PCWSTR, PWSTR};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn get_recent_crash_logs(lookback_seconds: u64) -> String {
    // A real implementation would use EvtQuery.
    // For simplicity, we can use WMI or shell out to powershell Get-EventLog,
    // or use advanced Win32 API.
    // Let's implement a shell out for robustness without pulling massive Win32 Eventing headers.
    
    let script = format!(
        "Get-WinEvent -FilterHashtable @{{LogName='System','Application'; Level=2; StartTime=(Get-Date).AddSeconds(-{})}} -MaxEvents 5 | Select-Object -Property TimeCreated, Message | ConvertTo-Json",
        lookback_seconds
    );

    let output = std::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", &script])
        .output();

    if let Ok(out) = output {
        let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !result.is_empty() {
            return format!("Recent Windows Events ({}s):\n{}", lookback_seconds, result);
        }
    }
    
    "No recent critical errors found in Event Viewer.".to_string()
}
