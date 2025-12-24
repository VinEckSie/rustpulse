// Use type-erased errors such as Box<dyn Error + Send + Sync> at selected boundaries so you can return heterogeneous errors and still use ?; 
// remember that pointer/wrapper types (like Box<T>) don’t automatically implement the traits of T, so you must use the right trait object type; 
// downcast type-erased errors with downcast_ref when you really need to recover a specific error type;
//  use the Try trait and From/Into patterns more broadly to design ergonomic error APIs.