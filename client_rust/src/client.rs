use chikin_airdrop::state::{ChikinAirdropPool, ChikinAirdropUser};
use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;

type Error = Box<dyn std::error::Error>;

pub fn get_airdrop_pool(
    rpc_client: &RpcClient,
    address: &Pubkey,
) -> Result<ChikinAirdropPool, Error> {
    let data = rpc_client.get_account_data(address)?;
    let object = ChikinAirdropPool::unpack_unchecked(&data)
        .map_err(|e| format!("Invalid airdrop pool {}: {}", address, e))?;
    Ok(object)
}

pub fn get_airdrop_user(
    rpc_client: &RpcClient,
    address: &Pubkey,
) -> Result<ChikinAirdropUser, Error> {
    let data = rpc_client.get_account_data(address)?;
    let object = ChikinAirdropUser::unpack_unchecked(&data)
        .map_err(|e| format!("Invalid airdrop user {}: {}", address, e))?;
    Ok(object)
}

