/// Separated new cold test databases are created during the tests,
/// and removed after test is performed, so the test can run in parallel

#[cfg(test)]
mod tests {
    use transaction_parsing::{produce_output, cards::Action};
    use crate::{handle_action, error::{Error, ActionFailure}, interpretation::interpret_action, sign_transaction::create_signature};
    use db_handling::{populate_cold, populate_cold_no_networks, populate_cold_no_meta};
    use constants::{METATREE, SPECSTREE};
    use std::fs;
    use sled::{Db, open, Tree};
    use regex::Regex;
    use lazy_static::lazy_static;
    
    const METADATA_FILE: &str = "for_tests/metadata_database.ts";
    const SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    const PWD: &str = "jaskier";
    const USER_COMMENT: &str = "";
    
    lazy_static! {
        static ref ACTION: Regex = Regex::new(r#"(?i)"action":\{.*?,"payload":(?P<action_line>\{[^}]*\})\}"#).expect("constructed from checked static value");
    }
    
    fn get_action_line(reply: &str) -> String {
        let caps = ACTION.captures(&reply).unwrap();
        caps.name("action_line").unwrap().as_str().to_string()
    }
    
    fn sign_action_test (action_line: &str, seed_phrase: &str, pwd_entry: &str, user_comment: &str, dbname: &str) -> anyhow::Result<String> {
        let action = interpret_action (action_line)?;
        if let Action::SignTransaction(checksum) = action {create_signature(seed_phrase, pwd_entry, user_comment, dbname, checksum)}
        else {return Err(Error::NoAction(ActionFailure::SignTransaction).show())}
    }
    
    fn meta_count_test (dbname: &str) -> usize {
         let database: Db = open(dbname).unwrap();
         let metadata: Tree = database.open_tree(METATREE).unwrap();
         metadata.len()
    }
    
    fn specs_count_test (dbname: &str) -> usize {
         let database: Db = open(dbname).unwrap();
         let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
         chainspecs.len()
    }
    
// can sign a parsed transaction
    #[test]
    fn can_sign_transaction_1() {
        let dbname = "for_tests/can_sign_transaction_1";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"transfer_keep_alive","pallet":"Balances","docs":" Same as the [`transfer`] call, but with a check that the transfer will not kill the
 origin account.

 99% of the time you want [`transfer`] instead.

 [`transfer`]: struct.Pallet.html#method.transfer
 # <weight>
 - Cheaper than transfer because account cannot be killed.
 - Base Weight: 51.4 µs
 - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
 #</weight>"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"100.000000000","units":"mWND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":9,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign_transaction","payload":{"type":"sign_transaction","checksum":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = sign_action_test(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        match result {
            Ok(signature) => assert!((signature.len() == 130) && (signature.starts_with("01")), "Wrong signature format,\nReceived:\n{}", signature),
            Err(e) => panic!("Was unable to sign. {}", e),
        }
        let result = sign_action_test(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {
            let err = e.to_string();
            let expected_err = String::from("Database checksum mismatch.");
            if err != expected_err {panic!("Expected wrong checksum. Got error: {}.", err)}
        }
        else {panic!("Checksum should have changed.")}
        fs::remove_dir_all(dbname).unwrap();
    }

// add_network for dock_main without verifier, then add_network with same metadata and with verifier
    #[test]
    fn add_network_add_two_verifiers_later() {
        
        let dbname = "for_tests/add_network_add_two_verifiers_later";
        populate_cold_no_networks(dbname).unwrap();
        let meta1 = meta_count_test(dbname);
        let specs1 = specs_count_test(dbname);
        
        let line = fs::read_to_string("for_tests/add_network_westendV9090_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received new network information is not verified."}],"new_network":[{"index":1,"indent":0,"type":"new_network","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b","base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"","encryption":"none"}}}],"action":{"type":"add_network","payload":{"type":"add_network","checksum":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to add network. {}", e)}
        
        let meta2 = meta_count_test(dbname);
        let specs2 = specs_count_test(dbname);
        
        let line = fs::read_to_string("for_tests/add_network_westendV9090_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Add network message is received for network that already has some entries in the database."},{"index":2,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":3,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":4,"indent":0,"type":"warning","payload":"Received metadata is already in database, both general verifier and network verifier could be added."}],"action":{"type":"add_two_verifiers","payload":{"type":"add_two_verifiers","checksum":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to update two verifiers. {}", e)}
        
        let meta3 = meta_count_test(dbname);
        let specs3 = specs_count_test(dbname);
        
        assert!(meta2 == meta1+1, "Did not add metadata to database on first step.");
        assert!(meta3 == meta2, "Number of meta entries somehow changed on second step.");
        assert!(specs2 == specs1+1, "Did not add specs to database on first step.");
        assert!(specs3 == specs2, "Number of specs entries somehow changed on second step.");
        
        fs::remove_dir_all(dbname).unwrap();
    }

// add_network for dock_main with verifier
    #[test]
    fn add_network_and_add_general_verifier() {
    
        let dbname = "for_tests/add_network_and_add_general_verifier";
        populate_cold_no_networks(dbname).unwrap();
        
        let meta1 = meta_count_test(dbname);
        let specs1 = specs_count_test(dbname);
        
        let line = fs::read_to_string("for_tests/add_network_westendV9090_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."}],"new_network":[{"index":2,"indent":0,"type":"new_network","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b","base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}}],"action":{"type":"add_network_and_add_general_verifier","payload":{"type":"add_network_and_add_general_verifier","checksum":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to add network and update general verifier. {}", e)}
        
        let meta2 = meta_count_test(dbname);
        let specs2 = specs_count_test(dbname);
        
        assert!(meta2 == meta1+1, "Did not add metadata to database.");
        assert!(specs2 == specs1+1, "Did not add specs to database.");
        
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn correct_checksum_no_transaction_to_sign() {
    
        let dbname = "for_tests/correct_checksum_no_transaction_to_sign";
        populate_cold_no_networks(dbname).unwrap();
        
        // real action: add_network
        let line = fs::read_to_string("for_tests/add_network_westendV9090_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: sign_transaction
        let mock_action_line = get_action_line(&reply).replace("add_network", "sign_transaction");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::SignTransaction).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn correct_checksum_no_approved_metadata() {
    
        let dbname = "for_tests/correct_checksum_no_approved_metadata";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: load_metadata
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "load_metadata");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::LoadMeta).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_metadata_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_metadata_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: add_metadata_verifier
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "add_metadata_verifier");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::AddVerifier).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_types_to_load() {
    
        let dbname = "for_tests/correct_checksum_no_types_to_load";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: load_types
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "load_types");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::LoadTypes).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_general_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_general_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: add_general_verifier
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "add_general_verifier");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::AddGeneralVerifier).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_two_verifiers() {
    
        let dbname = "for_tests/correct_checksum_no_two_verifiers";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: add_two_verifiers
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "add_two_verifiers");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::AddVerifier).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_load_meta_and_upd_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_load_meta_and_upd_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: load_metadata_and_add_general_verifier
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "load_metadata_and_add_general_verifier");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::LoadMeta).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_add_network() {
    
        let dbname = "for_tests/correct_checksum_no_add_network";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: add_network
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "add_network");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::AddNetwork).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn correct_checksum_no_add_network_and_general_verifier() {
    
        let dbname = "for_tests/correct_checksum_no_add_network_and_general_verifier";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        // real action: sign_transaction
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(&line.trim(), dbname);
        
        // wrong action: add_network_and_add_general_verifier
        let mock_action_line = get_action_line(&reply).replace("sign_transaction", "add_network_and_add_general_verifier");
        
        match handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname) {
            Ok(_) => panic!("Should have failed. Parser reply: {}\nMock action line: {}", reply, mock_action_line),
            Err(e) => {
                if e.to_string() != Error::NoAction(ActionFailure::AddNetwork).show().to_string() {
                    panic!("Should have failed\nwith correct checksum and wrong action. Parser reply: {}\nMock action line: {}", reply, mock_action_line)
                }
            },
        }
        fs::remove_dir_all(dbname).unwrap();
        
    }
    
