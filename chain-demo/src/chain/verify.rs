use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum InvalidReason {
    
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyResult(Vec<InvalidReason>);