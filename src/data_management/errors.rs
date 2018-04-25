#![allow(missing_docs, unused_doc_comment)]
error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }
    foreign_links {
        VdevError(::vdev::Error);
    }
    errors {
        DecompressionError
        DeserializationError
        SerializationError
        HandlerError
        CannotWriteBackError
        OutOfSpaceError
    }
}
