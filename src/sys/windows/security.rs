use windows::Security::Credentials::UI::UserConsentVerifier;
use windows::core::Result;

pub async fn verify_windows_hello(message: &str) -> bool {
    println!("🔐 [TEST MODE] Bypassing Windows Hello Verification: {}", message);
    true
}
