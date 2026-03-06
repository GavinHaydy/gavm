export const API_MODE = (import.meta.env.VITE_API_MODE as 'mock' | 'tauri') || 'tauri';

export const isMock = API_MODE === 'mock';
