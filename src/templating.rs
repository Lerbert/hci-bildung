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

#[cfg(test)]
mod test {
    use super::*;

    use rocket::http::Method;
    use rocket::route::dummy_handler;
    use rocket::Route;

    #[test]
    fn static_uri() {
        let uri = Route::new(Method::Get, "/test/static", dummy_handler).uri;
        let args = HashMap::new();
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/static").unwrap());
    }

    #[test]
    fn instantiate_basic() {
        let uri = Route::new(Method::Get, "/test/<instantiate>", dummy_handler).uri;
        let args = HashMap::from([("instantiate".to_owned(), to_value("instantiated").unwrap())]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/instantiated").unwrap());
    }

    #[test]
    fn instantiate_too_many_args() {
        let uri = Route::new(Method::Get, "/test/<instantiate>", dummy_handler).uri;
        let args = HashMap::from([
            ("instantiate".to_owned(), to_value("instantiated").unwrap()),
            ("other".to_owned(), to_value("foo").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/instantiated").unwrap());
    }

    #[test]
    fn instantiate_double() {
        let uri = Route::new(Method::Get, "/test/<one>/<two>", dummy_handler).uri;
        let args = HashMap::from([
            ("one".to_owned(), to_value("foo").unwrap()),
            ("two".to_owned(), to_value("bar").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/foo/bar").unwrap());
    }

    #[test]
    fn instantiate_path() {
        let uri = Route::new(Method::Get, "/test/<instantiate>/<path..>", dummy_handler).uri;
        let args = HashMap::from([
            ("instantiate".to_owned(), to_value("instantiated").unwrap()),
            ("path".to_owned(), to_value("path/to/res").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(
            result.unwrap(),
            to_value("/test/instantiated/path/to/res").unwrap()
        );
    }

    #[test]
    fn instantiate_query() {
        let uri = Route::new(Method::Get, "/test/static?<key>", dummy_handler).uri;
        let args = HashMap::from([("key".to_owned(), to_value("instantiated").unwrap())]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(
            result.unwrap(),
            to_value("/test/static?key=instantiated").unwrap()
        );
    }

    #[test]
    fn instantiate_query_empty() {
        let uri = Route::new(Method::Get, "/test/static?<key>", dummy_handler).uri;
        let args = HashMap::from([("key".to_owned(), to_value("").unwrap())]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/static?key").unwrap());
    }

    #[test]
    fn instantiate_query_double() {
        let uri = Route::new(Method::Get, "/test/static?<key>&<key2>", dummy_handler).uri;
        let args = HashMap::from([
            ("key".to_owned(), to_value("val").unwrap()),
            ("key2".to_owned(), to_value("val2").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(
            result.unwrap(),
            to_value("/test/static?key=val&key2=val2").unwrap()
        );
    }

    #[test]
    fn instantiate_query_double_empty() {
        let uri = Route::new(Method::Get, "/test/static?<key>&<key2>", dummy_handler).uri;
        let args = HashMap::from([
            ("key".to_owned(), to_value("").unwrap()),
            ("key2".to_owned(), to_value("val2").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(
            result.unwrap(),
            to_value("/test/static?key&key2=val2").unwrap()
        );
    }

    #[test]
    fn instantiate_both() {
        let uri = Route::new(
            Method::Get,
            "/test/<instantiate>/<path..>?<key>",
            dummy_handler,
        )
        .uri;
        let args = HashMap::from([
            ("instantiate".to_owned(), to_value("instantiated").unwrap()),
            ("path".to_owned(), to_value("path/to/res").unwrap()),
            ("key".to_owned(), to_value("val").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(
            result.unwrap(),
            to_value("/test/instantiated/path/to/res?key=val").unwrap()
        );
    }

    #[test]
    fn instantiate_both_empty() {
        let uri = Route::new(Method::Get, "/test/<instantiate>?<key>", dummy_handler).uri;
        let args = HashMap::from([
            ("instantiate".to_owned(), to_value("instantiated").unwrap()),
            ("key".to_owned(), to_value("").unwrap()),
        ]);
        let result = instantiate_uri(&uri, &args);
        assert_eq!(result.unwrap(), to_value("/test/instantiated?key").unwrap());
    }
}
