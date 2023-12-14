use smallvec::{smallvec, SmallVec};

pub fn transpose<T: Clone, const N: usize>(original: &[SmallVec<[T; N]>]) -> Vec<SmallVec<[T; N]>> {
    assert!(!original.is_empty());
    let mut transposed: Vec<SmallVec<[T; N]>> = vec![smallvec![]; original[0].len()];

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item.clone());
        }
    }

    transposed
}
pub fn transpose_vec<T: Clone>(original: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed: Vec<Vec<T>> = vec![vec![]; original[0].len()];

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item.clone());
        }
    }

    transposed
}
