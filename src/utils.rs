pub fn serde_attr(ser: bool, de: bool) -> &'static str {
    match (ser, de) {
        (true, true) => "#[derive(serde::Serialize, serde::Deserialize)]",
        (true, false) => "#[derive(serde::Serialize)]",
        (false, true) => "#[derive(serde::Deserialize)]",
        (false, false) => "",
    }
}

pub fn sqlx_type_attr() -> &'static str {
    "#[derive(sqlx::Type)]"
}

pub fn sqlx_from_row_attr() -> &'static str {
    "#[derive(sqlx::FromRow)]"
}

pub fn derive_builder_attr() -> &'static str {
    "#[derive(derive_builder::Builder)]"
}

pub fn derive_builder_into_attr() -> &'static str {
    "#[builder(setter(into), default)]"
}

pub fn derive_builder_option_attr() -> &'static str {
    "#[builder(setter(into, strip_option), default)]"
}
