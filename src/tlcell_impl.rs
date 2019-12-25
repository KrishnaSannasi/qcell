use core::marker::PhantomData;
use core::any::{Any, TypeId};

use std::cell::RefCell;
use std::collections::HashSet;

use crate::{ValueCell, ValueCellOwner};

pub type TLCell<Mark, T> = ValueCell<ThreadLocalSingletonOwner<Mark>, T>;

pub struct ThreadLocalSingletonOwner<Mark>(PhantomData<*mut Mark>);

pub struct ThreadLocalSingletonProxy<Mark>(PhantomData<RefCell<Mark>>);

thread_local! {
    static OWNERS: RefCell<HashSet<TypeId>> = RefCell::default();
}

impl<Mark: Any> Default for ThreadLocalSingletonOwner<Mark> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Mark: Any> ThreadLocalSingletonOwner<Mark> {
    pub fn new() -> Self {
        assert!(
            OWNERS.with(|owner| owner.borrow_mut().insert(TypeId::of::<Mark>())),
            "Invalid `SingletonOwner` was attempted to be created"
        );

        unsafe { Self::new_unchecked() }
    }
}

impl<Mark> ThreadLocalSingletonOwner<Mark> {
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        Self(PhantomData)
    }
    
    #[inline]
    pub fn ro<'a, T: ?Sized>(&'a self, cell: &'a ValueCell<Self, T>) -> &'a T {
        ValueCellOwner::ro(self, cell)
    }

    #[inline]
    pub fn rw<'a, T: ?Sized>(&'a mut self, cell: &'a ValueCell<Self, T>) -> &'a mut T {
        ValueCellOwner::rw(self, cell)
    }

    #[inline]
    pub fn rw2<'a, T: ?Sized, U: ?Sized>(
        &'a mut self,
        c1: &'a ValueCell<Self, T>,
        c2: &'a ValueCell<Self, U>,
    ) -> (&'a mut T, &'a mut U) {
        ValueCellOwner::rw2(self, c1 ,c2)
    }

    #[inline]
    pub fn rw3<'a, T: ?Sized, U: ?Sized, V: ?Sized>(
        &'a mut self,
        c1: &'a ValueCell<Self, T>,
        c2: &'a ValueCell<Self, U>,
        c3: &'a ValueCell<Self, V>,
    ) -> (&'a mut T, &'a mut U, &'a mut V) {
        ValueCellOwner::rw3(self, c1 ,c2, c3)
    }
}

impl<Mark, T> TLCell<Mark, T> {
    #[inline]
    pub fn new(value: T) -> Self {
        ThreadLocalSingletonOwner(PhantomData).cell(value)
    }
}

unsafe impl<Mark> ValueCellOwner for ThreadLocalSingletonOwner<Mark> {
    type Proxy = ThreadLocalSingletonProxy<Mark>;

    #[inline]
    fn validate_proxy(&self, &ThreadLocalSingletonProxy(PhantomData): &Self::Proxy) -> bool {
        true
    }

    #[inline]
    fn make_proxy(&self) -> Self::Proxy {
        ThreadLocalSingletonProxy(PhantomData)
    }
}