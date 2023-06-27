use std::str::FromStr;

pub enum WasmFramework {
    Yew,
    Dioxus,
    Leptos,
}

impl FromStr for WasmFramework {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yew" => Ok(WasmFramework::Yew),
            "dioxus" => Ok(WasmFramework::Dioxus),
            "leptos" => Ok(WasmFramework::Leptos),
            _ => Err(()),
        }
    }
}
