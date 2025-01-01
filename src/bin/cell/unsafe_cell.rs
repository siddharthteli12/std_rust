/// Creating custom unsafe cell which is primitive used for interior mutability
/// and used by mutex, RwLock and everything using &self but still updating the internal state.
#[derive(Debug, PartialEq, Eq)]
pub struct CustomUnsafeCell<T> {
    value: T,
}

impl<T> CustomUnsafeCell<T> {
    /// Returns new instance of `CustomUnsafeCell<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = CustomUnsafeCell::new("Hello");
    /// assert_eq!(result, CustomUnsafeCell { value: "Hello" });
    /// ```
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Get mutable pointer for an immutable reference type.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = CustomUnsafeCell::new("Hello");
    /// let pointer: *mut String = result.get();
    /// ```
    pub fn get(&self) -> *mut T {
        self as *const CustomUnsafeCell<T> as *const T as *mut T
    }

    /// Update the value on an immutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut unsafe_cell = CustomUnsafeCell::new("Hello".to_string());
    /// unsafe_cell.update_value("Pollo".to_string());
    /// assert_eq!(unsafe_cell.get_value(), "Pollo".to_string());
    /// ```
    pub fn update_value(&self, new_value: T) {
        unsafe {
            let pointer = self.get();
            *pointer = new_value;
        }
    }

    /// Get the current value (read-only).
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let unsafe_cell = CustomUnsafeCell::new("123456".to_string());

    // Passing immutable reference to update the value.
    unsafe_cell.update_value("12345".to_string());

    // Value should be updated.
    assert_eq!(*unsafe_cell.get_value(), "12345".to_string());
}
