import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Replace the Err block to print the actual parse error and the body it tried to parse
content = content.replace(
    '''        let mut chat_request: UniversalChatRequest = match serde_json::from_str(body_str) {
            Ok(req) => req,
            Err(_) => {
                let err = "HTTP/1.1 400 Bad Request\\r\\nContent-Length: 12\\r\\n\\r\\nInvalid JSON";
                let _ = stream.write_all(err.as_bytes());
                continue;
            }
        };''',
    '''        let mut chat_request: UniversalChatRequest = match serde_json::from_str(body_str.trim_end_matches('\\0').trim()) {
            Ok(req) => req,
            Err(e) => {
                println!("JSON Parse Error: {} | Body was: '{}'", e, body_str.trim_end_matches('\\0').trim());
                let err = "HTTP/1.1 400 Bad Request\\r\\nContent-Length: 12\\r\\n\\r\\nInvalid JSON";
                let _ = stream.write_all(err.as_bytes());
                continue;
            }
        };'''
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
