/// <reference types="svelte" />
/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_URL: string;
  readonly VITE_ADSENSE_CLIENT?: string;
  readonly VITE_ADSENSE_PREVIEW?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
