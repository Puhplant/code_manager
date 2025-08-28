use serde::Deserialize;

use crate::{middleware::validation::RequestValidator, models::api_error::{ApiError, BadRequestError, FieldError}};

pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
    pub column_id: Option<i32>,
    pub position: Option<f64>, 
    pub board_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTicketValidation {
    title: Option<String>,
    description: Option<String>,
    column_id: Option<i32>,
    position: Option<f64>,
    board_id: Option<i32>,
}

impl RequestValidator for CreateTicketValidation {
    type RequestOutput = CreateTicketRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {
        let title = self.title.ok_or(FieldError {
            field: "title",
            message: "Title is required".to_string(),
        }).and_then(|title| {
            if title.is_empty() {
                Err(FieldError {
                    field: "title",
                    message: "Title is required".to_string(),
                })
            } else {
                Ok(title)
            }
        });

        let description = self.description.ok_or(FieldError {
            field: "description",
            message: "Description is required".to_string(),
        }).and_then(|description| {
            if description.is_empty() {
                Err(FieldError {
                    field: "description",
                    message: "Description is required".to_string(),
                })
            } else {
                Ok(description)
            }
        });

        let board_id = self.board_id.ok_or(FieldError {
            field: "board_id",
            message: "Board ID is required".to_string(),
        }).and_then(|board_id| {
            if board_id <= 0 {
                Err(FieldError {
                    field: "board_id",
                    message: "Board ID must be greater than 0".to_string(),
                })
            } else {
                Ok(board_id)
            }
        });

        let column_id = if let Some(column_id) = self.column_id {
            if column_id <= 0 {
                Err(FieldError {
                    field: "column_id",
                    message: "Column ID must be greater than 0".to_string(),
                })
            } else {
                Ok(Some(column_id))
            }
        } else {
            Ok(None)
        };

        let position = if let Some(position) = self.position {
            if position < 0.0 {
                Err(FieldError {
                    field: "position",
                    message: "Position must be non-negative".to_string(),
                })
            } else {
                Ok(Some(position))
            }
        } else {
            Ok(None)
        };

        match (title, description, column_id, position, board_id) {
            (Ok(title), Ok(description), Ok(column_id), Ok(position), Ok(board_id)) => {
                if column_id.is_some() && position.is_none() {
                    return Err(ApiError::BadRequest(BadRequestError {
                        message: "Position is required when column_id is provided".to_string(),
                        field_errors: Some(vec![FieldError {
                            field: "position",
                            message: "Position is required when column_id is provided".to_string(),
                        }]),
                    }));
                }

                 Ok(CreateTicketRequest {
                title,
                description,
                column_id,
                position,
                board_id,
            })},
            (title, description, column_id, position, board_id) => {
                let mut validation_errors = Vec::new();
                _ = title.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = description.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = column_id.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = position.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = board_id.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
} 

#[derive(Deserialize)]
pub struct GetTicketsByBoardIdValidation {
    board_id: Option<i32>,
    ticket_type: Option<String>,
}

impl RequestValidator for GetTicketsByBoardIdValidation {
    type RequestOutput = GetTicketsByBoardIdRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {
        let board_id = self.board_id.ok_or(FieldError {
            field: "board_id",
            message: "Board ID is required".to_string(),
        }).and_then(|board_id| {
            if board_id <= 0 {
                Err(FieldError {
                    field: "board_id",
                    message: "Board ID must be greater than 0".to_string(),
                })
            } else {
                Ok(board_id)
            }
        });

        let ticket_type = self.ticket_type.unwrap_or_else(|| "board".to_string());
        let ticket_type = if ticket_type == "board" || ticket_type == "backlog" {
            Ok(ticket_type)
        } else {
            Err(FieldError {
                field: "ticket_type",
                message: "Ticket type must be either 'board' or 'backlog'".to_string(),
            })
        };

        match (board_id, ticket_type) {
            (Ok(board_id), Ok(ticket_type)) => Ok(GetTicketsByBoardIdRequest {
                board_id,
                ticket_type,
            }),
            (board_id, ticket_type) => {
                let mut validation_errors = Vec::new();
                if let Err(e) = board_id {
                    validation_errors.push(e);
                }
                if let Err(e) = ticket_type {
                    validation_errors.push(e);
                }
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
}

pub struct GetTicketsByBoardIdRequest {
    pub board_id: i32,
    pub ticket_type: String,
}

#[derive(Deserialize)]
pub struct EditTicketValidation {
    title: Option<String>,
    description: Option<String>,
    column_id: Option<i32>,
}

impl RequestValidator for EditTicketValidation {
    type RequestOutput = EditTicketRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {
        let title = self.title.ok_or(FieldError {
            field: "title",
            message: "Title is required".to_string(),
        }).and_then(|title| {
            if title.is_empty() {
                Err(FieldError {
                    field: "title",
                    message: "Title is required".to_string(),
                })
            } else {
                Ok(title)
            }
        });

        let description = self.description.ok_or(FieldError {
            field: "description",
            message: "Description is required".to_string(),
        }).and_then(|description| {
            if description.is_empty() {
                Err(FieldError {
                    field: "description",
                    message: "Description is required".to_string(),
                })
            } else {
                Ok(description)
            }
        });

        let column_id = if let Some(column_id) = self.column_id {
            if column_id <= 0 {
                Err(FieldError {
                    field: "column_id",
                    message: "Column ID must be greater than 0".to_string(),
                })
            } else {
                Ok(Some(column_id))
            }
        } else {
            Ok(None)
        };

        match (title, description, column_id) {
            (Ok(title), Ok(description), Ok(column_id)) => Ok(EditTicketRequest {
                title,
                description,
                column_id,
            }),
            (title, description, column_id) => {
                let mut validation_errors = Vec::new();
                if let Err(e) = title {
                    validation_errors.push(e);
                }
                if let Err(e) = description {
                    validation_errors.push(e);
                }
                if let Err(e) = column_id {
                    validation_errors.push(e);
                }
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
}

pub struct EditTicketRequest {
    pub title: String,
    pub description: String,
    pub column_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct MoveTicketValidation {
    column_id: Option<i32>,
    position: Option<f64>,
}

impl RequestValidator for MoveTicketValidation {
    type RequestOutput = MoveTicketRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {
        let column_id = if let Some(column_id) = self.column_id {
            if column_id <= 0 {
                Err(FieldError {
                    field: "column_id",
                    message: "Column ID must be greater than 0".to_string(),
                })
            } else {
                Ok(Some(column_id))
            }
        } else {
            Ok(None)
        };

        let position = if let Some(position) = self.position {
            if position < 0.0 {
                Err(FieldError {
                    field: "position",
                    message: "Position must be non-negative".to_string(),
                })
            } else {
                Ok(Some(position))
            }
        } else {
            Ok(None)
        };

        match (column_id, position) {
            (Ok(Some(column_id)), Ok(Some(position))) => Ok(MoveTicketRequest {
                position: Some(position),
                column_id: Some(column_id),
            }),
            (Ok(None), Ok(None)) => Ok(MoveTicketRequest {
                position: None,
                column_id: None,
            }),
            (Ok(Some(_)), Ok(None)) => Err(ApiError::BadRequest(BadRequestError {
                message: "Position is required when column_id is provided".to_string(),
                field_errors: Some(vec![FieldError {
                    field: "position",
                    message: "Position is required when column_id is provided".to_string(),
                }]),
            })),
            (Ok(None), Ok(Some(_))) => Err(ApiError::BadRequest(BadRequestError {
                message: "Column ID is required when position is provided".to_string(),
                field_errors: Some(vec![FieldError {
                    field: "column_id",
                    message: "Column ID is required when position is provided".to_string(),
                }]),
            })),
            (column_id, position) => {
                let mut validation_errors = Vec::new();
                if let Err(e) = column_id {
                    validation_errors.push(e);
                }
                if let Err(e) = position {
                    validation_errors.push(e);
                }
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
}

pub struct MoveTicketRequest {
    pub position: Option<f64>,
    pub column_id: Option<i32>,
} 