#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod transaction_tests {
    use bsv_wasm::*;
    extern crate wasm_bindgen_test;
    use bsv_wasm::TxIn;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialise_and_serialise_transaction_hex() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.get_size().unwrap(), 439);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0x02);

        let tx_in_0 = tx.get_input(0).unwrap();

        assert_eq!(tx_in_0.get_prev_tx_id(None), hex::decode("3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e").unwrap());
        assert_eq!(tx_in_0.get_vout(), 0);
        assert_eq!(tx_in_0.get_script_sig_size(), 0x8c);
        assert_eq!(tx_in_0.get_script_sig().to_bytes().len(), 0x8c);
        assert_eq!(tx_in_0.get_script_sig().to_bytes(), hex::decode("493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2").unwrap());
        assert_eq!(tx_in_0.get_sequence(), 4294967295);

        let tx_in_1 = tx.get_input(1).unwrap();
        assert_eq!(tx_in_1.get_vout(), 2);
        assert_eq!(tx_in_1.get_script_sig_size(), 0x8b);
        assert_eq!(tx_in_1.get_script_sig().to_bytes().len(), 0x8b);
        assert_eq!(tx_in_1.get_script_sig().to_bytes(), hex::decode("48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442").unwrap());
        assert_eq!(tx_in_1.get_sequence(), 4294967295);

        let tx_out_0 = tx.get_output(0).unwrap();
        assert_eq!(tx_out_0.get_satoshis(), 1076000);
        assert_eq!(tx_out_0.get_script_pub_key_size(), 25);
        assert_eq!(tx_out_0.get_script_pub_key(), Script::from_hex("76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac").unwrap());

        let tx_out_1 = tx.get_output(1).unwrap();
        assert_eq!(tx_out_1.get_satoshis(), 117488);
        assert_eq!(tx_out_1.get_script_pub_key_size(), 25);
        assert_eq!(tx_out_1.get_script_pub_key(), Script::from_hex("76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac").unwrap());

        let json_string = tx.to_json_string().unwrap();
        assert_eq!(json_string, "{\"version\":1,\"inputs\":[{\"prev_tx_id\":\"3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e\",\"vout\":0,\"script_sig\":[\"3046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f2701\",\"04510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2\"],\"sequence\":4294967295},{\"prev_tx_id\":\"6f653a93e7ff01c3317ee9eb9b75c85d4881684f8117f73f4765b61a7a5e19a3\",\"vout\":2,\"script_sig\":[\"304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c501\",\"042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442\"],\"sequence\":4294967295}],\"outputs\":[{\"value\":1076000,\"script_pub_key\":[\"OP_DUP\",\"OP_HASH160\",\"20bb5c3bfaef0231dc05190e7f1c8e22e098991e\",\"OP_EQUALVERIFY\",\"OP_CHECKSIG\"]},{\"value\":117488,\"script_pub_key\":[\"OP_DUP\",\"OP_HASH160\",\"9e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c\",\"OP_EQUALVERIFY\",\"OP_CHECKSIG\"]}],\"n_locktime\":0}");

        let rehydrated_tx = Transaction::from_json_string(&json_string).unwrap();

        assert_eq!(rehydrated_tx, tx, "Rehydrated JSON Tx doesnt match original Tx")
    }

    #[test]
    fn deserialise_transaction_hex_malformed() {
        let tx_hex = "FAKE01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex);

        assert!(tx.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new_transaction() {
        let tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_input_to_transaction() {
        let mut tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);

        let input = TxIn::new(&[], 0, &Script::default(), None);

        tx.add_input(&input);
        assert_eq!(tx.get_ninputs(), 1);
        assert_eq!(tx.get_input(0), Some(input));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_output_to_transaction() {
        let mut tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);

        let output = TxOut::new(0, &Script::default());

        tx.add_output(&output);
        assert_eq!(tx.get_noutputs(), 1);
        assert_eq!(tx.get_output(0), Some(output));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txin_to_hex() {
        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin = TxIn::from_hex(txin_hex).unwrap();

        assert_eq!(txin.to_hex().unwrap(), txin_hex);
    }

    #[test]
    fn txin_to_hex_fail() {
        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txin = TxIn::from_hex(txin_hex);

        assert!(txin.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txout_to_hex() {
        let txout_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout = TxOut::from_hex(txout_hex).unwrap();

        assert_eq!(txout.to_hex().unwrap(), txout_hex)
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn transaction_to_hex() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn add_txins_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_ninputs(), 2);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin1 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin1.to_hex().unwrap(), txin_hex);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin2 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin2.to_hex().unwrap(), txin_hex);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin3 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin3.to_hex().unwrap(), txin_hex);

        tx.add_inputs(vec![txin1, txin2, txin3]);

        assert_eq!(tx.get_ninputs(), 5);
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_txins_to_transaction_wasm() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_ninputs(), 2);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin1 = TxIn::from_hex(txin_hex).unwrap();

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin2 = TxIn::from_hex(txin_hex).unwrap();

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin3 = TxIn::from_hex(txin_hex).unwrap();

        let boxed_vals = Box::from(vec![JsValue::from(txin1.clone()), JsValue::from(txin2.clone()), JsValue::from(txin3.clone())]);
        tx.add_inputs(boxed_vals);

        assert_eq!(tx.get_ninputs(), 5);

        assert_eq!(tx.get_input(4).unwrap().get_outpoint_hex(None), txin3.get_outpoint_hex(None));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn add_txouts_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_noutputs(), 2);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout1 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout1.to_hex().unwrap(), txin_hex);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout2 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout2.to_hex().unwrap(), txin_hex);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout3 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout3.to_hex().unwrap(), txin_hex);

        tx.add_outputs(vec![txout1, txout2, txout3]);

        assert_eq!(tx.get_noutputs(), 5);
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_txouts_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_noutputs(), 2);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout1 = TxOut::from_hex(txin_hex).unwrap();

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout2 = TxOut::from_hex(txin_hex).unwrap();

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout3 = TxOut::from_hex(txin_hex).unwrap();

        let boxed_vals = Box::from(vec![JsValue::from(txout1.clone()), JsValue::from(txout2.clone()), JsValue::from(txout3.clone())]);
        tx.add_outputs(boxed_vals);

        assert_eq!(tx.get_noutputs(), 5);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let mut txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_1.set_satoshis(500);
        tx.add_input(&txin_1);
        let mut txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_2.set_satoshis(500);
        tx.add_input(&txin_2);
        let mut txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_3.set_satoshis(2);
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), Some(1002));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis_single_none_satoshis_returns_none() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let mut txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_1.set_satoshis(500);
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let mut txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_3.set_satoshis(2);
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis_all_none_satoshis_returns_none() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_output_satoshis() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(
            5000,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(
            400,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(
            9999999,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));

        assert_eq!(tx.satoshis_out(), 5000 + 400 + 9999999)
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_output_satoshis_all_zero_returns_zero() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));

        assert_eq!(tx.satoshis_out(), 0)
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn get_outpoints() {
        let mut tx = Transaction::new(1, 0);

        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            2,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        let outpoints = tx.get_outpoints();

        assert_eq!(&outpoints[0].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f00000000");
        assert_eq!(&outpoints[1].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f01000000");
        assert_eq!(&outpoints[2].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f02000000");
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(target_arch = "wasm32")]
    fn get_outpoints_wasm() {
        let mut tx = Transaction::new(1, 0);

        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            2,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        let outpoints = tx.get_outpoints().unwrap();

        let outpoint_slice: Vec<Vec<u8>> = outpoints.into_serde().unwrap();

        assert_eq!(&outpoint_slice[0].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f00000000");
        assert_eq!(&outpoint_slice[1].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f01000000");
        assert_eq!(&outpoint_slice[2].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f02000000");
    }

    // For future validation
    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn txin_error_when_given_invalid_txid() {
    //     let txin_1 = TxIn::new(
    //         &hex::decode("30333933366363303566333461616365383537616666663461643330656465353364646539366465313436303334363033303136356364313436613834356133").unwrap(),
    //         1,
    //         &Script::from_hex("76a9147a1e5b4edd76b81c816ecba65f61c78afb79bdb888ac").unwrap(),
    //         Some(0xffffffff),
    //     );
    //     assert!(txin_1.is_err(), "TxIn should error when passed an invalid txid")
    // }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txin_from_outpoint() {
        let outpoint = "9057a1b008f17a6e1b1b39522072598ecf0d73b256c6b2c34e98257d72ce3c7907000000";

        // 793cce727d25984ec3b2c656b2730dcf8e59722052391b1b6e7af108b0a15790 <-- we need this
        // 0100000001793cce727d25984ec3b2c656b2730dcf8e59722052391b1b6e7af108b0a15790070000006a47304402204628fa202f16798ef858baba566f34d96434c61241460ee453cab8193ce87a0102206fd052780b9a8efe4d5fa6d7a2f969220e14aeb7a10b1392b5fda11417f32699412102e6bb51b303ca9cb8e805fa3d104cb030e0ad6872678f5ddb2e3a14188c571f33ffffffff0223020000000000001976a914a0b2113032da4c2c0e22960f12045cecca837c7088ac2f020000000000001976a9146fb1a8e42086219215c45a5e9cb1d94d8fbd845388ac00000000

        let mut tx = Transaction::default();
        let txin = TxIn::from_outpoint_bytes(&hex::decode(outpoint).unwrap()).unwrap();
        tx.add_input(&txin);

        assert_eq!(&txin.get_outpoint_hex(Some(true)), outpoint)
    }

    #[test]
    fn txin_from_outpoint_slice_too_short_should_error() {
        let txin = TxIn::from_outpoint_bytes(&vec![]);

        assert!(txin.is_err(), "An Outpoint must be precisely 36 bytes long")
    }
}
