import { defineConfig } from 'vite'

const page = name => new URL(`./${name}`, import.meta.url).pathname

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        home: page('index.html'),
        demo: page('demo/index.html'),
        privacy: page('privacy/index.html'),
        terms: page('terms/index.html'),
      },
    },
  },
})
