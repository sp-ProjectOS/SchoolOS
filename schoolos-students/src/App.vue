<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import { Card } from '@/components/ui/card';

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
	</main>
</template>

<style>
textarea {
	font-family: 'Carter One';
}
</style>
