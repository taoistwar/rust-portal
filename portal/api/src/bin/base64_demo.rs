use base64::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v = "phoneUid".as_bytes();
    println!("input: {v:?}");
    let x = base64::engine::general_purpose::STANDARD.decode(v);
    match x {
        Ok(v) => {
            let x = String::from_utf8(v);
            println!("from utf-8 error: {x:?}");
        }
        Err(e) => {
            println!("base64 decode error: {e:?}");
        }
    }

    Ok(())
}
