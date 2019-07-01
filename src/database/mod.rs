pub mod database;
pub mod record;
pub mod terms;

fn vec_compare<T: PartialEq>(va: &[T], vb: &[T]) -> bool {
    (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a == b)
}
