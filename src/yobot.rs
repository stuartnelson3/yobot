//!
//! Yobot is the main struct of the bot. Add a bunch of listeners and you call `connect` to connect
//! the real time API and start listening for messages.
//!
//! # Example
//!
//! ```no_run
//! # extern crate yobot;
//! # fn main() {
//! use yobot::Yobot;
//!
//! let yobot = Yobot::new();
//!     .add_listener(listener::WhateverListener)
//!     .connect();
//! # }
//! ```
extern crate slack;

use slackhandler::SlackHandler;
use listener::{MessageListener, Message};

pub struct Yobot {
    listeners: Vec<Box<MessageListener>>,
}

impl Yobot {
    /// Create a new yobot instance.
    pub fn new() -> Yobot {
        Yobot { listeners: Vec::new() }
    }

    fn handle_message(&self, message: &Message, cli: &slack::RtmClient) {
        if message.text == "help" && message.is_addressed {
            let helps = self.listeners
                .iter()
                .map(|x| x.help())
                .collect::<Vec<_>>()
                .join("\n");

            let _ = cli.sender().send_message(&message.channel, &helps);
            return;
        }

        for listener in self.listeners.iter() {
            if listener.can_handle(message) {
                listener.handle(message, cli);
                break;
            }
        }
    }

    /// Connect slack Real Time API socket.
    ///
    /// Once the socket is connected, messages will be directed to the listener.
    pub fn connect(&self, token: String, bot_name: String) {
        let mut handler = SlackHandler::new(|message, cli| self.handle_message(message, cli));
        handler.login_and_run(token, bot_name);
    }

    /// Add a MessageListener to the bot
    ///
    /// The more listeners you have the more useful your bot becomes (for potentially loose
    /// definitions of useful :P).
    pub fn add_listener<T>(&mut self, listener: T) -> &mut Yobot
    where
        T: MessageListener + 'static,
    {
        self.listeners.push(Box::new(listener));
        self
    }
}
