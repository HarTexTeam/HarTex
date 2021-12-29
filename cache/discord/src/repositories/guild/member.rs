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

//! # The `member` Module
//!
//! This module contains the guild member repository trait.

use hartex_base::discord::model::id::{
    GuildId,
    UserId
};

use crate::{
    backend::Backend,
    entities::{
        guild::{
            member::MemberEntity,
            role::RoleEntity
        },
        user::UserEntity
    },
    relations,
    repository::{
        GetEntityFuture,
        Repository,
        StreamEntitiesFuture
    }
};

/// # Trait `MemberRepository`
///
/// A repository containing member objects.
#[allow(clippy::module_name_repetitions)]
pub trait MemberRepository<B: Backend>: Repository<MemberEntity, B> {
    /// # Trait Method `roles`
    ///
    /// Returns a stream of roles of a member.
    fn roles(
        &self,
        guild_id: GuildId,
        user_id: UserId
    ) -> StreamEntitiesFuture<'_, RoleEntity, B::Error>;

    /// # Trait Method `user`
    ///
    /// Returns the associated user of the member.
    fn user(
        &self,
        guild_id: GuildId,
        user_id: UserId
    ) -> GetEntityFuture<'_, UserEntity, B::Error> {
        let backend = self.backend();

        relations::map_entity(
            backend.members(),
            backend.users(),
            (guild_id, user_id),
            |member| member.user_id()
        )
    }
}
