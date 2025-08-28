// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateTicketParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub title: T1,
    pub description: T2,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub board_id: i32,
    pub account_id: i32,
    pub user_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetTicketByIdParams {
    pub id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetTicketsByColumnIdParams {
    pub column_id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetBoardTicketsByBoardIdParams {
    pub board_id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetBacklogTicketsByBoardIdParams {
    pub board_id: i32,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct EditTicketParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub title: T1,
    pub description: T2,
    pub column_id: Option<i32>,
    pub id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct MoveTicketParams {
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteTicketParams {
    pub id: i32,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct CreateTicketCommentParams<T1: crate::StringSql> {
    pub ticket_id: i32,
    pub content: T1,
    pub account_id: i32,
    pub user_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetTicketCommentsByTicketIdParams {
    pub ticket_id: i32,
    pub account_id: i32,
}
#[derive(Debug)]
pub struct EditTicketCommentParams<T1: crate::StringSql> {
    pub content: T1,
    pub id: i32,
    pub account_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteTicketCommentParams {
    pub id: i32,
    pub account_id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub account_id: i32,
    pub board_id: i32,
    pub user_id: i32,
}
pub struct TicketBorrowed<'a> {
    pub id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub account_id: i32,
    pub board_id: i32,
    pub user_id: i32,
}
impl<'a> From<TicketBorrowed<'a>> for Ticket {
    fn from(
        TicketBorrowed {
            id,
            title,
            description,
            column_id,
            position,
            account_id,
            board_id,
            user_id,
        }: TicketBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            column_id,
            position,
            account_id,
            board_id,
            user_id,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct MinTicket {
    pub id: i32,
    pub title: String,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub board_id: i32,
    pub account_id: i32,
    pub user_id: i32,
}
pub struct MinTicketBorrowed<'a> {
    pub id: i32,
    pub title: &'a str,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub board_id: i32,
    pub account_id: i32,
    pub user_id: i32,
}
impl<'a> From<MinTicketBorrowed<'a>> for MinTicket {
    fn from(
        MinTicketBorrowed {
            id,
            title,
            column_id,
            position,
            board_id,
            account_id,
            user_id,
        }: MinTicketBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            column_id,
            position,
            board_id,
            account_id,
            user_id,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TicketComment {
    pub id: i32,
    pub ticket_id: i32,
    pub content: String,
    pub account_id: i32,
    pub user_id: i32,
}
pub struct TicketCommentBorrowed<'a> {
    pub id: i32,
    pub ticket_id: i32,
    pub content: &'a str,
    pub account_id: i32,
    pub user_id: i32,
}
impl<'a> From<TicketCommentBorrowed<'a>> for TicketComment {
    fn from(
        TicketCommentBorrowed {
            id,
            ticket_id,
            content,
            account_id,
            user_id,
        }: TicketCommentBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            ticket_id,
            content: content.into(),
            account_id,
            user_id,
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
pub struct TicketQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TicketBorrowed, tokio_postgres::Error>,
    mapper: fn(TicketBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TicketQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(TicketBorrowed) -> R) -> TicketQuery<'c, 'a, 's, C, R, N> {
        TicketQuery {
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
pub struct MinTicketQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<MinTicketBorrowed, tokio_postgres::Error>,
    mapper: fn(MinTicketBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MinTicketQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(MinTicketBorrowed) -> R) -> MinTicketQuery<'c, 'a, 's, C, R, N> {
        MinTicketQuery {
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
pub struct TicketCommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TicketCommentBorrowed, tokio_postgres::Error>,
    mapper: fn(TicketCommentBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TicketCommentQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(TicketCommentBorrowed) -> R,
    ) -> TicketCommentQuery<'c, 'a, 's, C, R, N> {
        TicketCommentQuery {
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
pub struct CreateTicketStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_ticket() -> CreateTicketStmt {
    CreateTicketStmt(
        "INSERT INTO tickets (title, description, column_id, position, board_id, account_id, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) returning id",
        None,
    )
}
impl CreateTicketStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        title: &'a T1,
        description: &'a T2,
        column_id: &'a Option<i32>,
        position: &'a Option<f64>,
        board_id: &'a i32,
        account_id: &'a i32,
        user_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 7> {
        I32Query {
            client,
            params: [
                title,
                description,
                column_id,
                position,
                board_id,
                account_id,
                user_id,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateTicketParams<T1, T2>,
        I32Query<'c, 'a, 's, C, i32, 7>,
        C,
    > for CreateTicketStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateTicketParams<T1, T2>,
    ) -> I32Query<'c, 'a, 's, C, i32, 7> {
        self.bind(
            client,
            &params.title,
            &params.description,
            &params.column_id,
            &params.position,
            &params.board_id,
            &params.account_id,
            &params.user_id,
        )
    }
}
pub struct GetTicketByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_ticket_by_id() -> GetTicketByIdStmt {
    GetTicketByIdStmt(
        "SELECT id, title, description, column_id, position, account_id, board_id, user_id FROM tickets WHERE id = $1 AND account_id = $2",
        None,
    )
}
impl GetTicketByIdStmt {
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
        id: &'a i32,
        account_id: &'a i32,
    ) -> TicketQuery<'c, 'a, 's, C, Ticket, 2> {
        TicketQuery {
            client,
            params: [id, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TicketBorrowed, tokio_postgres::Error> {
                    Ok(TicketBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        column_id: row.try_get(3)?,
                        position: row.try_get(4)?,
                        account_id: row.try_get(5)?,
                        board_id: row.try_get(6)?,
                        user_id: row.try_get(7)?,
                    })
                },
            mapper: |it| Ticket::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetTicketByIdParams,
        TicketQuery<'c, 'a, 's, C, Ticket, 2>,
        C,
    > for GetTicketByIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetTicketByIdParams,
    ) -> TicketQuery<'c, 'a, 's, C, Ticket, 2> {
        self.bind(client, &params.id, &params.account_id)
    }
}
pub struct GetTicketsByColumnIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_tickets_by_column_id() -> GetTicketsByColumnIdStmt {
    GetTicketsByColumnIdStmt(
        "SELECT id, title, description, column_id, position, account_id, board_id, user_id FROM tickets WHERE column_id = $1 AND account_id = $2 ORDER BY position ASC",
        None,
    )
}
impl GetTicketsByColumnIdStmt {
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
        column_id: &'a i32,
        account_id: &'a i32,
    ) -> TicketQuery<'c, 'a, 's, C, Ticket, 2> {
        TicketQuery {
            client,
            params: [column_id, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TicketBorrowed, tokio_postgres::Error> {
                    Ok(TicketBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        column_id: row.try_get(3)?,
                        position: row.try_get(4)?,
                        account_id: row.try_get(5)?,
                        board_id: row.try_get(6)?,
                        user_id: row.try_get(7)?,
                    })
                },
            mapper: |it| Ticket::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetTicketsByColumnIdParams,
        TicketQuery<'c, 'a, 's, C, Ticket, 2>,
        C,
    > for GetTicketsByColumnIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetTicketsByColumnIdParams,
    ) -> TicketQuery<'c, 'a, 's, C, Ticket, 2> {
        self.bind(client, &params.column_id, &params.account_id)
    }
}
pub struct GetBoardTicketsByBoardIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_board_tickets_by_board_id() -> GetBoardTicketsByBoardIdStmt {
    GetBoardTicketsByBoardIdStmt(
        "SELECT id, title, column_id, position, board_id, account_id, user_id FROM ( select id, title, column_id, position, board_id, account_id, user_id, row_number() over (partition by column_id order by position ASC) as row_number from tickets where board_id = $1 AND account_id = $2 and column_id != null ) sub WHERE row_number <= 10",
        None,
    )
}
impl GetBoardTicketsByBoardIdStmt {
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
        board_id: &'a i32,
        account_id: &'a i32,
    ) -> MinTicketQuery<'c, 'a, 's, C, MinTicket, 2> {
        MinTicketQuery {
            client,
            params: [board_id, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<MinTicketBorrowed, tokio_postgres::Error> {
                    Ok(MinTicketBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        column_id: row.try_get(2)?,
                        position: row.try_get(3)?,
                        board_id: row.try_get(4)?,
                        account_id: row.try_get(5)?,
                        user_id: row.try_get(6)?,
                    })
                },
            mapper: |it| MinTicket::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetBoardTicketsByBoardIdParams,
        MinTicketQuery<'c, 'a, 's, C, MinTicket, 2>,
        C,
    > for GetBoardTicketsByBoardIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetBoardTicketsByBoardIdParams,
    ) -> MinTicketQuery<'c, 'a, 's, C, MinTicket, 2> {
        self.bind(client, &params.board_id, &params.account_id)
    }
}
pub struct GetBacklogTicketsByBoardIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_backlog_tickets_by_board_id() -> GetBacklogTicketsByBoardIdStmt {
    GetBacklogTicketsByBoardIdStmt(
        "SELECT id, title, column_id, position, account_id, board_id, user_id FROM tickets WHERE board_id = $1 AND account_id = $2 and column_id is null ORDER BY position ASC",
        None,
    )
}
impl GetBacklogTicketsByBoardIdStmt {
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
        board_id: &'a i32,
        account_id: &'a i32,
    ) -> MinTicketQuery<'c, 'a, 's, C, MinTicket, 2> {
        MinTicketQuery {
            client,
            params: [board_id, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<MinTicketBorrowed, tokio_postgres::Error> {
                    Ok(MinTicketBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        column_id: row.try_get(2)?,
                        position: row.try_get(3)?,
                        board_id: row.try_get(5)?,
                        account_id: row.try_get(4)?,
                        user_id: row.try_get(6)?,
                    })
                },
            mapper: |it| MinTicket::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetBacklogTicketsByBoardIdParams,
        MinTicketQuery<'c, 'a, 's, C, MinTicket, 2>,
        C,
    > for GetBacklogTicketsByBoardIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetBacklogTicketsByBoardIdParams,
    ) -> MinTicketQuery<'c, 'a, 's, C, MinTicket, 2> {
        self.bind(client, &params.board_id, &params.account_id)
    }
}
pub struct EditTicketStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn edit_ticket() -> EditTicketStmt {
    EditTicketStmt(
        "UPDATE tickets SET title = $1, description = $2, column_id = $3 WHERE id = $4 AND account_id = $5",
        None,
    )
}
impl EditTicketStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        title: &'a T1,
        description: &'a T2,
        column_id: &'a Option<i32>,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[title, description, column_id, id, account_id])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EditTicketParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EditTicketStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EditTicketParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.title,
            &params.description,
            &params.column_id,
            &params.id,
            &params.account_id,
        ))
    }
}
pub struct MoveTicketStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn move_ticket() -> MoveTicketStmt {
    MoveTicketStmt(
        "UPDATE tickets SET column_id = $1, position = $2 WHERE id = $3 AND account_id = $4",
        None,
    )
}
impl MoveTicketStmt {
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
        column_id: &'a Option<i32>,
        position: &'a Option<f64>,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[column_id, position, id, account_id])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        MoveTicketParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for MoveTicketStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a MoveTicketParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.column_id,
            &params.position,
            &params.id,
            &params.account_id,
        ))
    }
}
pub struct DeleteTicketStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_ticket() -> DeleteTicketStmt {
    DeleteTicketStmt(
        "DELETE FROM tickets WHERE id = $1 AND account_id = $2",
        None,
    )
}
impl DeleteTicketStmt {
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
        DeleteTicketParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteTicketStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteTicketParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.account_id))
    }
}
pub struct CreateTicketCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_ticket_comment() -> CreateTicketCommentStmt {
    CreateTicketCommentStmt(
        "INSERT INTO ticket_comments (ticket_id, content, account_id, user_id) VALUES ($1, $2, $3, $4) returning id",
        None,
    )
}
impl CreateTicketCommentStmt {
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
        ticket_id: &'a i32,
        content: &'a T1,
        account_id: &'a i32,
        user_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 4> {
        I32Query {
            client,
            params: [ticket_id, content, account_id, user_id],
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
        CreateTicketCommentParams<T1>,
        I32Query<'c, 'a, 's, C, i32, 4>,
        C,
    > for CreateTicketCommentStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateTicketCommentParams<T1>,
    ) -> I32Query<'c, 'a, 's, C, i32, 4> {
        self.bind(
            client,
            &params.ticket_id,
            &params.content,
            &params.account_id,
            &params.user_id,
        )
    }
}
pub struct GetTicketCommentsByTicketIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_ticket_comments_by_ticket_id() -> GetTicketCommentsByTicketIdStmt {
    GetTicketCommentsByTicketIdStmt(
        "SELECT id, ticket_id, content, account_id, user_id FROM ticket_comments WHERE ticket_id = $1 AND account_id = $2 ORDER BY created_at DESC",
        None,
    )
}
impl GetTicketCommentsByTicketIdStmt {
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
        ticket_id: &'a i32,
        account_id: &'a i32,
    ) -> TicketCommentQuery<'c, 'a, 's, C, TicketComment, 2> {
        TicketCommentQuery {
            client,
            params: [ticket_id, account_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TicketCommentBorrowed, tokio_postgres::Error> {
                    Ok(TicketCommentBorrowed {
                        id: row.try_get(0)?,
                        ticket_id: row.try_get(1)?,
                        content: row.try_get(2)?,
                        account_id: row.try_get(3)?,
                        user_id: row.try_get(4)?,
                    })
                },
            mapper: |it| TicketComment::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetTicketCommentsByTicketIdParams,
        TicketCommentQuery<'c, 'a, 's, C, TicketComment, 2>,
        C,
    > for GetTicketCommentsByTicketIdStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetTicketCommentsByTicketIdParams,
    ) -> TicketCommentQuery<'c, 'a, 's, C, TicketComment, 2> {
        self.bind(client, &params.ticket_id, &params.account_id)
    }
}
pub struct EditTicketCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn edit_ticket_comment() -> EditTicketCommentStmt {
    EditTicketCommentStmt(
        "UPDATE ticket_comments SET content = $1 WHERE id = $2 AND account_id = $3",
        None,
    )
}
impl EditTicketCommentStmt {
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
        content: &'a T1,
        id: &'a i32,
        account_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[content, id, account_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EditTicketCommentParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EditTicketCommentStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EditTicketCommentParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.content, &params.id, &params.account_id))
    }
}
pub struct DeleteTicketCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_ticket_comment() -> DeleteTicketCommentStmt {
    DeleteTicketCommentStmt(
        "DELETE FROM ticket_comments WHERE id = $1 AND account_id = $2",
        None,
    )
}
impl DeleteTicketCommentStmt {
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
        DeleteTicketCommentParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteTicketCommentStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteTicketCommentParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.id, &params.account_id))
    }
}
