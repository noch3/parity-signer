
# Crate `generate_message`

## Overview

This is a crate used to generate messages that could be parsed by Signer and to maintain the *hot* database.  

Crate is expected to work for message generation in two stages.  

First, the message payload is created either from the existing database or by fetching through rpc calls. Resulting message (in form of `Vec<u8>`) is saved in plaintext in `../files/for_signing` folder.  

This message could be then fed to signing tool, such as subkey, to generate a signature.  

After the plaintext export is used to create the signature, final message could be formed. Final message of `load_metadata`, `load_types` or `add_network` consists of:  

- 53xxyy, where xx is information about cryptography algorithm used, and yy is message type,  
- (if verified) public key of message verifier as hex line  
- message body as hex line  
- (if verified) signature of message verifier as hex line  

Final message could be exported as fountain qr and/or as textfile containing hex string.  


## Message types

Types of messages that could be generated:  

- 53xx80 `load_metadata` (contains `definitions::qr_transfers::ContentLoadMeta`)  
- 53xx81 `load_types` (contains `definitions::qr_transfers::ContentLoadTypes`)  
- 53xxc0 `add_network` (contains `definitions::qr_transfers::ContentAddNetwork`)  
- 53xxc1 `add_specs` (contains `definitions::qr_transfers::ContentAddSpecs`);  

Message `load_metadata` is used to load new versions of metadata for networks already in users database.  

Message `load_types` is used to load types information in users database, and hopefully will go obsolete soon with FrameMetadataV14 integration. Should be used really rarely.  

Message `add_network` is normally used to add entirely new networks, that are not yet in users database.  

Message `add_specs` is used to add network specs for networks that are not yet in users database.  


## Possible output formats

Functions from `generate_message` crate can generate hex line outputs as .txt files and/or generate fountain qr codes in apng format.  

Note that qr code generation could be quite resourse demanding operation.  


## Current usage

Database addressed by crate is `../database/database_hot`, its address is set in `constants` crate.  

Messages ready for signing are generated in `../files/for_signing/` folder, their names are set in `constants` crate.  

Final signed messages (as qr codes or as text files) are generated in `../files/signed/` folder,  their names are set in `constants` crate.  

Examples of names for intermediate files are:  
- `sign_me_add_network_kusamaV9070`
- `sign_me_load_metadata_polkadotV9050`
- `sign_me_add_specs_noname_ed25519`
- `sign_me_load_types`

Final file could be optionally named through the `-name` key, however, default name is generated as well during the message consistency check-up.  
Examples of default file names are:  
for apng export:  
- `add_network_kusamaV9070_unverified`  
- `load_metadata_polkadotV9050_Alice`  
- `load_types`  
for text export:  
- `add_network_kusamaV9070_unverified.txt`  
- `load_metadata_polkadotV9050_Alice.txt`  
- `load_types.txt`  
Unverified is added for names of unverified files, Alice is added for names of test files verified by Alice.  
Normally verified files are unmarked.  

Program is run by  

`$ cargo run COMMAND [KEY(s)]`

Possible commands are:  

- `show` followed by a key:  
    - `-database` to show network `specname` and `spec_version` for all networks in the metadata tree the database  
    - `-address_book` to show network `title`, `url address`, `encryption` and `(default)` marking if the encryption is default one for this network for all networks in the address_book tree of the database  
    
- `types` without any keys to generate `load_types` message  

- `load_metadata`, `add_network` and `add_specs` with following possible keys (only the key combinations most likely to be needed are implemented at the moment, tickets filing is suggested for others if they are needed):  
    - setting keys (maximum one can be used):  
        - `-d`: do NOT update the database, make rpc calls, and produce ALL requested output files  
        - `-f`: do NOT run rps calls, produce ALL requested output files from existing database  
        - `-k`: update database through rpc calls, produce requested output files only for UPDATED database entries  
        - `-p`: update database through rpc calls, do NOT produce any output files  
        - `-t` default setting: update database through rpc calls, produce ALL requested output files  
    - reference keys (exactly only one has to be used):  
        - `-a`: process all networks
        - `-n` followed by one name (network **specname** for load_metadata, i.e. `polkadot`, `westend` etc, the one that goes before version in output of `show -database`; network **title** for add_network and add_specs, i.e. `polkadot`, `westend-ed25519`, `rococo-AgainUpdatedGenesisHash` and the likes, whatever title shows in output of`show -address_book` (so far only vanilla names and vanilla names followed by encryption could be encountered))
        - `-u` followed by one url address
    - optional `-s` key to stop the program if any failure occurs. By default the program informs user of unsuccessful attempt and proceeds.  
    - encryption override keys (maximum one can be used), to be used for networks not in the database, and therefore to be used only for fetches through -u reference key; if multiple addresses are provided, same encryption override key is used for all networks:  
        - `-ed25519` if the network operates with ed25519 encryption algorithm  
        - `-sr25519` if the network operates with sr25519 encryption algorithm  
        - `-ecdsa` if the network operates with ecdsa encryption algorithm  
    
