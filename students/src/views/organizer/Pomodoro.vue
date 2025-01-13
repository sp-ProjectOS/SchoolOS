<script setup lang="ts">
import { Stepper, StepperDescription, StepperIndicator, StepperItem, StepperTitle, StepperTrigger } from '@/components/ui/stepper'
import { Label } from '@/components/ui/label'
import {
	NumberField,
	NumberFieldContent,
	NumberFieldDecrement,
	NumberFieldIncrement,
	NumberFieldInput,
} from '@/components/ui/number-field'
import { Progress } from '@/components/ui/progress'
import { onMounted, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Clock, Goal, Coffee, Bed, PlayCircle, PauseCircle, SkipForward } from 'lucide-vue-next'

import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import OrganizerLayout from '@/layouts/OrganizerLayout.vue'
import { useOrganizerStore } from '@/stores/organizer';


type PomodoroStep = {
	step: number,
	type: 'pomodoro' | 'shortBreak' | 'longBreak' | 'completed',
}

const { t } = useI18n({
	messages: {
		en: {
			targetTime: 'Target Time (minutes)',
			pomodoro: {
				title: 'Pomodoro',
				description: 'Time to work',
			},
			shortBreak: {
				title: 'Short Break',
				description: 'Time to rest',
			},
			longBreak: {
				title: 'Long Break',
				description: 'Time to rest',
			},
			completed: {
				title: 'Completed',
				description: 'Good job!',
			}
		},
		es: {
			targetTime: 'Tiempo objetivo (minutos)',
			pomodoro: {
				title: 'Pomodoro',
				description: 'Tiempo de trabajar',
			},
			shortBreak: {
				title: 'Descanso corto',
				description: 'Tiempo de descansar',
			},
			longBreak: {
				title: 'Descanso largo',
				description: 'Tiempo de descansar',
			},
			completed: {
				title: 'Meta cumplida',
				description: '¡Buen trabajo!',
			}
		}
	}
});


const steps = ref<PomodoroStep[]>([
	{
		step: 1,
		type: 'completed',
	}
])

function getTypeTime(type: PomodoroStep['type']) {
	if (type === 'pomodoro') {
		return 25 * 60
	} else if (type === 'shortBreak') {
		return 5 * 60
	} else if (type === 'longBreak') {
		return 15 * 60
	} else {
		return 0
	}
}

const targetTimeMinutes = ref(60)
const remainderTimeMinutes = ref(0)
const targetPomodoros = computed(() => {
	let pomodoros = 0
	/* Calculate Pomodoros depending on time */
	const fullSets = Math.floor(targetTimeMinutes.value / 30)
	remainderTimeMinutes.value = targetTimeMinutes.value % 30

	pomodoros = fullSets
	if (remainderTimeMinutes.value > 0) {
		pomodoros++
	}
	return pomodoros
})

const currentStep = ref(1)
const isRunning = ref(false)
const currentRemainingTimeSeconds = ref(25 * 60)

const progressBar = computed(() => {
	const currentType = steps.value[currentStep.value - 1].type
	const totalTime = getTypeTime(currentType)
	if (totalTime === 0) {
		return 100
	}
	return 100 - (currentRemainingTimeSeconds.value / totalTime) * 100
})

