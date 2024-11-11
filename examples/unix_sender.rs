// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod shared;

#[cfg(unix)]
fn main() {
    let sender = fasyslog::sender::unix_well_known().unwrap();
    shared::send_syslog_message(sender);
}

#[cfg(not(unix))]
fn main() {
    println!("This example is only for Unix-like systems.");
}
