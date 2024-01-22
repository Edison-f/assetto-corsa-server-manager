# Assetto Corsa Server Manager
A **WIP** alternative Assetto Corsa server manager written in rust

# Requirements
- An installation of rust (if you're building it yourself)
- Assetto Corsa
- Might need AC Content Manager and CSP, you should probably have both anyways

# Installation

`cargo run --release`

OR

Download a release

# Usage

Select your installtion location for Assetto Corsa, i.e.

`<SteamLibrary>\steamapps\common\assettocorsa`

- To load from the existing server_cfg and entry_list files click 'Load Config'

- To display a list of all available tracks or cars check the corresponding checkbox. This may take a while depending on how many tracks or cars you have installed. 

- To show all available skins for a car, right click on it. To add it to your entry list, left click. There is no input checking, so keep the data types correct and the car count within the pit count.

- 'Save to file' will save the config and entry list files to

`<SteamLibrary>\steamapps\common\assettocorsa\server\cfg\`

