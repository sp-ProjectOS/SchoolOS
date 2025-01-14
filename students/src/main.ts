import './assets/index.css'
import es from '@/assets/locales/es.json'
import en from '@/assets/locales/en.json'

import { createApp } from "vue";
import { createPinia } from 'pinia'
import { TauriPluginPinia } from 'tauri-plugin-pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { createI18n } from 'vue-i18n'
import router from './router'
import { useLoader } from './router/loader';
import App from './App.vue'
import { useSettingsStore } from "./stores/settings";
import { tauriGuard } from './lib/compatibility';

const pinia = createPinia()
tauriGuard(() => {
	pinia.use(piniaPluginPersistedstate)
	console.info('Using persisted local storage')
}, true)
tauriGuard(() => {
	pinia.use(TauriPluginPinia({
		filterKeysStrategy: 'pick',
	}))
	console.info('Using Tauri Pinia')
})
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

/* Persisted Storage */
tauriGuard(() => {
	useSettingsStore().$tauri.start()
	console.info('Using filesystem persisted storage')
})

app.use(i18n)
app.use(router)
app.mount('#app')

// Loading states for the router
useLoader(router)