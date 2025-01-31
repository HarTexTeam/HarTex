// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

pub struct CachedRoleUpsert {
    color: i64,
    icon: String,
    id: String,
    guild_id: String,
    flags: i32,
    hoist: bool,
    managed: bool,
    mentionable: bool,
    position: i32,
}
impl CachedRoleUpsert {
    #[must_use = "Queries must be executed after construction"]
    pub fn bind(
        color: i64,
        icon: String,
        id: String,
        guild_id: String,
        flags: i32,
        hoist: bool,
        managed: bool,
        mentionable: bool,
        position: i32,
    ) -> Self {
        Self {
            color,
            icon,
            id,
            guild_id,
            flags,
            hoist,
            managed,
            mentionable,
            position,
        }
    }
}
