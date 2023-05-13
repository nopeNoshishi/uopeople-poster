use crate::domains::documents::{Document, DocumentId, DocumentRepository};
use failure::Error;

pub fn get_document_list(repository: impl DocumentRepository) -> Result<Vec<Document>, Error> {
    repository.list()
}

pub fn get_document(
    repository: impl DocumentRepository,
    document_id: DocumentId,
) -> Result<Document, Error> {
    // ...
}

pub fn post_document(
    repository: impl DocumentRepository,
    document: &Document,
) -> Result<(), Error> {
    // ...
}

pub fn update_document(
    repository: impl DocumentRepository,
    document: &Document,
) -> Result<(), Error> {
    // ...
}

pub fn delete_document(
    repository: impl DocumentRepository,
    document_id: DocumentId,
) -> Result<(), Error> {
    // ...
}
