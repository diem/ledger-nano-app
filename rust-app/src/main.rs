#![no_std]
#![no_main]

mod crypto_helpers;
mod utils;
mod diem_types;

use core::str::from_utf8;
use crypto_helpers::*;
use nanos_sdk::buttons::ButtonEvent;
use nanos_sdk::ecc::Ed25519Signature;
use nanos_sdk::{ecc, io};
use nanos_sdk::io::{Comm, StatusWords, SyscallError};
use nanos_ui::ui;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

pub const DATA_BUFFER_MAX_SIZE: usize = 1280;
static mut DATA_BUFFER: [u8; 1280] = [0; DATA_BUFFER_MAX_SIZE];
static mut DATA_BUFFER_LEN: usize = 0;
pub const DATA_CHUNK_SIZE: u8 = 250;

/// Display public key in two separate
/// message scrollers
fn show_pubkey() {
    let pubkey = get_pubkey();
    match pubkey {
        Ok(pk) => {
            let hex0 = utils::to_hex(&pk).unwrap();
            let m = from_utf8(&hex0).unwrap();
            ui::MessageScroller::new(m).event_loop();
        }
        Err(_) => ui::popup("Error"),
    }
}

/// Basic nested menu. Will be subject
/// to simplifications in the future.
#[allow(clippy::needless_borrow)]
fn menu_example() {
    loop {
        match ui::Menu::new(&[&"PubKey", &"Infos", &"Back", &"Exit App"]).show() {
            0 => show_pubkey(),
            1 => loop {
                match ui::Menu::new(&[&"Copyright", &"Authors", &"Back"]).show() {
                    0 => ui::popup("2020 Ledger"),
                    1 => ui::popup("???"),
                    _ => break,
                }
            },
            2 => return,
            3 => nanos_sdk::exit_app(0),
            _ => (),
        }
    }
}

/// This is the UI flow for signing, composed of a scroller
/// to read the incoming message, a panel that requests user
/// validation, and an exit message.
fn sign_ui(message: &[u8]) -> Result<Option<Ed25519Signature>, SyscallError> {
    ui::popup("Sign Txn");
    ui::popup("Txn SHA3 Review");

    // First 32 bytes contains seed. After that it contains the RawTxn bytes
    // Only take hash of the RawTxn Bytes
    let sha_result =  ecc::sha3(&message[32..]);
    let hex = to_hex(&sha_result).unwrap();
    let sha_hex = from_utf8(&hex).unwrap();
    ui::MessageScroller::new(sha_hex).event_loop();

    if ui::Validator::new("Sign ?").ask() {
        let k = get_private_key()?;
        let sig = ed25519_sign(message, &k).unwrap();
        Ok(Some(sig))
    } else {
        ui::popup("Cancelled");
        Ok(None)
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    loop {
        // Draw some 'welcome' screen
        ui::SingleMessage::new("Trove Wallet").show();

        // Wait for either a specific button push to exit the app
        // or an APDU command
        match comm.next_event() {
            io::Event::Button(ButtonEvent::RightButtonRelease) => nanos_sdk::exit_app(0),
            io::Event::Command(ins) => match handle_apdu(&mut comm, ins) {
                Ok(()) => comm.reply_ok(),
                Err(sw) => comm.reply(sw),
            },
            _ => (),
        }
    }
}

#[repr(u8)]
enum Ins {
    GetPubkey,
    Sign,
    Menu,
    ShowPrivateKey,
    Exit,
}

impl From<u8> for Ins {
    fn from(ins: u8) -> Ins {
        match ins {
            2 => Ins::GetPubkey,
            3 => Ins::Sign,
            4 => Ins::Menu,
            0xfe => Ins::ShowPrivateKey,
            0xff => Ins::Exit,
            _ => Ins::GetPubkey,
        }
    }
}

use nanos_sdk::io::Reply;
use crate::utils::to_hex;

unsafe fn clear_data_buffer() {
    DATA_BUFFER.iter_mut().for_each(|m| *m = 0);
    DATA_BUFFER_LEN = 0;
}

// Read one chunk into the data buffer
// p1 represents chunk number starting from 0. If there are 4 chunks
// p1 for each of those chunks will be 0, 1, 2, 3
// p2 is 1 iff this chunk is the last chunk
// Returns an err if there is an overflow
// Returns Ok(bool) where true represents that data reading is complete
unsafe fn read_into_data_buffer(comm: &mut Comm) -> Result<bool, Reply> {
    // First chunk.
    if comm.get_p1() == 0 {
        // Reset data buffer first
        clear_data_buffer();
    }
    let data = comm.get_data()?;
    for x in data.iter() {
        if DATA_BUFFER_LEN == DATA_BUFFER_MAX_SIZE {
            return Err(StatusWords::BadLen.into());
        }
        DATA_BUFFER[DATA_BUFFER_LEN] = *x;
        DATA_BUFFER_LEN += 1;
    }
    return Ok(comm.get_p2() == 1)
}

fn handle_apdu(comm: &mut io::Comm, ins: Ins) -> Result<(), Reply> {
    if comm.rx == 0 {
        return Err(io::StatusWords::NothingReceived.into());
    }

    match ins {
        Ins::GetPubkey => comm.append(&(get_pubkey()?)),
        Ins::Sign => unsafe {
            let data_reading_complete = read_into_data_buffer(comm)?;
            if data_reading_complete {
                let out = sign_ui(&DATA_BUFFER[0..DATA_BUFFER_LEN])?;
                if let Some(o) = out {
                    comm.append(&o)
                }
            }
        }
        Ins::Menu => menu_example(),
        Ins::ShowPrivateKey => comm.append(&bip32_derive_ed25519(&BIP32_PATH)?),
        Ins::Exit => nanos_sdk::exit_app(0),
    }
    Ok(())
}
