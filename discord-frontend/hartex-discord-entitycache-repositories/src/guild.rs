/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_discord_entitycache_core::error::RepositoryResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::guild::GuildEntity;
use redis::Client;

pub struct CachedGuildRepository;

impl Repository<GuildEntity> for CachedGuildRepository {
    async fn get(&self, _: <GuildEntity as Entity>::Id) -> RepositoryResult<GuildEntity> {
        let _ = Client::open("redis://127.0.0.1/")?;

        todo!()
    }

    async fn upsert(&self, _: GuildEntity) -> RepositoryResult<()> {
        todo!()
    }
}
