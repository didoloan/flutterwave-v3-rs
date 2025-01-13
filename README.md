# Flutterwave-V3-rs

[![Crates.io](https://img.shields.io/crates/v/flutterwave-rs.svg)](https://crates.io/crates/flutterwave-v3)
[![docs.rs](https://docs.rs/flutterwave-rs/badge.svg)](https://docs.rs/flutterwave-v3)
[![Build Status](https://github.com/didoloan/flutterwave-v3-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/didoloan/flutterwave-v3-rs/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for interacting with the Flutterwave v3 API. This crate provides a convenient way to access various Flutterwave functionalities, simplifying the integration of payment and other financial services into your Rust applications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flutterwave-rs = "0.1.1-alpha.1" // Replace with the actual version
```

## Currently Implemented Functionality

This crate currently supports the following Flutterwave v3 API functionalities:

- **Virtual Account Numbers:**

  - `create_virtual_acct_no`: Create a virtual account number.
  - `create_bulk_virtual_acct_no`: Create virtual account numbers in bulk.
  - `get_bulk_virtual_acct_details`: Retrieve details of bulk virtual accounts.

- **Charges:**

  - `initiate_card_charge`: Initiate a card charge.
  - `initiate_bank_transfer`: Initiate a bank transfer.
  - `initiate_ach_payment`: Initiate an ACH payment.

- **Transactions:**

  - `verify_trans_by_id`: Verify a transaction by its ID.
  - `verify_trans_by_txref`: Verify a transaction by its transaction reference.

- **Validation:**

  - `validate_charge`: Validate a charge.

- **ACH Payments:**

  - `initiate_ach_payment`: Initiate an ACH payment.
  - `verify_ach_payment`: Verify an ACH payment.
  - `cancel_ach_payment`: Cancel an ACH payment.

- **PreAuthorization:**
  - `capture_preauth_charge`: Capture a pre-authorized charge.
  - `refund_preauth_charge`: Refund a pre-authorized.
  - `void_preauth_charge`: Void a pre-authorized charge.

## Usage

```

use flutterwave_rs::v3_client::FWV3Client;
// Import other necessary structs from flutterwave-models crate

#[tokio::main]
async fn main() {
    let client = FWV3Client::new(
        "YOUR_SECRET_KEY",
        "YOUR_PUBLIC_KEY",
        "YOUR_ENCRYPTION_KEY",
    );

    // Example: Create a virtual account number
    let response = client.create_virtual_acct_no(VirtualAcctCreationReq {
        // ... your request data
    }).await;

    match response {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{}", err);
        }
    }


     // Example: Initiate ACH Payment
     let response = client
         .initiate_ach_payment(AchReq {
             // ... your request data
         })
         .await;

     match response {
         Ok(res) => {
             println!("{:?}", res);
         },
         Err(e) => {
             println!("{}", e);
         }
     }

    // ... other operations
}
```
