/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[derive(Debug, Clone)]
struct TodoEntry {
    text: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Error came from the network")]
    NetworkError,
    #[error("Error doing a type conversion")]
    TypeError,
    #[error("No todo error!")]
    NoTodoError,
}

type Result<T, E = Error> = std::result::Result<T, E>;

// I am a simple Todolist
#[derive(Debug, Clone)]
struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item<S: Into<String>>(&mut self, item: S) -> Result<()> {
        self.items.push(item.into());
        Ok(())
    }

    fn get_last(&self) -> Result<String> {
        self.items.last().cloned().ok_or_else(|| Error::NoTodoError)
    }

    fn add_entry(&mut self, entry: TodoEntry) {
        self.items.push(entry.text)
    }

    fn get_last_entry(&self) -> TodoEntry {
        TodoEntry {
            text: self.items.last().cloned().unwrap(),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/todolist.uniffi.rs"));
