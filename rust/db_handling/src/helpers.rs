use sled::{Db, Tree, open, IVec};
use anyhow;
use definitions::{crypto::Encryption, metadata::{NameVersioned, VersionDecoded}, network_specs::{ChainSpecs, NetworkKey, NetworkKeySource, generate_network_key, generate_verifier_key, Verifier}, users::{AddressKey, AddressDetails}};
use meta_reading::decode_metadata::get_meta_const;
use parity_scale_codec::Decode;
use sp_runtime::MultiSigner;

use crate::error::{Error, NotDecodeable, NotFound, NotHex};

/// Wrapper for `open` with crate error
pub fn open_db (database_name: &str) -> anyhow::Result<Db> {
    match open(database_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `open_tree` with crate error
pub fn open_tree (database: &Db, tree_name: &[u8]) -> anyhow::Result<Tree> {
    match database.open_tree(tree_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `drop_tree` with crate error
pub fn drop_tree (database: &Db, tree_name: &[u8]) -> anyhow::Result<()> {
    match database.drop_tree(tree_name) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `flush` with crate error
pub fn flush_db (database: &Db) -> anyhow::Result<()> {
    match database.flush() {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `clear` with crate error
pub fn clear_tree(tree: &Tree) -> anyhow::Result<()> {
    match tree.clear() {
        Ok(()) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `insert` with crate error, not catching previous value during insertion
pub fn insert_into_tree(key: Vec<u8>, value: Vec<u8>, tree: &Tree) -> anyhow::Result<()> {
    match tree.insert(key, value) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `remove` with crate error, not catching previous value during removal
pub fn remove_from_tree(key: Vec<u8>, tree: &Tree) -> anyhow::Result<()> {
    match tree.remove(key) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode hex encoded &str into Vec<u8>,
/// `what` is enum of possible NotHex failures
pub fn unhex(hex_entry: &str, what: NotHex) -> anyhow::Result<Vec<u8>> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => return Err(Error::NotHex(what).show()),
    }
}

/// Function to get SCALE encoded network specs entry by given network_key, decode it
/// as ChainSpecs, and check for genesis hash mismatch. Is used forrom cold database
pub fn get_and_decode_chain_specs(chainspecs: &Tree, network_key: &NetworkKey) -> anyhow::Result<ChainSpecs> {
    match chainspecs.get(network_key) {
        Ok(Some(chain_specs_encoded)) => decode_chain_specs(chain_specs_encoded, network_key),
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_chain_specs(chain_specs_encoded: IVec, network_key: &NetworkKey) -> anyhow::Result<ChainSpecs> {
    match <ChainSpecs>::decode(&mut &chain_specs_encoded[..]) {
        Ok(a) => {
            if &generate_network_key(&a.genesis_hash.to_vec(), a.encryption) != network_key {return Err(Error::NetworkKeyMismatch.show())}
            Ok(a)
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_address_details(address_details_encoded: IVec) -> anyhow::Result<AddressDetails> {
    match <AddressDetails>::decode(&mut &address_details_encoded[..]) {
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressDetails).show()),
    }
}

/// Function to check metadata vector from the database, and output if it's ok
pub fn check_metadata(meta: Vec<u8>, versioned_name: &NameVersioned) -> anyhow::Result<Vec<u8>> {
    let version_vector = match get_meta_const(&meta.to_vec()) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Metadata).show()),
    };
    let version = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Version).show()),
    };
    if version.specname != versioned_name.name {return Err(Error::MetadataNameMismatch.show())}
    if version.spec_version != versioned_name.version {return Err(Error::MetadataVersionMismatch.show())}
    Ok(meta)
}

/// Function to find encryption aldorithm corresponding to network with known network key
pub fn get_network_encryption (chainspecs: &Tree, network_key: &NetworkKey) -> anyhow::Result<Encryption> {
    let from_specs = get_and_decode_chain_specs(chainspecs, network_key)?.encryption;
    let from_key = reverse_network_key(network_key)?.encryption;
    if from_specs == from_key {Ok(from_specs)}
    else {return Err(Error::EncryptionMismatchNetwork.show())}
}

/// Function to generate address key with crate error
pub fn generate_address_key (public: &Vec<u8>, encryption: Encryption) -> anyhow::Result<AddressKey> {
    match definitions::users::generate_address_key(public, encryption) {
        Ok(a) => Ok(a),
        Err(e) => return Err(Error::AddressKey(e.to_string()).show()),
    }
}


pub struct PublicKeyHelper {
    pub public_key: Vec<u8>,
    pub encryption: Encryption,
}


/// Function to produce public key and encryption from AddressKey
pub fn reverse_address_key (key: &Vec<u8>) -> anyhow::Result<PublicKeyHelper> {
    match <MultiSigner>::decode(&mut &key[..]) {
        Ok(MultiSigner::Ed25519(x)) => {
            Ok(PublicKeyHelper {
                public_key: x.to_vec(),
                encryption: Encryption::Ed25519,
            })
        },
        Ok(MultiSigner::Sr25519(x)) => {
            Ok(PublicKeyHelper {
                public_key: x.to_vec(),
                encryption: Encryption::Sr25519,
            })
        },
        Ok(MultiSigner::Ecdsa(x)) => {
            Ok(PublicKeyHelper {
                public_key: x.0.to_vec(),
                encryption: Encryption::Ecdsa,
            })
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressKey).show())
    }
}

/*
pub fn assert_encryption_match (address_key: &AddressKey, network_key: &NetworkKey, chainspecs: &Tree) -> anyhow::Result<()> {
    if check_encryption_match(address_key, network_key, chainspecs)? {Ok(())}
    else {return Err(Error::EncryptionMismatchId.show())}
}


pub fn check_encryption_match (address_key: &AddressKey, network_key: &NetworkKey, chainspecs: &Tree) ->anyhow::Result<bool> {
    let identity_encryption = reverse_address_key(address_key)?.encryption;
    let network_encryption = get_network_encryption (chainspecs, network_key)?;
    Ok(identity_encryption == network_encryption)
}
*/

/// Helper struct to get genesis hash and encryption from network key
pub struct NetworkKeyHelper {
    pub genesis_hash: Vec<u8>,
    pub encryption: Encryption,
}

/// Helper function to get genesis hash and encryption from network key
pub fn reverse_network_key (network_key: &NetworkKey) -> anyhow::Result<NetworkKeyHelper> {
    let network_key_source = match <NetworkKeySource>::decode(&mut &network_key[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::NetworkKey).show()),
    };
    match network_key_source {
        NetworkKeySource::Ed25519(genesis_hash) => Ok(NetworkKeyHelper{genesis_hash, encryption: Encryption::Ed25519}),
        NetworkKeySource::Sr25519(genesis_hash) => Ok(NetworkKeyHelper{genesis_hash, encryption: Encryption::Sr25519}),
        NetworkKeySource::Ecdsa(genesis_hash) => Ok(NetworkKeyHelper{genesis_hash, encryption: Encryption::Ecdsa}),
    }
}

/// Function to determine if there are entries with similar genesis hash left in the database;
/// searches through chainspecs tree of the cold database for the given genesis hash
pub fn genesis_hash_in_cold_db (genesis_hash: [u8; 32], chainspecs: &Tree) -> anyhow::Result<bool> {
    let mut out = false;
    for x in chainspecs.iter() {
        if let Ok((network_key, chain_specs_encoded)) = x {
            let network_specs = decode_chain_specs(chain_specs_encoded, &network_key.to_vec())?;
            if network_specs.genesis_hash == genesis_hash {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}

pub fn get_verifier (genesis_hash: [u8; 32], verifiers: &Tree) -> anyhow::Result<Verifier> {
    match verifiers.get(&generate_verifier_key(&genesis_hash.to_vec())) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Verifier).show()),
        },
        Ok(None) => return Err(Error::NotFound(NotFound::Verifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

pub fn remove_verifier (genesis_hash: [u8; 32], verifiers: &Tree) -> anyhow::Result<Verifier> {
    match verifiers.remove(&generate_verifier_key(&genesis_hash.to_vec())) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Verifier).show()),
        },
        Ok(None) => return Err(Error::NotFound(NotFound::Verifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}