- `make` to `make_message` with following possible keys:  
    - optional content key: `-qr` will generate only apng qr code, `-text` will generate only text file with hex encoded message; by default, both qr code and text message are generated; content keys are expected immediately after `make` command, if at all; keys to follow could go in any order, but with content immediately following the key.  
    - key `-crypto` followed by encryption variant used in message verification:  
        - `ed25519`  
        - `sr25519`  
        - `ecdsa`  
        - `none` if the message is not verified  
    - key `-msgtype` followed by message type:  
        - `load_types`  
        - `load_metadata`  
        - `add_network`  
        - `add_specs`
    - key `-verifier` (has to be entered if only the `-crypto` was `ed25519`, `sr25519`, or `ecdsa`), followed by:  
        - `Alice` to generate messages "verified" by Alice (used for tests)  
        - `-hex` followed by actual hex line of public key  
        - `-file` followed by file name ****, to read verifier public key as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-payload` followed by `****` - file name to read message content as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-signature` followed by:  
        - `-hex` followed by actual hex line of signature  
        - `-file` followed by file name ****, to read verifier signature as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - optional key `-name` followed by `****` - name override to save file named `****` for apng export and file named `****.txt` into folder `../files/signed/`  
    
- `sign` to `make_message` using sufficient crypto information received from elsewhere, for example, from signer device, with following keys:  
    - optional content key: `-qr` will generate only apng qr code, `-text` will generate only text file with hex encoded message; by default, both qr code and text message are generated; content keys are expected immediately after `make` command, if at all; keys to follow could go in any order, but with content immediately following the key.  
    - key `-sufficient` followed by:  
        - `-hex` followed by actual hex line of hex represented SCALE encoded sufficient crypto  
        - `-file` followed by file name ****, to read SCALE encoded sufficient crypto as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-msgtype` followed by message type:  
        - `load_types`  
        - `load_metadata`  
        - `add_network`  
        - `add_specs`
    - key `-payload` followed by `****` - file name to read message content as Vec<u8> from file named `****` from folder `../files/for_signing/`  
    - key `-signature` followed by:  
        
    - optional key `-name` followed by `****` - name override to save file named `****` for apng export and file named `****.txt` into folder `../files/signed/`  

- `remove` with following keys  
    - `-title` followed by network title, the storage key in address book; use this to remove `address_book` entry, corresponding `chainspecs` entry and if no entries for associated `specname` remain in `address_book`, also all metadata entries for `specname`  
    - `-name` followed by specname argument, followed by `-version`, followed by `u32` version argument; use this to remove specific metadata from the `metadata` tree in the database  

- `restore_defaults` without any keys to restore the database to its initial default form  

## Example commands  

`$ cargo run types` to generate payload of `load_types` message from the database.  

`$ cargo run load_metadata -a` to run rpc calls for all networks in `address_book` of the database to fetch current metadata, update the metadata entries in the database if needed, and generate the `load_metadata` messages for all networks; if an error occurs for one of the networks, program informs of that and proceeds to try others.  

`$ cargo run add_network -f -n westend` to generate `add_network` message based on current database. Here `westend` refers to title in address book.  

`$ cargo run make -crypto sr25519 -msgtype load_metadata -verifier -file mock_key -payload sign_me_load_metadata_kusamaV9070 -signature -file mock_signature` to create both apng and text files with default names with load_metadata content verified by given verified.  

`$ cargo run make -text -crypto sr25519 -msgtype add_network -verifier Alice -payload sign_me_add_network_kusamaV9070` to create text file "verified" by Alice with sr25519 encryption for add_network.  

`$ cargo run make -text -crypto sr25519 -msgtype load_types -verifier -hex 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d -payload sign_me_load_types -signature -hex 0x5a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe` to create text file of load_types "verified" by verifier with given hex public key with given signature.  

`$ cargo run sign -text -sufficient -hex 01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d5a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe -msgtype load_types -payload sign_me_load_types` to assemble text file with load_type message using sufficient crypto received from signer.  

`$ cargo run load_metadata -a -k` to run rpc calls for all networks in `address_book` of the database to fetch current metadata, update the metadata entries in the database if needed, and generate the `load_metadata` message(s) for updated networks; if an error occurs for one of the networks, program informs of that and proceeds to try others.  

`$ cargo run add_specs -n polkadot -f -ed25519` to generate `add_specs` for `polkadot` network with custom encryption `ed25519` by modifying already known network specs for polkadot from the database; no database modification happens.  

`$ cargo run load_metadata -d -u wss://mainnet-node.dock.io` to run rpc call for `dock` network using somehow obtained address, to fetch metadata, and generate `load_metadata` message without updating the database.  

