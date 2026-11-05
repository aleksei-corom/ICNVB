declare module '*.svelte' {
  import { SvelteComponentDev } from 'svelte';
  export default class SvelteComponentDev extends SvelteComponentDev {
    $$prop_def: Record<string, any>;
    constructor(options: any);
    $destroy(): void;
    $on(event: string, callback: (e: any) => void): () => void;
    $set(props: Record<string, any>): void;
  }
}
