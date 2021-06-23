#[derive(Debug)]
pub enum Side {
    Right,
    Left,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

/// Find indices where elements should be inserted to maintain order.
/// Find the indices into a sorted array `a` such that, if the
/// corresponding elements in `v` were inserted before the indices, the
/// order of `a` would be preserved.
/// Assuming that `a` is sorted:
///
///
/// | `side` |  returned index `i` satisfies |
/// |--------|-------------------------------|
/// | left   | ``a[i-1] < v <= a[i]``        |
/// | right  | ``a[i-1] <= v < a[i]``        |
///
///
/// ## Parameters
/// - `a` : 1-D array_like
///     Input array. If `sorter` is None, then it must be sorted in
///     ascending order, otherwise `sorter` must be an array of indices
///     that sort it.
/// - `v` : array_like
///     Values to insert into `a`.
/// side : {'left', 'right'}, optional
///     If 'left', the index of the first suitable location found is given.
///     If 'right', return the last such index.  If there is no suitable
///     index, return either 0 or N (where N is the length of `a`).
/// sorter : 1-D array_like, optional
///     Optional array of integer indices that sort array a into ascending
///     order. They are typically the result of argsort.
///     .. versionadded:: 1.7.0
///
/// ## Returns
/// indices : array of ints
///     Array of insertion points with the same shape as `v`.
///
/// ## See Also
/// sort : Return a sorted copy of an array.
/// histogram : Produce histogram from 1-D data.
///
/// ## Notes
/// Binary search is used to find the required insertion points.
/// As of NumPy 1.4.0 `searchsorted` works with real/complex arrays containing
/// `nan` values. The enhanced sort order is documented in `sort`.
/// This function uses the same algorithm as the builtin python `bisect.bisect_left`
/// (``side='left'``) and `bisect.bisect_right` (``side='right'``) functions,
/// which is also vectorized in the `v` argument.
///
/// ## Examples
/// ```
/// let a1 = vec![1, 2, 3, 4, 5];
/// let v1 = vec![3];
/// let r1 = np::search_sorted(a1, v1, Side::default());
/// assert_eq!(r1, vec![2]);
///
/// let a2 = vec![1, 2, 3, 4, 5];
/// let v2 = vec![3];
/// let r2 = np::search_sorted(a2, v2, Side::Right);
/// assert_eq!(r2, vec![3]);
///
/// let a3 = vec![1, 2, 3, 4, 5];
/// let v3 = vec![-10, 10, 2, 3];
/// let r3 = np::search_sorted(a3, v3, Side::default());
/// assert_eq!(r3, vec![0, 5, 1, 2]);
///
/// let a4 = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
/// let v4 = vec![-1., 1.4, 4.2, 10.2, 5.4, 6.5, 4.0, 6.0, 11.0];
/// let r4 = np::search_sorted(a4, v4, Side::default());
/// assert_eq!(r4, vec![0, 2, 5, 11, 6, 7, 4, 6, 11]);
/// ```

pub fn search_sorted<T: std::cmp::PartialOrd>(a: Vec::<T>, v: Vec::<T>, side: Side) -> Vec<usize> {
    let mut ret: Vec<usize> = vec![];
    let expr1: fn(&T, &T) -> bool;
    let expr2: fn(&T, &T) -> bool;
    match side {
        Side::Left => {
            expr1 = |v, a| v <= a;
            expr2 = |v, a| v > a;
        }
        Side::Right => {
            expr1 = |v, a| v < a;
            expr2 = |v, a| v >= a;
        }
    }
    for v_val in v {
        for (a_idx, a_val) in a.iter().enumerate() {
            match (expr1(&v_val, a_val), expr2(&v_val, a_val), a_idx == a.len() - 1) {
                (true, _, _) => {
                    ret.push(a_idx);
                    break;
                }
                (_, true, false) => { continue; }
                (_, true, true) => {
                    ret.push(a_idx + 1);
                    break;
                }
                (_, _, _) => {}
            }
        }
    }
    ret
}