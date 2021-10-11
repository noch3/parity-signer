use hex;
use sled::{Db, Tree};
use constants::{ADDNETWORK, METATREE, SETTREE, SPECSTREE, TRANSACTION, VERIFIERS};
use definitions::{history::Event, metadata::{MetaValuesDisplay, NameVersioned, NetworkDisplay, VersionDecoded}, network_specs::{ChainSpecsToSend, Verifier, generate_network_key, generate_verifier_key}, qr_transfers::ContentAddNetwork, transactions::{Transaction, AddNetwork}};
use meta_reading::decode_metadata::{get_meta_const_light};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use frame_metadata::RuntimeMetadata;

use crate::cards::{Action, Card, Warning};
use crate::error::{Error, BadInputData, DatabaseError, CryptoError};
use crate::check_signature::pass_crypto;
use crate::helpers::{open_db, open_tree, flush_db, insert_into_tree, get_checksum, get_verifier};
use crate::load_metadata::process_received_metadata;
use crate::utils::{get_chainspecs, get_general_verifier};

pub fn add_network (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and its trees: chainspecs, metadata, settings, transaction;

    let database = open_db(dbname)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let metadata = open_tree(&database, METATREE)?;
    let settings = open_tree(&database, SETTREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let verifiers = open_tree(&database, VERIFIERS)?;
    
    let current_verifier = get_general_verifier(&settings)?;
    
    let checked_info = pass_crypto(&data_hex)?;
    
    let (new_meta_vec, new_chain_specs) = match ContentAddNetwork::from_vec(&checked_info.message).meta_specs() {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeAddNetworkMessage)),
    };
    
    let verifier = checked_info.verifier;
    
    let new_network_key = generate_network_key(&new_chain_specs.genesis_hash.to_vec(), new_chain_specs.encryption);
    
    match get_chainspecs (&new_network_key, &chainspecs) {
        Ok(x) => {
        
        // network is already in the system,
        // need to warn about that and proceed to check the received metadata for version
        // can offer to add the metadata and/or update the network verifier and/or update the general verifier
        
        // network_verifier - known verifier for this newtork
        // current_verifier - current general verifier for adding networks and importing types
        // verifier - verifier of this particular message
        
        // first check if the important specs have changed: base58prefix, decimals, name, and unit
            if (x.base58prefix != new_chain_specs.base58prefix)|(x.decimals != new_chain_specs.decimals)|(x.encryption != new_chain_specs.encryption)|(x.name != new_chain_specs.name)|(x.unit != new_chain_specs.unit) {return Err(Error::BadInputData(BadInputData::ImportantSpecsChanged))}
        
        // get network verifier
            let network_verifier = get_verifier (x.genesis_hash, &verifiers)?;
        
        // need to check that verifier of message is not "worse" than current general verifier and the verifier of the network
            match verifier {
                Verifier::None => {
                // to proceed, both the general verifier and the verifier for this particular network also should be Verifier::None
                    if current_verifier == Verifier::None {
                        if network_verifier == Verifier::None {
                        // action appears only if the metadata is actually uploaded
                        // "only verifier" warning is not possible
                            let warning_card_1 = Card::Warning(Warning::NotVerified).card(0,0);
                            let warning_card_2 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                            let history = vec![Event::Warning(Warning::NotVerified.show()), Event::Warning(Warning::NetworkAlreadyHasEntries.show())];
                            let index = 2;
                            let upd_network = None;
                            let upd_general = false;
                            let (meta_card, action_card) = process_received_metadata(new_meta_vec, Some(&new_chain_specs.name), history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                            Ok(format!("{{\"warning\":[{},{}],\"meta\":[{}],{}}}", warning_card_1, warning_card_2, meta_card, action_card))
                        }
                        else {return Err(Error::CryptoError(CryptoError::NetworkExistsVerifierDisappeared))}
                    }
                    else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
                },
                _ => {
                let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
                // message has a verifier
                    if current_verifier == verifier {
                        if network_verifier == verifier {
                        // all verifiers are equal, can only update metadata if the version is newer
                        // action appears only if the metadata is actually uploaded
                        // "only verifier" warning is not possible
                            let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                            let history = vec![Event::Warning(Warning::NetworkAlreadyHasEntries.show())];
                            let index = 2;
                            let upd_network = None;
                            let upd_general = false;
                            let (meta_card, action_card) = process_received_metadata(new_meta_vec, Some(&new_chain_specs.name), history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, meta_card, action_card))
                        }
                        else {
                            if network_verifier == Verifier::None {
                            // update metadata if version is newer and update network verifier
                                let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                let warning_card_2 = Card::Warning(Warning::VerifierAppeared).card(2,0);
                                let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(3, 0);
                                let history = vec![Event::Warning(Warning::NetworkAlreadyHasEntries.show()), Event::Warning(Warning::VerifierAppeared.show())];
                                let index = 3;
                                let upd_network = Some(generate_verifier_key(&x.genesis_hash.to_vec()));
                                let upd_general = false;
                                let (meta_card, action_card) = process_received_metadata(new_meta_vec, Some(&new_chain_specs.name), history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                                if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                                else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                            }
                            else {return Err(Error::CryptoError(CryptoError::NetworkExistsVerifierDisappeared))}
                        }
                    }
                    else {
                        if current_verifier == Verifier::None {
                        // need to update the general verifier if the message is ok
                            if network_verifier == verifier {
                                let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared).card(2,0);
                                let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdGeneralVerifier).card(3, 0);
                                let history = vec![Event::Warning(Warning::NetworkAlreadyHasEntries.show()), Event::Warning(Warning::GeneralVerifierAppeared.show())];
                                let index = 3;
                                let upd_network = None;
                                let upd_general = true;
                                let (meta_card, action_card) = process_received_metadata(new_meta_vec, Some(&new_chain_specs.name), history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                                if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                                else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                            }
                            else {
                                if network_verifier == Verifier::None {
                                // need to update both the general verifier and the network verifier
                                    let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                    let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared).card(2,0);
                                    let warning_card_3 = Card::Warning(Warning::VerifierAppeared).card(3,0);
                                    let history = vec![Event::Warning(Warning::NetworkAlreadyHasEntries.show()), Event::Warning(Warning::GeneralVerifierAppeared.show()), Event::Warning(Warning::VerifierAppeared.show())];
                                    let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdBothVerifiers).card(4, 0);
                                    let index = 4;
                                    let upd_network = Some(generate_verifier_key(&x.genesis_hash.to_vec()));
                                    let upd_general = true;
                                    let (meta_card, action_card) = process_received_metadata(new_meta_vec, Some(&new_chain_specs.name), history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                                    if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, meta_card, action_card))}
                                    else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, meta_card, action_card))}
                                    }
                                else {return Err(Error::CryptoError(CryptoError::VerifierChanged{old_show: network_verifier.show_error(), new_show: verifier.show_error()}))}
                            }
                        }
                        else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old_show: current_verifier.show_error(), new_show: verifier.show_error()}))}
                    }
                },
            }
        },
        Err(Error::DatabaseError(DatabaseError::NoNetwork)) => {
        
        // network genesis hash is not on record, this is the most likely variant of add_network procedure

        // TODO it could be possible that the network did change genesis hash,
        // and there are networks with same name in metadata tree of the database;
        // also so far there was no way to ensure network name corresponds uniquely to genesis hash,
        // i.e. in chainspecs tree of the database each name is encountered only once;
        // this possibilities should be looked closer into later, maybe
        
            match verifier {
                Verifier::None => {
                    if current_verifier == Verifier::None {
                        let warning_card = Card::Warning(Warning::AddNetworkNotVerified).card(0,0);
                        let history = vec![Event::Warning(Warning::AddNetworkNotVerified.show())];
                        let index = 1;
                        let upd = false;
                        let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, history, index, verifier, upd, &transaction, &database)?;
                        Ok(format!("{{\"warning\":[{}],\"new_network\":[{}],{}}}", warning_card, new_network_card, action_card))
                    }
                    else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
                },
                _ => {
                    let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
                    if current_verifier == verifier {
                        let history: Vec<Event> = Vec::new();
                        let index = 1;
                        let upd = false;
                        let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, history, index, verifier, upd, &transaction, &database)?;
                        Ok(format!("{{\"verifier\":[{}],\"new_network\":[{}],{}}}", verifier_card, new_network_card, action_card))
                    }
                    else {
                        if current_verifier == Verifier::None {
                            let warning_card = Card::Warning(Warning::GeneralVerifierAppeared).card(1,0);
                            let history = vec![Event::Warning(Warning::GeneralVerifierAppeared.show())];
                            let index = 2;
                            let upd = true;
                            let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, history, index, verifier, upd, &transaction, &database)?;
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_network\":[{}],{}}}", verifier_card, warning_card, new_network_card, action_card))
                        }
                        else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old_show: current_verifier.show_error(), new_show: verifier.show_error()}))}
                    }
                },
            }
        },
        Err(e) => {
        // damaged database, generally unexpected outcome
            return Err(e)
        },
    }
    
}


