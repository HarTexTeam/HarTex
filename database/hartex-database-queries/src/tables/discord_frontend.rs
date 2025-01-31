// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

use wtx::database::Record as _;
use wtx::database::client::postgres::Record;
pub struct NightlyCachedRoles {
    id: String,
    guild_id: String,
    mentionable: bool,
    color: i64,
    hoist: bool,
    icon: Option<String>,
    flags: i32,
    managed: bool,
    position: i32,
}
impl NightlyCachedRoles {
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    #[must_use]
    pub fn guild_id(&self) -> &str {
        self.guild_id.as_str()
    }
    #[must_use]
    pub fn mentionable(&self) -> bool {
        self.mentionable
    }
    #[must_use]
    pub fn color(&self) -> i64 {
        self.color
    }
    #[must_use]
    pub fn hoist(&self) -> bool {
        self.hoist
    }
    #[must_use]
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
    #[must_use]
    pub fn flags(&self) -> i32 {
        self.flags
    }
    #[must_use]
    pub fn managed(&self) -> bool {
        self.managed
    }
    #[must_use]
    pub fn position(&self) -> i32 {
        self.position
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyCachedRoles
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            id: record.decode("id")?,
            guild_id: record.decode("guild_id")?,
            mentionable: record.decode("mentionable")?,
            color: record.decode("color")?,
            hoist: record.decode("hoist")?,
            icon: record.decode_opt("icon")?,
            flags: record.decode("flags")?,
            managed: record.decode("managed")?,
            position: record.decode("position")?,
        })
    }
}
pub struct NightlyCachedMembers {
    flags: i64,
    user_id: String,
    joined_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    nick: Option<String>,
    roles: Vec<String>,
    guild_id: String,
}
impl NightlyCachedMembers {
    #[must_use]
    pub fn flags(&self) -> i64 {
        self.flags
    }
    #[must_use]
    pub fn user_id(&self) -> &str {
        self.user_id.as_str()
    }
    #[must_use]
    pub fn joined_at(&self) -> Option<chrono::DateTime<chrono::offset::Utc>> {
        self.joined_at
    }
    #[must_use]
    pub fn nick(&self) -> Option<&str> {
        self.nick.as_deref()
    }
    #[must_use]
    pub fn roles(&self) -> &[String] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn guild_id(&self) -> &str {
        self.guild_id.as_str()
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyCachedMembers
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            flags: record.decode("flags")?,
            user_id: record.decode("user_id")?,
            joined_at: record.decode_opt("joined_at")?,
            nick: record.decode_opt("nick")?,
            roles: record.decode("roles")?,
            guild_id: record.decode("guild_id")?,
        })
    }
}
pub struct NightlyGuildConfigurations {
    guild_id: String,
    enabled_plugins: Vec<String>,
}
impl NightlyGuildConfigurations {
    #[must_use]
    pub fn guild_id(&self) -> &str {
        self.guild_id.as_str()
    }
    #[must_use]
    pub fn enabled_plugins(&self) -> &[String] {
        self.enabled_plugins.as_slice()
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyGuildConfigurations
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            guild_id: record.decode("guild_id")?,
            enabled_plugins: record.decode("enabled_plugins")?,
        })
    }
}
pub struct NightlyCachedUsers {
    discriminator: String,
    name: String,
    bot: bool,
    id: String,
    global_name: Option<String>,
    avatar: Option<String>,
}
impl NightlyCachedUsers {
    #[must_use]
    pub fn discriminator(&self) -> &str {
        self.discriminator.as_str()
    }
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    #[must_use]
    pub fn bot(&self) -> bool {
        self.bot
    }
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    #[must_use]
    pub fn global_name(&self) -> Option<&str> {
        self.global_name.as_deref()
    }
    #[must_use]
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyCachedUsers
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            discriminator: record.decode("discriminator")?,
            name: record.decode("name")?,
            bot: record.decode("bot")?,
            id: record.decode("id")?,
            global_name: record.decode_opt("global_name")?,
            avatar: record.decode_opt("avatar")?,
        })
    }
}
pub struct NightlyCachedGuilds {
    default_message_notifications: i16,
    features: Vec<String>,
    id: String,
    name: String,
    owner_id: String,
    explicit_content_filter: i16,
    icon: Option<String>,
    mfa_level: i16,
    verification_level: i16,
    premium_tier: i16,
    large: bool,
    premium_subscription_count: Option<i64>,
}
impl NightlyCachedGuilds {
    #[must_use]
    pub fn default_message_notifications(&self) -> i16 {
        self.default_message_notifications
    }
    #[must_use]
    pub fn features(&self) -> &[String] {
        self.features.as_slice()
    }
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    #[must_use]
    pub fn owner_id(&self) -> &str {
        self.owner_id.as_str()
    }
    #[must_use]
    pub fn explicit_content_filter(&self) -> i16 {
        self.explicit_content_filter
    }
    #[must_use]
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }
    #[must_use]
    pub fn mfa_level(&self) -> i16 {
        self.mfa_level
    }
    #[must_use]
    pub fn verification_level(&self) -> i16 {
        self.verification_level
    }
    #[must_use]
    pub fn premium_tier(&self) -> i16 {
        self.premium_tier
    }
    #[must_use]
    pub fn large(&self) -> bool {
        self.large
    }
    #[must_use]
    pub fn premium_subscription_count(&self) -> Option<i64> {
        self.premium_subscription_count
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyCachedGuilds
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            default_message_notifications: record.decode("default_message_notifications")?,
            features: record.decode("features")?,
            id: record.decode("id")?,
            name: record.decode("name")?,
            owner_id: record.decode("owner_id")?,
            explicit_content_filter: record.decode("explicit_content_filter")?,
            icon: record.decode_opt("icon")?,
            mfa_level: record.decode("mfa_level")?,
            verification_level: record.decode("verification_level")?,
            premium_tier: record.decode("premium_tier")?,
            large: record.decode("large")?,
            premium_subscription_count: record.decode_opt("premium_subscription_count")?,
        })
    }
}
pub struct NightlyCachedEmojis {
    name: String,
    animated: bool,
    id: String,
    guild_id: String,
    managed: bool,
}
impl NightlyCachedEmojis {
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    #[must_use]
    pub fn animated(&self) -> bool {
        self.animated
    }
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    #[must_use]
    pub fn guild_id(&self) -> &str {
        self.guild_id.as_str()
    }
    #[must_use]
    pub fn managed(&self) -> bool {
        self.managed
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyCachedEmojis
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            name: record.decode("name")?,
            animated: record.decode("animated")?,
            id: record.decode("id")?,
            guild_id: record.decode("guild_id")?,
            managed: record.decode("managed")?,
        })
    }
}
