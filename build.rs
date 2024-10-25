use hex::decode;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let chialisp_dictionary = [
        // CAT Puzzles
        ("CAT_PUZZLE", "./chia_puzzles/puzzles/cat_puzzles/cat_v2.clsp.hex"),
        ("DELEGATED_TAIL", "./chia_puzzles/puzzles/cat_puzzles/delegated_tail.clsp.hex"),
        ("EVERYTHING_WITH_SIGNATURE", "./chia_puzzles/puzzles/cat_puzzles/everything_with_signature.clsp.hex"),
        ("GENESIS_BY_COIN_ID_OR_SINGLETON", "./chia_puzzles/puzzles/cat_puzzles/genesis_by_coin_id_or_singleton.clsp.hex"),
        ("GENESIS_BY_COIN_ID", "./chia_puzzles/puzzles/cat_puzzles/genesis_by_coin_id.clsp.hex"),
        ("GENESIS_BY_PUZZLE_HASH", "./chia_puzzles/puzzles/cat_puzzles/genesis_by_puzzle_hash.clsp.hex"),
        // DAO Puzzles
        ("DAO_CAT_EVE", "./chia_puzzles/puzzles/dao_puzzles/dao_cat_eve.clsp.hex"),
        ("DAO_CAT_LAUNCHER", "./chia_puzzles/puzzles/dao_puzzles/dao_cat_launcher.clsp.hex"),
        ("DAO_FINISHED_STATE", "./chia_puzzles/puzzles/dao_puzzles/dao_finished_state.clsp.hex"),
        ("DAO_LOCKUP", "./chia_puzzles/puzzles/dao_puzzles/dao_lockup.clsp.hex"),
        ("DAO_PROPOSAL_TIMER", "./chia_puzzles/puzzles/dao_puzzles/dao_proposal_timer.clsp.hex"),
        ("DAO_PROPOSAL_VALIDATOR", "./chia_puzzles/puzzles/dao_puzzles/dao_proposal_validator.clsp.hex"),
        ("DAO_PROPOSAL", "./chia_puzzles/puzzles/dao_puzzles/dao_proposal.clsp.hex"),
        ("DAO_SPEND_P2_SINGLETON", "./chia_puzzles/puzzles/dao_puzzles/dao_spend_p2_singleton_v2.clsp.hex"),
        ("DAO_TREASURY", "./chia_puzzles/puzzles/dao_puzzles/dao_treasury.clsp.hex"),
        ("DAO_UPDATE_PROPOSAL", "./chia_puzzles/puzzles/dao_puzzles/dao_update_proposal.clsp.hex"), 
        // DID Puzzles
        ("DID_INNERPUZ", "./chia_puzzles/puzzles/did_puzzles/did_innerpuz.clsp.hex"),
        // NFT Puzzles
        ("CREATE_NFT_LAUNCHER_FROM_DID", "./chia_puzzles/puzzles/nft_puzzles/create_nft_launcher_from_did.clsp.hex"),
        ("NFT_INTERMEDIATE_LAUNCHER", "./chia_puzzles/puzzles/nft_puzzles/nft_intermediate_launcher.clsp.hex"),
        ("NFT_METADATA_UPDATER_DEFAULT", "./chia_puzzles/puzzles/nft_puzzles/nft_metadata_updater_default.clsp.hex"),
        ("NFT_METADATA_UPDATER_UPDATEABLE", "./chia_puzzles/puzzles/nft_puzzles/nft_metadata_updater_updateable.clsp.hex"),
        ("NFT_OWNERSHIP_LAYER", "./chia_puzzles/puzzles/nft_puzzles/nft_ownership_layer.clsp.hex"),
        ("NFT_OWNERSHIP_TRANSFER_PROGRAM_ONE_WAY_CLAIM_WITH_ROYALTIES", "./chia_puzzles/puzzles/nft_puzzles/nft_ownership_transfer_program_one_way_claim_with_royalties.clsp.hex"),
        ("NFT_STATE_LAYER", "./chia_puzzles/puzzles/nft_puzzles/nft_state_layer.clsp.hex"),
        // CR Puzzles
        ("CONDITIONS_W_FEE_ANNOUNCE", "./chia_puzzles/puzzles/vc_puzzles/cr_puzzles/conditions_w_fee_announce.clsp.hex"),
        ("CREDENTIAL_RESTRICTION", "./chia_puzzles/puzzles/vc_puzzles/cr_puzzles/credential_restriction.clsp.hex"),
        ("FLAG_PROOFS_CHECKER", "./chia_puzzles/puzzles/vc_puzzles/cr_puzzles/flag_proofs_checker.clsp.hex"),
        // VC Puzzles
        ("COVENANT_LAYER", "./chia_puzzles/puzzles/vc_puzzles/covenant_layer.clsp.hex"),
        ("EML_COVENANT_MORPHER", "./chia_puzzles/puzzles/vc_puzzles/eml_covenant_morpher.clsp.hex"),
        ("EML_TRANSFER_PROGRAM_COVENANT_ADAPTER", "./chia_puzzles/puzzles/vc_puzzles/eml_transfer_program_covenant_adapter.clsp.hex"),
        ("EML_UPDATE_METADATA_WITH_DID", "./chia_puzzles/puzzles/vc_puzzles/eml_update_metadata_with_DID.clsp.hex"),
        ("EXIGENT_METADATA_LAYER", "./chia_puzzles/puzzles/vc_puzzles/exigent_metadata_layer.clsp.hex"),
        ("P2_ANNOUNCED_DELEGATED_PUZZLE", "./chia_puzzles/puzzles/vc_puzzles/p2_announced_delegated_puzzle.clsp.hex"),
        ("STANDARD_VC_BACKDOOR_PUZZLE", "./chia_puzzles/puzzles/vc_puzzles/standard_vc_backdoor_puzzle.clsp.hex"),
        ("STD_PARENT_MORPHER", "./chia_puzzles/puzzles/vc_puzzles/std_parent_morpher.clsp.hex"),
        ("VIRAL_BACKDOOR", "./chia_puzzles/puzzles/vc_puzzles/viral_backdoor.clsp.hex"),
        // Misc Puzzles
        ("AUGMENTED_CONDITION", "./chia_puzzles/puzzles/augmented_condition.clsp.hex"),
        ("NOTIFICATION", "./chia_puzzles/puzzles/notification.clsp.hex"),
        ("P2_1_OF_N", "./chia_puzzles/puzzles/p2_1_of_n.clsp.hex"),
        ("P2_CONDITIONS", "./chia_puzzles/puzzles/p2_conditions.clsp.hex"),
        ("P2_DELEGATED_CONDITIONS", "./chia_puzzles/puzzles/p2_delegated_conditions.clsp.hex"),
        ("P2_DELEGATED_PUZZLE_OR_HIDDEN_PUZZZLE", "./chia_puzzles/puzzles/p2_delegated_puzzle_or_hidden_puzzle.clsp.hex"),
        ("P2_DELEGATED_PUZZLE", "./chia_puzzles/puzzles/p2_delegated_puzzle.clsp.hex"),
        ("P2_M_OF_N_DELEGATE_DIRECT", "./chia_puzzles/puzzles/p2_m_of_n_delegate_direct.clsp.hex"),
        ("P2_PARENT", "./chia_puzzles/puzzles/p2_parent.clsp.hex"),
        ("P2_SINGLETON_AGGREGATOR", "./chia_puzzles/puzzles/p2_singleton_aggregator.clsp.hex"),
        ("P2_SINGLETON_OR_DELAYED_PUZHASH", "./chia_puzzles/puzzles/p2_singleton_or_delayed_puzhash.clsp.hex"),
        ("P2_SINGLETON_VIA_DELEGATED_PUZZLE", "./chia_puzzles/puzzles/p2_singleton_via_delegated_puzzle.clsp.hex"),
        ("P2_SINGLETON", "./chia_puzzles/puzzles/p2_singleton.clsp.hex"),
        ("SETTLEMENT_PAYMENT", "./chia_puzzles/puzzles/settlement_payments.clsp.hex"),
        // Singleton Puzzle
        ("SINGLETON_LAUNCHER", "./chia_puzzles/puzzles/singleton_launcher.clsp.hex"),
        ("SINGLETON_TOP_LAYER_V1_1", "./chia_puzzles/puzzles/singleton_top_layer_v1_1.clsp.hex"),
        ("SINGLETON_TOP_LAYER", "./chia_puzzles/puzzles/singleton_top_layer.clsp.hex"),


    ];
    let dest_path = Path::new("./src/loaded_chialisp.rs");
    let f = File::create(dest_path).expect("unable to create file");
    let mut f = BufWriter::new(f);

    for (name, file_path) in chialisp_dictionary {
        let hex_file_path = Path::new(file_path);

        let hex_data = fs::read_to_string(hex_file_path).expect("Failed to read the hex file");

        println!("Loaded hex data from {}: {}", name, hex_data);

        let hex_data = hex_data.trim().replace(['\n', '\r'], "");

        let bytes = decode(&hex_data).expect("Failed to decode hex string");

        let byte_array_string = bytes
            .iter()
            .map(|byte| format!("0x{:02x}", byte))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            f,
            "pub const {}: [u8; {}] = [{}];",
            name,
            bytes.len(),
            byte_array_string
        )
        .expect("Failed to write to loaded_chialisp.rs");
    }
}
