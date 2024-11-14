use extism_pdk::*;
use plugin_common::PluginArgs;

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn kv_read(key: String) -> i64;
    fn kv_write(key: String, v: i64) -> Option<i64>;
}

#[plugin_fn]
pub fn process(input: PluginArgs) -> FnResult<String> {
    let date = input.date;
    let args = input.args;
    let kv_read = |key| unsafe { kv_read(key) };
    let kv_write = |key, v| unsafe { kv_write(key, v) };

    let res = plugin(&date, &args, kv_read, kv_write)?;
    Ok(res)
}
pub fn plugin(
    date: &str,
    args: &str,
    kv_read: fn(String) -> Result<i64, extism_pdk::Error>,
    kv_write: fn(String, i64) -> Result<Option<i64>, extism_pdk::Error>,
) -> Result<String, extism_pdk::Error> {
    let key = "hello".to_string();
    let old: i64 = kv_read(key.clone())?;

    let new: i64 = old + 1;
    kv_write(key, new)?;
    println!(
        "input:date={}, args={:?}\n\tvalue: old={}, new{}",
        date, args, old, new
    );
    Ok(new.to_string())
}

#[cfg(test)]
mod test {
    use crate::plugin;

    #[test]
    fn test_process() -> Result<(), extism_pdk::Error> {
        let date = "2024-11-14 14:28:00.000";
        let args = "";
        let x = plugin(date, args, plugin_mock::kv_read, plugin_mock::kv_write)?;
        assert_eq!("1", &x);
        Ok(())
    }
}
