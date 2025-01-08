<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Toaster from '@/components/ui/toast/Toaster.vue'
import DarkMode from '@/components/DarkMode.vue';

import NavMenu from '@/components/NavMenu.vue';
import { SidebarProvider } from './components/ui/sidebar';
import { useAppLoadingStore, LoadingComponent } from './router/loader';

const appLoadingStore = useAppLoadingStore();

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
		<NavMenu>
			<RouterView v-show="!appLoadingStore.loading" />
			<LoadingComponent v-show="appLoadingStore.loading" />
		</NavMenu>
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
