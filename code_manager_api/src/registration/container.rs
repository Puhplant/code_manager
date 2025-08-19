use std::{sync::Arc, sync::OnceLock};
use clorinde::deadpool_postgres::Pool;

use crate::services::auth::{jwt_auth::JwtAuth, login_service::{ILoginService, LoginService}, refresh_token_creator::{IRefreshTokenCreator, RefreshTokenCreator}, registration_service::{IRegistrationService, RegistrationService}, user_creator::{IUserCreator, UserCreator}, user_email_getter::{IUserEmailGetter, UserEmailGetter}};
use crate::services::ticket::ticket_creation_service::{ITicketCreationService, TicketCreationService};
use crate::services::ticket::ticket_retrieval_service::{ITicketRetrievalService, TicketRetrievalService};
use crate::services::ticket::ticket_board_service::{ITicketBoardService, TicketBoardService};
use crate::services::ticket::ticket_editing_service::{ITicketEditingService, TicketEditingService};

pub struct Scope {
    container: Container,
    login_service: OnceLock<Arc<dyn ILoginService>>,
    user_email_getter: OnceLock<Arc<dyn IUserEmailGetter>>,
    refresh_token_creator: OnceLock<Arc<dyn IRefreshTokenCreator>>,
    registration_service: OnceLock<Arc<dyn IRegistrationService>>,
    user_creator: OnceLock<Arc<dyn IUserCreator>>,
    ticket_service: OnceLock<Arc<dyn ITicketCreationService>>,
    ticket_retrieval_service: OnceLock<Arc<dyn ITicketRetrievalService>>,
    ticket_board_service: OnceLock<Arc<dyn ITicketBoardService>>,
    ticket_editing_service: OnceLock<Arc<dyn ITicketEditingService>>,
}

impl Scope {
    fn create_login_service(&self) -> Arc<dyn ILoginService> {
        Arc::new(LoginService {
            jwt_auth: self.container.jwt_auth.clone(),
            user_email_getter: self.get_user_email_getter(),
            refresh_token_creator: self.get_refresh_token_creator(),
        })
    }

    pub fn get_login_service(&self) -> Arc<dyn ILoginService> {
        self.login_service.get_or_init(|| self.create_login_service()).clone()
    }

    fn create_user_email_getter(&self) -> Arc<dyn IUserEmailGetter> {
        Arc::new(UserEmailGetter {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_user_email_getter(&self) -> Arc<dyn IUserEmailGetter> {
        self.user_email_getter.get_or_init(|| self.create_user_email_getter()).clone()
    }

    fn create_refresh_token_creator(&self) -> Arc<dyn IRefreshTokenCreator> {
        Arc::new(RefreshTokenCreator {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_refresh_token_creator(&self) -> Arc<dyn IRefreshTokenCreator> {
        self.refresh_token_creator.get_or_init(|| self.create_refresh_token_creator()).clone()
    }

    fn create_registration_service(&self) -> Arc<dyn IRegistrationService> {
        Arc::new(RegistrationService {
            user_email_getter: self.get_user_email_getter(),
            user_creator: self.get_user_creator(),
        })
    }

    pub fn get_registration_service(&self) -> Arc<dyn IRegistrationService> {
        self.registration_service.get_or_init(|| self.create_registration_service()).clone()
    }

    fn create_user_creator(&self) -> Arc<dyn IUserCreator> {
        Arc::new(UserCreator {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_user_creator(&self) -> Arc<dyn IUserCreator> {
        self.user_creator.get_or_init(|| self.create_user_creator()).clone()
    }

    fn create_ticket_service(&self) -> Arc<dyn ITicketCreationService> {
        Arc::new(TicketCreationService {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_ticket_service(&self) -> Arc<dyn ITicketCreationService> {
        self.ticket_service.get_or_init(|| self.create_ticket_service()).clone()
    }

    fn create_ticket_retrieval_service(&self) -> Arc<dyn ITicketRetrievalService> {
        Arc::new(TicketRetrievalService {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_ticket_retrieval_service(&self) -> Arc<dyn ITicketRetrievalService> {
        self.ticket_retrieval_service.get_or_init(|| self.create_ticket_retrieval_service()).clone()
    }

    fn create_ticket_board_service(&self) -> Arc<dyn ITicketBoardService> {
        Arc::new(TicketBoardService {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_ticket_board_service(&self) -> Arc<dyn ITicketBoardService> {
        self.ticket_board_service.get_or_init(|| self.create_ticket_board_service()).clone()
    }

    fn create_ticket_editing_service(&self) -> Arc<dyn ITicketEditingService> {
        Arc::new(TicketEditingService {
            pool: self.container.pool.clone(),
        })
    }

    pub fn get_ticket_editing_service(&self) -> Arc<dyn ITicketEditingService> {
        self.ticket_editing_service.get_or_init(|| self.create_ticket_editing_service()).clone()
    }
}

#[derive(Clone)]
pub struct Container {
    pub pool: Pool,
    pub jwt_auth: Arc<JwtAuth>,
}

impl Container {
    pub fn new(pool: Pool, jwt_auth: JwtAuth) -> Self {
        Self { pool, jwt_auth: Arc::new(jwt_auth) }
    }

    pub fn create_scope(&self) -> Scope {
        Scope {
            container: self.clone(),
            login_service: OnceLock::new(),
            user_email_getter: OnceLock::new(),
            refresh_token_creator: OnceLock::new(),
            registration_service: OnceLock::new(),
            user_creator: OnceLock::new(),
            ticket_service: OnceLock::new(),
            ticket_retrieval_service: OnceLock::new(),
            ticket_board_service: OnceLock::new(),
            ticket_editing_service: OnceLock::new(),
        }
    }
}

