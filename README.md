# Clipboard_magic
This is a command line tool that takes whatever is in your clipboard and sends it to another machine

You will need to change main2.rs to main.rs and Cargo2.toml to Cargo.toml in the folder

#Steps to run it (still in development so you need to compile yourself)
1) open termianl and type, cargo new *whatever name you want*
2) download the main.rs and Cargo.toml file
3) replace the the files you downloaded with the files in the folder with the same name
4) then run "cargo run"

#If you get "error: linking with `cc` failed: exit status: 1" then type in this command
1)  sudo apt install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
