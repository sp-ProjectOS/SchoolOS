<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n';
import { PlusIcon } from "lucide-vue-next"

import OrganizerLayout from '@/layouts/OrganizerLayout.vue'
import { Button } from '@/components/ui/button'
import {
	Dialog,
	DialogScrollContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { ScheduledTask, useOrganizerStore } from '@/stores/organizer';
import Task from '@/components/organizer/tasklist/Task.vue'
import { CalendarDateTime, fromDate, getLocalTimeZone } from '@internationalized/date'
import { useToast } from '@/components/ui/toast/use-toast'
import Textarea from '@/components/ui/textarea/Textarea.vue';

const { toast } = useToast()

const { t } = useI18n({
	messages: {
		en: {
			addTask: {
				title: 'Add Task',
				description: 'Add a new task to your list',
				added: 'Task added'
			},
			name: 'Name',
			description: 'Description',
			save: 'Save'
		},
		es: {
			addTask: {
				title: 'Agregar Tarea',
				description: 'Agrega una nueva tarea a tu lista',
				added: 'Tarea añadida'
			},
			name: 'Nombre',
			description: 'Descripción',
			save: 'Guardar'
		}
	}
});

const organizerStore = useOrganizerStore()
const tasks = computed(() => {
	const tasks = organizerStore.orderedTasks
	console.log(tasks)
	return tasks
})

function saveNewTask() {
	const formName = document.getElementById('name') as HTMLInputElement
	const formDescription = document.getElementById('description') as HTMLInputElement
	const newTask: ScheduledTask = {
		id: organizerStore.scheduledTasks.length + 1,
		name: formName.value,
		description: formDescription.value,
		date: fromDate(new Date(), getLocalTimeZone()),
	}
	organizerStore.addTask(newTask)
	toast({
		title: t('addTask.added')
	})

	formName.value = ''
	formDescription.value = ''
}


</script>

<template>
	<OrganizerLayout>
		<div class="h-full min-h-96 h-content flex flex-col space-y-8 p-8">
			<!-- <DataTable :data="tasks" :columns="columns" /> -->
			<Task v-for="task in tasks" :key="task.id" :task="task" />
		</div>
		<div class="fixed bottom-0 right-0 p-8">
			<Dialog>
				<DialogTrigger as-child>
					<Button class="w-12 h-12">
						<PlusIcon />
					</Button>
				</DialogTrigger>
				<DialogScrollContent class="sm:max-w-[425px]">
					<DialogHeader>
						<DialogTitle>
							{{ t('addTask.title') }}
						</DialogTitle>
						<DialogDescription>
							{{ t('addTask.description') }}
						</DialogDescription>
					</DialogHeader>
					<div class="grid gap-4 py-4">
						<div class="grid grid-cols-4 items-center gap-4">
							<Label for="name" class="text-right">
								{{ t('name') }}
							</Label>
							<Input id="name" class="col-span-3" required />
						</div>
						<div class="grid grid-cols-4 items-center gap-4">
							<Label for="description" class="text-right">
								{{ t('description') }}
							</Label>
							<Textarea id="description" class="col-span-3" required style="font-family: Arial, Helvetica, sans-serif"/>
						</div>
					</div>
					<DialogFooter>
						<Button @click="saveNewTask">
							{{ t('save') }}
						</Button>
					</DialogFooter>
				</DialogScrollContent>
			</Dialog>

		</div>
	</OrganizerLayout>
</template>