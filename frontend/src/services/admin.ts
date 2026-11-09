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
  // No establecer Content-Type para FormData (el navegador lo hace con boundary)

  const res = await fetch(`${API_BASE}/api/admin/gallery`, {
    method: 'POST',
    headers,
    body: formData,
  });

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

/** Cerrar sesión */
export function logout() {
  removeAuthToken();
  window.location.href = '/admin/login';
}
