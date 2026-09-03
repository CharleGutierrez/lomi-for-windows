import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace(
    '    let listener = std::net::TcpListener::bind(&address).expect("Failed to bind");',
    '    use std::io::{Read, Write};\n    let listener = std::net::TcpListener::bind(&address).expect("Failed to bind");'
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
