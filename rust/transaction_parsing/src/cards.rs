use hex;
use definitions::crypto::Encryption;

use crate::error::Error;

pub enum Card <'a> {
    Call {pallet: &'a str, method: &'a str, docs: &'a str},
    Pallet (&'a str),
    Varname (&'a str),
    Default (&'a str),
    PathDocs {path: &'a str, docs: &'a str},
    Id (&'a str),
    None,
    IdentityField (&'a str),
    BitVec (String), // String from printing BitVec
    Balance {number: &'a str, units: &'a str},
    FieldName {name: &'a str, docs: &'a str},
    FieldNumber {number: usize, docs: &'a str},
    EnumVariantName {name: &'a str, docs: &'a str},
    Range {start: String, end: String, inclusive: bool},
    EraImmortalNonce (u64),
    EraMortalNonce {phase: u64, period: u64, nonce: u64},
    Tip {number: &'a str, units: &'a str},
    TipPlain (u128),
    BlockHash (&'a str),
    TxSpec {network: &'a str, version: u32, tx_version: u32},
    TxSpecPlain {gen_hash: &'a str, version: u32, tx_version: u32},
    Author {base58_author: &'a str, seed_name: &'a str, path: &'a str, has_pwd: bool, name: &'a str},
    AuthorPlain (&'a str),
    AuthorPublicKey{author_public_key: Vec<u8>, encryption: Encryption},
    Verifier(String),
    Meta(String), // get String after applying show() to MetaValuesDisplay
    TypesInfo(&'a str),
    NewNetwork(String), // get String after applying show() to NetworkDisplay
    Warning (Warning),
    Error (Error),
}

pub enum Warning {
    AuthorNotFound,
    NewerVersion {used_version: u32, latest_version: u32},
    NoNetworkID,
    VerifierAppeared,
    NotVerified,
    UpdatingTypes,
    TypesNotVerified,
    GeneralVerifierAppeared,
    TypesAlreadyThere,
    MetaAlreadyThereUpdBothVerifiers,
    MetaAlreadyThereUpdMetaVerifier,
    MetaAlreadyThereUpdGeneralVerifier,
    NetworkAlreadyHasEntries,
    AddNetworkNotVerified,
}

impl Warning {
    pub fn show (&self) -> String {
        match &self {
            Warning::AuthorNotFound => String::from("Transaction author public key not found."),
            Warning::NewerVersion {used_version, latest_version} => format!("Transaction uses outdated runtime version {}. Latest known available version is {}.", used_version, latest_version),
            Warning::NoNetworkID => String::from("Public key is on record, but not associated with the network used."),
            Warning::VerifierAppeared => String::from("Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."),
            Warning::NotVerified => String::from("Received network metadata is not verified."),
            Warning::UpdatingTypes => String::from("Updating types (really rare operation)."),
            Warning::TypesNotVerified => String::from("Received types information is not verified."),
            Warning::GeneralVerifierAppeared => String::from("Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."),
            Warning::TypesAlreadyThere => String::from("Received types information is already in database, only verifier could be added."),
            Warning::MetaAlreadyThereUpdBothVerifiers => String::from("Received metadata is already in database, both general verifier and network verifier could be added."),
            Warning::MetaAlreadyThereUpdMetaVerifier => String::from("Received metadata is already in database, only network verifier could be added."),
            Warning::MetaAlreadyThereUpdGeneralVerifier => String::from("Received metadata is already in database, only general verifier could be added."),
            Warning::NetworkAlreadyHasEntries => String::from("Add network message is received for network that already has some entries in the database."),
            Warning::AddNetworkNotVerified => String::from("Received new network information is not verified."),
        }
    }
}

fn fancy (index: u32, indent: u32, card_type: &str, decoded_string: &str) -> String {
    format!("{{\"index\":{},\"indent\":{},\"type\":\"{}\",\"payload\":{}}}", index, indent, card_type, decoded_string)
}

impl <'a> Card <'a> {
    pub fn card (&self, index: u32, indent: u32) -> String {
        match &self {
            Card::Call {pallet, method, docs} => fancy(index, indent, "call", &format!("{{\"method\":\"{}\",\"pallet\":\"{}\",\"docs\":\"{}\"}}", method, pallet, hex::encode(docs.as_bytes()))),
            Card::Pallet (pallet_name) => fancy(index, indent, "pallet", &format!("\"{}\"", pallet_name)),
            Card::Varname (varname) => fancy(index, indent, "varname", &format!("\"{}\"", varname)),
            Card::Default (decoded_string) => fancy(index, indent, "default", &format!("\"{}\"", decoded_string)),
            Card::PathDocs {path, docs} => fancy(index, indent, "path_and_docs", &format!("{{\"path\":{},\"docs\":\"{}\"}}", path, hex::encode(docs.as_bytes()))),
            Card::Id (base58_id) => fancy(index, indent, "Id", &format!("\"{}\"", base58_id)),
            Card::None => fancy(index, indent, "none", "\"\""),
            Card::IdentityField (variant) => fancy(index, indent, "identity_field", &format!("\"{}\"", variant)),
            Card::BitVec (bv) => fancy(index, indent, "bitvec", &format!("\"{}\"", bv)),
            Card::Balance {number, units} => fancy(index, indent, "balance", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
            Card::FieldName {name, docs} => fancy(index, indent, "field_name", &format!("{{\"name\":\"{}\",\"docs\":\"{}\"}}", name, hex::encode(docs.as_bytes()))),
            Card::FieldNumber {number, docs} => fancy(index, indent, "field_number", &format!("{{\"number\":\"{}\",\"docs\":\"{}\"}}", number, hex::encode(docs.as_bytes()))),
            Card::EnumVariantName {name, docs} => fancy(index, indent, "enum_variant_name", &format!("{{\"name\":\"{}\",\"docs\":\"{}\"}}", name, hex::encode(docs.as_bytes()))),
            Card::Range {start, end, inclusive} => fancy(index, indent, "range", &format!("{{\"start\":\"{}\",\"end\":\"{}\",\"inclusive\":\"{}\"}}", start, end, inclusive)),
            Card::EraImmortalNonce (nonce) => fancy(index, indent, "era_immortal_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", nonce)),
            Card::EraMortalNonce {phase, period, nonce} => fancy(index, indent, "era_mortal_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, nonce)),
            Card::Tip {number, units} => fancy(index, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
            Card::TipPlain (x) => fancy(index, indent, "tip_plain", &format!("\"{}\"", x)),
            Card::BlockHash (hex_block_hash) => fancy(index, indent, "block_hash", &format!("\"{}\"", hex_block_hash)),
            Card::TxSpec {network, version, tx_version} => fancy(index, indent, "tx_spec", &format!("{{\"network\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", network, version, tx_version)),
            Card::TxSpecPlain {gen_hash, version, tx_version} => fancy(index, indent, "tx_spec_plain", &format!("{{\"network_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", gen_hash, version, tx_version)),
            Card::Author {base58_author, seed_name, path, has_pwd, name} => fancy(index, indent, "author", &format!("{{\"base58\":\"{}\",\"seed\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":{},\"name\":\"{}\"}}", base58_author, seed_name, path, has_pwd, name)),
            Card::AuthorPlain (base58_author) => fancy(index, indent, "author_plain", &format!("{{\"base58\":\"{}\"}}", base58_author)),
            Card::AuthorPublicKey{author_public_key, encryption} => fancy(index, indent, "author_public_key", &format!("{{\"hex\":\"{}\",\"crypto\":\"{}\"}}", hex::encode(author_public_key), encryption.show())),
            Card::Verifier(x) => fancy(index, indent, "verifier", x),
            Card::Meta(x) => fancy(index, indent, "meta", &format!("{{{}}}", x)),
            Card::TypesInfo(x) => fancy(index, indent, "types_hash", &format!("\"{}\"", x)),
            Card::NewNetwork(x) => fancy(index, indent, "new_network", &format!("{{{}}}", x)),
            Card::Warning (warn) => fancy(index, indent, "warning", &format!("\"{}\"", warn.show())),
            Card::Error (err) => fancy(index, indent, "error", &format!("\"{}\"", err.show())),
        }
    }
}


pub enum Action {
    SignTransaction (u32),
    LoadMetadata (u32),
    AddMetadataVerifier (u32),
    LoadTypes (u32),
    AddGeneralVerifier (u32),
    AddTwoVerifiers (u32),
    LoadMetadataAndAddGeneralVerifier (u32),
    AddNetwork (u32),
    AddNetworkAndAddGeneralVerifier (u32),
}

fn print_action (action: &str, checksum: &u32) -> String {
    format!("\"action\":{{\"type\":\"{}\",\"payload\":{{\"type\":\"{}\",\"checksum\":\"{}\"}}}}", action, action, checksum)
}

impl Action {
    pub fn card (&self) -> String {
        match &self {
            Action::SignTransaction(x) => print_action("sign_transaction", x),
            Action::LoadMetadata(x) => print_action("load_metadata", x),
            Action::AddMetadataVerifier(x) => print_action("add_metadata_verifier", x),
            Action::LoadTypes(x) => print_action("load_types", x),
            Action::AddGeneralVerifier(x) => print_action("add_general_verifier", x),
            Action::AddTwoVerifiers(x) => print_action("add_two_verifiers", x),
            Action::LoadMetadataAndAddGeneralVerifier(x) => print_action("load_metadata_and_add_general_verifier", x),
            Action::AddNetwork(x) => print_action("add_network", x),
            Action::AddNetworkAndAddGeneralVerifier (x) => print_action("add_network_and_add_general_verifier", x),
        }
    }
}
