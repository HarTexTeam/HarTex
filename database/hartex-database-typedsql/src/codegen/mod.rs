/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;

use crate::schema::SchemaInfo;

pub(crate) mod tables;
pub(crate) mod types;

pub(crate) const DO_NOT_MODIFY_HEADER: &str =
    "// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

";

pub(crate) fn generate_table_structs_from_schemas<P>(
    schemas: HashMap<String, SchemaInfo>,
    root_path: P,
) -> crate::error::Result<()>
where
    P: AsRef<Path>,
{
    let tables_dir = root_path.as_ref().join("tables");
    fs::create_dir_all(&tables_dir)?;

    let _ = schemas
        .into_iter()
        .map(tables::generate_table_structs_from_schema)
        .process_results(|iter| {
            iter.map(|file| {
                let path = tables_dir.clone().join(file.filename);

                fs::write(
                    &path,
                    DO_NOT_MODIFY_HEADER.to_owned() + file.content.as_str(),
                )?;

                Ok::<(), crate::error::Error>(())
            })
            .process_results(|iter| iter.collect_vec())
        })??;

    Ok(())
}
