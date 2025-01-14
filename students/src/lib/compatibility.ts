// This function receives a closure and executes it only if the app is running in Tauri.
export async function tauriGuard(closure: Function | Promise<void>, reverse = false) {
	if (isUsingTauri() && !reverse) {
		return closure instanceof Function ? closure() : await closure;
	}
	if (!isUsingTauri() && reverse) {
		return closure instanceof Function ? closure() : await closure;
	}
	console.warn('Not running in Tauri, skipping closure')
}
// This function returns a boolean indicating if the app is running in Tauri.
export function isUsingTauri() {
	// @ts-expect-error
	return window.__TAURI_INTERNALS__ !== undefined
}
// This function returns a boolean indicating if the app is running in development mode.
export function isDevelopment() {
	return import.meta.env.DEV
}