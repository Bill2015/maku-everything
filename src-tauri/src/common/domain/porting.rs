
pub trait Porting<T> : Sized {
    type Err;
    fn import_from(data: T) -> Result<Self, Self::Err>;

    fn export_to(self) -> Result<T, Self::Err>;
}
