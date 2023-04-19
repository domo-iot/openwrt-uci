use std::collections::HashMap;

use crate::Section;
use chumsky::{prelude::*, text::Character};

#[derive(Debug)]
enum Opt {
    Value { key: String, value: String },
    ListItem { key: String, value: String },
}

pub fn parser() -> impl Parser<char, Vec<Section>, Error = Simple<char>> {
    //    let s = choice((quoted('"'), quoted('\''), text::ident()));
    let quoted = |quote| {
        let escape = just('\\').ignore_then(
            just('\\')
                .or(just('/'))
                .or(just(quote))
                .or(just('b').to('\x08'))
                .or(just('f').to('\x0C'))
                .or(just('n').to('\n'))
                .or(just('r').to('\r'))
                .or(just('t').to('\t')),
        );

        just(quote)
            .ignore_then(
                filter(move |c| *c != '\\' && *c != quote)
                    .or(escape)
                    .repeated(),
            )
            .then_ignore(just(quote))
            .collect::<String>()
    };

    let s = choice((text::ident(), quoted('\''), quoted('"'))).then_ignore(
        filter(|c: &char| c.is_inline_whitespace())
            .ignored()
            .repeated(),
    );
    let value = just("option")
        .padded()
        .ignore_then(s)
        .then(s)
        .map(|(key, value)| Opt::Value { key, value });
    let list = just("list")
        .padded()
        .ignore_then(s)
        .then(s)
        .map(|(key, value)| Opt::ListItem { key, value });

    let options = choice((value, list)).repeated();

    just("config")
        .padded()
        .ignore_then(s)
        .then(s.or_not())
        .then(options)
        .map(|((type_, name), opts)| {
            let mut options = HashMap::<String, Vec<String>>::new();
            // TODO: manage duplicated options
            for opt in opts {
                match opt {
                    Opt::Value { key, value } => options.entry(key).or_default().push(value),
                    Opt::ListItem { key, value } => options.entry(key).or_default().push(value),
                }
            }

            Section {
                name,
                type_,
                options,
            }
        })
        .repeated()
        .collect()
}

#[cfg(test)]
mod test {
    use chumsky::Parser;

    use super::parser;
    #[test]
    fn single_section() {
        let input = r#"config route 'mdns'
        option type 'multicast'
        option target '224.0.0.0/4'
        option interface 'iotlan'
"#;
        println!("{:?}", parser().parse(input));
    }

    #[test]
    fn many_sections() {
        let input = r#"config interface 'loopback'
        option device 'lo'
        option proto 'static'
        option ipaddr '127.0.0.1'
        option netmask '255.0.0.0'

config globals 'globals'
        option ula_prefix 'fd52:d7cd:e90b::/48'

config device
        option name 'br-iot'
        option type 'bridge'

config interface 'iotlan'
        option device 'br-iot'
        option proto 'static'
        option ipaddr '10.0.2.1'
        option netmask '255.255.255.0'
        list dns '1.1.1.1'
        list dns '8.8.8.8'

config device
        option name 'br-cabled'
        option type 'bridge'
        list ports 'lan1'
        list ports 'lan2'
        list ports 'lan3'
        list ports 'lan4'
        list ports 'eth1'
        list ports 'wan'
        list ports 'sfp2'
        list ports 'bat0.1'

config interface 'mesh'
        option device 'bat0.2'
        option proto 'static'
        option ipaddr '192.168.20.2'
        option netmask '255.255.255.0'

config interface 'customerdhcp'
        option device 'br-cabled'
        option proto 'dhcp'

config device
        option name 'eth1'
        option macaddr 'fe:1b:16:9c:6e:fc'
"#;

        println!("{:#?}", parser().parse(input));
    }
}
