use std::{collections::HashMap, fmt::Debug};

pub trait Setting: Debug {
    fn set_in(self, settings: &mut Settings, setting_name: String) where Self: Sized;
}
impl Setting for bool {
    fn set_in(self, settings: &mut Settings, setting_name: String) where Self: Sized {
        settings.bool_settings.insert(setting_name, self);
    }
}
impl Setting for f32 {
    fn set_in(self, settings: &mut Settings, setting_name: String) where Self: Sized {
        settings.float_setting.insert(setting_name, self);
    }
}
impl Setting for String {
    fn set_in(self, settings: &mut Settings, setting_name: String) where Self: Sized {
        settings.string_setting.insert(setting_name, self);
    }
}

pub struct Settings {
    bool_settings: HashMap<String, bool>,
    float_setting: HashMap<String, f32>,
    string_setting: HashMap<String, String>,
}
impl Settings {
    fn new() -> Self {
        Settings { 
            bool_settings: HashMap::new(), 
            float_setting: HashMap::new(), 
            string_setting: HashMap::new(),
        }
    }

    fn get(&self, setting_name: &str) -> Option<Box<&dyn Setting>> {
        if let Some(bool) = self.bool_settings.get(setting_name) {
            return Some(Box::new(bool));
        }
        if let Some(float) = self.float_setting.get(setting_name) {
            return Some(Box::new(float));
        }
        if let Some(string) = self.string_setting.get(setting_name) {
            return Some(Box::new(string));
        }
        return None;
    }

    fn set<S: Setting>(&mut self, setting_name: &str, value: S) {
        value.set_in(self, setting_name.to_owned())
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use crate::engine::window::WindowHandler;

    use super::*;  

    #[test]
    fn test_settings() {
        let mut settings = Settings::new();

        settings.set("bool_setting", false);
        settings.set("float_setting", 1.0);
        settings.set("string_setting", "string".to_owned());

        
    }
}