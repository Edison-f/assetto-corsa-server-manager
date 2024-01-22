# Assetto Corsa Server Manager
A **WIP** alternative Assetto Corsa server manager written in rust

# Requirements
- An installation of rust (if you're building it yourself)
- Assetto Corsa
- Might need AC Content Manager and CSP, you should probably have both anyways

# Installation\Usage

`cargo run --release`

Select your installtion location for Assetto Corsa, i.e.

`<SteamLibrary>\steamapps\common\assettocorsa`

To load from the existing server.cfg file click 'Load Config'

To display a list of all available tracks or cars check the corresponding checkbox. This may take a while depending on how many tracks or cars you have installed.

You cannot edit entry_list.ini from here yet, but 'Save to file' will save the config file to

`<SteamLibrary>\steamapps\common\assettocorsa\server\cfg\temp_server_cfg.ini`

