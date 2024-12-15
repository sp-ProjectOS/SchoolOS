<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Toaster from '@/components/ui/toast/Toaster.vue'
import DarkMode from '@/components/DarkMode.vue';

import NavMenu from '@/components/NavMenu.vue';

const { t } = useI18n({
	messages: {
		en: {

		},
		es: {

		}
	}
});

</script>

<template>
	<main class="h-screen w-screen">
		<NavMenu />
		<RouterView />

		<!-- Invisible, load without click -->
		<div class="hidden">
			<DarkMode />
		</div>
		<Toaster />
	</main>
</template>

<style>
textarea {
	font-family: 'Carter One';
}
</style>
