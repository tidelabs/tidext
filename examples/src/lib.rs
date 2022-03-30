// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tidechain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

use simple_logger::SimpleLogger;
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
  SimpleLogger::new()
    .with_module_level("subxt", log::LevelFilter::Off)
    .with_module_level("jsonrpsee_ws_client", log::LevelFilter::Off)
    .with_module_level("soketto", log::LevelFilter::Off)
    .with_module_level("tracing", log::LevelFilter::Off)
    .with_module_level("mio", log::LevelFilter::Off)
    .init()?;
  Ok(())
}
