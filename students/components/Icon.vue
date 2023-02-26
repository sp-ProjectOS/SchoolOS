<template>
	<i class="material-icons" :style="{ fontSize: size + 'rem', color }">
		{{ name }}
	</i>
</template>
<script setup lang="ts">
defineProps<{
	name: string;
	size: number;
	color: string;
}>();

</script>
<style>
@import url('https://fonts.googleapis.com/css2?family=Material+Icons&display=swap');
</style>