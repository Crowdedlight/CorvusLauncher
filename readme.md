# CorvusLauncher

A Arma3 server launcher made for internal use in the Digby WarRoom community. Makes it easier to select what serverprofile, modpacks, servermods and clientsides to load and launch the amount of headless clients needed.

All processes spawned by the launcher is orphaned, so you can close the launcher as you like without killing any of the arma server processes. 

Its built using Iced framework for rust gui applications. 


Documentation TODO:   
- Write readme for documentation of .txt format expected in modlist files and general usage, including config file location
- Fix remaining warnings and unused imports etc.
- Check remaining TODOs
- Add dialog to add new server profiles? 
- test on actual server
- setup release CI on github, like AMDU

Unofficial iced guide
https://jl710.github.io/iced-guide/app_structure/view-helper.html