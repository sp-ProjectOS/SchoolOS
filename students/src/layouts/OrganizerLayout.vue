<script setup lang="ts">
import OrganizerNavMenu from '@/components/organizer/NavMenu.vue'
</script>
<template>
	<div class="flex flex-col gap-4 h-full">
		<!-- Navigation -->
		<!-- <div class="flex items-center p-2 border-b justify-end">
			<OrganizerNavMenu />
		</div> -->
		<slot />
	</div>
</template>