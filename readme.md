# CorvusLauncher

A Arma3 server launcher made for internal use in the Digby WarRoom community. Makes it easier to select what serverprofile, modpacks, servermods and clientsides to load and launch the amount of headless clients needed.

All processes spawned by the launcher is orphaned, so you can close the launcher as you like without killing any of the arma server processes. 

![image](https://github.com/user-attachments/assets/a7d09eb3-c116-4cae-8258-c03f5eaac7a0)

It is built using Iced framework for rust gui applications.
It is only tested on Windows machine, as that is what the community use to run gameservers on, but it is made to support crossplatform. 

**TODO:**
- Write readme for documentation of .txt format expected in modlist files and general usage, including config file location
- Add gui dialog to add new server profiles 
- Verify if loggin is actually written to file, and if not, then why

## Modpreset fileformat
On the first launch the Launcher asks you to set paths to folders containing the modlist presets, clientside presets and the server presets.
![image](https://github.com/user-attachments/assets/402dcbf0-636a-448d-8282-ee42b68c42b5)

Each folder consist of a number of `.txt` files. Each text file follow the "PAR" format for presets with arma3_server.exe but omits the 
prefix of `-mods=`. 
So as an example for a modpack: `basic.txt`, it would contain:
```
 gm;vn;mods\@CBA_A3;mods\@ace;
```
Each mods path is given relative to the `arma3_server.exe` file and seperated by `;`. 

## Reset config
If you wish to change the config of where the launcher is looking for modpacks, or the launchers path to the `arma3_server.exe` you can manually
change it in the config file stored at: `%appdata%\corvuslauncher.toml` on windows. (full path: `C:\Users\<username>\AppData\Roaming\corvuslauncher.toml`)

## Launcher Config File
The launchers config file with server profiles, and paths to folders needed to operate, is set in the systems "appdata" folder, so for windows in 
`%appdata%\corvuslauncher.toml` or `C:\Users\<username>\AppData\Roaming\corvuslauncher.toml`. 
On other platforms than windows it follows the default Xdg base-strategy with a `.config` folder. 

It consists of the following entries localized to your machine after you have run the launcher the first time.   
For now the only way to add new server_profiles is by adding it to the list here.
``` 
a3_root = 'C:\Users\crow\Downloads\A3Master'
a3_server_executable = 'C:\Users\crow\Downloads\A3Master\arma3server_x64.exe'
folder_modlists = 'C:\Users\crow\Downloads\Modlists'
folder_clientside = 'C:\Users\crow\Downloads\Clientsides'
folder_servermods = 'C:\Users\crow\Downloads\ServerMods'
server_profiles = ["ServerNormal", "ServerEvent", "ServerATF", "ServerMF"]
```

## Server Profiles
Server profiles will set the parameter for the server on what profile it would use. If you select "ServerNormal" for an example
it will use `<arma3_server_root>\ServerNormal` folder as the profile, and thus load the settings and network config defined there. 

Currently you can only add new profiles here by adding them to the config file as mentioned above. 

Unofficial iced guide
https://jl710.github.io/iced-guide/app_structure/view-helper.html