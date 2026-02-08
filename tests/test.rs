use std::collections::HashMap;
use std::fs;
use serde_json::json;

// #[test]
// fn test_genesis_deserialization() {
//     // Create a temporary genesis file for testing
//     let test_genesis = json!({
//         "balances": {
//             "user1": 1000,
//             "user2": 2000
//         }
//     });

//     // Create test directory if it doesn't exist
//     fs::create_dir_all("./database").unwrap();
    
//     // Write test genesis file
//     fs::write(
//         "./database/genesis.json",
//         serde_json::to_string_pretty(&test_genesis).unwrap(),
//     ).unwrap();

//     // Try to load the state
//     match rust_blockchain::state::new_state_from_disk() {
//         Ok(state) => {
//             // Verify the balances were loaded correctly
//             assert_eq!(state.balances.len(), 2);
//             assert_eq!(state.balances.get("user1").unwrap(), &1000);
//             assert_eq!(state.balances.get("user2").unwrap(), &2000);
//         },
//         Err(e) => panic!("Failed to load state: {}", e),
//     }

//     // Clean up test file
//     fs::remove_file("./database/genesis.json").unwrap();
// }