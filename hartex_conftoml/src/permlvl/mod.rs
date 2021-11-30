//! # The `permlvl` Module
//!
//! This module contains configuration models for permission levels.

// [PermissionLevels.Roles]
// RoleId1 = <perm id>
// RoleId2 = <perm id>
//
// [PermissionLevels.Users]
// UserId1 = <perm id>

pub mod map;

use hartex_core::discord::model::id::RoleId;
use serde::Deserialize;

/// # Struct `PermissionLevels`
///
/// Represents the permission levels configured.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PermissionLevels {
    pub Roles: map::PermissionLevelMap<RoleId>
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Debug,
        num::NonZeroU64
    };

    use dashmap::DashMap;
    use hartex_core::discord::model::id::RoleId;
    use serde_test::Token;

    use super::{
        map::PermissionLevelMap,
        Deserialize,
        PermissionLevels
    };

    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Deserialize<'deserialize> + PartialEq
        >() {
        }

        static_assert_impl_all::<PermissionLevels>();
    };

    #[test]
    fn test_dashacc_de() {
        let mut dashmap = DashMap::new();
        dashmap.insert(RoleId::new(1234567887654321).unwrap(), 100);
        dashmap.insert(RoleId::new(2345678998765432).unwrap(), 90);
        dashmap.insert(RoleId::new(3456789009876543).unwrap(), 80);
        dashmap.insert(RoleId::new(9876543223456789).unwrap(), 50);
        dashmap.insert(RoleId::new(8765432112345678).unwrap(), 10);

        serde_test::assert_de_tokens(
            &PermissionLevels {
                Roles: PermissionLevelMap {
                    map: dashmap
                }
            },
            &[
                Token::Struct {
                    name: "PermissionLevels",
                    len: 5
                },
                Token::I64(1234567887654321),
                Token::I64(100),
                Token::I64(2345678998765432),
                Token::I64(90),
                Token::I64(3456789009876543),
                Token::I64(80),
                Token::I64(9876543223456789),
                Token::I64(50),
                Token::I64(8765432112345678),
                Token::I64(10),
                Token::StructEnd
            ]
        );
    }
}