fn process_received_network_info (meta: Vec<u8>, new_chain_specs: ChainSpecsToSend, history: Vec<Event>, index: u32, verifier: Verifier, upd: bool, transaction: &Tree, database: &Db) -> Result<(String, String), Error> {
    if !meta.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::BadInputData(BadInputData::NotMeta))}
    if meta[4] < 12 {return Err(Error::BadInputData(BadInputData::MetaVersionBelow12))}
    match RuntimeMetadata::decode(&mut &meta[4..]) {
        Ok(received_metadata) => {
            match get_meta_const_light(&received_metadata) {
                Ok(x) => {
                    match VersionDecoded::decode(&mut &x[..]) {
                        Ok(y) => {
                            if y.specname != new_chain_specs.name {return Err(Error::BadInputData(BadInputData::MetaMismatch))}
                            
                            let new_network = (NetworkDisplay{
                                meta_values: MetaValuesDisplay {
                                    name: &y.specname,
                                    version: y.spec_version,
                                    meta_hash: &hex::encode(blake2b(32, &[], &meta).as_bytes()),
                                },
                                network_specs: &new_chain_specs,
                                verifier_line: verifier.show_card(),
                            }).show();
                            
                            let new_network_card = Card::NewNetwork(new_network).card(index, 0);
                            
                            let received_versioned_name = NameVersioned {
                                name: y.specname.to_string(),
                                version: y.spec_version,
                            };
                            let add_network = Transaction::AddNetwork(AddNetwork{
                                versioned_name: received_versioned_name,
                                meta,
                                chainspecs: new_chain_specs,
                                verifier,
                                history,
                            });
                            insert_into_tree(ADDNETWORK.to_vec(), add_network.encode(), transaction)?;
                            flush_db(database)?;
                            let checksum = get_checksum(database)?;
                            let action_card = {
                                if upd {Action::AddNetworkAndAddGeneralVerifier(checksum).card()}
                                else {Action::AddNetwork(checksum).card()}
                            };
                            Ok((new_network_card, action_card))
                        },
                        Err(_) => return Err(Error::BadInputData(BadInputData::VersionNotDecodeable)),
                    }
                },
                Err(_) => return Err(Error::BadInputData(BadInputData::NoMetaVersion)),
            }
        },
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeMeta)),
    }
}
