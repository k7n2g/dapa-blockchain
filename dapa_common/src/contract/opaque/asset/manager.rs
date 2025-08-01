use blake3::hash;
use xelis_vm::{
    Context,
    EnvironmentError,
    FnInstance,
    FnParams,
    FnReturnType,
    Primitive
};
use crate::{
    asset::{AssetData, AssetOwner},
    config::COST_PER_TOKEN,
    contract::{from_context, get_optional_asset_from_cache, AssetChanges, ContractOutput, ContractProvider},
    crypto::{Hash, HASH_SIZE},
    versioned_type::VersionedState
};
use super::Asset;

// Maximum size for the ticker
pub const TICKER_LEN: usize = 8;

// Verify if the asset str is valid
fn is_valid_str_for_asset(name: &str, whitespace: bool, uppercase_only: bool) -> bool {
    if whitespace {
        if name.starts_with(" ") || name.ends_with(" ") {
            return false
        }
    }

    name.chars().all(|c| is_valid_char_for_asset(c, whitespace, uppercase_only))
}

// Check if the char for an asset is valid
fn is_valid_char_for_asset(c: char, whitespace: bool, uppercase_only: bool) -> bool {
    match c {
        'A'..='Z'
        | '0'..='9' => true,
        | 'a'..='z' if !uppercase_only => true,
        | ' ' if whitespace => true,
        _ => false
    }
}

// Create a new asset
// Return None if the asset already exists
pub fn asset_create<P: ContractProvider>(_: FnInstance, mut params: FnParams, context: &mut Context) -> FnReturnType {
    let (provider, chain_state) = from_context::<P>(context)?;

    let max_supply = match params.remove(4).into_owned()?.take_as_optional()? {
        Some(v) => Some(v.to_u64()?),
        _ => None,
    };
    let decimals = params.remove(3)
        .into_owned()?
        .to_u8()?;

    let ticker = params.remove(2)
        .into_owned()?
        .into_string()?;

    if ticker.len() > TICKER_LEN {
        return Err(EnvironmentError::Expect("Asset ticker is too long".to_owned()).into());
    }

    // Ticker can be ASCII & upper case only
    // No whitespace is allowed in it
    if !is_valid_str_for_asset(&ticker, false, true) {
        return Err(EnvironmentError::Expect("Asset ticker must be ASCII only".to_owned()).into());
    }

    let name = params.remove(1)
        .into_owned()?
        .into_string()?;
    if name.len() > u8::MAX as usize {
        return Err(EnvironmentError::Expect("Asset name is too long".to_owned()).into());
    }

    // Name can be ASCII only
    if !is_valid_str_for_asset(&name, true, false) {
        return Err(EnvironmentError::Expect("Asset name must be ASCII only".to_owned()).into());
    }

    let id = params.remove(0).as_u64()?;

    let mut buffer = [0u8; 40];
    buffer[0..HASH_SIZE].copy_from_slice(chain_state.contract.as_bytes());
    buffer[HASH_SIZE..].copy_from_slice(&id.to_be_bytes());

    let asset_hash = Hash::new(hash(&buffer).into());
    // We must be sure that we don't have this asset already
    if get_optional_asset_from_cache(provider, chain_state, asset_hash.clone())?.is_some() {
        return Ok(Some(Primitive::Null.into()));
    }

    let data = AssetData::new(decimals, name, ticker, max_supply, Some(AssetOwner::new(chain_state.contract.clone(), id)));
    chain_state.assets.insert(asset_hash.clone(), Some(AssetChanges {
        data: (VersionedState::New, data.clone()),
        supply: None
    }));

    // If we have a max supply, we need to mint it to the contract
    if let Some(max_supply) = max_supply {
        // We don't bother to check if it already exists, because it shouldn't exist before we create it.
        chain_state.cache.balances.insert(asset_hash.clone(), Some((VersionedState::New, max_supply)));
    }

    chain_state.outputs.push(ContractOutput::NewAsset { asset: asset_hash.clone() });

    // Pay the cost for a new token
    context.increase_gas_usage(COST_PER_TOKEN)?;

    let asset = Asset {
        hash: asset_hash
    };
    Ok(Some(Primitive::Opaque(asset.into()).into()))
}

pub fn asset_get_by_id<P: ContractProvider>(_: FnInstance, params: FnParams, context: &mut Context) -> FnReturnType {
    let id = params[0].as_u64()?;
    let (provider, chain_state) = from_context::<P>(context)?;

    let mut buffer = [0u8; 40];
    buffer[0..HASH_SIZE].copy_from_slice(chain_state.contract.as_bytes());
    buffer[HASH_SIZE..].copy_from_slice(&id.to_be_bytes());

    let asset_hash = Hash::new(hash(&buffer).into());
    if get_optional_asset_from_cache(provider, chain_state, asset_hash.clone())?.is_none() {
        return Ok(Some(Primitive::Null.into()))
    }

    let asset = Asset {
        hash: asset_hash
    };
    Ok(Some(Primitive::Opaque(asset.into()).into()))
}

pub fn asset_get_by_hash<P: ContractProvider>(_: FnInstance, mut params: FnParams, context: &mut Context) -> FnReturnType {
    let hash: Hash = params.remove(0)
        .into_owned()?
        .into_opaque_type()?;

    let (provider, chain_state) = from_context::<P>(context)?;

    if get_optional_asset_from_cache(provider, chain_state, hash.clone())?.is_none() {
        return Ok(Some(Primitive::Null.into()))
    }

    let asset = Asset {
        hash
    };
    Ok(Some(Primitive::Opaque(asset.into()).into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_various_asset_names() {
        assert!(is_valid_str_for_asset("DAPA", true, false));
        assert!(is_valid_str_for_asset("DAPAAI99", true, false));
        assert!(is_valid_str_for_asset("DAPA POW 123", true, false));
        assert!(is_valid_str_for_asset("ZZZZZZ", true, true));

        // check only uppercase
        assert!(!is_valid_str_for_asset("ZZZZZZzzzZ", true, true));

        // check whitespaces
        assert!(!is_valid_str_for_asset(" DAPA", true, false));
        assert!(!is_valid_str_for_asset("DAPA   ", true, false));
    }
}