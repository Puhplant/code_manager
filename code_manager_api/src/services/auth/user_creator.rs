use clorinde::deadpool_postgres::Pool;
use async_trait::async_trait;
use clorinde::queries::auth::create_user;
use crate::models::api_error::ApiError;
use crate::services::helpers::get_connection;


#[async_trait]
pub trait IUserCreator: Send + Sync {
    async fn create_user(&self, name: &str, email: &str, password: &str, account_id: i32) -> Result<(), ApiError>;
}

pub struct UserCreator {
    pub pool: Pool,
}

#[async_trait]
impl IUserCreator for UserCreator {
    async fn create_user(&self, name: &str, email: &str, password: &str, account_id: i32) -> Result<(), ApiError> {
        let client = get_connection(&self.pool).await?;
        
        create_user().bind(&client, &name, &email, &email.to_lowercase(), &password, &account_id).await.map_err(|e| {
            println!("Error creating user: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(())
    }
}
