use utoipa::ToSchema;

pub mod health;

#[derive(ToSchema)]
struct Error<'a> {
    message: &'a str,
}
