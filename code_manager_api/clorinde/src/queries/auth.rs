// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertRefreshTokenParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub token: T1,
    pub expires_at: chrono::NaiveDateTime,
}
#[derive(Debug)]
pub struct CreateUserParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub name: T1,
    pub email: T2,
    pub normalized_email: T3,
    pub password_hash: T4,
    pub account_id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserPassword {
    pub id: i32,
    pub password_hash: String,
    pub account_id: i32,
}
pub struct UserPasswordBorrowed<'a> {
    pub id: i32,
    pub password_hash: &'a str,
    pub account_id: i32,
}
impl<'a> From<UserPasswordBorrowed<'a>> for UserPassword {
    fn from(
        UserPasswordBorrowed {
            id,
            password_hash,
            account_id,
        }: UserPasswordBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            password_hash: password_hash.into(),
            account_id,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserPasswordQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UserPasswordBorrowed, tokio_postgres::Error>,
    mapper: fn(UserPasswordBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserPasswordQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UserPasswordBorrowed) -> R,
    ) -> UserPasswordQuery<'c, 'a, 's, C, R, N> {
        UserPasswordQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct InsertRefreshTokenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_refresh_token() -> InsertRefreshTokenStmt {
    InsertRefreshTokenStmt(
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)",
        None,
    )
}
impl InsertRefreshTokenStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        token: &'a T1,
        expires_at: &'a chrono::NaiveDateTime,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, token, expires_at]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertRefreshTokenParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertRefreshTokenStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a InsertRefreshTokenParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.token, &params.expires_at))
    }
}
pub struct CreateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_user() -> CreateUserStmt {
    CreateUserStmt(
        "INSERT INTO users (name, email, normalized_email, password_hash, account_id) VALUES ($1, $2, $3, $4, $5)",
        None,
    )
}
impl CreateUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        email: &'a T2,
        normalized_email: &'a T3,
        password_hash: &'a T4,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[name, email, normalized_email, password_hash, account_id],
            )
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateUserParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateUserParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.name,
            &params.email,
            &params.normalized_email,
            &params.password_hash,
            &params.account_id,
        ))
    }
}
pub struct GetUserByEmailStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user_by_email() -> GetUserByEmailStmt {
    GetUserByEmailStmt(
        "SELECT id, password_hash, account_id FROM users WHERE normalized_email = $1 limit 1",
        None,
    )
}
impl GetUserByEmailStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        email: &'a T1,
    ) -> UserPasswordQuery<'c, 'a, 's, C, UserPassword, 1> {
        UserPasswordQuery {
            client,
            params: [email],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UserPasswordBorrowed, tokio_postgres::Error> {
                    Ok(UserPasswordBorrowed {
                        id: row.try_get(0)?,
                        password_hash: row.try_get(1)?,
                        account_id: row.try_get(2)?,
                    })
                },
            mapper: |it| UserPassword::from(it),
        }
    }
}
