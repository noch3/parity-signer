/// Separated new cold test databases are created during the tests,
/// and removed after test is performed, so the test can run in parallel

#[cfg(test)]
mod tests {
    use crate::produce_output;
    use db_handling::{populate_cold, populate_cold_no_meta, populate_cold_no_networks, manage_history::print_history};
    use std::fs;
    
    const METADATA_FILE: &str = "for_tests/metadata_database.ts";
    
    #[test]
    fn add_network_westend9090_when_no_network_info_not_signed() {
        let dbname = "for_tests/add_network_westend9090_when_no_network_info_not_signed";
        populate_cold_no_networks(dbname).unwrap();
        let current_history = print_history(dbname).unwrap();
        assert!(current_history == "[]", "Current history: \n{}", current_history);
        let line = fs::read_to_string("for_tests/add_network_westendV9090_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r##"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received new network information is not verified."}],"new_network":[{"index":1,"indent":0,"type":"new_network","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b","base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"","encryption":"none"}}}],"action":{"type":"add_network","payload":{"type":"add_network","checksum":"439183898"}}}"##;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_network_westend9090_when_no_network_info_alice_signed() {
        let dbname = "for_tests/add_network_westend9090_when_no_network_info_alice_signed";
        populate_cold_no_networks(dbname).unwrap();
        let current_history = print_history(dbname).unwrap();
        assert!(current_history == "[]", "Current history: \n{}", current_history);
        let line = fs::read_to_string("for_tests/add_network_westendV9090_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r##"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."}],"new_network":[{"index":2,"indent":0,"type":"new_network","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b","base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}}],"action":{"type":"add_network_and_add_general_verifier","payload":{"type":"add_network_and_add_general_verifier","checksum":"3922263540"}}}"##;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_known_not_signed() {
        let dbname = "for_tests/load_types_known_not_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Types information already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_types_known_alice_signed() {
        let dbname = "for_tests/load_types_known_alice_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Received types information is already in database, only verifier could be added."}],"action":{"type":"add_general_verifier","payload":{"type":"add_general_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn load_types_unknown_not_signed() {
        let dbname = "for_tests/load_types_unknown_not_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":2,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_types_unknown_alice_signed() {
        let dbname = "for_tests/load_types_unknown_alice_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/updating_types_info_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":2,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."}],"types_info":[{"index":3,"indent":0,"type":"types_hash","payload":"d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574"}],"action":{"type":"load_types","payload":{"type":"load_types","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_westend_50_not_in_db() {
        let dbname = "for_tests/parse_transaction_westend_50_not_in_db";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27da40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003200000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"error":[{"index":1,"indent":0,"type":"error","payload":"No metadata on file for this version."}],"extrinsics":[{"index":2,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"27","period":"64","nonce":"46"}},{"index":3,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":4,"indent":0,"type":"block_hash","payload":"538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33"},{"index":5,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"50","tx_version":"5"}}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_1() {
        let dbname = "for_tests/parse_transaction_1";
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
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_2() {
        let dbname = "for_tests/parse_transaction_2";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d550210020c060000d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0700b864d9450006050800aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d0608008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48f501b4003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"batch_all","pallet":"Utility","docs":" Send a batch of dispatch calls and atomically execute them.
 The whole transaction will rollback and fail if any of the calls failed.

 May be called from any origin.

 - `calls`: The calls to be dispatched from the same origin.

 If origin is root then call are dispatch without checking origin filter. (This includes
 bypassing `frame_system::Config::BaseCallFilter`).

 # <weight>
 - Complexity: O(C) where C is the number of calls to be batched.
 # </weight>"}},{"index":2,"indent":1,"type":"varname","payload":"calls"},{"index":3,"indent":2,"type":"call","payload":{"method":"bond","pallet":"Staking","docs":" Take the origin account as a stash and lock up `value` of its balance. `controller` will
 be the account that controls it.

 `value` must be more than the `minimum_balance` specified by `T::Currency`.

 The dispatch origin for this call must be _Signed_ by the stash account.

 Emits `Bonded`.

 # <weight>
 - Independent of the arguments. Moderate complexity.
 - O(1).
 - Three extra DB entries.

 NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned
 unless the `origin` falls below _existential deposit_ and gets removed as dust.
 ------------------
 Weight: O(1)
 DB Weight:
 - Read: Bonded, Ledger, [Origin Account], Current Era, History Depth, Locks
 - Write: Bonded, Payee, [Origin Account], Locks, Ledger
 # </weight>"}},{"index":4,"indent":3,"type":"varname","payload":"controller"},{"index":5,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":6,"indent":5,"type":"Id","payload":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"},{"index":7,"indent":3,"type":"varname","payload":"value"},{"index":8,"indent":4,"type":"balance","payload":{"amount":"300.000000000","units":"mWND"}},{"index":9,"indent":3,"type":"varname","payload":"payee"},{"index":10,"indent":4,"type":"enum_variant_name","payload":{"name":"Staked","docs":""}},{"index":11,"indent":2,"type":"call","payload":{"method":"nominate","pallet":"Staking","docs":" Declare the desire to nominate `targets` for the origin controller.

 Effects will be felt at the beginning of the next era. This can only be called when
 [`EraElectionStatus`] is `Closed`.

 The dispatch origin for this call must be _Signed_ by the controller, not the stash.
 And, it can be only called when [`EraElectionStatus`] is `Closed`.

 # <weight>
 - The transaction's complexity is proportional to the size of `targets` (N)
 which is capped at CompactAssignments::LIMIT (MAX_NOMINATIONS).
 - Both the reads and writes follow a similar pattern.
 ---------
 Weight: O(N)
 where N is the number of targets
 DB Weight:
 - Reads: Era Election Status, Ledger, Current Era
 - Writes: Validators, Nominators
 # </weight>"}},{"index":12,"indent":3,"type":"varname","payload":"targets"},{"index":13,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":14,"indent":5,"type":"Id","payload":"5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ"},{"index":15,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":16,"indent":5,"type":"Id","payload":"5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f"},{"index":17,"indent":2,"type":"call","payload":{"method":"set_controller","pallet":"Staking","docs":" (Re-)set the controller of a stash.

 Effects will be felt at the beginning of the next era.

 The dispatch origin for this call must be _Signed_ by the stash, not the controller.

 # <weight>
 - Independent of the arguments. Insignificant complexity.
 - Contains a limited number of reads.
 - Writes are limited to the `origin` account key.
 ----------
 Weight: O(1)
 DB Weight:
 - Read: Bonded, Ledger New Controller, Ledger Old Controller
 - Write: Bonded, Ledger New Controller, Ledger Old Controller
 # </weight>"}},{"index":18,"indent":3,"type":"varname","payload":"controller"},{"index":19,"indent":4,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":20,"indent":5,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}],"extrinsics":[{"index":21,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"31","period":"64","nonce":"45"}},{"index":22,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":23,"indent":0,"type":"block_hash","payload":"314e9f9aef4e836a54bdd109aba380106e05e2ea83fbc490206b476840cd68e3"},{"index":24,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign_transaction","payload":{"type":"sign_transaction","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn parse_transaction_3() {
        let dbname = "for_tests/parse_transaction_3";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = "530100d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27dac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001750365010f00c06e31d910013223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ea8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cde143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let reply = produce_output(line, dbname);
        let reply_known_part = r#"{"author":[{"index":0,"indent":0,"type":"author","payload":{"base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":"Alice_test_westend"}}],"method":[{"index":1,"indent":0,"type":"call","payload":{"method":"transfer_keep_alive","pallet":"Balances","docs":" Same as the [`transfer`] call, but with a check that the transfer will not kill the
 origin account.

 99% of the time you want [`transfer`] instead.

 [`transfer`]: struct.Pallet.html#method.transfer
 # <weight>
 - Cheaper than transfer because account cannot be killed.
 - Base Weight: 51.4 µs
 - DB Weight: 1 Read and 1 Write to dest (sender is in overlay already)
 #</weight>"}},{"index":2,"indent":1,"type":"varname","payload":"dest"},{"index":3,"indent":2,"type":"enum_variant_name","payload":{"name":"Id","docs":""}},{"index":4,"indent":3,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":5,"indent":1,"type":"varname","payload":"value"},{"index":6,"indent":2,"type":"balance","payload":{"amount":"300.000000000000","units":"WND"}}],"extrinsics":[{"index":7,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"55","period":"64","nonce":"89"}},{"index":8,"indent":0,"type":"tip","payload":{"amount":"300.000000000000","units":"WND"}},{"index":9,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":10,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"9010","tx_version":"5"}}],"action":{"type":"sign_transaction","payload":{"type":"sign_transaction","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn print_all_cards() {
        let dbname = "for_tests/print_all_cards";
        populate_cold_no_networks(dbname).unwrap();
        let line = "5300f0";
        let reply = produce_output(line, dbname);
        let reply_known = r##"{"method":[{"index":0,"indent":0,"type":"call","payload":{"method":"test_Method","pallet":"test_Pallet","docs":"test docs description"}},{"index":1,"indent":0,"type":"pallet","payload":"test_pallet_v14"},{"index":2,"indent":0,"type":"varname","payload":"test_Varname"},{"index":3,"indent":0,"type":"default","payload":"12345"},{"index":4,"indent":0,"type":"path_and_docs","payload":{"path":["frame_system","pallet","Call"],"docs":"test docs"}},{"index":5,"indent":0,"type":"Id","payload":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"},{"index":6,"indent":0,"type":"none","payload":""},{"index":7,"indent":0,"type":"identity_field","payload":"Twitter"},{"index":8,"indent":0,"type":"bitvec","payload":"[00000100, 00100000, 11011001]"},{"index":9,"indent":0,"type":"balance","payload":{"amount":"300.000000","units":"KULU"}},{"index":10,"indent":0,"type":"field_name","payload":{"name":"test_FieldName","docs":""}},{"index":11,"indent":0,"type":"field_number","payload":{"number":"1","docs":""}},{"index":12,"indent":0,"type":"enum_variant_name","payload":{"name":"test_EnumVariantName","docs":""}},{"index":13,"indent":0,"type":"range","payload":{"start":"3","end":"14","inclusive":"false"}},{"index":14,"indent":0,"type":"era_immortal_nonce","payload":{"era":"Immortal","nonce":"4980"}},{"index":15,"indent":0,"type":"era_mortal_nonce","payload":{"era":"Mortal","phase":"55","period":"64","nonce":"89"}},{"index":16,"indent":0,"type":"tip","payload":{"amount":"0","units":"pWND"}},{"index":17,"indent":0,"type":"tip_plain","payload":"8800"},{"index":18,"indent":0,"type":"block_hash","payload":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd"},{"index":19,"indent":0,"type":"tx_spec","payload":{"network":"westend","version":"50","tx_version":"5"}},{"index":20,"indent":0,"type":"tx_spec_plain","payload":{"network_genesis_hash":"a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd","version":"50","tx_version":"5"}},{"index":21,"indent":0,"type":"author","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","seed":"Alice","derivation_path":"//Alice","has_password":false,"name":""}},{"index":22,"indent":0,"type":"author_plain","payload":{"base58":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"}},{"index":23,"indent":0,"type":"author_public_key","payload":{"hex":"8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","crypto":"sr25519"}},{"index":24,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}},{"index":25,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9033","meta_hash":"69300be6f9f5d14ee98294ad15c7af8d34aa6c16f94517216dc4178faadacabb"}},{"index":26,"indent":0,"type":"types_hash","payload":"345f53c073281fc382d20758aee06ceae3014fd53df734d3e94d54642a56dd51"},{"index":27,"indent":0,"type":"new_network","payload":{"specname":"westend","spec_version":"9033","meta_hash":"69300be6f9f5d14ee98294ad15c7af8d34aa6c16f94517216dc4178faadacabb","base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}},{"index":28,"indent":0,"type":"warning","payload":"Transaction author public key not found."},{"index":29,"indent":0,"type":"warning","payload":"Transaction uses outdated runtime version 50. Latest known available version is 9010."},{"index":30,"indent":0,"type":"warning","payload":"Public key is on record, but not associated with the network used."},{"index":31,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":32,"indent":0,"type":"warning","payload":"Received network metadata is not verified."},{"index":33,"indent":0,"type":"warning","payload":"Updating types (really rare operation)."},{"index":34,"indent":0,"type":"warning","payload":"Received types information is not verified."},{"index":35,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":36,"indent":0,"type":"warning","payload":"Received types information is already in database, only verifier could be added."},{"index":37,"indent":0,"type":"warning","payload":"Received metadata is already in database, both general verifier and network verifier could be added."},{"index":38,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."},{"index":39,"indent":0,"type":"warning","payload":"Received metadata is already in database, only general verifier could be added."},{"index":40,"indent":0,"type":"warning","payload":"Add network message is received for network that already has some entries in the database."},{"index":41,"indent":0,"type":"warning","payload":"Received new network information is not verified."},{"index":42,"indent":0,"type":"error","payload":"Data is too short."},{"index":43,"indent":0,"type":"error","payload":"Only Substrate transactions are supported. Transaction is expected to start with 0x53."},{"index":44,"indent":0,"type":"error","payload":"Input data not in hex format."},{"index":45,"indent":0,"type":"error","payload":"Crypto type not supported."},{"index":46,"indent":0,"type":"error","payload":"Expected mortal transaction due to prelude format. Found immortal transaction."},{"index":47,"indent":0,"type":"error","payload":"Expected immortal transaction due to prelude format. Found mortal transaction."},{"index":48,"indent":0,"type":"error","payload":"Wrong payload type, as announced by prelude."},{"index":49,"indent":0,"type":"error","payload":"Genesis hash from extrinsics not matching with genesis hash at the transaction end."},{"index":50,"indent":0,"type":"error","payload":"Block hash for immortal transaction not matching genesis hash for the network."},{"index":51,"indent":0,"type":"error","payload":"After decoding some data remained unused."},{"index":52,"indent":0,"type":"error","payload":"First characters in metadata are expected to be 0x6d657461."},{"index":53,"indent":0,"type":"error","payload":"Received metadata could not be decoded. Runtime metadata version is below 12."},{"index":54,"indent":0,"type":"error","payload":"Received metadata specname does not match."},{"index":55,"indent":0,"type":"error","payload":"Metadata already in database."},{"index":56,"indent":0,"type":"error","payload":"Attempt to load different metadata for same name and version."},{"index":57,"indent":0,"type":"error","payload":"Received metadata version could not be decoded."},{"index":58,"indent":0,"type":"error","payload":"No version in received metadata."},{"index":59,"indent":0,"type":"error","payload":"Unable to decode received metadata."},{"index":60,"indent":0,"type":"error","payload":"Unable to decode received types information."},{"index":61,"indent":0,"type":"error","payload":"Types information already in database."},{"index":62,"indent":0,"type":"error","payload":"Unable to decode received add network message."},{"index":63,"indent":0,"type":"error","payload":"Network already has entries. Important chainspecs in received add network message are different."},{"index":64,"indent":0,"type":"error","payload":"Unable to separate transaction vector, extrinsics, and genesis hash."},{"index":65,"indent":0,"type":"error","payload":"Error on decoding. Expected method and pallet information. Found data is shorter."},{"index":66,"indent":0,"type":"error","payload":"Error on decoding. Expected pallet information. Found data is shorter."},{"index":67,"indent":0,"type":"error","payload":"Method number 2 not found in pallet test_Pallet."},{"index":68,"indent":0,"type":"error","payload":"Pallet with index 3 not found."},{"index":69,"indent":0,"type":"error","payload":"Method number 5 too high for pallet number 3. Only 4 indices available."},{"index":70,"indent":0,"type":"error","payload":"No calls found in pallet test_pallet_v14."},{"index":71,"indent":0,"type":"error","payload":"Error decoding with v14 metadata. Referenced type could not be resolved."},{"index":72,"indent":0,"type":"error","payload":"Argument type error."},{"index":73,"indent":0,"type":"error","payload":"Argument name error."},{"index":74,"indent":0,"type":"error","payload":"Error decoding call contents. Expected primitive type. Found Option<u8>."},{"index":75,"indent":0,"type":"error","payload":"Error decoding call contents. Expected compact. Not found it."},{"index":76,"indent":0,"type":"error","payload":"Error decoding call contents. Data too short for expected content."},{"index":77,"indent":0,"type":"error","payload":"Error decoding call content. Unable to decode part of data as u32."},{"index":78,"indent":0,"type":"error","payload":"Error decoding call content. Encountered unexpected Option<_> variant."},{"index":79,"indent":0,"type":"error","payload":"Error decoding call content. IdentityField description error."},{"index":80,"indent":0,"type":"error","payload":"Error decoding call content. Unable to decode part of data as an [u8; 32] array."},{"index":81,"indent":0,"type":"error","payload":"Error decoding call content. Unexpected type encountered for Balance"},{"index":82,"indent":0,"type":"error","payload":"Error decoding call content. Encountered unexpected enum variant."},{"index":83,"indent":0,"type":"error","payload":"Error decoding call content. Unexpected type inside compact."},{"index":84,"indent":0,"type":"error","payload":"Error decoding call content. Type inside compact cound not be transformed into primitive."},{"index":85,"indent":0,"type":"error","payload":"Error decoding call content. No description found for type T::SomeUnknownType."},{"index":86,"indent":0,"type":"error","payload":"Error decoding call content. Declared type is not suitable BitStore type for BitVec."},{"index":87,"indent":0,"type":"error","payload":"Error decoding call content. Declared type is not suitable BitOrder type for BitVec."},{"index":88,"indent":0,"type":"error","payload":"Error decoding call content. Could not decode BitVec."},{"index":89,"indent":0,"type":"error","payload":"Error decoding call content. Declared type is not suitable index type for Range."},{"index":90,"indent":0,"type":"error","payload":"Error decoding call content. Could not decode Range."},{"index":91,"indent":0,"type":"error","payload":"Database internal error. Collection [1] does not exist"},{"index":92,"indent":0,"type":"error","payload":"Database internal error. Unsupported: Something Unsupported."},{"index":93,"indent":0,"type":"error","payload":"Database internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"},{"index":94,"indent":0,"type":"error","payload":"Database internal error. IO error: oh no!"},{"index":95,"indent":0,"type":"error","payload":"Database internal error. Read corrupted data at file offset None backtrace ()"},{"index":96,"indent":0,"type":"error","payload":"ChainSpecs from database could not be decoded."},{"index":97,"indent":0,"type":"error","payload":"Network not found. Please add the network."},{"index":98,"indent":0,"type":"error","payload":"Address details from database could not be decoded."},{"index":99,"indent":0,"type":"error","payload":"Types database from database could not be decoded."},{"index":100,"indent":0,"type":"error","payload":"Types information not found in the database"},{"index":101,"indent":0,"type":"error","payload":"Network versioned name from metadata database could not be decoded."},{"index":102,"indent":0,"type":"error","payload":"No metadata on file for this version."},{"index":103,"indent":0,"type":"error","payload":"No metadata on file for this network."},{"index":104,"indent":0,"type":"error","payload":"General verifier information from database could not be decoded."},{"index":105,"indent":0,"type":"error","payload":"No general verifier information in the database."},{"index":106,"indent":0,"type":"error","payload":"System error. Balance printing failed."},{"index":107,"indent":0,"type":"error","payload":"System error. First characters in metadata are expected to be 0x6d657461."},{"index":108,"indent":0,"type":"error","payload":"System error. Metadata could not be decoded. Runtime metadata version is below 12."},{"index":109,"indent":0,"type":"error","payload":"Network metadata entry corrupted in database. Please remove the entry and download the metadata for this network."},{"index":110,"indent":0,"type":"error","payload":"System error. No version in metadata."},{"index":111,"indent":0,"type":"error","payload":"System error. Retrieved from metadata version constant could not be decoded."},{"index":112,"indent":0,"type":"error","payload":"System error. Unable to decode metadata."},{"index":113,"indent":0,"type":"error","payload":"System error. Unexpected regular expressions error."},{"index":114,"indent":0,"type":"error","payload":"Corrupted data. Bad signature."},{"index":115,"indent":0,"type":"error","payload":"Different verifier was used for this network previously. Previously used public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Current attempt public key: 5a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969, encryption: sr25519."},{"index":116,"indent":0,"type":"error","payload":"Saved metadata for this network was signed by a verifier. This metadata is not."},{"index":117,"indent":0,"type":"error","payload":"Different general verifier was used previously. Previously used public key: d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d, encryption: ed25519. Current attempt public key: 5a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969, encryption: sr25519."},{"index":118,"indent":0,"type":"error","payload":"General verifier information exists in the database. Received information could be accepted only from the same general verifier."},{"index":119,"indent":0,"type":"error","payload":"Network already has specs recorded in database. Received add network message is not signed, previously this network information was signed."}]}"##;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9070_not_signed() {
        let dbname = "for_tests/load_westend9070_not_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."}],"meta":[{"index":1,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9070_alice_signed() {
        let dbname = "for_tests/load_westend9070_alice_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/network_metadata_westendV9070_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."}],"meta":[{"index":2,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9070","meta_hash":"e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn add_network_westend9090_not_signed() {
        let dbname = "for_tests/add_network_westend9090_not_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/add_network_westendV9090_unverified.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"warning":[{"index":0,"indent":0,"type":"warning","payload":"Received network metadata is not verified."},{"index":1,"indent":0,"type":"warning","payload":"Add network message is received for network that already has some entries in the database."}],"meta":[{"index":2,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b"}}],"action":{"type":"load_metadata","payload":{"type":"load_metadata","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn add_network_westend9090_alice_signed() {
        let dbname = "for_tests/add_network_westend9090_alice_signed";
        populate_cold_no_meta(dbname, true).unwrap();
        let line = fs::read_to_string("for_tests/add_network_westendV9090_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Add network message is received for network that already has some entries in the database."},{"index":2,"indent":0,"type":"warning","payload":"Previously unverified information now received signed by a verifier. If accepted, updating types and adding networks could be verified only by this verifier."},{"index":3,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."}],"meta":[{"index":4,"indent":0,"type":"meta","payload":{"specname":"westend","spec_version":"9090","meta_hash":"62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b"}}],"action":{"type":"load_metadata_and_add_general_verifier","payload":{"type":"load_metadata_and_add_general_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_not_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_not_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9000_already_in_db_alice_signed() {
        let dbname = "for_tests/load_westend9000_already_in_db_alice_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9000_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9010_already_in_db_not_signed() {
        let dbname = "for_tests/load_westend9010_already_in_db_not_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9010_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_westend9010_already_in_db_alice_signed() {
        let dbname = "for_tests/load_westend9010_already_in_db_alice_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_westendV9010_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_kusama2030_already_in_db_not_signed() {
        let dbname = "for_tests/load_kusama2030_already_in_db_not_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_kusamaV2030_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_kusama2030_already_in_db_alice_signed() {
        let dbname = "for_tests/load_kusama2030_already_in_db_alice_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_kusamaV2030_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_polkadot30_already_in_db_not_signed() {
        let dbname = "for_tests/load_polkadot30_already_in_db_not_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_polkadotV30_None.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known = r#"{"error":[{"index":0,"indent":0,"type":"error","payload":"Metadata already in database."}]}"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn load_polkadot30_already_in_db_alice_signed() {
        let dbname = "for_tests/load_polkadot30_already_in_db_alice_signed";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        let line = fs::read_to_string("for_tests/network_from_db_polkadotV30_Alice.txt").unwrap();
        let reply = produce_output(&line.trim(), dbname);
        let reply_known_part = r#"{"verifier":[{"index":0,"indent":0,"type":"verifier","payload":{"hex":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","encryption":"sr25519"}}],"warning":[{"index":1,"indent":0,"type":"warning","payload":"Previously unverified network metadata now received signed by a verifier. If accepted, only metadata from same verifier could be received for this network."},{"index":2,"indent":0,"type":"warning","payload":"Received metadata is already in database, only network verifier could be added."}],"action":{"type":"add_metadata_verifier","payload":{"type":"add_metadata_verifier","checksum":""#;
        assert!(reply.contains(reply_known_part), "Expected: {}...\nReceived: {}", reply_known_part, reply);
        fs::remove_dir_all(dbname).unwrap();
    }

}
