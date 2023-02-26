// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	modules: [
		'@vite-pwa/nuxt'
	],
	pwa: {

	},
	runtimeConfig: {
		public: {

		},
		location: {
			center: {
				lat: 20.08416,
				lng: -98.74054
			},
		}
	},
})
