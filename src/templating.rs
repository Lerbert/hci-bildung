use std::collections::HashMap;

use rocket::http::uri::Query;
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
        .map(|s| match get_dynamic_segment_name(s) {
            Some(name) => args
                .get(name)
                .and_then(|v| from_value::<String>(v.clone()).ok())
                .ok_or_else(|| format!("No argument found for {}", name)),
            None => Ok(s.into()),
        })
        .collect::<Result<Vec<String>, _>>()?
        .join("/");

    if let Some(query) = uri.origin.query() {
        let query = instantiate_query(query, args)?;
        Ok(to_value(format!("/{}?{}", path, query)).unwrap())
    } else {
        Ok(to_value(format!("/{}", path)).unwrap())
    }
}

fn instantiate_query(
    query: Query<'_>,
    args: &HashMap<String, tera::Value>,
) -> tera::Result<String> {
    let query: Result<Vec<String>, tera::Error> = query
        .segments()
        .map(|(k, v)| {
            let (k, v) = match get_dynamic_segment_name(k) {
                Some(name) => args
                    .get(name)
                    .and_then(|v| from_value::<String>(v.clone()).ok())
                    .map(|v| (name, v))
                    .ok_or_else(|| format!("No argument found for {}", name)),
                None => Ok((k, v.into())),
            }?;
            if v.is_empty() {
                Ok(k.to_string())
            } else {
                Ok(format!("{}={}", k, v))
            }
        })
        .collect();
    Ok(query?.join("&"))
}

fn get_dynamic_segment_name(segment: &str) -> Option<&str> {
    if segment.starts_with('<') && segment.ends_with('>') {
        let mut name = &segment[1..(segment.len() - 1)];

        // Path segment
        if name.ends_with("..") {
            name = &name[..(name.len() - 2)];
        }

        Some(name)
    } else {
        None
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
    fn instantiate_missing_arg() {
        let uri = Route::new(Method::Get, "/test/<instantiate>", dummy_handler).uri;
        let args = HashMap::new();
        let result = instantiate_uri(&uri, &args);
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("No argument found for instantiate"), "Message {} does not contain \"No argument found for instantiate\"", msg);
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
