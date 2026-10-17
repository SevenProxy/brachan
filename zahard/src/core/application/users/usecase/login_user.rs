use std::sync::Arc;

use crate::{
    core::{application::users::dto::UserInput, domain::users::repositories::UsersRepository},
    error::AppError,
};

pub struct LoginUser {
    repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl LoginUser {
    pub fn new(repository: Arc<dyn UsersRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_login: UserInput) -> Result<String, AppError> {
        let user = self
            .repository
            .auth(user_login.username, user_login.password)
            .await?;

        Ok(user.username)
    }
}
