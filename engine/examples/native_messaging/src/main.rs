// Copyright 2020 IOTA Stiftung
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
// the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
// an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

mod client;
mod connection;
mod provider;
mod snap;
mod state;

use crate::{
    client::Client,
    provider::Provider,
    snap::{deserialize_from_snapshot, get_snapshot_path, serialize_to_snapshot},
};

use engine::vault::{Base64Decodable, Id, Key};

use chrome_native_messaging::{read_input, write_output};
#[macro_use]
use serde_json::{json, Error, from_value};
use serde::Deserialize;
use std::io;

// create a line error with the file and the line number
#[macro_export]
macro_rules! line_error {
    () => {
        concat!("Error at ", file!(), ":", line!())
    };
    ($str:expr) => {
        concat!($str, " @", file!(), ":", line!())
    };
}

fn list_command(pld: Payload) {
    if let Some(password) = pld.password {
        let pass = password.as_str();

        let snapshot = get_snapshot_path();
        let client: Client<Provider> = deserialize_from_snapshot(&snapshot, pass);

        let res = client.list_of_ids();
        let body = json!({ "records": res });
        write_output(io::stdout(), &body).expect("failed to write to stdout");

        let snapshot = get_snapshot_path();
        serialize_to_snapshot(&snapshot, pass, client);
    }
}

fn encrypt_command(pld: Payload) {
    let snapshot = get_snapshot_path();

    if let Some(password) = pld.password {
        let pass = password.as_str();
        if let Some(plaintext) = pld.plaintext {
            let plain = plaintext.as_str();
            if snapshot.exists() {
                let snapshot = get_snapshot_path();
                let client: Client<Provider> = deserialize_from_snapshot(&snapshot, pass);

                client.create_record(plain.as_bytes().to_vec());

                let snapshot = get_snapshot_path();
                serialize_to_snapshot(&snapshot, pass, client);
            } else {
                let key = Key::<Provider>::random().expect("Unable to generate a new key");
                let id = Id::random::<Provider>().expect("Unable to generate a new id");
                let client = Client::create_chain(key, id);
                client.create_record(plain.as_bytes().to_vec());

                let snapshot = get_snapshot_path();
                serialize_to_snapshot(&snapshot, pass, client);
            }
            let body = json!({ "success": true });
            write_output(io::stdout(), &body).expect("failed to write to stdout");
        };
    };
}

fn decrypt_command(pld: Payload) {
    if let Some(password) = pld.password {
        let pass = password.as_str();
        if let Some(identifier) = pld.id {
            let id = identifier.as_str();

            let snapshot = get_snapshot_path();
            let client: Client<Provider> = deserialize_from_snapshot(&snapshot, pass);

            let id = Vec::from_base64(id.as_bytes()).expect("couldn't convert the id to from base64");
            let id = Id::load(&id).expect("Couldn't build a new Id");

            let callback = |val| {
                let body = json!({ "plaintext": val });
                write_output(io::stdout(), &body).expect("failed to write to stdout");
            };
            client.return_record_by_id(id, callback);

            let snapshot = get_snapshot_path();
            serialize_to_snapshot(&snapshot, pass, client);
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    cmd: String,
    password: Option<String>,
    plaintext: Option<String>,
    id: Option<String>,
}

fn parse_command(pld: Payload) {
    match pld.cmd.as_str() {
        "list" => list_command(pld),
        "encrypt" => encrypt_command(pld),
        "decrypt" => decrypt_command(pld),
        &_ => (),
    };
    // thread::sleep(Duration::from_millis(500))
}

fn main() {
    loop {
        // wait for input
        match read_input(io::stdin()) {
            Ok(v) => {
                let payload: Result<Payload, Error> = from_value(v);
                match payload {
                    Ok(pld) => parse_command(pld),
                    Err(e) => {
                        let v = json!({ "error": e.to_string() });
                        return write_output(io::stdout(), &v).expect("failed to write to stdout");
                    }
                }
            }
            Err(e) => {
                let v = json!({ "error": e.to_string() });
                return write_output(io::stdout(), &v).expect("failed to write to stdout");
            }
        }
    }
}
