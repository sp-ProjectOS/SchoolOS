<script setup lang="ts">
import { Button } from '@/components/ui/button'

import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from '@/components/ui/collapsible'
import { LucideAlignJustify } from 'lucide-vue-next'

import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'

import LangSelect from '@/components/LangSelect.vue'
import DarkMode from '@/components/DarkMode.vue'

const isOpen = ref(false)


const { t, locale } = useI18n({
	useScope: 'global',
	messages: {
		es: {
			home: 'Inicio',
			org: "Organizador"
		},
		en: {
			home: 'Home',
			org: "Scheduling"
		}
	}
})
</script>

<template>
	<Collapsible v-model:open="isOpen" class="absolute top-0 left-0 nav-menu">
		<div class="flex items-center justify-start">
			<CollapsibleTrigger as-child>
				<Button variant="ghost" size="sm" class="flex w-12 rounded-br-3xl border p-0 animate-in animate-out"
					v-if="!isOpen">
					<LucideAlignJustify class="h-6 w-6" />
					<span class="sr-only">Toggle</span>
				</Button>
				<Button variant="ghost" size="sm"
					class="flex sm:rounded-br-3xl border pl-3 justify-start sm:w-96 w-screen animate-in animate-out"
					v-else>
					<LucideAlignJustify class="h-6 w-6" />
					<div class="mx-auto text-lg" style="font-family: Carter One;">
						SchoolOS
					</div>
					<span class="sr-only">Toggle</span>
				</Button>
			</CollapsibleTrigger>
		</div>

		<CollapsibleContent class="c-animated border w-screen sm:w-[90%]" :style="{ width: isOpen ? '' : '0' }">
			<nav class="h-[90svh] justify-start flex flex-col p-2 c-nav
			bg-background">
				<RouterLink to="/"
					class="block p-2 text-primary hover:text-secondary font-bold text-center hover:bg-primary border-primary border-b">
					{{ t('home') }}
				</RouterLink>
				<RouterLink to="/organizer"
					class="block p-2 text-primary hover:text-secondary font-bold text-center hover:bg-primary border-primary border-b">
					{{ t('org') }}
				</RouterLink>

				<div class="mt-auto gap-2 flex" v-if="isOpen">
					<DarkMode />
					<LangSelect />
				</div>
			</nav>
		</CollapsibleContent>
	</Collapsible>
</template>
<style scoped>
.nav-menu {
	z-index: 20;
}

.c-animated {
	transition: width 0.3s ease-in-out;
	transition-behavior: smooth;
}

.c-nav {
	overflow-y: auto;
	overflow-x: hidden;
}
</style>