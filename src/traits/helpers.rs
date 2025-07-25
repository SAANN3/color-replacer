pub trait Separator<T> {
    fn separate(&self, separator: T) -> Vec<T>;
}
impl<T> Separator<T> for Vec<T>
    where T: Clone {
    fn separate(&self, separator: T) -> Vec<T> {
        let mut vec = Vec::<T>::new();
        for i in self {
            vec.push(separator.clone());
            vec.push(i.clone());
        }
        vec.push(separator.clone());
        vec
    }
}