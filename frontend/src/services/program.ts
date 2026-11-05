import type { MonthlyEvent } from '../types';

/**
 * Genera la lista de meses disponibles a partir del mes actual,
 * cubriendo el mes en curso y los dos siguientes.
 */
export function generateMonths(): { value: string; label: string }[] {
  const months: { value: string; label: string }[] = [];
  const now = new Date();
  const monthNames = [
    'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
    'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre',
  ];

  for (let i = 0; i < 3; i++) {
    const date = new Date(now.getFullYear(), now.getMonth() + i, 1);
    const value = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`;
    const label = `${monthNames[date.getMonth()]} ${date.getFullYear()}`;
    months.push({ value, label });
  }

  return months;
}

export const CATEGORIES = [
  'Todos',
  'Culto General',
  'Jóvenes',
  'Damas',
  'Ayuno',
  'Vigilia',
  'Enseñanza',
];

export const DEFAULT_EVENTS: MonthlyEvent[] = [
  {
    id: 'evt-nov-01',
    title: 'Culto de Celebración y Santa Cena',
    date: '2026-11-01',
    time: '9:00 AM - 11:30 AM',
    category: 'Culto General',
    description: 'Primer domingo del mes: Conmemoramos el sacrificio de Cristo en la cruz con la Santa Cena y acción de gracias comunitaria.',
    scripture: '1 Corintios 11:24-26',
    is_featured: true,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-04',
    title: 'Estudio Bíblico: Fundamentos de la Fe',
    date: '2026-11-04',
    time: '7:00 PM - 9:00 PM',
    category: 'Enseñanza',
    description: 'Serie doctrinal para profundizar en las promesas y mandamientos de Dios para la familia cristiana.',
    scripture: '2 Timoteo 3:16-17',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-05-am',
    title: 'Mañana de Ayuno e Intercesión por las Familias',
    date: '2026-11-05',
    time: '9:00 AM - 12:00 M',
    category: 'Ayuno',
    description: 'Clamor por sanidad de los enfermos, restauración de hogares y cobertura espiritual sobre el corregimiento de Bremen.',
    scripture: 'Esdras 8:23',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-05-pm',
    title: 'Reunión de Damas: Valientes y Esforzadas',
    date: '2026-11-05',
    time: '7:00 PM - 9:00 PM',
    category: 'Damas',
    description: 'Culto especial para mujeres con alabanza y una palabra de fe y ánimo a cargo de la Pastora Martha Escudero.',
    scripture: 'Proverbios 31:25-30',
    is_featured: true,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-06',
    title: 'Encuentro de Jóvenes y Jovencitos: Generación Sin Límites',
    date: '2026-11-06',
    time: '7:00 PM - 9:00 PM',
    category: 'Jóvenes',
    description: 'Alabanza contemporánea, testimonios y mensaje desafiante para la juventud. ¡No te quedes en casa!',
    scripture: '1 Timoteo 4:12',
    is_featured: true,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-08',
    title: 'Culto Dominical de Familia y Amigos',
    date: '2026-11-08',
    time: '9:00 AM - 11:30 AM',
    category: 'Culto General',
    description: 'Trae a tu familia y vecinos a escuchar un mensaje de esperanza y vida eterna.',
    scripture: 'Josué 24:15',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-12-am',
    title: 'Ayuno Congregacional de Sanidad',
    date: '2026-11-12',
    time: '9:00 AM - 12:00 M',
    category: 'Ayuno',
    description: 'Oración unida creyendo en el poder de Dios para sanar cuerpos y sanar almas.',
    scripture: 'Santiago 5:14-15',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-12-pm',
    title: 'Culto de Damas: Edificando el Hogar',
    date: '2026-11-12',
    time: '7:00 PM - 9:00 PM',
    category: 'Damas',
    description: 'Estudio práctico para damas sobre sabiduría bíblica en el hogar.',
    scripture: 'Proverbios 14:1',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-13',
    title: 'Noche de Jóvenes: Pasión por Su Presencia',
    date: '2026-11-13',
    time: '7:00 PM - 9:00 PM',
    category: 'Jóvenes',
    description: 'Tiempo especial de adoración íntima y reconciliación espiritual para jóvenes y adolescentes.',
    scripture: 'Salmos 119:9',
    is_featured: false,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-20',
    title: 'Gran Vigilia Mensual Unida: Clamor por Sucre y Bremen',
    date: '2026-11-20',
    time: '8:00 PM - 1:00 AM',
    category: 'Vigilia',
    description: 'Noche extraordinaria de oración, intercesión continua, predicación y búsqueda intensa del Espíritu Santo.',
    scripture: 'Lucas 6:12; Jeremías 33:3',
    is_featured: true,
    location: 'Templo Central ICNVB - Bremen',
  },
  {
    id: 'evt-nov-29',
    title: 'Culto Especial de Acción de Gracias & Cierre de Mes',
    date: '2026-11-29',
    time: '9:00 AM - 11:30 AM',
    category: 'Culto General',
    description: 'Damos gracias a Dios por sus maravillas durante todo el mes con testimonios y alabanza con toda la congregación.',
    scripture: 'Salmos 100:1-5',
    is_featured: true,
    location: 'Templo Central ICNVB - Bremen',
  },
];

/**
 * Formatea una fecha YYYY-MM-DD en partes legibles (es-CO).
 */
export function formatDateInfo(dateStr: string) {
  const parts = dateStr.split('-');
  const year = parseInt(parts[0], 10);
  const month = parseInt(parts[1], 10) - 1;
  const day = parseInt(parts[2], 10);
  const date = new Date(year, month, day);

  const dayName = date.toLocaleDateString('es-CO', { weekday: 'long' });
  const monthName = date.toLocaleDateString('es-CO', { month: 'short' });

  return {
    dayNumber: String(day).padStart(2, '0'),
    dayName: dayName.toUpperCase(),
    monthName: monthName.toUpperCase(),
  };
}

/**
 * Devuelve las clases de Tailwind para la etiqueta de categoría.
 */
export function getCategoryStyles(category: string) {
  switch (category.toLowerCase()) {
    case 'culto general':
      return 'bg-church-100 text-church-800 border-church-200';
    case 'jóvenes':
      return 'bg-amber-100 text-amber-800 border-amber-200';
    case 'damas':
      return 'bg-pink-100 text-pink-800 border-pink-200';
    case 'ayuno':
      return 'bg-emerald-100 text-emerald-800 border-emerald-200';
    case 'vigilia':
      return 'bg-indigo-900 text-indigo-100 border-indigo-700 font-bold';
    case 'enseñanza':
      return 'bg-sky-100 text-sky-800 border-sky-200';
    default:
      return 'bg-slate-100 text-slate-800 border-slate-200';
  }
}

/**
 * Filtra eventos por mes y categoría.
 */
export function filterEvents(
  events: MonthlyEvent[],
  selectedMonth: string,
  selectedCategory: string
): MonthlyEvent[] {
  return events.filter((e) => {
    const matchesMonth = e.date.startsWith(selectedMonth);
    if (!matchesMonth) return false;

    if (selectedCategory === 'Todos') return true;
    return e.category.toLowerCase() === selectedCategory.toLowerCase();
  });
}
