extern crate generational_arena;
use std::{borrow::BorrowMut, rc::Rc};

use generational_arena::{Arena, Index};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::ops::Deref;

struct Shared<T> {
    v: Rc<RefCell<T>>,
}
impl<T> Shared<T> {
    fn new(t: T) -> Shared<T> {
        Shared {
            v: Rc::new(RefCell::new(t)),
        }
    }
    fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        (*self.v).borrow_mut()
    }

    fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
}

impl<T: fmt::Display> fmt::Display for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T: fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<'a, T> Deref for Shared<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.as_ptr().as_ref().unwrap() }
    }
}
// struct parent {
//     children: Vec<child>,
//     arena_ptr: Rc<Arena<String>>,
// }

struct child {
    my_ptr: Index,
    arena_ptr: Rc<RefCell<Arena<String>>>,
}

fn main() {
    let arena: Arena<String> = Arena::new();

    let ptr = Shared::new(arena);

    let rza = ptr
        .borrow_mut()
        .insert("Robert Fitzgerald Diggs".to_string());
    // Insert some elements into the ptr.borrow_mut().
    let gza = ptr.borrow_mut().insert("Gary Grice".to_string());
    let bill = ptr.borrow_mut().insert("Bill Gates".to_string());

    // Inserted elements can be accessed infallibly via indexing (and missing
    // entries will panic).
    assert_eq!(ptr.borrow_mut()[rza], "Robert Fitzgerald Diggs");

    // Alternatively, the `ge;t` and `get_mut` methods provide fallible lookup.
    if let Some(genius) = ptr.borrow_mut().get(gza) {
        println!("The gza gza genius: {}", genius);
    }
    if let Some(val) = ptr.borrow_mut().get_mut(bill) {
        *val = "Bill Gates doesn't belong in this set...".to_string();
    }

    // We can remove elements.
    ptr.borrow_mut().remove(bill);

    // Insert a new one.
    let murray = ptr.borrow_mut().insert("Bill Murray".to_string());

    // The arena does not contain `bill` anymore, but it does contain `murray`, even
    // though they are almost certainly at the same index within the arena in
    // practice. Ambiguities are resolved with an associated generation tag.
    assert!(!ptr.borrow_mut().contains(bill));
    assert!(ptr.borrow_mut().contains(murray));
}
