use std::{cell::UnsafeCell, mem::MaybeUninit};

#[derive(Debug, Default)]
struct Location {
    x: f32,
    y: f32,
}

struct CustomRefCell<T> {
    data: UnsafeCell<T>,
}

impl<T> CustomRefCell<T> {
    fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }

    unsafe fn safe_get(&mut self) -> &T {
        &*self.data.get()
    }

    unsafe fn safe_get_mut(&mut self) -> &mut T {
        &mut *self.data.get_mut()
    }

    unsafe fn unsafe_get(&self) -> *const T {
        self.data.get()
    }
}

fn main() {
    using_custom_refcell();
    multiple_mutable_pointers();
    into_inner();
    from_location();
    default();
    raw_get();
}

fn raw_get() {
    let location = MaybeUninit::<UnsafeCell<Location>>::uninit();
    let mut location = unsafe {
        let pointer = UnsafeCell::raw_get(location.as_ptr());
        (*pointer).x = 10.0;
        (*pointer).y = 20.0;
        location.assume_init()
    };
    dbg!(location.get_mut());
}

fn using_custom_refcell() {
    let location = Location { x: 10.0, y: 50.0 };
    let mut wrapped_location = CustomRefCell::new(location);
    let unwrapped_mutable_location = unsafe { wrapped_location.safe_get_mut() };
    unwrapped_mutable_location.x += 1.0;
    let unwrapped_immutable_location = unsafe { wrapped_location.safe_get() };
    dbg!(unwrapped_immutable_location);
}

fn multiple_mutable_pointers() {
    let location = Location { x: 10.0, y: 50.0 };
    let wrapped_location = CustomRefCell::new(location);
    unsafe {
        let first = wrapped_location.unsafe_get();
        let second = wrapped_location.unsafe_get();
        let unwrapped_mutable_first = first as *mut Location;
        let unwrapped_mutable_second = second as *mut Location;
        (*unwrapped_mutable_first).x += 10.0;
        (*unwrapped_mutable_second).y += 100.0;
        dbg!(&*second);
    }
}

fn into_inner() {
    let wrapped_location = UnsafeCell::new(Location { x: 10.0, y: 20.0 });
    let unwrapped_location = wrapped_location.into_inner();
    dbg!(unwrapped_location);
    // let a = wrapped_location.get(); // can't do this since we consumed the wrapped location with tne into_inner
}

fn default() {
    let wrapped_default_location = UnsafeCell::<Location>::default();
    dbg!(wrapped_default_location.into_inner());
}

fn from_location() {
    let wrapped_location: UnsafeCell<Location> = Location::default().into();
    dbg!(wrapped_location.into_inner());
}
