//! error types
//!

use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Token {
    #[error("internal error")]
    InternalError,
    #[error("error deserializing or verifying the token")]
    Format(Format),
    #[error("the authority block must have the index 0")]
    InvalidAuthorityIndex(u32),
    #[error("the block index does not match its position")]
    InvalidBlockIndex(InvalidBlockIndex),
    #[error("multiple blocks declare the same symbols")]
    SymbolTableOverlap,
    #[error(r#"the symbol table is missing either "authority" or "ambient""#)]
    MissingSymbols,
    #[error("tried to append a block to a sealed token")]
    Sealed,
    #[error("caveat validation failed")]
    FailedLogic(Logic),
    #[error("Datalog parsing error")]
    ParseError,
    #[error("Reached Datalog execution limits")]
    RunLimit(RunLimit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidBlockIndex {
    pub expected: u32,
    pub found: u32,
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Format {
    #[error("failed verifying the signature")]
    Signature(Signature),
    #[error("failed verifying the signature of a sealed token")]
    SealedSignature,
    #[error("the token does not provide intermediate public keys")]
    EmptyKeys,
    #[error("the root public key was not recognized")]
    UnknownPublicKey,
    #[error("could not deserialize the wrapper object")]
    DeserializationError(String),
    #[error("could not serialize the wrapper object")]
    SerializationError(String),
    #[error("could not deserialize the block")]
    BlockDeserializationError(String),
    #[error("could not serialize the block")]
    BlockSerializationError(String),
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Signature {
    #[error("could not parse the signature elements")]
    InvalidFormat,
    #[error("the signature did not match")]
    InvalidSignature,
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Logic {
    #[error("a fact of the authority block did not have the authority tag")]
    InvalidAuthorityFact(String),
    #[error("a fact provided or generated by the verifier did not have the ambient tag")]
    InvalidAmbientFact(String),
    #[error("a fact provided or generated by a block had the authority or ambient tag")]
    InvalidBlockFact(u32, String),
    #[error("a rule provided by a block is generating facts with the authority or ambient tag")]
    InvalidBlockRule(u32, String),
    #[error("list of caveats that failed validation")]
    FailedCaveats(Vec<FailedCaveat>),
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum FailedCaveat {
    #[error("a caveat failed in a block")]
    Block(FailedBlockCaveat),
    #[error("a caveat provided by the verifier failed")]
    Verifier(FailedVerifierCaveat),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailedBlockCaveat {
    pub block_id: u32,
    pub caveat_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailedVerifierCaveat {
    pub caveat_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum RunLimit {
    #[error("too many facts generated")]
    TooManyFacts,
    #[error("too many engine iterations")]
    TooManyIterations,
    #[error("spent too much time verifying")]
    Timeout,
}
