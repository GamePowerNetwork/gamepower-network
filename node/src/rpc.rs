//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use gamepower_runtime::{opaque::Block, BlockNumber, AccountId, Balance, Hash, Index};
use sp_api::ProvideRuntimeApi;
use sc_client_api::light::{Fetcher, RemoteBlockchain};
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_block_builder::BlockBuilder;
use sp_transaction_pool::TransactionPool;
use sc_finality_grandpa::FinalityProofProvider;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Nonce = u32;

/// Full client dependencies.
pub struct FullDeps<C, P, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,

	pub grandpa: GrandpaDeps<B>,
}

/// Light client extra dependencies.
pub struct LightDeps<C, F, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Remote access to the blockchain (async).
	pub remote_blockchain: Arc<dyn RemoteBlockchain<Block>>,
	/// Fetcher instance.
	pub fetcher: Arc<F>,
}

/// Dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: sc_finality_grandpa::SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: sc_finality_grandpa::SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: sc_finality_grandpa::GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: sc_rpc::SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, B>(
	deps: FullDeps<C, P, B>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	use substrate_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use sc_finality_grandpa_rpc::{GrandpaApi, GrandpaRpcHandler};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		deny_unsafe,
		grandpa,
	} = deps;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe))
	);

	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
	);

	io.extend_with(
		GrandpaApi::to_delegate(GrandpaRpcHandler::new(
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			subscription_executor,
			finality_provider,
		))
	);

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	io
}

/// Instantiate all RPC extensions for light node.
pub fn create_light<C, P, F>(deps: LightDeps<C, F, P>) -> RpcExtension
	where
		C: ProvideRuntimeApi<Block>,
		C: HeaderBackend<Block>,
		C: Send + Sync + 'static,
		C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
		C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
		P: TransactionPool + Sync + Send + 'static,
		F: Fetcher<Block> + 'static,
{
	use substrate_frame_rpc_system::{LightSystem, SystemApi};

	let LightDeps {
		client,
		pool,
		remote_blockchain,
		fetcher,
	} = deps;
	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		SystemApi::<Hash, AccountId, Nonce>::to_delegate(LightSystem::new(client, remote_blockchain, fetcher, pool))
	);
	io
}
