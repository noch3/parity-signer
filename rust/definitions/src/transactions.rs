use parity_scale_codec_derive::{Decode, Encode};

use crate::history::Event;
use crate::metadata::NameVersioned;
use crate::network_specs::{ChainSpecsToSend, Verifier, VerifierKey};
use crate::types::TypeEntry;
use crate::users::AddressKey;

/// Enum to classify possible actions, and store corresponding information in the database
#[derive(Decode, Encode)]
pub enum Transaction {
    Sign(Sign),
    LoadMeta(LoadMeta),
    UpdMetaVerifier(UpdMetaVerifier),
    UpdGeneralVerifier(UpdGeneralVerifier),
    LoadTypes(LoadTypes),
    AddNetwork(AddNetwork),
}

/// Struct to store sign_transaction action information
#[derive(Decode, Encode)]
pub struct Sign {
    pub path: String,
    pub transaction: Vec<u8>,
    pub has_pwd: bool,
    pub address_key: AddressKey,
    pub history: Vec<Event>,
}

pub struct SignDisplay <'a> {
    pub transaction: &'a str, // hex encoded transaction
    pub author_line: String, // signature author in Verifier.show_card() format
    pub user_comment: &'a str, // user entered comment for transaction
}

impl <'a> SignDisplay <'a> {
    pub fn show(&self) -> String {
        format!("\"transaction\":\"{}\",\"signed_by\":{},\"user_comment\":\"{}\"", &self.transaction, &self.author_line, &self.user_comment)
    }
}

/// Struct to store load_metadata action information
#[derive(Decode, Encode)]
pub struct LoadMeta {
    pub versioned_name: NameVersioned,
    pub meta: Vec<u8>, // metadata
    pub upd_network: Option<VerifierKey>, // verifier key if the network verifier should be updated
    pub verifier: Verifier, // transaction verifier, whether this goes anywhere after approval is determined by the action card type
    pub history: Vec<Event>,
}

/// Struct to store transferred information for cases when only
/// verifier is to be updated, without loading new metadata.
/// Also is used in updating both network verifier and general verifier,
/// the exact type of action is determined by the action card type
#[derive(Decode, Encode)]
pub struct UpdMetaVerifier {
    pub verifier_key: VerifierKey,
    pub verifier: Verifier,
    pub history: Vec<Event>,
}

#[derive(Decode, Encode)]
pub struct UpdGeneralVerifier {
    pub verifier: Verifier,
    pub history: Vec<Event>,
}


/// Struct to store load_types action information
#[derive(Decode, Encode)]
pub struct LoadTypes {
    pub types_info: Vec<TypeEntry>,
    pub verifier: Verifier,
    pub upd_verifier: bool,
    pub history: Vec<Event>,
}

/// Struct to store add_network action information
#[derive(Decode, Encode)]
pub struct AddNetwork {
    pub versioned_name: NameVersioned,
    pub meta: Vec<u8>,
    pub chainspecs: ChainSpecsToSend,
    pub verifier: Verifier,
    pub history: Vec<Event>,
}

