use std::future::Future;

/// An identifier for a unique piece of software that connects to the zcash network.
/// User agent is defined here. [<https://developer.mozilla.org/en-US/docs/Glossary/User_agent>]
pub struct UserAgentId {
    /// I.E. the name of the software
    pub paradigm: String,
    /// The version of the software.
    pub version: String,
}

/// A reference to a block on a blockchain.
/// See librustzcash zcash_protocol/src/consensus.rs
#[derive(PartialEq)] //
#[derive(Eq)] //
#[derive(PartialOrd)] //
#[derive(Ord)] //
#[derive(Debug)] //
pub struct BlockHeight(pub u32);

/// A low-level request for a payment.
/// see librustzcash components zip321/src/lib.rs  
pub struct Payment {
    /// The address to which the payment should be sent.
    pub recipient_address: String,

    /// The amount of the payment that is being requested.
    pub amount: u64,

    /// A memo that, if included, must be provided with the payment.
    /// If a memo is present and [`recipient_address`] is not a shielded
    /// address, the wallet should report an error.
    ///
    /// [`recipient_address`]: #structfield.recipient_address
    pub memo: Option<String>,

    /// A list of other arbitrary key/value pairs associated with this payment.
    pub other_params: Vec<(String, String)>,
}

pub trait Wallet {
    /// To return the name and version of the wallet software.
    fn user_agent_id() -> UserAgentId;

    /// To create a new wallet from scratch.
    /// This wallet is recommended be empty. It should not perform any actions without being instructed.
    /// It should not generate any keys.
    /// It should not connect to any servers.
    fn new_wallet() -> impl Future<Output = Self> + Send;

    /// An error that can result from trying to connect to or sync from a server.
    type BeginScanningServerRangeError;
    /// To connect to a server and scan it.
    /// The wallet should immediately ping the server and confirm that they use compatible protocols. If not, or if the server does not answer, return an error.
    /// The wallet should sync the specified blocks.
    /// If `minimum_block` is omitted, use current server height.
    /// If `maximum_block` is omitted, scan to indefinite height.
    fn begin_scanning_server_range(
        &mut self,
        server_address: String,
        minimum_block: Option<BlockHeight>,
        maximum_block: Option<BlockHeight>,
    ) -> impl Future<Output = Result<(), Self::BeginScanningServerRangeError>> + Send;

    /// An error that can result from trying to add a key. The [`Self::add_key`] method comments explain.
    type AddKeyError;
    /// To add a key.
    /// The wallet must interpret the string as a zcash key.
    /// The wallet should use this key to attempt to decrypt blocks and discover notes.
    fn add_key(
        &mut self,
        key_string: String,
    ) -> impl Future<Output = Result<(), Self::AddKeyError>> + Send;

    /// An error that can result from querying the max height the wallet has scanned for a server. The [`Self::get_max_scanned_height_for_server`] method comments explain.
    type GetMaxScannedHeightError;
    /// To report what height it has reached scanning the chain provided by a particular server.
    /// If the wallet is not following this server, it must return an error.
    fn get_max_scanned_height_for_server(
        &mut self,
        server: String,
    ) -> impl Future<Output = Result<BlockHeight, Self::GetMaxScannedHeightError>> + Send;

    /// An error that can result from attempting a payment. The [`Self::pay`] method comments explain.
    type PayError;
    /// To make a payment.
    /// The wallet must construct a well-formed transaction with the provided specifications.
    /// The wallet must try to cause this transaction to be confirmed on chain.
    fn pay(
        &mut self,
        payments: Vec<Payment>,
    ) -> impl Future<Output = Result<(), Self::PayError>> + Send;
}