`$ cargo run add_network -d -u wss://mainnet-node.dock.io -sr25519` to run rpc call for `dock` network using somehow obtained address, to fetch metadata and network specs, and generate `add_metadata` message without updating the database.  



## Example full run  

Let's say we want to generate QR code with signed load_metadata message for freshly fetched westend metadata using subkey tool for signing.  

1. `$ cargo run load_metadata -n westend` This will fetch fresh westend metadata, update the database with it, and - most relevant to us currently - generate file with message body at `../files/for_signing/sign_me_load_metadata_westendV9080` (or whatever version it fetches). This file contains stuff that needs to be signed.  

2. Run file `../files/for_signing/sign_me_load_metadata_westendV9080` through subkey to generate the signature. Say, we are using ed25519 encryption.  

3. `$ cargo run make -qr -crypto ed25519 -msgtype load_metadata -verifier -hex <public_key_in_hex> -payload sign_me_load_metadata_westendV9080 -signature -hex <signature_in_hex>` This will assemble the message (prelude, verifier, message body, and signature), and generate apng qr. Before assembling, however, it will check that all things match, i.e. message type corresponds to contents of the message body, signature is good etc, to avoid attention errors.  

Done!  


## If the database somehow got corrupted or does not exist:  

The database operated by `generate_message` crate is referred as *hot* both here and in crate `db_handling`. To restore the database to defaults, run through steps 1-2 of **Instruction for a fresh start** in `db_handling` readme.  


## List of currently supported command and key combinations (without `make` and `sign` variants)  

`$ cargo run show -database`  
`$ cargo run show -address_book`  

`$ cargo run load_types`  

`$ cargo run load_metadata -f -a`  
`$ cargo run load_metadata -f -n network_specname`  
`$ cargo run load_metadata -d -a`  
`$ cargo run load_metadata -d -n network_specname`  
`$ cargo run load_metadata -d -u network_url`  
`$ cargo run load_metadata -k -a`  
`$ cargo run load_metadata -k -n network_specname`  
`$ cargo run load_metadata -p -a`  
`$ cargo run load_metadata -p -n network_specname`  
`$ cargo run load_metadata -t -a` or identical `$ cargo run load_metadata -a`  
`$ cargo run load_metadata -t -n network_specname` or identical `$ cargo run load_metadata -n network_specname`  

`$ cargo run add_network -f -a`  
`$ cargo run add_network -f -n network_title`  
`$ cargo run add_network -d -u network_url -ed25519` (*)  
`$ cargo run add_network -k -u network_url -ed25519` (*)  
`$ cargo run add_network -p -u network_url -ed25519` (*)  
`$ cargo run add_network -t -u network_url -ed25519` (*)  
`$ cargo run add_network -u network_url -ed25519` (*)  

`$ cargo run add_specs -f -a`  
`$ cargo run add_specs -f -n network_title`  
`$ cargo run add_specs -f -n network_title -ed25519` (*)  
`$ cargo run add_specs -f -u network_url`  
`$ cargo run add_specs -f -u network_url -ed25519` (*)  
`$ cargo run add_specs -d -u network_url -ed25519` (*)  
`$ cargo run add_specs -p -n network_title -ed25519` (*)  
`$ cargo run add_specs -p -u network_url -ed25519` (*)  
`$ cargo run add_specs -t -n network_title -ed25519` (*)  
`$ cargo run add_specs -t -u network_url -ed25519` (*)  
`$ cargo run add_specs -n network_title -ed25519` (*)  
`$ cargo run add_specs -u network_url -ed25519` (*)  

`$ cargo run remove -title westend-ed25519`  
`$ cargo run remove -name kusama -version 9090`  

`$ cargo run restore_defaults`  

(*) encryption override key should correspond to appropriate encryption for the network in question
