# approveapi-rs
[![approveapi-rs on crates.io](https://img.shields.io/crates/v/approveapi.svg)](https://crates.io/crates/approveapi)
[![approveapi-rs on docs.rs](https://docs.rs/approveapi/badge.svg)](https://docs.rs/approveapi)

Rust API bindings for the [ApproveAPI HTTP API](https://approveapi.com).

*ApproveAPI is a simple API to request a user's real-time approval on anything via email, sms + mobile push.*

## Features
- [x] Send Prompt
  - [x] web redirect actions (i.e. magic links)
  - [x] custom approve/reject buttons
  - [x] metadata
  - [x] long polling
- [x] Retrieve Prompt
- [x] Check Prompt status 
- [x] Futures support
- [x] Webhook callbacks

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
approveapi = "1.0.2"
```

Import the crate for Rust < 1.31:
```rust
extern crate approveapi; // Optional for Rust >= 1.31 
```

And add it to your modules:

```rust
use approveapi; //
```

## Getting Started

To get started, we create a client:

```rust
let client = approveapi::create_client('sk_yourapikeyhere');
```

Now we can make API calls. For example, let's send an approval prompt to confirm a financial transaction.

```rust
use approveapi::{CreatePromptRequest};
let mut prompt_request = CreatePromptRequest::new(
    r#"A transfer of $1337.45 from acct 0294 to acct 1045
          has been initiated. Do you want to authorize this
          transfer?"#.to_string(),
    "alice@approveapi.com".to_string(),
);
prompt_request.approve_text = Some("Authorize".to_string());
prompt_request.reject_text = Some("Reject".to_string());
prompt_request.long_poll = Some(true);
match client.create_prompt(prompt_request).sync() {
    Ok(prompt) => {
        if let Some(answer) = prompt.answer {
            if answer.result {
                println!("Request approved");
            } else {
                println!("Request rejected");
            }
        } else {
            println!("No response from user");
        }
    }
    Err(e) => println!("ApproveAPI->create_prompt error: {:?}", e),
};
```
