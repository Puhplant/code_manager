/**
 * Fetch abstraction with automatic authorization token handling
 * Provides methods for GET, POST, and PUT requests with JSON content
 */

export interface RequestOptions {
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

export interface ApiResponse<T = any> {
  data: T;
  status: number;
  statusText: string;
  headers: Headers;
  ok: boolean;
}

// Rust ApiError BadRequest compatible structure
export interface BadRequestError {
  message: string;
  field_errors?: FieldError[];
}

export interface FieldError {
  field: string;
  message: string;
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public statusText: string,
    public response?: Response,
    public badRequest?: BadRequestError
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export class AuthToken {
  constructor(public token: string, public expiresAt: Date) {}
}

// In-memory token storage
let authToken: AuthToken | null = null;
let refreshAuthInterval: NodeJS.Timeout | null = null;

/**
 * Get the authorization token from memory
 */
function getAuthToken(): AuthToken | null {
  return authToken;
}

async function refreshToken(): Promise<void> {
  const response = await fetch('/api/auth/refresh', {
    method: 'POST',
  });

  if (response.ok) {
    const data = await response.json();
    setAuthToken(data.token, new Date(data.expires_at));
  }
}

//Refresh token on page load
let refreshTokenPromise: Promise<void> | null = null;
(() => {
  if(!authToken) {
    refreshTokenPromise = refreshToken();
  }
})();

/**
 * Create headers with authorization token if available
 */
async function createHeaders(customHeaders?: Record<string, string>): Promise<Headers> {
  const headers = new Headers();
  
  // Set default content type for JSON
  headers.set('Content-Type', 'application/json');
  
  // Add authorization token if available
  if(refreshTokenPromise) {
    await refreshTokenPromise;
  }
  const token = getAuthToken();
  if (token) {
    headers.set('Authorization', `Bearer ${token.token}`);
  }
  
  // Add custom headers
  if (customHeaders) {
    Object.entries(customHeaders).forEach(([key, value]) => {
      headers.set(key, value);
    });
  }
  
  return headers;
}

/**
 * Process the response and handle errors
 */
async function processResponse<T>(response: Response): Promise<ApiResponse<T>> {
  if (!response.ok) {
    let errorMessage = `HTTP ${response.status}: ${response.statusText}`;
    let badRequest: BadRequestError | undefined;
    
    // For 400 errors, try to parse as BadRequestError
    if (response.status === 400) {
      try {
        const errorData = await response.json();
        if (errorData.message) {
          errorMessage = errorData.message;
          badRequest = {
            message: errorData.message,
            field_errors: errorData.field_errors
          };
        }
      } catch {
        // If we can't parse error response as JSON, use the default message
      }
    }
    else {
      // For other error statuses, try to get error message
      try {
        const errorData = await response.json();
        if (errorData.message) {
          errorMessage = errorData.message;
        }
      } catch {
        // If we can't parse error response as JSON, use the default message
      }
    }
    
    throw new ApiError(errorMessage, response.status, response.statusText, response, badRequest);
  }
  
  let data: T;
  try {
    data = await response.json();
  } catch {
    // If response is not JSON, try to get text
    const text = await response.text();
    data = text as T;
  }
  
  return {
    data,
    status: response.status,
    statusText: response.statusText,
    headers: response.headers,
    ok: response.ok
  };
}

/**
 * GET request
 */
export async function get<T = any>(
  url: string,
  data?: Record<string, string>,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const headers = await createHeaders(options.headers);
  
  const queryString = data ? new URLSearchParams(data).toString() : '';
  url = queryString ? `${url}?${queryString}` : url;

  const response = await fetch(url, {
    method: 'GET',
    headers,
    signal: options.signal
  });
  
  return processResponse<T>(response);
}

/**
 * POST request
 */
export async function post<T = any>(
  url: string,
  data?: any,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const headers = await createHeaders(options.headers);
  
  const body = data ? JSON.stringify(data) : undefined;
  
  const response = await fetch(url, {
    method: 'POST',
    headers,
    body,
    signal: options.signal
  });
  
  return processResponse<T>(response);
}

/**
 * PUT request
 */
export async function put<T = any>(
  url: string,
  data?: any,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const headers = await createHeaders(options.headers);
  
  const body = data ? JSON.stringify(data) : undefined;
  
  const response = await fetch(url, {
    method: 'PUT',
    headers,
    body,
    signal: options.signal
  });
  
  return processResponse<T>(response);
}

/**
 * DELETE request
 */
export async function del<T = any>(
  url: string,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const headers = await createHeaders(options.headers);
  
  const response = await fetch(url, {
    method: 'DELETE',
    headers,
    signal: options.signal
  });
  
  return processResponse<T>(response);
}

/**
 * PATCH request
 */
export async function patch<T = any>(
  url: string,
  data?: any,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const headers = await createHeaders(options.headers);
  
  const body = data ? JSON.stringify(data) : undefined;
  
  const response = await fetch(url, {
    method: 'PATCH',
    headers,
    body,
    signal: options.signal
  });
  
  return processResponse<T>(response);
}

/**
 * Set the authorization token in memory
 */
export function setAuthToken(token: string, expiresAt: Date): void {
  authToken = new AuthToken(token, expiresAt);
  if (refreshAuthInterval) {
    clearInterval(refreshAuthInterval);
  }
  let nextRefresh = new Date(expiresAt).getTime() - Date.now();
  if (nextRefresh < 120000) {
    nextRefresh = 600000;
  }
  nextRefresh -= 30000;
  refreshAuthInterval = setInterval(refreshToken, nextRefresh);
}

/**
 * Remove the authorization token from memory
 */
export function removeAuthToken(): void {
  authToken = null;
  if (refreshAuthInterval) {
    clearInterval(refreshAuthInterval);
    refreshAuthInterval = null;
  }
}

/**
 * Check if user is authenticated
 */
export function isAuthenticated(): boolean {
  return authToken !== null;
}

// Export the main functions as default
export default {
  get,
  post,
  put,
  delete: del,
  patch,
  setAuthToken,
  removeAuthToken,
  isAuthenticated
};
