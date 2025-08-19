// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateBoardParams<T1: crate::StringSql> {
    pub name: T1,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct EditBoardParams<T1: crate::StringSql> {
    pub name: T1,
    pub id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteBoardParams {
    pub id: i32,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct CreateColumnParams<T1: crate::StringSql> {
    pub name: T1,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct EditColumnParams<T1: crate::StringSql> {
    pub name: T1,
    pub id: i32,
    pub account_id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub id: i32,
    pub name: String,
}
pub struct BoardBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
}
impl<'a> From<BoardBorrowed<'a>> for Board {
    fn from(BoardBorrowed { id, name }: BoardBorrowed<'a>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub id: i32,
    pub name: String,
}
pub struct ColumnBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
}
impl<'a> From<ColumnBorrowed<'a>> for Column {
    fn from(ColumnBorrowed { id, name }: ColumnBorrowed<'a>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct I32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i32, tokio_postgres::Error>,
    mapper: fn(i32) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I32Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i32) -> R) -> I32Query<'c, 'a, 's, C, R, N> {
        I32Query {
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
pub struct BoardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<BoardBorrowed, tokio_postgres::Error>,
    mapper: fn(BoardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BoardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(BoardBorrowed) -> R) -> BoardQuery<'c, 'a, 's, C, R, N> {
        BoardQuery {
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
pub struct ColumnQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ColumnBorrowed, tokio_postgres::Error>,
    mapper: fn(ColumnBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ColumnQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ColumnBorrowed) -> R) -> ColumnQuery<'c, 'a, 's, C, R, N> {
        ColumnQuery {
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
pub struct CreateBoardStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_board() -> CreateBoardStmt {
    CreateBoardStmt(
        "INSERT INTO boards (name, account_id) VALUES ($1, $2) returning id",
        None,
    )
}
impl CreateBoardStmt {
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
        name: &'a T1,
        account_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        I32Query {
            client,
            params: [name, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateBoardParams<T1>,
        I32Query<'c, 'a, 's, C, i32, 2>,
        C,
    > for CreateBoardStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateBoardParams<T1>,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        self.bind(client, &params.name, &params.account_id)
    }
}
pub struct GetBoardsByAccountIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_boards_by_account_id() -> GetBoardsByAccountIdStmt {
    GetBoardsByAccountIdStmt("SELECT id, name FROM boards WHERE account_id = $1", None)
}
impl GetBoardsByAccountIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        account_id: &'a i32,
    ) -> BoardQuery<'c, 'a, 's, C, Board, 1> {
        BoardQuery {
            client,
            params: [account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<BoardBorrowed, tokio_postgres::Error> {
                Ok(BoardBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                })
            },
            mapper: |it| Board::from(it),
        }
    }
}
pub struct EditBoardStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn edit_board() -> EditBoardStmt {
    EditBoardStmt(
        "UPDATE boards SET name = $1 WHERE id = $2 AND account_id = $3",
        None,
    )
}
impl EditBoardStmt {
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
        name: &'a T1,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[name, id, account_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EditBoardParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EditBoardStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EditBoardParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.name, &params.id, &params.account_id))
    }
}
pub struct DeleteBoardStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_board() -> DeleteBoardStmt {
    DeleteBoardStmt("DELETE FROM boards WHERE id = $1 AND account_id = $2", None)
}
impl DeleteBoardStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id, account_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteBoardParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteBoardStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteBoardParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.account_id))
    }
}
pub struct CreateColumnStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_column() -> CreateColumnStmt {
    CreateColumnStmt(
        "INSERT INTO columns (name, account_id) VALUES ($1, $2) returning id",
        None,
    )
}
impl CreateColumnStmt {
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
        name: &'a T1,
        account_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        I32Query {
            client,
            params: [name, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateColumnParams<T1>,
        I32Query<'c, 'a, 's, C, i32, 2>,
        C,
    > for CreateColumnStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateColumnParams<T1>,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        self.bind(client, &params.name, &params.account_id)
    }
}
pub struct GetColumnsByAccountIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_columns_by_account_id() -> GetColumnsByAccountIdStmt {
    GetColumnsByAccountIdStmt("SELECT id, name FROM columns WHERE account_id = $1", None)
}
impl GetColumnsByAccountIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        account_id: &'a i32,
    ) -> ColumnQuery<'c, 'a, 's, C, Column, 1> {
        ColumnQuery {
            client,
            params: [account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ColumnBorrowed, tokio_postgres::Error> {
                    Ok(ColumnBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                    })
                },
            mapper: |it| Column::from(it),
        }
    }
}
pub struct EditColumnStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn edit_column() -> EditColumnStmt {
    EditColumnStmt(
        "UPDATE columns SET name = $1 WHERE id = $2 AND account_id = $3",
        None,
    )
}
impl EditColumnStmt {
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
        name: &'a T1,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[name, id, account_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EditColumnParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EditColumnStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EditColumnParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.name, &params.id, &params.account_id))
    }
}
