# Aion version 2.5

ProductVersion: 2724.405.514.4768

## Launch command

"C:\Program Files\GameforgeGames\AionClassic\bin64\aion.bin" -lang:ENG -usehqserver -hqip:79.110.83.156 -hqport:13001 -np:79.110.83.118 -dwsm -loginex -pwd16 -ingamewebshop  -npsa -probability -nospamcheck -dnpshop -st -dpetition -recserveridx:1 -old_charInstance

## IP Addresses

events.gameforge.com A 79.110.82.93
gameforge.com A 172.66.43.23, 172.66.40.232

Hard coded IP to South Korea
"211.233.91.10"

```bin64/config.ini
[SET_DNS_TCP_CLIENT]
	BIND_ADDR = 70.102.1.113

[ServerAddr]
	BIND_ADDR = 70.0.0.150
	BIND_PORT = 10241
```

## Protocols

Cant see any UDP apart from DNS to gameforge

## Client arguments

### HQ Auth, require all 3 together
* -usehqserver (use HQ authentication?)
* -hqip (default 172.66.40.232, which does not respond)
* -hpport (default 443)

## Login Types

* AQ?
* L2AUTH?
* HQ?
* AC?

## Game/chat Servers

79.110.83.112
79.110.83.113
79.110.83.114

## Login Networking
79.110.83.156:13001 = Auth Server
79.110.83.118:6600 = STS Server


## Encryption

#### The STS protocol - used by gameforge and innova

* Use AES256_CBC and modified RSA to exchange the AuthToken information and RC4 stream keys
* Port 6600 Federated identity STS
* uses openssl (libeay32.dll version 1.0.2)
* uses RC4 stream cipher after the exchange of rand and keys
* rc4_set_key is called twice, one is a decrypt key and one encrypt key

#### RC4

RC4 Keys are seeded with

ServerRand - Plain text base64
ClientRand - Plain text base64

EncryptionKey? - it is encrypted with the RSA public key that the server sends and decrypted by the servers private key
PremasterSecret - AES256 CBC encrypted base64 (The Initialisation Vector is the decoded base64 EncryptionKey)

Two RC4 keys generated, one is client and one is server, generation is wierd custom logic in our krypto crate


### The game server list packet

## Server list advert
* Must be 201 length

* IP Address start byte (21(0x15),  85(0x54), 147(0x93))
* Characters on server is byte (17(0x11))
* Each world server is 63(0x3F) bytes long
* World list length byte (6)???
* byte 0 is payload len
* Suffix byte is last logged in server ID
* byte 13 is server id
* byte 12 is online status
* Server ID is linked to Server name, id 0x01 is QA Server 1
* 611e is the Big Endian port number, so 7777
* 04 server is normal 05 it's not and won't show up

BOTH CLIENT AND SERVER
* Auth server is byte 0 is len
* byte 2 is Message Type
* byte 7 is server count

## RESPONSE TO SERVER LIST
Message Type == 0x04

Last 4 bytes of Server selection are random bytes


### The GAME SERVER PROTOCOL WOOOOOOOO

* First two bytes of a message ARE the len and are never encrypted
* one packet CAN contain multiple messages
* Server sends first packet, bytes 2-6 are static [0xc8,0x01,0x40,0x37,0xfe] and the next 4 are the key?
* Server responds with OS version information


## Examples
quitting game:
[23, 94, 49, 82, 8B, 84]


