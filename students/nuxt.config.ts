// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	modules: [
		// '@sidebase/nuxt-auth',
	],
	runtimeConfig: {
		public: {
			oauthUrl: "https://oauth2.our-space.xyz",
		},
		location: {
			center: {
				lat: 20.08416,
				lng: -98.74054
			},
		}
	},
})
