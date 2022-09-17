use crate::types::Nep171TokenMetadata;
use frame_support::dispatch::DispatchError;
use sp_runtime::KeyTypeId;
use sp_std::prelude::*;

pub trait AppchainInterface {
	fn is_activated() -> bool;

	fn next_set_id() -> u32;
}

/// Something that can provide a set of validators for the next era.
pub trait ValidatorsProvider<AccountId> {
	/// A new set of validators.
	fn validators() -> Vec<(AccountId, u128)>;
}

pub trait TokenIdAndAssetIdProvider<AssetId> {
	type Err;

	fn try_get_asset_id(token_id: impl AsRef<[u8]>) -> Result<AssetId, Self::Err>;

	fn try_get_token_id(asset_id: AssetId) -> Result<Vec<u8>, Self::Err>;
}

pub trait LposInterface<AccountId> {
	fn is_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<AccountId>;

	fn active_stake_of(who: &AccountId) -> u128;

	fn active_total_stake() -> Option<u128>;
}

pub trait UpwardMessagesInterface<AccountId> {
	fn submit(
		who: Option<AccountId>,
		payload_type: crate::types::PayloadType,
		payload: &[u8],
	) -> Result<u64, DispatchError>;
}

pub trait ConvertIntoNep171 {
	type CollectionId;
	type ItemId;
	fn convert_into_nep171_metadata(
		collection: Self::CollectionId,
		item: Self::ItemId,
	) -> Option<Nep171TokenMetadata>;
}

impl ConvertIntoNep171 for () {
	type CollectionId = u128;
	type ItemId = u128;
	fn convert_into_nep171_metadata(
		_collection: Self::CollectionId,
		_item: Self::ItemId,
	) -> Option<Nep171TokenMetadata> {
		None
	}
}
