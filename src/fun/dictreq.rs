use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Environ {
    pub enclosing: Option<Rc<RefCell<Environ>>>,
    vals: HashMap<String, i32>,
}

impl Environ {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            vals: HashMap::new(),
        }
    }

    pub fn set_enc(&mut self, env: Environ) {
        self.enclosing = Some(Rc::new(RefCell::new(env)));
    }

    pub fn get(&self, val: &String) -> Option<i32> {
        if self.vals.contains_key(val) {
            Some(self.vals[val])
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(val)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: String, val: i32) {
        self.vals.insert(key, val);
    }

    pub fn assign(&mut self, key: String, val: i32) -> Result<(), ()> {
        if self.vals.contains_key(&key) {
            self.vals.insert(key, val);
            return Ok(());
        } else if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(key, val);
        }

        Err(())
    }
}
