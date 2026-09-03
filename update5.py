import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace(
    'let mut semantic_cache = HashMap::new();',
    'let mut semantic_cache: HashMap<u64, String> = HashMap::new();'
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
