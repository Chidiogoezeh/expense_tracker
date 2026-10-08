use uuid::Uuid;

use crate::{error::AppError, expense::repository::ExpenseRepository};

pub struct ExpenseService {
    repository: ExpenseRepository,
}

impl ExpenseService {
    pub fn new(repository: ExpenseRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        description: String,
        amount: f64,
        category: String,
    ) -> Result<ExpenseResult, AppError> {
        let expense = self
            .repository
            .create(user_id, description, amount, category)
            .await
            .map_err(|_| AppError::Database)?;

        Ok(ExpenseResult {
            id: expense.id,
            description: expense.description,
            amount: expense.amount,
            category: expense.category,
        })
    }

    pub async fn get_all(&self, user_id: Uuid) -> Result<Vec<ExpenseResult>, AppError> {
        let expenses = self
            .repository
            .find_all_by_user(user_id)
            .await
            .map_err(|_| AppError::Database)?;

        Ok(expenses
            .into_iter()
            .map(|expense| ExpenseResult {
                id: expense.id,
                description: expense.description,
                amount: expense.amount,
                category: expense.category,
            })
            .collect())
    }

    pub async fn delete(&self, expense_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let deleted = self
            .repository
            .delete(expense_id, user_id)
            .await
            .map_err(|_| AppError::Database)?;

        if !deleted {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    pub async fn get_total(&self, user_id: Uuid) -> Result<f64, AppError> {
        self.repository
            .get_total(user_id)
            .await
            .map_err(|_| AppError::Database)
    }

    pub async fn get_category_totals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CategoryTotalResult>, AppError> {
        let totals = self
            .repository
            .get_category_totals(user_id)
            .await
            .map_err(|_| AppError::Database)?;

        Ok(totals
            .into_iter()
            .map(|item| CategoryTotalResult {
                category: item.category,
                total: item.total.unwrap_or(0.0),
            })
            .collect())
    }
}

pub struct ExpenseResult {
    pub id: Uuid,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

pub struct CategoryTotalResult {
    pub category: String,
    pub total: f64,
}
