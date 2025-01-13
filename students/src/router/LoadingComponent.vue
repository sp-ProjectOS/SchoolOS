<script setup>
import { Icon } from '@iconify/vue';
</script>

<template>
	<div class="h-full w-full">
		<div class="flex flex-col items-center justify-center h-full w-full">
			<Icon icon="mdi:loading" class="h-24 w-24 animate-spin text-primary" />
			<p class="text-primary font-bold text-lg mt-4">
				Loading...
			</p>
		</div>
	</div>
</template>