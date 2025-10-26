use {
    crate::{
        routes::HttpError,
        utils::authorization::{extract_header, extract_ip_from_request},
        App,
    },
    actix_web::{
        body::MessageBody,
        dev::{ServiceRequest, ServiceResponse},
        http::{header::AUTHORIZATION, Method},
        middleware::Next,
        web, Error, HttpMessage,
    },
};

const AUTHLESS_ROUTES: [(Method, &str); 2] = [(Method::POST, "/auth"), (Method::GET, "/books")];

pub async fn authorization_middleware(
    req: ServiceRequest, next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    for route in AUTHLESS_ROUTES {
        if req.method() == route.0 && req.path().contains(route.1) {
            return Ok(next.call(req).await?);
        };
    }

    let token = extract_header(&req.request(), AUTHORIZATION)?;
    let ip = extract_ip_from_request(&req.request())?;

    let user = req
        .app_data::<web::Data<App>>()
        .unwrap()
        .database
        .fetch_user_by_token_and_ip(token, &ip)
        .await
        .ok_or(HttpError::Unauthorized)?;

    req.extensions_mut().insert(user);

    next.call(req).await
}
