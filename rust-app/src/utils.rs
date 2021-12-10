use core::char;
use core::str;
use core::fmt::Write;
use heapless::{String, Vec};

// Max length supported for heapless vec and string
const MAX_LENGTH : usize = 50;

/// Convert to hex. Returns a static buffer of 64 bytes
#[inline]
pub fn to_hex(m: &[u8]) -> Result<[u8; 64], ()> {
    if 2 * m.len() > 64 {
        return Err(());
    }
    let mut hex = [0u8; 64];
    let mut i = 0;
    for c in m {
        let c0 = char::from_digit((c >> 4).into(), 16).unwrap();
        let c1 = char::from_digit((c & 0xf).into(), 16).unwrap();
        hex[i] = c0 as u8;
        hex[i + 1] = c1 as u8;
        i += 2;
    }
    Ok(hex)
}

#[derive(Debug)]
pub struct TransactionFieldsToReview {
    sequence_number: u64,
    account_address: Option<String<MAX_LENGTH>>,
    module_name: Option<String<MAX_LENGTH>>,
    function_name: Option<String<MAX_LENGTH>>,
}

impl TransactionFieldsToReview {
    pub fn build_transaction_fields(sequence_number: u64) -> Self {
        Self {
            sequence_number,
            account_address: None,
            module_name: None,
            function_name: None,
        }
    }
}

/// Encode the byte array in vec to a heapless utf8 string.
fn encode_to_utf8(target: Vec<u8, MAX_LENGTH>) -> String<MAX_LENGTH>{
    let function_name_str = str::from_utf8(&target).unwrap();

    function_name_str.into()
}

/// Extract part of the byte array to a heapless vec.
fn extract_to_vec(original: &[u8], start: usize, len: usize) -> Vec<u8, MAX_LENGTH>{
    let mut target: Vec<u8, MAX_LENGTH> = Vec::new();
    for i in start..start + len {
        target.push(original[i]);
    }

    target
}

/// Extract fields from a byte array of raw transaction.
#[inline]
fn get_transaction_fields_to_review(raw_txn: &[u8]) -> Result<TransactionFieldsToReview, ()> {
    const ACCOUNT_ADDRESS_LENGTH: usize = 16;
    const SEQUENCE_NUMBER_LENGTH: usize = 8;
    // 3 is the index of ScriptFunction in TransactionPayload Enum
    const SCRIPT_FUNCTION_IN_TRANSACTION_PAYLOAD: u8 = 3;

    // Sequence number
    let mut moving_index: usize = ACCOUNT_ADDRESS_LENGTH;
    let mut sequence_number_byte_array: [u8; SEQUENCE_NUMBER_LENGTH] = [0; SEQUENCE_NUMBER_LENGTH];
    sequence_number_byte_array
        .copy_from_slice(&raw_txn[moving_index..moving_index + SEQUENCE_NUMBER_LENGTH]);
    moving_index += SEQUENCE_NUMBER_LENGTH;
    let sequence_number = u64::from_le_bytes(sequence_number_byte_array);

    let index_in_enum = raw_txn[moving_index];
    moving_index += 1;

    // Support ScriptFunction only
    if index_in_enum == SCRIPT_FUNCTION_IN_TRANSACTION_PAYLOAD {
        // Account address
        let account_address_byte_array = extract_to_vec(&raw_txn, moving_index, ACCOUNT_ADDRESS_LENGTH);
        moving_index += ACCOUNT_ADDRESS_LENGTH;
        let mut account_address_str: String<MAX_LENGTH> = String::new();
        for byte in account_address_byte_array {
            write!(account_address_str, "{:02X}", byte);
        }

        // Module name
        let module_name_length: usize = raw_txn[moving_index] as usize;
        moving_index += 1;
        let module_name_vec = extract_to_vec(&raw_txn, moving_index, module_name_length);
        let module_name_str = encode_to_utf8(module_name_vec);
        moving_index += module_name_length;

        // Function name
        let function_name_length = raw_txn[moving_index] as usize;
        moving_index += 1;
        let function_name_vec = extract_to_vec(&raw_txn, moving_index, function_name_length);
        let function_name_str = encode_to_utf8(function_name_vec);
        moving_index += function_name_length;

        // TransactionFieldsToReview
        let fields = TransactionFieldsToReview {
            account_address: Some(account_address_str),
            module_name: Some(module_name_str),
            function_name: Some(function_name_str),
            sequence_number
        };

        Ok(fields)
    } else {
        let fields = TransactionFieldsToReview::build_transaction_fields(sequence_number);

        Ok(fields)
    }
}