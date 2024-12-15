<script setup lang="ts">
import { Calendar } from '@/components/ui/calendar'
import OrganizerLayout from '@/layouts/OrganizerLayout.vue'
import { useOrganizerStore } from '@/stores/organizer';
import { CalendarDate, type DateValue, getLocalTimeZone, today } from '@internationalized/date'
import { type Ref, ref, computed } from 'vue'
import Task from '@/components/organizer/tasklist/Task.vue'

const calendarSelectedDate = computed({
	get: () => {
		if (!organizerStore.selectedDate) {
			return today(getLocalTimeZone())
		}
		return new CalendarDate(
			organizerStore.selectedDate.getFullYear(),
			organizerStore.selectedDate.getMonth() + 1,
			organizerStore.selectedDate.getDate()
		)
	},
	set: (newValue: DateValue) => {
		organizerStore.selectedDate = newValue.toDate(getLocalTimeZone())
	}
})

const organizerStore = useOrganizerStore()
const tasks = computed(() => {
	const tasks = organizerStore.orderedTasks.filter(task => {
		return task.date.day == calendarSelectedDate.value.day &&
			task.date.month == calendarSelectedDate.value.month &&
			task.date.year == calendarSelectedDate.value.year
	})
	console.log(tasks)
	return tasks
})
</script>

<template>
	<OrganizerLayout>
		<div class="flex flex-row items-center justify-between h-full w-full border gap-2">
			<div class="h-full w-full min-h-96 h-content flex flex-col space-y-8 p-8">
				<!-- <DataTable :data="tasks" :columns="columns" /> -->
				<Task v-for="task in tasks" :key="task.id" :task="task" />
			</div>
			<div class="flex flex-col items-center justify-center h-full">
				<Calendar v-model="calendarSelectedDate" class="rounded-md border" />
			</div>
		</div>
	</OrganizerLayout>

</template>