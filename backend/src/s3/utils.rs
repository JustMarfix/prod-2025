use axum::body::Bytes;
use minio::s3::{args::PutObjectApiArgs, utils::Multimap};
use tracing::info;

use crate::{errors::ProdError, AppState};

pub async fn upload_file(
    state: &AppState,
    name: &str,
    content_type: String,
    content: Bytes,
) -> Result<(), ProdError> {
    let client = state.s3.clone();
    let bucket_name = state.bucket_name.clone();

    let mut args = PutObjectApiArgs::new(&bucket_name, name, content.as_ref())
        .map_err(|err| ProdError::S3Error(format!("Failed to create PutObjectApiArgs - {err}")))?;

    let headers = Multimap::from_iter(vec![("Content-Type".to_string(), content_type.clone())]);
    args.headers = Some(&headers);

    let response = client
        .put_object_api(&args)
        .await
        .map_err(|err| ProdError::S3Error(err.to_string()))?;

    info!("Response: {:?}", response);

    Ok(())
}
