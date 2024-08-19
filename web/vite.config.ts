import { sveltekit } from '@sveltejs/kit/vite';
import { readFileSync } from 'fs';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

const hexPlugin = () => {
  return {
    name: 'hex',
    transform: (_: string, id: string) => {
      const [path, query] = id.split('?');
      if (query != 'hex') return null;
      const data = readFileSync(path).toString('hex');
      return `export default '${data}';`;
    },
  };
};

export default defineConfig({
  plugins: [sveltekit(), wasmPack('./neones-web'), hexPlugin()],
});
