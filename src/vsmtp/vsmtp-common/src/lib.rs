//! vSMTP common definition

/*
 * vSMTP mail transfer agent
 * Copyright (C) 2022 viridIT SAS
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see https://www.gnu.org/licenses/.
 *
*/

#![doc(html_no_source)]
#![deny(missing_docs)]
//
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
//
#![allow(clippy::doc_markdown)]
#![allow(clippy::use_self)]

/// Default smtp port
pub const SMTP_PORT: u16 = 25;

/// Default submission port
pub const SUBMISSION_PORT: u16 = 587;

/// Default submission over TLS port
///
/// Defined in [RFC8314](https://tools.ietf.org/html/rfc8314)
pub const SUBMISSIONS_PORT: u16 = 465;

mod log_channels {
    pub const QUEUE: &str = "server::queue";
}

#[macro_use]
mod r#type {
    #[macro_use]
    pub mod address;
    pub mod code_id;
    pub mod reply;
    pub mod reply_code;
}

mod message {
    pub mod mail;
    pub mod mime_type;
}

pub use message::{mail::*, mime_type::*};
pub use r#type::{address::Address, code_id::CodeID, reply::Reply, reply_code::*};

///
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ReplyOrCodeID {
    ///
    CodeID(CodeID),
    ///
    Reply(Reply),
}

/// envelop of a transaction
pub mod envelop;

/// parsed command of the client
pub mod event;

/// abstraction of the libc
pub mod libc_abstraction;

/// content generated by a smtp transaction
pub mod mail_context;

/// state of a smtp transaction
pub mod state;

/// status of the mail context
pub mod status;

/// rcpt data structure.
pub mod rcpt;

/// queues
pub mod queue;

/// transfer method for delivery / forwarding.
pub mod transfer;

mod mechanism;

/// Data related to ESMTP Authentication
pub mod auth {
    pub use crate::mechanism::Mechanism;
}

mod r#trait {
    pub mod mail_parser;
}

pub use r#trait::mail_parser::{MailParser, MailParserOnFly};

#[cfg(test)]
mod tests {
    mod event;

    mod libc_abstraction;
}

///
pub mod re {
    pub use addr;
    pub use anyhow;
    pub use base64;
    pub use libc;
    pub use log;
    pub use serde_json;
    pub use strum;
    pub use tokio;
    pub use vsmtp_rsasl;
}

#[doc(hidden)]
#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}
