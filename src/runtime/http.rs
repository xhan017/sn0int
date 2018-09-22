use errors::*;

use engine::ctx::State;
use hlua::{self, AnyLuaValue, AnyHashableLuaValue};
use std::sync::Arc;
use std::collections::HashMap;
use web::{RequestOptions, HttpRequest};


pub fn http_mksession(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("http_mksession", hlua::function0(move || -> String {
        state.http_mksession()
    }))
}

pub fn http_request(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("http_request", hlua::function4(move |session: String, method: String, url: String, options: AnyLuaValue| -> Result<AnyLuaValue> {
        RequestOptions::try_from(options)
            .context("invalid request options")
            .map_err(|err| state.set_error(Error::from(err)))
            .map(|options| {
                state.http_request(&session, method, url, options).into()
            })
    }))
}

pub fn http_send(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("http_send", hlua::function1(move |request: AnyLuaValue| -> Result<HashMap<AnyHashableLuaValue, AnyLuaValue>> {
        let req = match HttpRequest::try_from(request)
                                .context("invalid http request object") {
            Ok(req) => req,
            Err(err) => return Err(state.set_error(Error::from(err))),
        };

        req.send(state.as_ref())
            .map_err(|err| state.set_error(err))
            .map(|resp| resp.into())
    }))
}


#[cfg(test)]
mod tests {
    use engine::ctx::Script;
    use std::time::{Instant, Duration};

    #[test]
    #[ignore]
    fn verify_request() {
        let script = Script::load_unchecked(r#"
        function run()
            session = http_mksession()
            req = http_request(session, "GET", "https://httpbin.org/anything", {})
            x = http_send(req)
            if last_err() then return end
            print(x)

            if x['status'] ~= 200 then
                return 'wrong status code'
            end
        end
        "#).expect("failed to load script");
        script.test().expect("Script failed");
    }

    #[test]
    #[ignore]
    fn verify_timeout() {
        let start = Instant::now();
        let script = Script::load_unchecked(r#"
        function run()
            session = http_mksession()
            req = http_request(session, "GET", "http://1.2.3.4", {
                timeout=250
            })
            x = http_send(req)
            if last_err() then return end
        end
        "#).expect("failed to load script");
        script.test().err().expect("Script should have failed");
        let end = Instant::now();

        assert!(end.duration_since(start) < Duration::from_secs(1));
    }

    #[test]
    #[ignore]
    fn verify_post() {
        let script = Script::load_unchecked(r#"
        function run()
            session = http_mksession()

            headers = {}
            headers['Content-Type'] = "application/json"
            req = http_request(session, "POST", "https://httpbin.org/anything", {
                headers=headers,
                query={
                    foo="bar"
                },
                json={
                    hello="world"
                }
            })
            x = http_send(req)
            if last_err() then return end
            print(x)

            o = json_decode(x['text'])
            if last_err() then return end

            if o['args']['foo'] ~= 'bar' or o['json']['hello'] ~= 'world' then
                return "reply didn't contain all params"
            end
        end
        "#).expect("failed to load script");
        script.test().expect("Script failed");
    }

    #[test]
    #[ignore]
    fn verify_cookies() {
        let script = Script::load_unchecked(r#"
        function run()
            session = http_mksession()

            req = http_request(session, "GET", "https://httpbin.org/cookies/set", {
                query={
                    foo="bar",
                    fizz="buzz"
                }
            })
            x = http_send(req)

            req = http_request(session, "GET", "https://httpbin.org/cookies", {})
            x = http_send(req)
            if last_err() then return end
            print(x)

            o = json_decode(x['text'])
            if last_err() then return end

            if o['cookies']['fizz'] ~= 'buzz' or o['cookies']['foo'] ~= 'bar' then
                return "reply didn't contain all cookies"
            end
        end
        "#).expect("failed to load script");
        script.test().expect("Script failed");
    }
}
