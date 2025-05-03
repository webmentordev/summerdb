// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  ssr: true,
  css: ['~/assets/css/tailwind.css'],

  runtimeConfig: {
    api: process.env.NUXT_API_URL
  },

  vite: {
    plugins: [
      tailwindcss(),
    ],
  },

  app: {
    head: {
      title: "SummerDB",
      meta: [
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ],
      link: [
        { rel: 'shortcut icon', href: '/summerdb-1.png' },
        { rel: 'icon', href: '/summerdb-1.png' }
      ],
    }
  },

  modules: ['@nuxt/icon', '@nuxt/ui']
})