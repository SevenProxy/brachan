use std::sync::Mutex;

use async_trait::async_trait;
use chrono::Utc;

use crate::{
    core::{
        application::users::password::verify_password,
        domain::users::{
            entities::{ActiveModel, Model},
            repositories::UsersRepository,
        },
    },
    error::AppError,
};

pub fn make_user(id: i64, username: &str, password: &str, role: &str, is_active: bool) -> Model {
    Model {
        id,
        username: username.to_string(),
        password: password.to_string(),
        role: role.to_string(),
        is_active,
        created_at: Utc::now().fixed_offset(),
    }
}

pub struct MockUsersRepository {
    users: Mutex<Vec<Model>>,
    next_id: Mutex<i64>,
}

impl MockUsersRepository {
    pub fn new() -> Self {
        Self::with_users(Vec::new())
    }

    pub fn with_users(users: Vec<Model>) -> Self {
        let next_id = users.iter().map(|user| user.id).max().unwrap_or(0) + 1;

        Self {
            users: Mutex::new(users),
            next_id: Mutex::new(next_id),
        }
    }

    pub fn users(&self) -> Vec<Model> {
        self.users.lock().unwrap().clone()
    }
}

impl Default for MockUsersRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UsersRepository for MockUsersRepository {
    async fn create(&self, new_user: ActiveModel) -> Result<Model, AppError> {
        let username = new_user.username.unwrap();
        let mut users = self.users.lock().unwrap();

        if users.iter().any(|user| user.username == username) {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let model = Model {
            id,
            username,
            password: new_user.password.unwrap(),
            role: new_user.role.unwrap(),
            is_active: new_user.is_active.unwrap(),
            created_at: new_user.created_at.unwrap(),
        };

        users.push(model.clone());

        Ok(model)
    }

    async fn update(&self, update: ActiveModel) -> Result<Model, AppError> {
        let id = update.id.unwrap();
        let mut users = self.users.lock().unwrap();

        let user = users
            .iter_mut()
            .find(|user| user.id == id)
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.username = update.username.unwrap();
        user.password = update.password.unwrap();
        user.role = update.role.unwrap();
        user.is_active = update.is_active.unwrap();
        user.created_at = update.created_at.unwrap();

        Ok(user.clone())
    }

    async fn count_by_role(&self, role: &str) -> Result<u64, AppError> {
        let users = self.users.lock().unwrap();

        Ok(users.iter().filter(|user| user.role == role).count() as u64)
    }

    async fn auth(&self, username: String, password: String) -> Result<Model, AppError> {
        let users = self.users.lock().unwrap();

        let user = users
            .iter()
            .find(|user| user.username == username)
            .ok_or(AppError::Unauthorized)?;

        if verify_password(&password, &user.password)? {
            Ok(user.clone())
        } else {
            Err(AppError::Unauthorized)
        }
    }

    async fn find_by_username(&self, username: String) -> Result<Option<Model>, AppError> {
        let users = self.users.lock().unwrap();

        Ok(users.iter().find(|user| user.username == username).cloned())
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Model>, AppError> {
        let users = self.users.lock().unwrap();

        Ok(users.iter().find(|user| user.id == id).cloned())
    }
}
