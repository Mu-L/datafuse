// Copyright 2020 Datafuse Labs.
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

// The api module only used for internal communication, such as GRPC between cluster and the managed HTTP REST API.

mod flight_server;
mod http;
mod http_service;
pub mod rpc;

#[cfg(test)]
mod http_service_test;

pub use flight_server::FlightServer;
pub use http_service::HttpService;
