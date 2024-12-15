<script setup lang="ts">
import { Card, CardContent, CardTitle, CardHeader } from '@/components/ui/card';


import { useI18n } from 'vue-i18n';
const { t } = useI18n({
	messages: {
		en: {
			welcome: 'Welcome to',
			name: 'SchoolOS',
			description: 'SchoolOS is a platform that helps you manage your school life.',
		},
		es: {
			welcome: 'Bienvenid@ a',
			name: 'SchoolOS',
			description: 'SchoolOS es una plataforma que te ayuda a gestionar tu vida escolar.',
		}
	}
});
</script>


<template>
	<div class="h-screen w-screen flex justify-center items-center">
		<Card>
			<CardHeader>
				<CardTitle>
					{{ t('welcome') }}
					<span style="font-family: Carter One;">{{ t('name') }}</span>
				</CardTitle>

			</CardHeader>
			<CardContent>{{ t('description') }}</CardContent>
		</Card>
	</div>
</template>