use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

pub async fn authorization_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    //TODO: Make this with wrap stuff in config
    if req.path().contains("/auth") {
        return Ok(next.call(req).await?);
    };

    // req.extensions_mut().insert();

    next.call(req).await
}
