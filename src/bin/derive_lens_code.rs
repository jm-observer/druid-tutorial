#![allow(dead_code)]
use druid::im::HashMap;
fn main() {}
struct AppData {
    b: HashMap<String, String>,
}
///Derived lenses for [`AppData`].
pub mod app_data_derived_lenses {
    ///Lens for the field `b` on [`AppData`](super::AppData).
    #[allow(non_camel_case_types)]
    pub struct b();
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for b {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "b")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for b {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for b {
        #[inline]
        fn clone(&self) -> b {
            *self
        }
    }
    impl b {
        ///Creates a new lens for the field `b` on [`AppData`](super::AppData). Use [`AppData::b`](super::AppData::b) instead.
        pub const fn new() -> Self {
            Self()
        }
    }
}
impl druid::Lens<AppData, HashMap<String, String>> for app_data_derived_lenses::b {
    fn with<V, F: FnOnce(&HashMap<String, String>) -> V>(&self, data: &AppData, f: F) -> V {
        f(&data.b)
    }
    fn with_mut<V, F: FnOnce(&mut HashMap<String, String>) -> V>(
        &self,
        data: &mut AppData,
        f: F,
    ) -> V {
        f(&mut data.b)
    }
}
#[allow(non_upper_case_globals)]
impl AppData {
    /// Lens for the corresponding field.
    pub const b: app_data_derived_lenses::b = app_data_derived_lenses::b::new();
}
