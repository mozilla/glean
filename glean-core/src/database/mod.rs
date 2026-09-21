// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "sqlite")]
mod conn_ext;

#[cfg(feature = "sqlite")]
pub mod migration;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "sqlite")]
pub use conn_ext::ConnExt;

#[cfg(not(feature = "sqlite"))]
mod rkv;

#[cfg(feature = "sqlite")]
pub use sqlite::Database;

#[cfg(not(feature = "sqlite"))]
pub use rkv::Database;
