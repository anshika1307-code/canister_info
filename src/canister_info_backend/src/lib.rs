use candid::{CandidType, Principal};
// Returns canister info with all available canister changes 
// foolowed by given principal

// principal??? later 

// Traps if the canister_info management call is rejected 
#[ic_cdk::update]
async fn info(canister_id: Principal) -> CanisterInfoResponse {
    let request = CanisterInfoRequest {
        canister_id,
        num_requested_changes: Some(20),
    };
    canister_info(request).await.unwrap().0
}

// Performs Breadth-First Search over controllers.
// Uses the info function to retrieve canister information.
//Checks if the principal characterizes a canister by determining if it is an opaque principal.
//this func returns all controllers (reflexive and transitive) of a canister characterized by a given principal.
#[ic_cdk::update]
async fn reflexive_transitive_controllers(canister_id: Principal) -> Vec<Principal> {
    // Initialize vectors
    let mut ctrls = vec![canister_id]; //to store controllers
    let mut queue = vec![canister_id];  // manage the BFS queue

    // Perform BFS over ctrls
    //continues until the BFS queue is empty
    while !queue.is_empty() {
        // Pop the current canister ID from the queue
        let cur = queue.pop().unwrap();

        // this is Checking if the principal characterizes a canister by determining if it is an varid principal [last byte of the principal is 0x01]
        if cur.as_slice().last() == Some(&0x01) {
            // Call the info function asynchronously to get canister info
            let info = info(cur).await;

            // Iterate over ctrls in the canister info
            for c in info.controllers {
                // Check if the controller is not already in the list
                if !ctrls.contains(&c) {
                    // Add the controller to the list and enqueue it for further operation
                    ctrls.push(c);
                    queue.push(c);
                }
            }
        }
    }

    // Return all ctrls
    ctrls
}
 //canister snapshot
 //Unix timestamp in nanoseconds or canister version

#[derive(CandidType, Deserialize, Clone)]
pub enum CanisterSnapshot {
    #[serde(rename = "at_timestamp")]  //still not very clear what this line is doing
    AtTimestamp(u64),
    #[serde(rename = "at_version")]   //still not very clear what this line is doing
    AtVersion(u64),
}

