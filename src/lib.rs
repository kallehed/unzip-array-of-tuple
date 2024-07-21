/// Unzips an array of tuples like [(i32, i64); N] into two arrays -> ([i32; N], [i64; N])
/// Useful
///
/// # Examples
/// ```
/// # use unzip_array_of_tuple::unzip_array_of_tuple;
/// #
/// let cats = [("Oliver", 3), ("Kitty", 7), ("Rosie", 1)];
///
/// let (cat_names, cat_ages) = unzip_array_of_tuple(cats);
///
/// assert_eq!(cat_names, ["Oliver", "Kitty", "Rosie"]);
/// assert_eq!(cat_ages, [3, 7, 1]);
/// ```
pub fn unzip_array_of_tuple<T1, T2, const N: usize>(arr: [(T1,T2); N]) -> ([T1; N], [T2; N])
{
    use std::mem::{MaybeUninit, self};
    
    let mut first: [MaybeUninit<T1>; N] = unsafe {MaybeUninit::uninit().assume_init()};
    let mut second: [MaybeUninit<T2>; N] = unsafe {MaybeUninit::uninit().assume_init()};

    for (idx, a) in arr.into_iter().enumerate()
    {
        first[idx] = MaybeUninit::new(a.0);
        second[idx] = MaybeUninit::new(a.1);
    }
    
    // should be safe, as MaybeUninit doesn't have Drop
    unsafe { (mem::transmute_copy(&first), mem::transmute_copy(&second)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_basic()
    {
        let tmp = [(1,3),(2,4)];
        let res = unzip_array_of_tuple(tmp);
        assert_eq!(res, ([1,2],[3,4]));
    }

    #[test]
    fn droppable_types()
    {
        use String as S;
        let tmp = [(S::from("1"), S::from("3")),(S::from("2"), S::from("4"))];
        let res = unzip_array_of_tuple(tmp);
        assert_eq!(res, ([S::from("1"), S::from("2")], [S::from("3"), S::from("4")]));
    }

    #[test]
    fn droppable_types_with_side_effects()
    {
        use std::cell::RefCell;
        let a = RefCell::new(0);
        {
            struct Droppy <'a>{
                d: &'a RefCell<i32>,
                v: i32,
            }

            impl <'a> Drop for Droppy<'a>
            {
                fn drop(&mut self)
                {
                    *self.d.borrow_mut() += self.v;
                }
            }
            let tmp = [(Droppy {d: &a, v: 1}, Droppy {d: &a, v: 3}), (Droppy {d: &a, v: 2}, Droppy {d: &a, v:4})];
            
            let res = unzip_array_of_tuple(tmp);

            assert_eq!(res.0[0].v, 1);
            assert_eq!(res.0[1].v, 2);
            assert_eq!(res.1[0].v, 3);
            assert_eq!(res.1[1].v, 4);
        }
        assert_eq!(1+2+3+4, *a.borrow());

    }

    #[test]
    fn long_array()
    {
        let tmp = [(349, 20.4); 1000];
        let res = unzip_array_of_tuple(tmp);
        assert_eq!(res.0, [349;1000]);
        assert_eq!(res.1, [20.4;1000]);
    }
}
