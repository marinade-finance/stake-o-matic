use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    std::error,
};

#[derive(Debug, PartialEq)]
pub enum ValidatorStakeState {
    None,     // Validator should receive no stake
    Baseline, // Validator has been awarded a baseline stake
    Bonus,    // Validator has been awarded a bonus stake in addition to the baseline stake
}

pub struct ValidatorStake {
    pub identity: Pubkey,
    pub vote_address: Pubkey,
    pub stake_state: ValidatorStakeState,
    pub memo: String,
}

pub trait GenericStakePool {
    fn is_enrolled(&self, validator_identity: &Pubkey) -> bool;
    fn apply(
        &mut self,
        rpc_client: &RpcClient,
        dry_run: bool,
        authorized_staker: &Keypair,
        desired_validator_stake: &[ValidatorStake],
    ) -> Result<Vec<String>, Box<dyn error::Error>>;
}