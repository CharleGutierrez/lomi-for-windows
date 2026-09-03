import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Add missing braces for the for loop and the function
content = content.replace('            }\n        }\n"""', '            }\n        }\n    }\n}\n"""')

# Wait, the python script generated bad code. I'll just write a script that appends the missing braces.
