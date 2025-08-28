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

export interface ColumnTickets {
  column_id: number;
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