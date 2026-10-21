/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** Adresse des Studio-Backends unter Vite (vite.config.ts › define), im Build leer. */
  readonly VITE_STUDIO_API?: string;
}
