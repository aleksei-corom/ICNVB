# 📋 Guía Completa: Publicar la Web Gratis y Controlar la Programación desde Google Sheets

Esta guía explica en pasos sencillos cómo tener la página web de la **Iglesia Cristiana Nueva Vida** en internet de forma **100% gratuita** y cómo cualquier hermano o pastor puede modificar la programación desde su celular usando Google Sheets (Excel en Google Drive).

---

## PARTE 1: Cómo Crear la Hoja de Google Sheets para Modificar Eventos

Solo debes hacer este paso **una sola vez**:

1. Abre [Google Sheets (drive.google.com)](https://sheets.google.com) desde tu cuenta de Gmail.
2. Crea una **Hoja de Cálculo nueva en blanco** y nómbrala: **`Programación ICNVB`**.
3. En la **Fila 1 (Cabecera)**, escribe exactamente estos nombres en las columnas:
   * Columna A: `fecha`
   * Columna B: `hora`
   * Columna C: `titulo`
   * Columna D: `categoria`
   * Columna E: `descripcion`
   * Columna F: `cita_biblica`
   * Columna G: `lugar`
   * Columna H: `destacado`

*(O si prefieres, puedes importar directamente el archivo que te dejamos listo en `data/plantilla_programacion_google_sheets.csv` en Archivo > Importar > Subir).*

4. **Publicar la Hoja en la Web (Para que la página pueda leerla):**
   * En el menú superior de Google Sheets, ve a **Archivo** > **Compartir** > **Publicar en la web**.
   * En la ventana que aparece:
     * Deja seleccionado **"Todo el documento"** (o Hoja 1).
     * En lugar de "Página web", cámbialo a **"Valores separados por comas (.csv)"**.
     * Haz clic en el botón azul **Publicar**.
   * Google te dará un enlace que se ve más o menos así:  
     `https://docs.google.com/spreadsheets/d/e/2PACX-1v.../pub?output=csv`
5. Copia ese enlace y pégalo en el archivo [`frontend/src/config.ts`](frontend/src/config.ts) en la línea:
   ```typescript
   googleSheetsCsvUrl: "TU_ENLACE_AQUI_ENTRE_COMILLAS",
   ```

¡Y listo! A partir de ese momento, **cada vez que cualquier persona agregue, edite o borre una fila en esa hoja de Google Sheets (incluso desde la app del celular), la página web mostrará los cambios inmediatamente.**

---

## PARTE 2: Dónde y Cómo Colgar la Página Web 100% Gratis

La mejor plataforma del mundo para alojar esta web de forma gratuita, segura y ultra-rápida es **Vercel** (o **Netlify**). Ambas son gratuitas para siempre para este tipo de sitios.

### Método Rápido con Vercel (Recomendado):

1. **Crear cuenta gratuita:**
   * Entra en [vercel.com](https://vercel.com) y regístrate gratis (puedes registrarte con tu correo de Google o GitHub).
2. **Subir el proyecto:**
   * **Opción A (Arrastrar y Soltar / Despliegue Directo):**
     * En la carpeta `d:\Proyectos\ICNVB\frontend\dist` está la web ya compilada y lista.
     * Puedes desplegarla instalando la herramienta de Vercel en la terminal:
       ```bash
       npm install -g vercel
       cd d:\Proyectos\ICNVB\frontend
       vercel deploy --prod
       ```
   * **Opción B (Conectado a GitHub - La más profesional):**
     * Subes la carpeta del proyecto a una cuenta de GitHub.
     * En el panel de Vercel presionas **"Add New Project"**, seleccionas tu repositorio y Vercel se encarga de todo.
3. **Tu enlace gratis:**
   * Vercel te entregará de inmediato una dirección web como:  
     **`https://icnvb.vercel.app`** (o el nombre que elijas).
   * Tiene certificado de seguridad (el candadito verde **HTTPS**), carga a la velocidad de la luz y nunca se cae.

---

## PARTE 3: Cuando Compren el Dominio Propio Más Adelante

Cuando decidan registrar un dominio propio (por ejemplo: `icnuevavida.org`, `icnvb.com` o `iglesianuevavidabremen.com`):

1. En el panel de Vercel vas a **Settings > Domains**.
2. Escribes tu dominio y Vercel te mostrará 2 registros DNS muy simples para poner en el registrador donde compren el dominio (ej. GoDaddy, Namecheap o DonDominio).
3. En menos de 10 minutos la web ya responderá con el dominio oficial sin pagar nada extra por el hosting.

