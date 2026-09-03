import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace(
    '"model": "qwen2.5-coder:1.5b", // Fallback local model',
    '"model": "qwen2.5:0.5b", // Force local test model'
)

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
