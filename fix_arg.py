import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace(
    '        #[arg(short, long)]\n        #[arg(index = 1)]\n        query: Option<String>,',
    '        #[arg(index = 1)]\n        query: Option<String>,'
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
