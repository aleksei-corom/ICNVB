import type {
  GalleryItem,
  LoginRequest,
  LoginResponse,
  MonthlyEvent,
  CreateEventInput,
  UpdateEventInput,
} from '../types';
import { APP_CONFIG } from '../config';

const API_BASE = APP_CONFIG.apiBaseUrl;

export interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

export interface ExportResponse {
  success: boolean;
  message: string;
  filename: string;
  content_type: string;
  data: string;
}

/** Guarda el token de autenticación en localStorage */
export function setAuthToken(token: string) {
  localStorage.setItem('icnvb_admin_token', token);
}

/** Obtiene el token guardado */
export function getAuthToken(): string | null {
  return localStorage.getItem('icnvb_admin_token');
}

/** Elimina el token (logout) */
export function removeAuthToken() {
  localStorage.removeItem('icnvb_admin_token');
}

/** Verifica si hay un token válido */
export function isAuthenticated(): boolean {
  return getAuthToken() !== null;
}

const authHeaders = () => {
  const token = getAuthToken();
  return {
    'Content-Type': 'application/json',
    Authorization: token ? `Bearer ${token}` : '',
  };
};

/** Login de administrador */
export async function adminLogin(
  credentials: LoginRequest
): Promise<LoginResponse> {
  const res = await fetch(`${API_BASE}/api/admin/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(credentials),
  });

  if (!res.ok) {
    throw new Error('Credenciales incorrectas');
  }

  const data: LoginResponse = await res.json();
  setAuthToken(data.token);
  return data;
}

/** Obtener todos los eventos (admin) */
export async function getAdminEvents(): Promise<MonthlyEvent[]> {
  const res = await fetch(`${API_BASE}/api/admin/events`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error obteniendo eventos');
  }

  const data = await res.json();
  return data.data || [];
}

/** Crear un nuevo evento */
export async function createEvent(input: CreateEventInput): Promise<MonthlyEvent> {
  const res = await fetch(`${API_BASE}/api/admin/events`, {
    method: 'POST',
    headers: authHeaders(),
    body: JSON.stringify(input),
  });

  if (!res.ok) {
    throw new Error('Error creando evento');
  }

  const data = await res.json();
  return data.data!;
}

/** Actualizar un evento existente */
export async function updateEvent(
  id: string,
  input: UpdateEventInput
): Promise<MonthlyEvent> {
  const res = await fetch(`${API_BASE}/api/admin/events/${id}`, {
    method: 'PUT',
    headers: authHeaders(),
    body: JSON.stringify(input),
  });

  if (!res.ok) {
    throw new Error('Error actualizando evento');
  }

  const data = await res.json();
  return data.data!;
}

/** Eliminar un evento */
export async function deleteEvent(id: string): Promise<void> {
  const res = await fetch(`${API_BASE}/api/admin/events/${id}`, {
    method: 'DELETE',
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error eliminando evento');
  }
}

/** Cambiar contraseña de administrador */
export async function changePassword(
  req: ChangePasswordRequest
): Promise<void> {
  const res = await fetch(`${API_BASE}/api/admin/change-password`, {
    method: 'POST',
    headers: authHeaders(),
    body: JSON.stringify(req),
  });

  if (!res.ok) {
    const errorText = await res.text();
    throw new Error(errorText || 'Error cambiando contraseña');
  }
}

/** Obtener items de la galería */
export async function getGalleryItems(): Promise<GalleryItem[]> {
  const res = await fetch(`${API_BASE}/api/admin/gallery`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error obteniendo galería');
  }

  const data = await res.json();
  return data.data || [];
}

/** Subir archivo a la galería */
export async function uploadGalleryFile(
  formData: FormData
): Promise<GalleryItem> {
  const token = getAuthToken();
  const headers: Record<string, string> = {};
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  const res = await fetch(`${API_BASE}/api/admin/gallery`, {
    method: 'POST',
    headers,
    body: formData,
  });

  if (res.status === 413) {
    throw new Error('El archivo es demasiado grande (máximo 10 MB)');
  }
  if (res.status === 415) {
    throw new Error('Tipo de archivo no soportado. Usa JPG, PNG, WEBP, GIF, MP4, WEBM o MOV');
  }
  if (!res.ok) {
    throw new Error('Error subiendo archivo');
  }

  const data = await res.json();
  return data.data!;
}

/** Eliminar item de la galería */
export async function deleteGalleryItem(id: string): Promise<void> {
  const res = await fetch(`${API_BASE}/api/admin/gallery/${id}`, {
    method: 'DELETE',
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error eliminando elemento de galería');
  }
}

/** Exportar eventos a JSON */
export async function exportEventsJson(): Promise<ExportResponse> {
  const res = await fetch(`${API_BASE}/api/admin/export/events`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error exportando eventos');
  }

  return res.json();
}

/** Exportar eventos a CSV */
export async function exportEventsCsv(): Promise<ExportResponse> {
  const res = await fetch(`${API_BASE}/api/admin/export/events/csv`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error exportando eventos a CSV');
  }

  return res.json();
}

/** Exportar peticiones de oración a JSON */
export async function exportPrayersJson(): Promise<ExportResponse> {
  const res = await fetch(`${API_BASE}/api/admin/export/prayers`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error exportando peticiones');
  }

  return res.json();
}

/** Exportar galería a JSON */
export async function exportGalleryJson(): Promise<ExportResponse> {
  const res = await fetch(`${API_BASE}/api/admin/export/gallery`, {
    headers: authHeaders(),
  });

  if (!res.ok) {
    throw new Error('Error exportando galería');
  }

  return res.json();
}

/** Descargar archivo exportado */
export function downloadExport(data: ExportResponse) {
  const blob = new Blob([data.data], { type: data.content_type });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = data.filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/** Cerrar sesión */
export function logout() {
  removeAuthToken();
  window.location.href = '/admin/login';
}
