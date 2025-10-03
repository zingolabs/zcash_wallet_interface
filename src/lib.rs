use std::future::Future;

/// An identifier for a unique piece of software that connects to the zcash network.
pub struct ProtocolId {
    /// The name of the software.
    pub name: String,
    /// The version of the software.
    pub version: String,
}

/// A reference to a block on a blockchain.
/// See librustzcash zcash_protocol/src/consensus.rs
pub struct BlockHeight(u32);

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
    fn protocol_id() -> ProtocolId;

    /// To create a new wallet from scratch.
    /// This wallet is recommended be empty. It should not perform any actions without being instructed.
    /// It should not generate any keys.
    /// It should not connect to any servers.
    fn new_wallet() -> impl Future<Output = Self> + Send;

    /// An error that can result from trying to add a server connection. The [add_server] method comments explain.
    type AddServerError;
    /// To connect to a server.
    /// The wallet should immediately ping the server and confirm that they use compatible protocols. If not, or if the server does not answer, return an error.
    /// The wallet is now responsible for interpreting data from the server and should begin to sync a shared understanding of the chain state.
    fn add_server(
        &mut self,
        server_address: String,
    ) -> impl Future<Output = Result<(), Self::AddServerError>> + Send;

    /// An error that can result from trying to add a key. The [add_key] method comments explain.
    type AddKeyError;
    /// To add a key.
    /// The wallet must interpret the string as a zcash key.
    /// The wallet should use this key to attempt to decrypt blocks and discover notes.
    fn add_key(
        &mut self,
        key_string: String,
    ) -> impl Future<Output = Result<(), Self::AddKeyError>> + Send;

    /// An error that can result from querying the max height the wallet has scanned for a server. The [get_max_scanned_height_for_server] method comments explain.
    type GetMaxScannedHeightError;
    /// To report what height it has reached scanning the chain provided by a particular server.
    /// If the wallet is not following this server, it must return an error.
    fn get_max_scanned_height_for_server(
        &mut self,
        server: String,
    ) -> impl Future<Output = Result<BlockHeight, Self::GetMaxScannedHeightError>> + Send;

    /// An error that can result from attempting a payment. The [pay] method comments explain.
    type PayError;
    /// To make a payment.
    /// The wallet must construct a well-formed transaction with the provided specifications.
    /// The wallet must try to cause this transaction to be confirmed on chain.
    fn pay(
        &mut self,
        payments: Vec<Payment>,
    ) -> impl Future<Output = Result<(), Self::PayError>> + Send;
}
