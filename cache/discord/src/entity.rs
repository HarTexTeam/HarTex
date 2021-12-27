/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `entity` Module
//!
//! This module contains a base entity trait for different cache entities.

use std::hash::Hash;

/// Trait `Entity`
///
/// A cachable entity.
pub trait Entity: Send + Sync {
    /// # Typealias `Id`
    ///
    /// The id of the entity.
    type Id: Copy + Eq + Hash + Send + Sync;

    /// # Trait Method `id`
    ///
    /// Returns the id of the entity.
    fn id(&self) -> Self::Id;
}
