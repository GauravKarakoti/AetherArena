use linera_sdk::{
    base::{ContractAbi, ServiceAbi, Owner, ChainId},
    contract::system_api,
    GraphQLMessage, GraphQLResponse, Service,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use async_graphql::{Request, Response, SimpleObject, Enum};

// Define the ABI for the application
pub struct AetherArenaAbi;

impl ContractAbi for AetherArenaAbi {
    type Parameters = ();
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for AetherArenaAbi {
    type Query = Query;
    type QueryResponse = QueryResponse;
}

// Core data structures
#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Arena {
    pub id: u64,
    pub question: String,
    pub outcomes: Vec<String>,
    pub status: ArenaStatus,
    pub total_stakes: Vec<u64>,
    pub creator: Owner,
    pub created_at: u64,
    pub resolution_outcome: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Enum, PartialEq, Eq)]
pub enum ArenaStatus {
    Open,
    Live,
    Resolving,
    Closed,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Prediction {
    pub arena_id: u64,
    pub owner: Owner,
    pub outcome_index: usize,
    pub amount: u64,
    pub placed_at: u64,
}

// Operation and Message types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Operation {
    CreateArena {
        question: String,
        outcomes: Vec<String>,
    },
    PlacePrediction {
        arena_id: u64,
        outcome_index: usize,
        amount: u64,
    },
    ResolveArena {
        arena_id: u64,
        winning_outcome: usize,
    },
    UpdateArenaStatus {
        arena_id: u64,
        new_status: ArenaStatus,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    PlacePrediction {
        arena_id: u64,
        outcome_index: usize,
        amount: u64,
    },
    ResolveArena {
        arena_id: u64,
        winning_outcome: usize,
    },
}

// Query types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Query {
    GetArena { arena_id: u64 },
    GetArenas { status_filter: Option<ArenaStatus> },
    GetUserPredictions { user: Owner },
    GetArenaPredictions { arena_id: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QueryResponse {
    Arena(Option<Arena>),
    Arenas(Vec<Arena>),
    Predictions(Vec<Prediction>),
    Empty,
}

// Contract state
pub struct AetherArena {
    pub arenas: BTreeMap<u64, Arena>,
    pub predictions: BTreeMap<(u64, Owner), Prediction>,
    pub next_arena_id: u64,
}

impl Default for AetherArena {
    fn default() -> Self {
        Self {
            arenas: BTreeMap::new(),
            predictions: BTreeMap::new(),
            next_arena_id: 0,
        }
    }
}

linera_sdk::contract!(AetherArena);

impl AetherArena {
    fn create_arena(&mut self, question: String, outcomes: Vec<String>, creator: Owner) -> u64 {
        let id = self.next_arena_id;
        self.next_arena_id += 1;
        
        let arena = Arena {
            id,
            question,
            outcomes,
            status: ArenaStatus::Open,
            total_stakes: vec![0; outcomes.len()],
            creator,
            created_at: system_api::current_system_time().as_secs(),
            resolution_outcome: None,
        };
        
        self.arenas.insert(id, arena);
        id
    }

    fn place_prediction(&mut self, arena_id: u64, outcome_index: usize, amount: u64, owner: Owner) -> Result<(), String> {
        let arena = self.arenas.get_mut(&arena_id)
            .ok_or("Arena not found")?;
            
        if arena.status != ArenaStatus::Open {
            return Err("Arena is not accepting predictions".to_string());
        }
        
        if outcome_index >= arena.outcomes.len() {
            return Err("Invalid outcome index".to_string());
        }
        
        // Update total stakes
        arena.total_stakes[outcome_index] += amount;
        
        // Store prediction
        let prediction = Prediction {
            arena_id,
            owner,
            outcome_index,
            amount,
            placed_at: system_api::current_system_time().as_secs(),
        };
        
        self.predictions.insert((arena_id, owner), prediction);
        Ok(())
    }

    fn resolve_arena(&mut self, arena_id: u64, winning_outcome: usize) -> Result<(), String> {
        let arena = self.arenas.get_mut(&arena_id)
            .ok_or("Arena not found")?;
            
        if winning_outcome >= arena.outcomes.len() {
            return Err("Invalid winning outcome".to_string());
        }
        
        arena.status = ArenaStatus::Closed;
        arena.resolution_outcome = Some(winning_outcome);
        
        // TODO: In future waves, implement payout logic here
        Ok(())
    }

    fn update_arena_status(&mut self, arena_id: u64, new_status: ArenaStatus) -> Result<(), String> {
        let arena = self.arenas.get_mut(&arena_id)
            .ok_or("Arena not found")?;
        
        // Validate state transitions
        match (&arena.status, &new_status) {
            (ArenaStatus::Open, ArenaStatus::Live) => {},
            (ArenaStatus::Live, ArenaStatus::Resolving) => {},
            (ArenaStatus::Resolving, ArenaStatus::Closed) => {},
            _ => return Err("Invalid status transition".to_string()),
        }
        
        arena.status = new_status;
        Ok(())
    }
}

impl linera_sdk::contract::Contract for AetherArena {
    type Abi = AetherArenaAbi;

    fn initialize(&mut self, _argument: Self::InitializationArgument) {}

    fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::CreateArena { question, outcomes } => {
                let creator = system_api::current_application_id().owner;
                self.create_arena(question, outcomes, creator);
            }
            Operation::PlacePrediction { arena_id, outcome_index, amount } => {
                let owner = system_api::current_application_id().owner;
                if let Err(e) = self.place_prediction(arena_id, outcome_index, amount, owner) {
                    log::warn!("Failed to place prediction: {}", e);
                }
            }
            Operation::ResolveArena { arena_id, winning_outcome } => {
                if let Err(e) = self.resolve_arena(arena_id, winning_outcome) {
                    log::warn!("Failed to resolve arena: {}", e);
                }
            }
            Operation::UpdateArenaStatus { arena_id, new_status } => {
                if let Err(e) = self.update_arena_status(arena_id, new_status) {
                    log::warn!("Failed to update arena status: {}", e);
                }
            }
        }
    }

    fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::PlacePrediction { arena_id, outcome_index, amount } => {
                let owner = system_api::current_application_id().owner;
                let _ = self.place_prediction(arena_id, outcome_index, amount, owner);
            }
            Message::ResolveArena { arena_id, winning_outcome } => {
                let _ = self.resolve_arena(arena_id, winning_outcome);
            }
        }
    }
}

// Service implementation for GraphQL
impl linera_sdk::service::Service for AetherArena {
    type Abi = AetherArenaAbi;

    fn handle_query(&mut self, request: Request) -> Response {
        // Handle GraphQL queries here
        // This will be implemented in the next step
        Response::default()
    }
}