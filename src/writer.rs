use serde::Serialize;
use std::fmt::Write;

pub fn to_string<T: Serialize>(t: &T) -> anyhow::Result<String> {
    let mut out = String::new();
    let doc = toml_edit::ser::to_document(t)?;
    println!("{doc:?}");

    for (key, val) in doc.iter() {
        let values: Vec<&str> = key.split('.').collect();
        let len = values.len();

        for v in val.as_array().unwrap().iter() {
            if len == 1 {
                writeln!(&mut out, "\nconfig {} ", values[0])?;
            } else if len == 2 {
                writeln!(&mut out, "\nconfig {} '{}'", values[0], values[1])?;
            }
            for (k, v) in v.as_inline_table().unwrap().iter() {
                if let Some(arr) = v.as_array() {
                    for v in arr.iter() {
                        writeln!(&mut out, "list {k} {v}")?;
                    }
                } else {
                    writeln!(&mut out, "option {k} {v}")?;
                }
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::to_string;
    #[test]
    fn from_toml() {
        let t = r#"
[[device]]
name = 'br-cabled'
type = 'bridge'
ports = ['lan1','lan2','lan3','lan4']

[[device]]
name = 'br-iot'
type = 'bridge'

[["interface.iot-lan"]]
device = 'br-iot'
proto = 'static'
ipaddr = '10.0.2.1'
netmask = '255.255.255.0'
dns = ['1.1.1.1', '8.8.8.8']
"#;

        let doc: toml::Table = toml_edit::de::from_str(t).unwrap();

        println!("{doc:#?}");

        let s = to_string(&doc).unwrap();

        println!("{s}");
    }
}
