#![cfg_attr(not(feature = "std"), no_std)]


use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// An index to a block.
pub type BlockNumber = u64;
/// Alias to 512-bit hash when used in the context of a transaction signature on
/// the chain.
pub type Signature = MultiSignature;
/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;
/// Alias to the opaque account ID type for this chain, actually a
/// `AccountId32`. This is always 32 bytes.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;
/// Game Id
pub type GameId = u64;
/// Amount for transaction type
pub type Amount = i128;
/// Currency Id type
pub type CurrencyId = u32;
/// Group collection id type
pub type SeriesId = u64;
/// AssetId for all NFT and FT
pub type AssetId = u64;
/// AuctionId
pub type AuctionId = u64;

/// Public item id for auction
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ItemId {
    NFT(AssetId),
    Block(u64),
}