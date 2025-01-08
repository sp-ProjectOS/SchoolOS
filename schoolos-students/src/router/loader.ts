import { defineStore } from "pinia";
import { ref } from "vue";
import { Router } from "vue-router";

import LoadingComponent from "./LoadingComponent.vue";
export { LoadingComponent }

export const useAppLoadingStore = defineStore("apploader", () => {
	const loading = ref(false)
	return {
		loading
	}
})

export function useLoader(router: Router) {

	const appLoadingStore = useAppLoadingStore()
	router.beforeEach((to, from, next) => {
		appLoadingStore.loading = true
		next()
	})

	router.afterEach(() => {
		appLoadingStore.loading = false
	})
}