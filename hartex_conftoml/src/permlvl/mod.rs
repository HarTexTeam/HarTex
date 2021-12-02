//! # The `permlvl` Module
//!
//! This module contains configuration models for permission levels.

// [PermissionLevels.Roles]
// RoleId1 = <perm id>
// RoleId2 = <perm id>
//
// [PermissionLevels.Users]
// UserId1 = <perm id>

use std::{
    fmt::{
        Formatter,
        Result as FmtResult
    },
    num::NonZeroU64
};

use dashmap::DashMap;
use serde::{
    de::{
        Error,
        Visitor
    },
    Deserialize,
    Deserializer
};

pub mod map;

/// # Struct `PermissionLevels`
///
/// Represents the permission levels configured.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PermissionLevels {
    pub Roles: map::PermissionLevelMap<RoleId>
}

impl Default for PermissionLevels {
    fn default() -> Self {
        Self {
            Roles: map::PermissionLevelMap {
                map: DashMap::new()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RoleId(NonZeroU64);

impl<'deserialize> Deserialize<'deserialize> for RoleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'deserialize> {
        deserializer.deserialize_str(PermissionLevelsRolesMapRoleIdDeserializerRefstrVisitor)
    }
}

pub struct PermissionLevelsRolesMapRoleIdDeserializerRefstrVisitor;

impl <'visitor> Visitor<'visitor> for PermissionLevelsRolesMapRoleIdDeserializerRefstrVisitor {
    type Value = RoleId;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "a string representing a role id")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: Error {
        let res = v.parse::<u64>();
        if res.is_err() {
            return Err(Error::custom("invalid integer"));
        }

        let nonzero_uint = NonZeroU64::new(res.unwrap());
        if nonzero_uint.is_none() {
            return Err(Error::custom("role id must not be zero"));
        }

        Ok(RoleId(nonzero_uint.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Debug,
        num::NonZeroU64
    };

    use dashmap::DashMap;
    use serde_test::Token;

    use super::{
        map::PermissionLevelMap,
        Deserialize,
        PermissionLevels,
        RoleId
    };

    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Deserialize<'deserialize>
        >() {
        }

        static_assert_impl_all::<PermissionLevels>();
    };

    #[test]
    fn test_dashacc_de() {
        let dashmap = DashMap::new();
        dashmap.insert(RoleId(NonZeroU64(1234567887654321)), 100);
        dashmap.insert(RoleId(NonZeroU64(2345678998765432)), 90);
        dashmap.insert(RoleId(NonZeroU64(3456789009876543)), 80);
        dashmap.insert(RoleId(NonZeroU64(9876543223456789)), 50);
        dashmap.insert(RoleId(NonZeroU64(8765432112345678)), 10);

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
                Token::Str("Roles"),
                Token::Map {
                    len: Some(5)
                },
                Token::TupleStruct {
                    name: "RoleId",
                    len: 1
                },
                Token::Str("1234567887654321"),
                Token::TupleStructEnd,
                Token::I64(100),
                Token::TupleStruct {
                    name: "RoleId",
                    len: 1
                },
                Token::Str("2345678998765432"),
                Token::TupleStructEnd,
                Token::I64(90),
                Token::TupleStruct {
                    name: "RoleId",
                    len: 1
                },
                Token::Str("3456789009876543"),
                Token::TupleStructEnd,
                Token::I64(80),
                Token::TupleStruct {
                    name: "RoleId",
                    len: 1
                },
                Token::Str("9876543223456789"),
                Token::TupleStructEnd,
                Token::I64(50),
                Token::TupleStruct {
                    name: "RoleId",
                    len: 1
                },
                Token::Str("8765432112345678"),
                Token::TupleStructEnd,
                Token::I64(10),
                Token::MapEnd,
                Token::StructEnd
            ]
        );
    }
}
