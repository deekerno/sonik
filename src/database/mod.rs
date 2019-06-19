pub mod record;
pub mod terms;
pub mod database;

fn vec_compare<T: PartialEq>(va: &[T], vb: &[T]) -> bool {
    (va.len() == vb.len()) &&
        va.iter()
        .zip(vb)
        .all(|(a, b)| a == b)
}
