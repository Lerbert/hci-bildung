use std::collections::HashMap;

use rocket::route::RouteUri;
use tera::{self, from_value, to_value, Function};

pub fn make_url_for(urls: HashMap<String, RouteUri<'static>>) -> impl Function {
    move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        match args.get("endpoint") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => instantiate_uri(
                    urls.get(&v).ok_or_else::<tera::Error, _>(|| {
                        format!("Endpoint {} not found", v).into()
                    })?,
                    args,
                ),
                Err(e) => Err(format!("Error parsing JSON: {}", e).into()),
            },
            None => Err("No endpoint argument specified".into()),
        }
    }
}

// Cannot use uri! Macro because we have to compute this dynamically
fn instantiate_uri(
    uri: &RouteUri,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let path = uri
        .origin
        .path()
        .segments()
        .map(|s| {
            if s.starts_with('<') && s.ends_with('>') {
                let mut name = &s[1..(s.len() - 1)];

                if name.ends_with("..") {
                    name = &name[..(name.len() - 2)];
                }

                args.get(name)
                    .and_then(|val| from_value::<String>(val.clone()).ok())
                    .unwrap_or_else(|| s.into())
            } else {
                s.into()
            }
        })
        .collect::<Vec<String>>()
        .join("/");

    let query = uri.origin.query().map(|q| {
        {
            q.segments().map(|(k, v)| {
                let (k, v) = if k.starts_with('<') && k.ends_with('>') {
                    let mut name = &k[1..(k.len() - 1)];

                    if name.ends_with("..") {
                        name = &name[..(name.len() - 2)];
                    }

                    let v = args
                        .get(name)
                        .and_then(|val| from_value::<String>(val.clone()).ok())
                        .unwrap_or_else(|| v.into());
                    (name, v)
                } else {
                    (k, v.into())
                };
                if v.is_empty() {
                    k.to_string()
                } else {
                    format!("{}={}", k, v)
                }
            })
        }
        .collect::<Vec<String>>()
        .join("&")
    });
    if let Some(query) = query {
        Ok(to_value(format!("/{}?{}", path, query)).unwrap())
    } else {
        Ok(to_value(format!("/{}", path)).unwrap())
    }
}
