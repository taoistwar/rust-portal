use extism_pdk::*;
use plugin_common::PluginArgs;

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn kv_read(key: String) -> i64;
    fn kv_write(key: String, v: i64) -> Option<i64>;
}

#[plugin_fn]
pub fn process(input: PluginArgs) -> FnResult<String> {
    let key = "hello".to_string();
    let old = unsafe { kv_read(key.clone())? };

    let new = old + 1;
    unsafe {
        kv_write(key, new)?;
    }
    config::get("key")?;
    let res = format!("input: {:?}\n<br/>\n value:{}, {}", input, old, new);
    Ok(res)
}
