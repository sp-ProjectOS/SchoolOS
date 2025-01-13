<script setup lang="ts">
import { Card, CardContent, CardHeader, CardDescription, CardTitle } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { ScheduledTask, useOrganizerStore } from '@/stores/organizer';
import { Button } from '@/components/ui/button';
import { TrashIcon, Check } from 'lucide-vue-next';
import { DateValue } from '@internationalized/date'


const organizerStore = useOrganizerStore();

const props = defineProps<{
	task: ScheduledTask;
}>();

function saveCompleted() {
	console.info('Task completed');
	organizerStore.updateTask(props.task.id, { completed: true });
}

function readableDate(date: DateValue) {
	return `${date.day} / ${date.month} / ${date.year}`;
}

</script>

<template>
	<Card class="w-full min-h-24 h-content">
		<CardHeader class="flex items-center flex-row gap-4 justify-between">
			<div class="flex items-center flex-row gap-4">
				<Checkbox id="completed" v-model="props.task.completed" />
				<CardTitle>{{ props.task.name }}</CardTitle>
				<CardTitle class="text-sm">{{ readableDate(props.task.date) }}</CardTitle>
			</div>
			<div class="flex items-center flex-row gap-4">
				<Button @click="organizerStore.removeTask(props.task.id)">
					<TrashIcon class="w-6 h-6" />
				</Button>
				<Button @click="saveCompleted">
					<Check class="w-6 h-6" />
				</Button>
			</div>
		</CardHeader>
		<CardContent>
			<CardDescription class="text-lg">
				{{ props.task.description }}
			</CardDescription>
		</CardContent>
	</Card>
</template>