<script setup lang="ts">
import { Button } from '@/components/ui/button'
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
	DialogFooter,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Icon } from '@iconify/vue'
import {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
} from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'

import { ref } from 'vue'
import { DataInstances, DataInstancesN, useSettingsStore } from '@/stores/settings'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { tauriGuard } from '@/lib/compatibility'

const settingsStore = useSettingsStore()

const { instance } = storeToRefs(settingsStore)

const isSaving = ref(false)
async function forceSave() {
	isSaving.value = true
	await tauriGuard(async () => {
		await settingsStore.$tauri.saveAllNow()
		/* Wait a bit */
		await new Promise((resolve) => setTimeout(resolve, 2000))
	})
	isSaving.value = false
}

const { t } = useI18n({
	messages: {
		en: {
			title: 'App Settings',
			description: 'Configure the app behavior and settings.',
			save_disclaimer: 'Settings are saved automatically, however you can force save them.',
			save: 'Save',
			core_instance: {
				title: 'Core Instance',
				description: 'Configure the core instance to use.',
			}
		},
		es: {
			title: 'Configuración de la Aplicación',
			description: 'Configura el comportamiento y ajustes de la aplicación.',
			save_disclaimer: 'Los ajustes se guardan automáticamente, sin embargo puedes forzar su guardado.',
			save: 'Guardar',
			core_instance: {
				title: 'Instancia del Núcleo',
				description: 'Configura la instancia del núcleo a usar.',
			}
		}
	}
});

</script>

<template>
	<Dialog>
		<DialogTrigger as-child>
			<Button variant="outline" class="w-12 h-10">
				<Icon icon="line-md:cog-filled-loop" class="h-12 w-12" width="2em" height="2em" />
				<span class="sr-only">Open Settings</span>
			</Button>
		</DialogTrigger>
		<DialogContent class="sm:max-w-[425px]">
			<DialogHeader>
				<DialogTitle>{{ t('title') }}</DialogTitle>
				<DialogDescription>
					{{ t('description') }}
				</DialogDescription>
			</DialogHeader>
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Icon icon="mdi:server" width="2em" height="2em" />
					<Label for="encoder" class="text-right">
						{{ t('core_instance.title') }}
					</Label>

					<Select v-model:model-value="instance" disabled>
						<SelectTrigger class="w-[180px]">
							<SelectValue placeholder="Select" />
						</SelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectLabel>
									{{ t('core_instance.description') }}
								</SelectLabel>
								<SelectItem v-for="enc of DataInstances" :key="enc" :value="enc">
									{{ DataInstancesN[enc] }}
								</SelectItem>
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>

			</div>
			<DialogFooter class="flex  flex-row items-center text-sm">
				{{ t('save_disclaimer') }}
				<Button class="w-24 ml-auto" @click="forceSave" :disabled="isSaving">
					{{ t('save') }}
					<Icon icon="mdi:content-save" class="w-6 h-6" v-if="!isSaving" />
					<Icon icon="line-md:loading-twotone-loop" class="w-6 h-6 animate-spin" v-else="isSaving" />
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>