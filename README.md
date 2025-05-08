# Aion Classic Private Server (GameForge)

This is POC Aion Private Server for the gameforge variant of the aion classic private server. 

As it stands I can log into the game with an unpatched binary client, select a character and fight monsters/emotes/move around. It even supports multiple players.

A lot of work would need to be done to make it a fully fledged private server but since the classic servers have diverged significally from the true "classic Aion" I have lost motivation to continue on this right now.

I am very proud of what I have achieved so far so I am putting it up in its unfinished state as I spent many months reverse engineering this to get as far as I did.

## Crates

### sts-server

Does a bunch of crpto stuff documented in [AUTH.md](./docs/AUTH.md)

1. Stats talking in plaintext 
2. Gets Authorization Code from client (proof we are logged in to gameforge)
2. Uses this information and some rands to seed some crypto (AES RSA)
3. Crypto keys are used to generate an RC4 stream
4. Continues talking now encrypted by RC4 stream
5. Passes the client off the auth server with proof we logged in

### launcher

Emulates the conversation between the Aion Client and the Gameforge client.

1. Starts listening on a win32 Named Pipe
2. Opens the Aion Client
3. Tells the Aion Client who is logged in
4. Sends some Auth stuff

Long term plan was to add my own OAuth here and use a JWT to authenticate but it was too big. Will need to do OAuth with a token so it fits in this conversation.

### Game Server

Where all the fun stuff happens, once the client is authenticated with a game server a connection starts to the game server

1. Creates a XOR key for the opcode packet encryption and sends to clint
2. Sends the player the character select information, all of your characters and statuses
3. Allows players to enter the world
4. Handles the game loop...

The game server runs in a single thread with every client getting there own thread for now. I use crossbeam channels to do bidirectional communication between the server logic and the client threads.

The thread is responsible for sending and recieving the packets from clients and will instantly respond to things that do not require server logic.

### auth-server

1. First server client connects once launched
2. Waits until STS authentication is finished
3. Gives client game server list and information
4. passes player off to game server

### pcap-parser

Dev tool that implements the packet + opcode decryption. You can provide this tool with a pcap file that includes aion game-server traffic. VERY USEFUL. Full introspection.

### bxml

Aion game files are stored in some kind of binary XML format, once you unzip them they can be parsed using this tool. Used for seeding game-server logic for example the stats of weapons and armour

### pak

Used for unzipping the game files, they are callled .pak but are basically zip files

### krypt

All the cryptography needed for the project is in the crate so it can be shared






