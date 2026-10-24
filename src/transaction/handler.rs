use crate::{
    error::AppError,
    middleware::AuthUser,
    state::AppState,
    transaction::{
        model::{
            CreateTransactionRequest, TransactionListResponse, TransactionResponse,
            TransactionType, UpdateTransactionRequest,
        },
        repository::TransactionFilters,
    },
};
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    #[serde(rename = "type")]
    pub transaction_type: Option<TransactionType>,
    pub category: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/transactions",
            post(create_transaction).get(list_transactions),
        )
        .route(
            "/transactions/{id}",
            get(get_transaction)
                .patch(update_transaction)
                .delete(delete_transaction),
        )
}

#[utoipa::path(
    post,
    path = "/transactions",
    request_body = CreateTransactionRequest,
    responses(
        (status = 201, description = "Transaction created", body = TransactionResponse),
        (status = 400, description = "Invalid transaction data", body = crate::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Transactions"
)]
pub async fn create_transaction(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(input): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<TransactionResponse>), AppError> {
    input.validate().map_err(|_| AppError::BadRequest)?;

    let response = state.transaction_service.create(user.id, input).await?;

    tracing::info!(
        user_id = %user.id,
        transaction_id = %response.id,
        "Transaction created"
    );

    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/transactions/{id}",
    params(
        ("id" = Uuid, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction found", body = TransactionResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 404, description = "Transaction not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Transactions"
)]
pub async fn get_transaction(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, AppError> {
    let response = state.transaction_service.get(id, user.id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/transactions",
    params(
        ("type" = Option<TransactionType>, Query, description = "Filter by INCOME or EXPENSE"),
        ("category" = Option<String>, Query, description = "Category; maximum 50 characters"),
        ("from" = Option<NaiveDate>, Query, description = "Inclusive start date, YYYY-MM-DD"),
        ("to" = Option<NaiveDate>, Query, description = "Inclusive end date, YYYY-MM-DD"),
        ("page" = Option<u32>, Query, description = "Page number, default 1, minimum 1"),
        ("limit" = Option<u32>, Query, description = "Page size, default 20, maximum 100")
    ),
    responses(
        (status = 200, description = "Paginated transaction list", body = TransactionListResponse),
        (status = 400, description = "Invalid filter or pagination value", body = crate::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Transactions"
)]
pub async fn list_transactions(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<ListTransactionsQuery>,
) -> Result<Json<TransactionListResponse>, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    if page == 0 || limit == 0 || limit > 100 {
        return Err(AppError::BadRequest);
    }

    if let (Some(from), Some(to)) = (query.from, query.to)
        && from > to
    {
        return Err(AppError::BadRequest);
    }

    let from = query.from.map(|date| {
        DateTime::<Utc>::from_naive_utc_and_offset(
            date.and_hms_opt(0, 0, 0).expect("midnight is valid"),
            Utc,
        )
    });

    let until = query.to.map(|date| {
        let next_day = date.succ_opt();
        next_day
            .and_then(|day| day.and_hms_opt(0, 0, 0))
            .map(|time| DateTime::<Utc>::from_naive_utc_and_offset(time, Utc))
    });

    // The day after 9999-12-31 has no representable date.
    let until = match until {
        Some(None) => return Err(AppError::BadRequest),
        Some(Some(value)) => Some(value),
        None => None,
    };

    let offset = page
        .checked_sub(1)
        .and_then(|value| value.checked_mul(limit))
        .ok_or(AppError::BadRequest)?;

    let category = query.category.map(|value| value.trim().to_owned());

    if category
        .as_ref()
        .is_some_and(|value| value.is_empty() || value.chars().count() > 50)
    {
        return Err(AppError::BadRequest);
    }

    let filters = TransactionFilters {
        transaction_type: query
            .transaction_type
            .map(|value| value.as_str().to_owned()),
        category,
        from,
        until,
        limit: i64::from(limit),
        offset: i64::from(offset),
    };

    let items = state.transaction_service.list(user.id, &filters).await?;

    Ok(Json(TransactionListResponse { items, page, limit }))
}

#[utoipa::path(
    patch,
    path = "/transactions/{id}",
    params(
        ("id" = Uuid, Path, description = "Transaction ID")
    ),
    request_body = UpdateTransactionRequest,
    responses(
        (status = 200, description = "Transaction updated", body = TransactionResponse),
        (status = 400, description = "Invalid update data", body = crate::error::ErrorResponse),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 404, description = "Transaction not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Transactions"
)]
pub async fn update_transaction(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    input.validate().map_err(|_| AppError::BadRequest)?;

    let response = state.transaction_service.update(id, user.id, input).await?;

    tracing::info!(
        user_id = %user.id,
        transaction_id = %id,
        "Transaction updated"
    );

    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/transactions/{id}",
    params(
        ("id" = Uuid, Path, description = "Transaction ID")
    ),
    responses(
        (status = 204, description = "Transaction deleted"),
        (status = 401, description = "Authentication required", body = crate::error::ErrorResponse),
        (status = 404, description = "Transaction not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Transactions"
)]
pub async fn delete_transaction(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.transaction_service.delete(id, user.id).await?;

    tracing::info!(
        user_id = %user.id,
        transaction_id = %id,
        "Transaction deleted"
    );

    Ok(StatusCode::NO_CONTENT)
}
