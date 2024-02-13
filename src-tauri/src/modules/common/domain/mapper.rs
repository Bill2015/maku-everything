pub trait DomainModelMapper<V> : Sized {
    fn to_domain(self) -> V;

    fn from_domain(value: V) -> Self;
}
