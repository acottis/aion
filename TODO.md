# TODO

## Launcher
* Make my own launcher that is long lived
* Work out launcher flags
* Finish OAUTH implementation
* IOCP Windows Pipes

## STS Server
* Do real authentication 

## Game Server
* Review Clones
* Reviews Vec's (Pass vec for server packets from root client thread so we only alloc one)
* Review Sender<Update> vs Sender<Vec<Update>>
* Remove users from the GLOBAL STATE when they need to be
* Ensure disconnected clients are properly disconnected
* Implement database (mariadb probably)

## Auth Server
* Tidy up and refactor, its very raw

## Game Files
* Load game data at compile time that we parsed from bxml
