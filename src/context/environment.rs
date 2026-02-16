use std::collections::HashMap;
use std::sync::{Mutex, Arc};

pub trait Singleton {
    fn get_instance() -> Arc<Self>;

    fn get_var(key: &str) -> Option<String>;

    #[allow(dead_code)]
    fn set_var(key: String, value: String);

    #[allow(dead_code)]
    fn remove_var(key: &str);
}

#[derive(Debug)]
pub struct Environment {
    pub vars: Mutex<HashMap<String, String>>,
}

impl Singleton for Environment {
    fn get_instance() -> Arc<Self> {
        static INSTANCE: Mutex<Option<Arc<Environment>>> = Mutex::new(None);
        
        let mut env = INSTANCE.lock().unwrap();
        if let Some(ref env) = *env {
            env.clone()
        } else {
            let new_env = Environment {
                vars: Mutex::new(
                    std::env::vars().collect()
                ),
            };
            let arc = Arc::new(new_env);
            *env = Some(arc.clone());
            arc
        }
    }

    fn get_var(key: &str) -> Option<String> {
        let env = Environment::get_instance();
        env.vars.lock().unwrap().get(key).cloned()
    }

    fn set_var(key: String, value: String) {
        let env = Environment::get_instance();
        env.vars.lock().unwrap().insert(key, value);
    }

    fn remove_var(key: &str) {
        let env = Environment::get_instance();
        env.vars.lock().unwrap().remove(key);
    }
}