use core::fmt::Debug;
use std::fmt::Display;

/// trait `Stack<T>` defines the method a generic monomorphic stack needs to implement.
/// We require that that the values that we push to a stack implement a set of traits.
///  - `Debug` since an implementation of `Stack<T>` is required to implement trait `Debug`
///  - `PartialEq` since we want to support `assert_eq` in our code examples
///  - `Clone` since an implementation of `Stack<T>` is required to implement trait `Clone`
///  - `Display` since an implementation of `Stack<T>` is required to implement trait `Display`
pub trait Stack<T: Debug + PartialEq + Clone + Display>:
    Debug + Display + Clone + PartialEq
{
    /// Create a new monomorphic stack storing elements of type `<T>`.
    /// # Example
    ///
    /// ```compile_fail
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u128
    /// let mut stack : GenericStack<u128> = GenericStack::new();
    /// ```
    fn new() -> Self;

    /// push a new element on the top element of the stack.
    ///
    /// # Arguments
    ///  - `element` to be pushed on the stack
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::GenericStack;
    /// // We create a stack of u64
    /// let mut stack = GenericStack::new();
    ///
    /// // we an push an element to the stack
    /// stack.push(1u64);
    /// assert_eq!(stack.peek(), Some(&1u64));
    /// ```
    fn push(&mut self, elem: T);

    /// Returns the top element of the stack if it exists, i.e.,
    /// the last element that was pushed on the stack and not yet
    /// removed by a preceding call to `pop`
    ///
    /// # Arguments
    ///  - `pop` does not take any arguments.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// // We need to import this trait to use the methods of this trait.
    /// // We can import an implementation like `ll_stack`
    /// use stack_trait::Stack;
    /// use ll_stack::{GenericStack,Iterators};
    /// // We create a stack of i32
    /// let mut stack = GenericStack::new();
    ///
    /// // Initially, the stack is empty:
    /// assert_eq!(stack.pop(), None);
    /// // we an push an element to the stack
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    fn pop(&mut self) -> Option<T>;

    /// borrows the top element of the stack if the stack is not empty.
    /// This will return `None` if the stack is empty.
    ///
    /// # Example
    ///
    /// ```compile_fail
    ///     println!("Top element: {:?}", stack.peek());
    /// ```
    fn peek(&self) -> Option<&T>;

    /// borrows the top element of the stack as a mutable value if the stack is not empty.
    /// This will return `None` if the stack is empty.
    ///
    /// # Example
    ///
    /// ```compile_fail
    ///   stack.peek_mut().map(|node| node.value += 1);
    /// ```

    fn peek_mut(&mut self) -> Option<&mut T>;
}
