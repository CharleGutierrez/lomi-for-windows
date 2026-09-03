use std::fs;
use std::process::Command;

pub fn run_in_sandbox(script_content: &str) {
    let temp_dir = std::env::temp_dir().join("lomi_sandbox");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }

    let script_path = temp_dir.join("script.bat");
    fs::write(&script_path, script_content).unwrap();

    let host_folder = temp_dir.to_string_lossy().to_string();
    let sandbox_config = format!(r#"<Configuration>
    <vGpu>Disable</vGpu>
    <Networking>Disable</Networking>
    <MappedFolders>
        <MappedFolder>
            <HostFolder>{}</HostFolder>
            <SandboxFolder>C:\SandboxShare</SandboxFolder>
            <ReadOnly>false</ReadOnly>
        </MappedFolder>
    </MappedFolders>
    <LogonCommand>
        <Command>C:\SandboxShare\script.bat</Command>
    </LogonCommand>
</Configuration>"#, host_folder);

    let wsb_path = temp_dir.join("lomi.wsb");
    fs::write(&wsb_path, sandbox_config).unwrap();

    Command::new("cmd")
        .args(["/C", "start", "", &wsb_path.to_string_lossy()])
        .spawn()
        .unwrap();
}
