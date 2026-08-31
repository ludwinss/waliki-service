use crate::core::domain::tienda::entities::country::Country;

pub trait CountryRepository {
    fn find_by_phone_prefix(&self, raw_phone: &str) -> Option<Country>;
}
