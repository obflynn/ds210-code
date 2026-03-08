use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        if i < self.len {
            unsafe {
                let val = &*self.ptr_to_data.add(i);
                return val;
            }
        } else {
            panic!("FastVec: get out of bounds");
        }       
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            todo!("implement growing the vector by doubling the size!");
        } else {
            todo!("implement pushing t directly since the vector still has capacity!");
        }
    }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {
       
        if i >= self.len { // checks if element to be removed is out of bounds
            panic!("FastVec: remove out of bounds");
        } 
        else { 
            unsafe{
                let _discard = ptr::read(self.ptr_to_data.add(i)); // read the ith element to free its memory
                
                for j in i..self.len-1 { // loop to move all elements (after the discarded element) left by 1 position
                    let ptr_j = self.ptr_to_data.add(j); // pointer to the jth element 
                    let ptr_j_plus_1 = self.ptr_to_data.add(j+1); // pointer to the j+1 element 
                    ptr::write(ptr_j, ptr::read(ptr_j_plus_1)); // move the j+1 element to the jth position & free memory occupied by the j+1 element
                }
                
                self.len = self.len - 1; // decrease vector length by 1 --> idk why this is required but the code fails the remove_numbers test w/o it
            }

        }
               
    } 

    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) { // according to the internet self can only be called inside an implementation block
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
} // moving the curly bracket to include the clear function in the impl<T> FastVec<T> block allows self to be called

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}