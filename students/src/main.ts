import './assets/index.css'
import es from '@/assets/locales/es.json'
import en from '@/assets/locales/en.json'

import { createApp } from "vue";
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { createI18n } from 'vue-i18n'
import router from './router'
import { useLoader } from './router/loader';
import App from './App.vue'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
export const i18n = createI18n({
	legacy: false,
	locale: 'es',
	fallbackLocale: 'en',
	messages: {
		es,
		en
	}
})
const app = createApp(App)

app.use(pinia)
app.use(i18n)
app.use(router)
app.mount('#app')


// Loading states for the router

useLoader(router)