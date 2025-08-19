export interface LoginResponse {
    token: string;
    expires_at: Date;
    user_id: number;
}