const remainingTimeString = computed(() => {
	const minutes = Math.floor(currentRemainingTimeSeconds.value / 60)
	const seconds = currentRemainingTimeSeconds.value % 60

	return `${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
})

function setMaxRemainingTime() {
	/* If last and timeRemainderMinutes, set to that */
	if (remainderTimeMinutes.value > 0 && currentStep.value === steps.value.length - 1) {
		currentRemainingTimeSeconds.value = remainderTimeMinutes.value * 60
		return
	}
	const currentType = steps.value[currentStep.value - 1].type
	currentRemainingTimeSeconds.value = getTypeTime(currentType)
}

let intervalHandle: number | NodeJS.Timeout | undefined = undefined

function count() {
	if (currentRemainingTimeSeconds.value > 0) {
		currentRemainingTimeSeconds.value--
	} else {
		clearInterval(intervalHandle as number)
		intervalHandle = undefined
	}
	if (currentRemainingTimeSeconds.value === 0) {
		nextStep()
	}
}

function startTimer() {
	isRunning.value = true
	if (intervalHandle === undefined) {
		intervalHandle = setInterval(count, 1000)
	}
}

function stopTimer() {
	isRunning.value = false
	if (intervalHandle !== undefined) {
		clearInterval(intervalHandle as number)
		intervalHandle = undefined
	}
}

function toggleTimer() {
	isRunning.value = !isRunning.value

	if (isRunning.value) {
		startTimer()
	} else {
		stopTimer()
	}
}

function nextStep() {
	if (currentStep.value < steps.value.length) {
		currentStep.value++
		setMaxRemainingTime()
	}
	else {
		stopTimer()
	}
}


function configurePomodoro() {
	stopTimer()
	steps.value = []
	console.log("Steps Start", steps.value.length)
	/* Add all steps except last pomodoro */
	for (let i = 0; i < targetPomodoros.value - 1; i++) {
		steps.value.push({ step: steps.value.length + 1, type: 'pomodoro' })
		steps.value.push({ step: steps.value.length + 1, type: 'shortBreak' })
	}
	/* Add last pomodoro without break */
	steps.value.push({ step: steps.value.length + 1, type: 'pomodoro' })
	steps.value.push({ step: steps.value.length + 1, type: 'completed' })

	setMaxRemainingTime()

	console.log("Target Pomodoros", targetPomodoros.value)
	console.log("Steps End", steps.value)
}

onMounted(() => {
	configurePomodoro()
})


</script>

<template>
	<OrganizerLayout>
		<div class="flex flex-row items-center justify-between h-full w-full border gap-2">
			<div class="flex flex-col items-center justify-center h-full w-full border">
				<div class="flex flex-col p-4 h-full w-full items-center justify-between">
					<Progress v-model="progressBar" class="w-3/5" />
				</div>
				<div class="flex flex-col p-4 h-full w-full items-center justify-between">
					<span class="text-6xl">
						{{ remainingTimeString }}
					</span>
				</div>
				<div class="flex flex-row p-4 h-content w-full justify-between">
					<div class="flex flex-row gap-2">
						<Button variant="outline" aria-label="Toggle Timer" @click="toggleTimer" class="w-24 h-24">
							<template v-if="isRunning">
								<PauseCircle class="w-12 h-12" />
							</template>
							<template v-else>
								<PlayCircle class="w-12 h-12" />
							</template>
						</Button>
						<Button variant="outline" aria-label="Skip Timer" @click="nextStep" class="w-24 h-24">
							<SkipForward class="w-12 h-12" />
						</Button>
					</div>
					<div class="flex flex-col-reverse gap-2">
						<NumberField id="targetTime" :default-value="60" :min="0" :model-value="targetTimeMinutes"
							:max="480" @update:model-value="(v) => {
								if (v) {
									targetTimeMinutes = v
									configurePomodoro()
								}
								else {
									targetTimeMinutes = 0
									configurePomodoro()
								}
							}">
							<Label for="targetTime">
								{{ t('targetTime') }}
							</Label>
							<NumberFieldContent>
								<NumberFieldDecrement />
								<NumberFieldInput />
								<NumberFieldIncrement />
							</NumberFieldContent>
						</NumberField>
					</div>
				</div>
			</div>
			<div class="flex flex-col items-center justify-center h-full">
				<Stepper class="flex flex-col gap-4 py-16 justify-center items-center overflow-y-scroll overflow-x-hidden max-h-[90svh]"
					v-model="currentStep">
					<StepperItem v-for="item in steps" :key="item.step" class="basis-1/4 col" :step="item.step">
						<div class="flex flex-row justify-center items-center gap-4">
							<StepperIndicator>
								<template v-if="item.type === 'pomodoro'">
									<Clock class="w-8 h-8" />
								</template>
								<template v-else-if="item.type === 'shortBreak'">
									<Coffee class="w-8 h-8" />
								</template>
								<template v-else-if="item.type === 'longBreak'">
									<Bed class="w-8 h-8" />
								</template>
								<template v-else-if="item.type === 'completed'">
									<Goal class="w-8 h-8" />
								</template>
							</StepperIndicator>
							<div class="flex flex-col justify-center items-center">
								<StepperTitle>
									{{ t(item.type + '.title') }}
								</StepperTitle>
								<StepperDescription>
									{{ t(item.type + '.description') }}
								</StepperDescription>
							</div>
						</div>
						<StepperTrigger>
						</StepperTrigger>
						<!-- <StepperSeparator v-if="item.step !== steps[steps.length - 1].step" class="w-full h-px" /> -->
					</StepperItem>
				</Stepper>
			</div>
		</div>
	</OrganizerLayout>

</template>