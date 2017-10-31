/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/// Lightroom local id Used as a catalog globel identifier. Values
/// seems to be unique accross types.
pub type LrId = i64;

/// Basic object from the catalog.
/// `Collection` as an exception
pub trait LrObject {
    /// The local id
    fn id(&self) -> LrId;
    /// The global id. A valid UUID. Doesn't seem to be used anywhere
    /// though.
    fn uuid(&self) -> &str;
}
