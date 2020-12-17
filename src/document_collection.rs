use crate::{
    document::Document,
    types::{QLDBExtractError, QLDBExtractResult},
};
use ion_binary_rs::IonValue;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct DocumentCollection {
    documents: Vec<Document>,
}

impl DocumentCollection {
    /// From a collection of documents, it will extract the given property from each and add them.
    /// It will fail in case of an overflow, so it is safer to use this function with BigUint/BigInt.
    /// In case of unsigned numeric types on return type, Overflow means the addition ended with a negative number.
    pub fn extract_and_add<T>(&self, name: &str, initial_value: T) -> QLDBExtractResult<T>
    where
        T: TryFrom<IonValue> + Send + Sync + Clone + Default + num_traits::CheckedAdd,
        <T as TryFrom<IonValue>>::Error: std::error::Error + Send + Sync + 'static,
    {
        let mut value = initial_value;

        for document in &self.documents {
            let element = match document.info.get(name) {
                Some(elem) => elem,
                None => return Err(QLDBExtractError::MissingProperty(name.to_string())),
            };

            let conversion_result = T::try_from(element.clone())
                .map_err(|err| QLDBExtractError::BadDataType(Box::new(err)))?;

            value = value
                .checked_add(&conversion_result)
                .ok_or(QLDBExtractError::Overflow)?;
        }

        Ok(value)
    }
}
