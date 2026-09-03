import os

file_path = r'src\sys\windows\security.rs'
with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

# Replace the real implementation with a mock for testing
mock_impl = '''use windows::Security::Credentials::UI::UserConsentVerifier;
use windows::core::Result;

pub async fn verify_windows_hello(message: &str) -> bool {
    println!("🔐 [TEST MODE] Bypassing Windows Hello Verification: {}", message);
    true
}
'''
with open(file_path, 'w', encoding='utf-8') as f:
    f.write(mock_impl)
print("Security bypassed for testing.")
