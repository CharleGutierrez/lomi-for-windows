import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Fix 1: Add reason phrase to 200 OK and proper headers for 400 Bad Request
content = content.replace(
    'let err = "HTTP/1.1 400 Bad Request\\r\\n\\r\\nInvalid JSON";',
    'let err = "HTTP/1.1 400 Bad Request\\r\\nContent-Length: 12\\r\\n\\r\\nInvalid JSON";'
)

# Fix 2: Add OK to 200 status
content = content.replace(
    '"HTTP/1.1 {}\\r\\nContent-Type: application/json',
    '"HTTP/1.1 {} OK\\r\\nContent-Type: application/json'
)

# Fix 3: Make VectorSearch query positional so `lomi-win vector-search "memory tuner"` works
content = content.replace(
    '        query: Option<String>,',
    '        #[arg(index = 1)]\n        query: Option<String>,'
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
