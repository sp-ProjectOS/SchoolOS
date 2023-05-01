<script setup>
import { onMounted, onUnmounted } from "vue";
import { RouterView } from "vue-router";
import { useRouter } from "vue-router";
import { listen } from '@tauri-apps/api/event';

const router = useRouter();

let listenerhandle = null;
onMounted(async () => {
	listenerhandle = await listen('navigate_to', (event) => {
		console.log("Navigating to: " + event.payload);
		router.push(event.payload);
	});
});

onUnmounted(() => {
	if (listenerhandle) {
		listenerhandle();
	}
});
</script>

<template>
	<RouterView />
</template>
