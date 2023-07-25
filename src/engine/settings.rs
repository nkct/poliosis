use std::collections::HashMap;

pub trait OtherSetting: std::fmt::Debug{
    fn box_clone(&self) -> Box<dyn OtherSetting>;
}
impl Clone for Box<dyn OtherSetting> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub enum Setting {
    Bool(bool),
    Float(f32),
    String(String),
    Other(Box<dyn OtherSetting>)
}
impl From<bool> for Setting {
    fn from(value: bool) -> Self {
        Setting::Bool(value)
    }
}
impl From<f32> for Setting {
    fn from(value: f32) -> Self {
        Setting::Float(value)
    }
}
impl From<String> for Setting {
    fn from(value: String) -> Self {
        Setting::String(value)
    }
}
impl<O> From<O> for Setting 
where
    O: OtherSetting + 'static
{
    fn from(value: O) -> Self {
        Setting::Other(Box::new(value))
    }
}

impl From<&Setting> for Option<bool> {
    fn from(value: &Setting) -> Self {
        if let Setting::Bool(value) = value {
            return Some(*value);
        }
        return None;
    }
}
impl From<&Setting> for Option<f32> {
    fn from(value: &Setting) -> Self {
        if let Setting::Float(value) = value {
            return Some(*value);
        }
        return None;
    }
}
impl From<&Setting> for Option<String> {
    fn from(value: &Setting) -> Self {
        if let Setting::String(value) = value {
            return Some(value.to_string());
        }
        return None;
    }
}

impl From<&Setting> for Option<Box<dyn OtherSetting>> {
    fn from(value: &Setting) -> Self {
        if let Setting::Other(value) = value {
            return Some(value.clone());
        }
        return None;
    }
}

pub struct Settings {
    settings: HashMap<String, Setting>,
}
impl Settings {
    fn new() -> Self {
        Settings { 
            settings: HashMap::new(), 
        }
    }

    fn get_setting(&self, setting_name: &str) -> Option<&Setting> {
        self.settings.get(setting_name)
    }
    fn get<'a, I>(&'a self, setting_name: &str) -> Option<I> 
    where 
        Option<I>: From<&'a Setting>
    {
        if let Some(setting) = self.get_setting(setting_name) {
            return setting.into()
        } else {
            return None
        }
    }

    fn set<S: Into<Setting>>(&mut self, setting_name: &str, value: S) {
        self.settings.insert(setting_name.to_owned(), value.into());
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*;  

    #[test]
    fn test_settings_getters() {
        let mut settings = Settings::new();

        settings.set("bool_setting", false);
        settings.set("float_setting", 1.0);
        settings.set("string_setting", "string".to_owned());
        
        assert_eq!(
            Some(false), 
            settings.get::<bool>("bool_setting"),
            "ERROR: failed assertion when getting bool from settings"
        );
        assert_eq!(
            Some(1.0), 
            settings.get::<f32>("float_setting"),
            "ERROR: failed assertion when getting f32 from settings"
        );
        assert_eq!(
            Some("string".to_owned()), 
            settings.get::<String>("string_setting"),
            "ERROR: failed assertion when getting String from settings"
        );
    }

    #[test]
    fn test_settings_other() {
        let mut settings = Settings::new();

        #[derive(Clone, Debug)]
        enum TestOtherSetting {
            Variant1,   
            Variant2,   
            Variant3,   
        }
        impl OtherSetting for TestOtherSetting {
            fn box_clone(&self) -> Box<dyn OtherSetting> {
                Box::new((*self).clone())
            }
        }

        settings.set("other_setting", TestOtherSetting::Variant2);
        let other_setting = settings.get::<Box<dyn OtherSetting>>("other_setting");
    }
}