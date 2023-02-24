This package currently provides one function that takes in an array containing tuples of two types; the function then returns two arrays, the first containing all the first elements of the tuples, and the second array containing the second elements of the tuples.
This functionaly is available in iterators through unzip, but unzip can only return collections which implement Extend; which primitive arrays do not. Therefore, unzip works fine for Vec or other dynamic types, but not for simple, beautiful arrays, with lengths known at compiletime.
My implementation is hopefully quite efficient, as it just moves data around, without using too much costly abstractions like std::array::from_fn.
This crate has 4 tests that I think cover basically everything; still it could be unsound..