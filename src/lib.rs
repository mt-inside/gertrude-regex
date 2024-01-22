wai_bindgen_rust::export!("plugin.wai");

use std::borrow::Cow;

pub struct Plugin;

// TODO: work back through the messages and apply to the first one that matches. Update "didn't
// apply" to include how many you tried
impl plugin::Plugin for Plugin {
    fn handle_privmsg(msgs: Vec<String>) -> Option<String> {
        if msgs.len() < 2 {
            return None;
        }

        let r = &msgs[msgs.len() - 1];
        let target = &msgs[msgs.len() - 2];

        sedregex::find_and_replace(target, &[r])
            .map(Cow::into_owned)
            .map(|s| {
                if s.eq(target) {
                    return "Regex didn't do anything, idiot".to_owned();
                };
                s
            })
            .ok()
    }
}