// load_metadata for westend9070 not verified, then load same metadata, but with verifier
    #[test]
    fn load_network_unsigned_add_verifier_later() {
        
        let dbname = "for_tests/load_network_unsigned_add_verifier_later";
        populate_cold_no_meta(dbname, true).unwrap();
        
        let meta1 = meta_count_test(dbname);
        let specs1 = specs_count_test(dbname);
        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to load metadata for westend 9070 network. {}", e)}
        
        let meta2 = meta_count_test(dbname);
        let specs2 = specs_count_test(dbname);
        
        assert!(meta2 == meta1+1, "Did not add metadata to database.");
        assert!(specs2 == specs1, "Number of specs entries somehow changed.");
        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":""##;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to add verifier for westend. {}", e)}
        
        let meta3 = meta_count_test(dbname);
        let specs3 = specs_count_test(dbname);
        
        assert!(meta3 == meta2, "Number of meta entries somehow changed.");
        assert!(specs3 == specs2, "Number of specs entries somehow changed.");
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
// load_types not verified, then load same types message, but with verifier
    #[test]
    fn load_types_unsigned_add_verifier_later() {
        
        let dbname = "for_tests/load_types_unsigned_add_verifier_later";
        populate_cold_no_networks(dbname).unwrap();
        
        let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to load types. {}", e)}
        
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is already in database, only verifier could be added."}],"action":{"type":"add_general_verifier","payload":{"type":"add_general_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to add general verifier. {}", e)}
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
// load_types with verifier, general verifier appears, still can load metadata without verifier, but cannot add networks unverified
    #[test]
    fn load_types_verified_then_test_unverified_load_metadata_and_unverified_add_network() {
        
        let dbname = "for_tests/load_types_verified_then_test_unverified_load_metadata_and_unverified_add_network";
        populate_cold_no_meta(dbname, true).unwrap();

        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to load types with adding general verifier. {}", e)}

    // loading metadata without verifier - should work        
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":""#;
        assert!(reply.contains(reply_known_part), "Error in action.\nReceived: {}", reply);
        let mock_action_line = get_action_line(&reply);
        let result = handle_action(&mock_action_line, SEED_PHRASE, PWD, USER_COMMENT, dbname);
        if let Err(e) = result {panic!("Was unable to load metadata without signature after general verifier appeared. {}", e)}
        
    // adding network without verifier - should not work
        let line = fs::read_to_string("for_tests/add_network_westendV9090_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."}]}"#;
        assert!(reply == reply_known, "Error in parsing outcome.\nReceived: {}", reply);
        
        fs::remove_dir_all(dbname).unwrap();
    }

}
