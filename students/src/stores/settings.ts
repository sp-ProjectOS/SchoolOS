import { defineStore } from "pinia";
import { computed, ComputedRef, Ref, ref } from "vue";

export const DataInstancesN = {
	'localhost': 'LocalOnly',
	"https://devschoolos.our-space.xyz": "Development Server",
	'https://schoolos.our-space.xyz': 'Main Server',
} as const
export type DataInstance = keyof typeof DataInstancesN
export const DataInstances = Object.keys(DataInstancesN) as DataInstance[]

type AppSettings = {
	instance: DataInstance
}

export const useSettingsStore = defineStore('settings',
	() => {
		const instance = ref(DataInstances[0])
		const allSettings: ComputedRef<AppSettings> = computed(() => {
			return {
				instance: instance.value,
			}
		})

		return {
			allSettings,
			instance
		}
	},
	{
		persist: {
			storage: localStorage
		},
		tauri: {
			saveOnChange: true
		},
	}
)