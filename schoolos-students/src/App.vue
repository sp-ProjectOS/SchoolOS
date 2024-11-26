<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import { Card } from '@/components/ui/card';

import DarkMode from '@/components/DarkMode.vue';
import LangSelect from '@/components/LangSelect.vue';


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
	<div class="h-full flex-1 flex-col space-y-8 p-8">
		<div class="flex items-center justify-between space-y-2">
			<div class="flex flex-row items-center justify-center space-x-4 w-full">
				Title
			</div>
			<div class="flex flex-row items-center space-x-2">
				<LangSelect />
				<DarkMode />
			</div>
		</div>
		<div class="flex flex-row justify-evenly">
			Body
		</div>
		<div class="w-full flex flex-row-reverse">
			Footer
		</div>
	</div>
</template>

<style>
textarea {
	font-family: 'Carter One';
}
</style>
