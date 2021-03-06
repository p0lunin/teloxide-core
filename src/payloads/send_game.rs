// This file is auto generated by `cg` <https://github.com/teloxide/cg> (be02d84).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::Message;

impl_payload! {
    /// Use this method to send a game. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendGame (SendGameSetters) => Message {
        required {
            /// Unique identifier for the target chat
            pub chat_id: u32,
            /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
            pub game_short_name: String [into],
        }
    }
}
