import os

with open('src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

if 'pub mod ui;' not in content:
    content = content.replace('pub mod core;\n', 'pub mod core;\npub mod ui;\n')

if 'Gui {' not in content:
    content = content.replace(
        '    Swarm {\n',
        '    Gui,\n    Swarm {\n'
    )

    # In match &cli.command
    content = content.replace(
        '        Commands::Swarm {',
        '        Commands::Gui => {\n            let _ = crate::ui::gui::slint_app::launch_slint_app();\n        }\n        Commands::Swarm {'
    )

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
