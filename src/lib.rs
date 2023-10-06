wai_bindgen_rust::export!("plugin.wai");

use std::borrow::Cow;

pub struct Plugin;

impl plugin::Plugin for Plugin {
    // TODO: returning Vecs gives out-of-bound string read errors
    fn handle_privmsg(msgs: Vec<String>) -> Option<String> {
        if msgs.len() < 2 {
            return None;
        }

        let r = &msgs[msgs.len() - 1];
        let target = &msgs[msgs.len() - 2];

        sedregex::find_and_replace(target, &[r])
            .map(Cow::into_owned)
            .map(|mut s| {
                if s.eq(target) {
                    s = "Regex didn't do anything, idiot".to_owned()
                };
                s
            })
            .ok()
    }
}
