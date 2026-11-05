import type { MonthlyEvent } from '../types';

/**
 * Parsea una línea de texto CSV respetando comillas
 */
function parseCsvLine(line: string): string[] {
  const result: string[] = [];
  let current = '';
  let inQuotes = false;

  for (let i = 0; i < line.length; i++) {
    const char = line[i];
    if (char === '"') {
      if (inQuotes && line[i + 1] === '"') {
        current += '"';
        i++;
      } else {
        inQuotes = !inQuotes;
      }
    } else if (char === ',' && !inQuotes) {
      result.push(current.trim());
      current = '';
    } else {
      current += char;
    }
  }
  result.push(current.trim());
  return result;
}

/**
 * Normaliza fechas tipo DD/MM/YYYY o YYYY-MM-DD a YYYY-MM-DD
 */
function normalizeDate(rawDate: string): string {
  const cleaned = rawDate.trim();
  if (cleaned.includes('/')) {
    const parts = cleaned.split('/');
    if (parts.length === 3) {
      if (parts[2].length === 4) {
        // DD/MM/YYYY -> YYYY-MM-DD
        const day = parts[0].padStart(2, '0');
        const month = parts[1].padStart(2, '0');
        const year = parts[2];
        return `${year}-${month}-${day}`;
      }
    }
  }
  return cleaned;
}

/**
 * Descarga y transforma los datos desde la URL pública de Google Sheets en formato CSV
 */
export async function fetchEventsFromGoogleSheets(csvUrl: string): Promise<MonthlyEvent[]> {
  if (!csvUrl || !csvUrl.trim()) {
    return [];
  }

  // Agregamos un timestamp para evitar que el navegador guarde en caché eventos viejos
  const cacheBusterUrl = csvUrl.includes('?') 
    ? `${csvUrl}&_t=${Date.now()}` 
    : `${csvUrl}?_t=${Date.now()}`;

  const response = await fetch(cacheBusterUrl);
  if (!response.ok) {
    throw new Error(`Error al conectar con Google Sheets: ${response.statusText}`);
  }

  const csvText = await response.text();
  const lines = csvText.split(/\r?\n/).filter(line => line.trim().length > 0);

  if (lines.length <= 1) {
    return [];
  }

  // Cabeceras esperadas:
  // fecha, hora, titulo, categoria, descripcion, cita_biblica, lugar, destacado, predicador, director_alabanza
  const headers = parseCsvLine(lines[0]).map(h => h.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, ""));
  
  const getCol = (cols: string[], name: string): string => {
    const idx = headers.findIndex(h => h.includes(name));
    return idx !== -1 && cols[idx] ? cols[idx] : '';
  };

  const events: MonthlyEvent[] = [];

  for (let i = 1; i < lines.length; i++) {
    const cols = parseCsvLine(lines[i]);
    const fechaRaw = getCol(cols, 'fecha') || cols[0];
    const hora = getCol(cols, 'hora') || cols[1] || '7:00 PM';
    const titulo = getCol(cols, 'titulo') || cols[2] || 'Culto Congregacional';

    if (!fechaRaw || !titulo) continue;

    const date = normalizeDate(fechaRaw);
    const categoria = getCol(cols, 'categoria') || cols[3] || 'Culto General';
    const descripcion = getCol(cols, 'descripcion') || cols[4] || '';
    const cita = getCol(cols, 'cita') || getCol(cols, 'biblica') || cols[5] || '';
    const lugar = getCol(cols, 'lugar') || cols[6] || 'Templo Central ICNVB - Bremen';
    const destacadoRaw = (getCol(cols, 'destacado') || cols[7] || '').toUpperCase();
    const isFeatured = destacadoRaw === 'SI' || destacadoRaw === 'SÍ' || destacadoRaw === 'TRUE' || destacadoRaw === '1';
    const predicador = getCol(cols, 'predicador') || cols[8] || undefined;
    const directorAlabanza = getCol(cols, 'director') || getCol(cols, 'alabanza') || cols[9] || undefined;

    events.push({
      id: `evt-gsheet-${i}-${date}`,
      title: titulo,
      date,
      time: hora,
      category: categoria,
      description: descripcion,
      scripture: cita ? cita : undefined,
      predicador: predicador ? predicador : undefined,
      directorAlabanza: directorAlabanza ? directorAlabanza : undefined,
      is_featured: isFeatured,
      location: lugar,
    });
  }

  // Ordenar cronológicamente
  events.sort((a, b) => a.date.localeCompare(b.date));

  return events;
}

