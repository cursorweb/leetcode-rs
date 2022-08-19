use core::fmt::Debug;

// just for 'fun'
#[derive(Clone, Debug)]
pub struct LList<T>
where
    T: PartialEq,
    T: Clone,
    T: Debug,
{
    i: usize,
    value: T,
    next: Option<Box<LList<T>>>,
}

impl<T> LList<T>
where
    T: PartialEq,
    T: Clone,
    T: Debug,
{
    pub fn new(value: T) -> Self {
        Self {
            i: 0,
            value,
            next: None
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        if v.len() == 0 {
            panic!("Vector can't be empty!");
        }

        let mut iter = v.into_iter();
        let mut out = Self::new(iter.next().unwrap());

        for x in iter {
            out.append(x);
        }

        out
    }

    pub fn append(&mut self, val: T) {
        match self.next {
            Some(ref mut next) => {
                next.append(val);
            }
            None => {
                let node = Self::new(val);
                self.next = Some(Box::new(node))
            }
        }
    }

    pub fn delete(&mut self, val: T) -> Result<(), String> {
        match self.next {
            Some(ref mut next) => {
                if next.value == val {
                    self.next = next.next.clone();
                    Ok(())
                } else {
                    next.delete(val)
                }
            }
            None => Err(String::from(format!("{:?} not found in linked list", val))),
        }
    }

    pub fn set(&mut self, idx: usize, val: T) -> Result<(), String> {
        let mut i = 0;
        let mut v = self;

        if i == idx {
            v.value = val;
            return Ok(());
        }

        while i < idx {
            match v.next {
                Some(ref mut next) => v = next,
                None => {
                    return Err(String::from(format!("{} out of bounds.", idx)));
                }
            }

            i += 1;
        }

        v.value = val;
        Ok(())
    }

    pub fn get(&mut self, idx: usize) -> Result<&T, String> {
        let mut i = 0;
        let mut v = self;

        if i == idx {
            return Ok(&v.value);
        }

        while i < idx {
            i += 1;

            match v.next {
                Some(ref mut next) => {
                    if i == idx {
                        return Ok(&next.value);
                    }

                    v = next;
                }
                None => {
                    return Err(String::from(format!("{} out of bounds.", idx)));
                }
            }
        }

        return Err(String::from(format!("{} out of bounds.", idx)));
    }
}

impl<T> Iterator for LList<T>
where
    T: PartialEq,
    T: Clone,
    T: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        match self.get(self.i - 1) {
            Ok(v) => Some(v.clone()),
            Err(_) => None
        }
    }
}
