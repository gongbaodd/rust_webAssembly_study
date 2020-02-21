#[allow(dead_code)]
pub enum List<T> {
    Value(T, Box<List<T>>),
    Null,
}

use crate::linked_list::List::*;

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> List<T> {
        List::Null
    }

    pub fn prepend(self, item: T) -> List<T> {
        List::Value(item, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Value(_, ref next) => next.len() + 1,

            Null => 0,
        }
    }

    pub fn to_vec(&self) -> Vec<&T> {
        match &*self {
            Value(value, ref next) => {
                let mut arr = vec![value];
                arr.append(&mut next.to_vec());
                arr
            }

            Null => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn create_linked_list() {
        let list = List::<u32>::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    pub fn prepend_node_to_linked_list() {
        let mut list = List::new();
        list = list.prepend(1);

        assert_eq!(list.len(), 1);

        let mut vec = list.to_vec();
        while let Some(i) = vec.pop() {
            assert_eq!(i, &1)
        }
    }
}
