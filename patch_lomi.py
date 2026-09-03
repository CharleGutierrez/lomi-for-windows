import os

# Update Cargo.toml
toml_path = r'C:\Users\CharleOGutierrez\.gemini\antigravity-cli\brain\c72337a6-024d-44fc-8cfd-bac0259b59fe\scratch\lomi\Cargo.toml'
with open(toml_path, 'r', encoding='utf-8') as f:
    content = f.read()

if 'windows =' not in content:
    content = content.replace(
        '[target.\'cfg(target_os = "windows")\'.dependencies]\nwinreg = "0.56.0"',
        '[target.\'cfg(target_os = "windows")\'.dependencies]\nwinreg = "0.56.0"\nwindows = { version = "0.62.2", features = ["Security_Credentials_UI", "Win32_System_JobObjects", "Win32_System_Threading", "Win32_Foundation", "Win32_System_EventLog", "Win32_Security", "Security", "Security_Credentials", "Win32", "Win32_System"] }'
    )
with open(toml_path, 'w', encoding='utf-8') as f:
    f.write(content)

# Update mod.rs
mod_path = r'C:\Users\CharleOGutierrez\.gemini\antigravity-cli\brain\c72337a6-024d-44fc-8cfd-bac0259b59fe\scratch\lomi\src\sys\windows\mod.rs'
with open(mod_path, 'r', encoding='utf-8') as f:
    mod_content = f.read()
if 'pub mod security;' not in mod_content:
    mod_content += '\npub mod security;\npub mod vault;\npub mod etw_rag;\n'
with open(mod_path, 'w', encoding='utf-8') as f:
    f.write(mod_content)

print("Updated Cargo.toml and mod.rs")
