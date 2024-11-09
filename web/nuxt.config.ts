// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-11-02',
    modules: ['@primevue/nuxt-module', '@nuxtjs/i18n', '@pinia/nuxt'],
    css: ['~/assets/scss/style.scss'],
    postcss: {
        plugins: {
            'postcss-import': {},
            tailwindcss: {},
            autoprefixer: {},
            ...(process.env.NODE_ENV === 'production' ? {cssnano: {}} : {})
        },
    },
    primevue: {
        options: {
            theme: 'none',
            ripple: true,
        }
    },
    i18n: {
        lazy: true,
        locales: [{
            code: 'es',
            language: 'es-ES',
            name: 'Español',
            files: ['common.ts', 'es-ES.ts'],
        }],
        defaultLocale: 'es',
        detectBrowserLanguage: {
            useCookie: true,
            cookieKey: 'i18n_redirected',
            redirectOn: 'root',
        },
        restructureDir: 'lang',
        langDir: '.',
        //baseUrl: 'https://my-nuxt-app.com'
    },
    imports: {
        dirs: [
            'composables',
            'composables/*/*.ts',
            //'composables/**'
        ]
    },

    devtools: {enabled: true},
    telemetry: false,
    ssr: false,
    devServer: {
        host: process.env.TAURI_DEV_HOST || 'localhost'
    },
    vite: {
        // Better support for Tauri CLI output
        clearScreen: false,
        // Enable environment variables
        // Additional environment variables can be found at
        // https://v2.tauri.app/reference/environment-variables/
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            // Tauri requires a consistent port
            strictPort: true
        },
    },
})