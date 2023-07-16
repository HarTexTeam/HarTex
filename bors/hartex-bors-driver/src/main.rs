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

//! # Bors for HarTex
//!
//! A reimplementation of Bors in Rust for usage in the HarTex repository.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(let_chains)]

use std::env;
use std::fs::File;
use std::io::Read;

use dotenvy::Error;
use hartex_bors_database::client::SeaORMDatabaseClient;
use hartex_bors_github::GithubBorsState;
use hartex_errors::dotenv;
use hartex_log::log;
use miette::IntoDiagnostic;
use state::InitCell;
use tokio::runtime::Builder;

mod event;
mod process;
mod queue;
mod workflows;

static STATE: InitCell<GithubBorsState> = InitCell::new();

/// Entry point.
pub fn main() -> miette::Result<()> {
    hartex_log::initialize();

    actual_main()?;

    Ok(())
}

/// Actual entry point, building the runtime and other stuff.
fn actual_main() -> miette::Result<()> {
    log::trace!("loading environment variables");
    if let Err(error) = dotenvy::dotenv() {
        match error {
            Error::LineParse(content, index) => Err(dotenv::LineParseError {
                src: content,
                err_span: (index - 1, 1).into(),
            })?,
            _ => todo!(),
        }
    }

    log::trace!("constructing runtime");
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .into_diagnostic()?;

    log::trace!("loading github application state");
    let app_id = env::var("APP_ID")
        .into_diagnostic()?
        .parse::<u64>()
        .into_diagnostic()?;

    log::trace!("initializing sqlite database");
    let database = runtime.block_on(hartex_bors_database::initialize_database(true))?;

    let mut private_key_file = File::open("../bors-private-key.pem").into_diagnostic()?;
    let mut private_key = String::new();
    private_key_file
        .read_to_string(&mut private_key)
        .into_diagnostic()?;

    let client = SeaORMDatabaseClient::new(database);
    let (state, rx) = runtime.block_on(GithubBorsState::load(
        app_id.into(),
        client.clone(),
        private_key.into_bytes().into(),
    ))?;
    STATE.set(state);

    let future = process::bors_process(STATE.get());

    runtime.spawn(future);
    runtime.block_on(queue::queue_processor(STATE.get(), rx, Box::new(client)))?;

    Ok(())
}
