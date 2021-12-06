use heapless::{Vec, String};

pub struct AccountAddress([u8; AccountAddress::LENGTH]);
impl AccountAddress {
    /// The number of bytes in an address.
    pub const LENGTH: usize = 16;
}
pub struct Identifier(String<16>);

pub struct ChainId(u8);

// unsupported
pub struct WriteSetPayload;
// unsupported
pub struct Script;
// unsupported
pub struct Module;

pub struct ModuleId {
    address: AccountAddress,
    name: Identifier,
}

pub enum TypeTag {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector,
    Struct,
}


pub struct ScriptFunction {
    module: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag, 4>,
    args: Vec<Vec<u8, 32>, 6>,
}

pub enum TransactionPayload {
    /// A system maintenance transaction.
    WriteSet(WriteSetPayload),
    /// A transaction that executes code.
    Script(Script),
    /// A transaction that publishes code.
    Module(Module),
    /// A transaction that executes an existing script function published on-chain.
    ScriptFunction(ScriptFunction),
}

pub struct RawTransaction {
    /// Sender's address.
    sender: AccountAddress,

    /// Sequence number of this transaction. This must match the sequence number
    /// stored in the sender's account at the time the transaction executes.
    sequence_number: u64,

    /// The transaction payload, e.g., a script to execute.
    payload: TransactionPayload,

    /// Maximal total gas to spend for this transaction.
    max_gas_amount: u64,

    /// Price to be paid per gas unit.
    gas_unit_price: u64,

    /// The currency code, e.g., "XUS", used to pay for gas. The `max_gas_amount`
    /// and `gas_unit_price` values refer to units of this currency.
    gas_currency_code: String<4>,

    /// Expiration timestamp for this transaction, represented
    /// as seconds from the Unix Epoch. If the current blockchain timestamp
    /// is greater than or equal to this time, then the transaction has
    /// expired and will be discarded. This can be set to a large value far
    /// in the future to indicate that a transaction does not expire.
    expiration_timestamp_secs: u64,

    /// Chain ID of the Diem network this transaction is intended for.
    chain_id: ChainId,
}
