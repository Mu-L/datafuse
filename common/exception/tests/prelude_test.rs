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

use common_exception::prelude::*;

#[test]
fn test_prelude() -> anyhow::Result<()> {
    let x: std::result::Result<(), std::fmt::Error> = Err(std::fmt::Error {});
    let y: common_exception::Result<()> = x.map_err_to_code(ErrorCode::UnknownException, || 123);

    assert_eq!(
        "Code: 1000, displayText = 123, cause: an error occurred when formatting an argument.",
        format!("{}", y.unwrap_err())
    );
    Ok(())
}
