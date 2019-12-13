use shaku_internals::error::Error as DIError;
use syn::{ self, Data, Field };

use parser::{ Extractor, ExtractorIterator };

/// Extract the `Field` data of a struct
/// - Enum => return an error
/// - Struct::Tuple => return an empty vector
/// - Other cases => return a `ExtractorIterator<Field>`
impl Extractor<Field> for syn::DeriveInput {
    fn extract(&self) -> Result<ExtractorIterator<Field>, DIError> {
        let fields_vect = match self.data {
            Data::Struct(ref variant_data) => Ok(variant_data.fields.clone()),
            _ => Err(DIError::ExtractError("only structs are currently supported".to_string())),
        }?;

        Ok(ExtractorIterator::from(fields_vect.into_iter()))
    }
}