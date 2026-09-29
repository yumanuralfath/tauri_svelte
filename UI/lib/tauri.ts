import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
    appName: string;
    version: string;
    platform: string;
}

export interface AuthRequest {
    mode: "signIn" | "signUp";
    email: string;
    password: string;
    name?: string;
}

export function greet(name: string): Promise<string> {
    return invoke("greet", { name });
}

export function getAppInfo(): Promise<AppInfo> {
    return invoke("get_app_info");
}

export function authenticate(request: AuthRequest): Promise<void> {
    return invoke("authenticate", { request });
}