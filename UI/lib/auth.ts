import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  appName: string;
  version: string;
  platform: string;
}

export type AuthMode = "signIn" | "signUp";

export interface AuthRequest {
  mode: AuthMode;
  email: string;
  password: string;
  name?: string;
}

export interface UserData {
  created_at: string;
  email: string;
  id: string;
  is_active: boolean;
  is_verified: boolean;
  role: string;
  username: string;
}

export interface AuthData {
  access_token: string;
  expires_in: number;
  refresh_token: string;
  token_type: string;
  user: UserData;
}

export interface ApiResponse {
  data: AuthData;
  success: boolean;
}

export function greet(name: string): Promise<string> {
  return invoke("greet", { name });
}

export function getAppInfo(): Promise<AppInfo> {
  return invoke("get_app_info");
}

export function authenticate(request: AuthRequest): Promise<ApiResponse> {
  return invoke<ApiResponse>("authenticate", { request });
}
