import { defineStore } from "pinia";
import { computed, Ref, ref } from "vue";
import { DateValue, getLocalTimeZone } from "@internationalized/date";


export type ScheduledTask = {
	id: number;
	name: string;
	description: string;
	date: DateValue;

	completed?: boolean;
	estimatedPomodoros?: number;
}

export const useOrganizerStore = defineStore("organizer", () => {
	const scheduledTasks: Ref<ScheduledTask[]> = ref([]);
	const selectedDate = ref<Date | null>(null);
	const selectedTask = ref<ScheduledTask | null>(null);

	const addTask = (task: ScheduledTask) => {
		scheduledTasks.value.push(task);
	};

	const removeTask = (id: number) => {
		const index = scheduledTasks.value.findIndex((task) => task.id === id);
		if (index !== -1) {
			scheduledTasks.value.splice(index, 1);
		}
	};

	const updateTask = (id: number, task: Partial<ScheduledTask>) => {
		console.info("updateTask", id, task);
		const index = scheduledTasks.value.findIndex((task) => task.id === id);
		if (index !== -1) {
			scheduledTasks.value[index] = {
				...scheduledTasks.value[index],
				...task
			};
			console.info("updated task", scheduledTasks.value[index]);
		}
	}

	const tasksFromToDate = (from: Date, to: Date) => computed(() => {
		return scheduledTasks.value.filter((task) => {
			const asDate = task.date.toDate(getLocalTimeZone())
			return asDate >= from && asDate <= to;
		});
	});

	const orderedTasks = computed(() => {
		// Sort tasks by date
		// Move completed tasks to the bottom
		return scheduledTasks.value.sort((a, b) => {
			if (a.date < b.date) {
				return -1;
			} else if (a.date > b.date) {
				return 1;
			} else {
				if (a.completed && !b.completed) {
					return 1;
				} else if (!a.completed && b.completed) {
					return -1;
				} else {
					return 0;
				}
			}
		});
	});

	return {
		scheduledTasks,
		orderedTasks,
		addTask,
		removeTask,
		updateTask,
		tasksFromToDate,
		selectedDate,
		selectedTask,
	};
}, {
	persist: [
		{
			pick: ["scheduledTasks"],
			storage: localStorage
		}
	]
});