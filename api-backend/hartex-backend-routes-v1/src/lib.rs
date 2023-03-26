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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_backend_ratelimiter::limitable::Limitable;
use governor::Quota;
use rocket::http::Method;
use hartex_log::log;

pub mod uptime;

pub struct RateLimitGuard;

impl<'r> Limitable<'r> for RateLimitGuard {
    fn evaluate_limit(method: Method, route: &str) -> Quota {
        match (method, route) {
            (Method::Post, hmm) => {
                log::debug!("{hmm}");

                Quota::per_second(Self::non_zero(1))
            },
            _ => Quota::per_second(Self::non_zero(1))
        }
    }
}
