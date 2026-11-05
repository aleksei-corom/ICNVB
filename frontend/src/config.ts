// Configuración de la Iglesia y Google Sheets
export const APP_CONFIG = {
  churchName: "Iglesia Cristiana Nueva Vida",
  shortName: "ICNVB",
  location: "Corregimiento Bremen, Sincelejo, Sucre, Colombia",
  pastors: [
    {
      name: "Pastor Benjamín Ramos",
      phone: "+57 311 327 4403",
      rawPhone: "573113274403",
    },
    {
      name: "Pastora Martha Escudero",
      phone: "+57 313 509 4547",
      rawPhone: "573135094547",
    }
  ],

  /**
   * ENLACE DE GOOGLE SHEETS:
   * Para conectar tu propia hoja de Google:
   * 1. Crea una hoja en Google Sheets con las columnas:
   *    fecha | hora | titulo | categoria | descripcion | cita_biblica | lugar | destacado
   * 2. Ve a Archivo > Compartir > Publicar en la web.
   * 3. Elige formato "Valores separados por comas (.csv)" y copia el enlace.
   * 4. Pega aquí el enlace en googleSheetsCsvUrl.
   */
  googleSheetsCsvUrl: "https://docs.google.com/spreadsheets/d/e/2PACX-1vSLxPlCE-ilJz1rmjLpjbVjBZ0H2b2l6Hu-8qgqoYQU3_xnCoRYH-DpRnvG56Jza6RIkNEZGhECugke/pub?output=csv", // Si está vacío, usa los eventos locales predeterminados

  /** Enlaces de contacto pastoral centralizados */
  contact: {
    pastors: [
      {
        name: "Pastor Benjamín Ramos",
        phone: "+57 311 327 4403",
        rawPhone: "573113274403",
        role: "Pastor Principal",
      },
      {
        name: "Pastora Martha Escudero",
        phone: "+57 313 509 4547",
        rawPhone: "573135094547",
        role: "Pastora & Líder de Ministerio Femenino",
      },
    ],
    whatsapp: {
      benjamin: "573113274403",
      martha: "573135094547",
    },
    phone: {
      benjamin: "+57 311 327 4403",
      martha: "+57 313 509 4547",
    },
  },

  /**
   * Construye un enlace de WhatsApp con mensaje opcional.
   * Ej: waLink("573113274403", "Hola Pastor")
   */
  waLink(rawPhone: string, text?: string): string {
    const base = `https://wa.me/${rawPhone}`;
    return text ? `${base}?text=${encodeURIComponent(text)}` : base;
  },

  /**
   * Construye un enlace de llamada telefónica.
   */
  telLink(phone: string): string {
    return `tel:${phone.replace(/[^0-9+]/g, "")}`;
  },
};

