export interface Board {
  id: number;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

export interface MinTicketResponse {
  id: number;
  title: string;
  column_id: number | null;
  position: number | null;
  account_id: number;
  user_id: number;
}

export interface ColumnResponse {
  id: number;
  name: string;
}

export interface ColumnTickets {
  column: ColumnResponse;
  tickets: MinTicketResponse[];
}

export interface BoardTicketsResponse {
  columns: ColumnTickets[];
}

export interface BacklogTicketsResponse {
  tickets: MinTicketResponse[];
}

export interface BoardResponse {
  boards: Board[];
}

export type GetTicketsByBoardIdResponse = BoardTicketsResponse | BacklogTicketsResponse;

export interface CreateTicketRequest {
  title: string;
  description: string;
  column_id?: number;
  position?: number;
  board_id: number;
}

export interface CreateTicketResponse {
  id: number;
}

export interface EditTicketRequest {
  title: string;
  description: string;
  column_id?: number;
}

export interface GetTicketResponse {
  id: number;
  title: string;
  description: string;
  column_id: number | null;
  position: number | null;
  account_id: number;
  user_id: number;
} 