// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

pub struct StartTimestampSelectByComponent {
    component: String,
}
impl StartTimestampSelectByComponent {
    #[must_use = "Queries must be executed after construction"]
    pub fn bind(component: String) -> Self {
        Self { component }
    }
}
