use js_sys::Intl::DateTimeFormat;
use js_sys::Reflect;

pub fn get_local_zone() -> Option<String> {
    let opts = DateTimeFormat::default().resolved_options();
    Reflect::get(&opts, &"timeZone".into())
        .ok()
        .and_then(|x| x.as_string())
}
