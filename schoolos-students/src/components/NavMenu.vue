<script setup lang="ts">

import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from '@/components/ui/collapsible'

import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'

import LangSelect from '@/components/LangSelect.vue'
import DarkMode from '@/components/DarkMode.vue'


import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarHeader,
	SidebarInset,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarMenuSub,
	SidebarMenuSubButton,
	SidebarMenuSubItem,
	SidebarProvider,
	SidebarTrigger,
} from '@/components/ui/sidebar'
import { Icon } from '@iconify/vue/dist/iconify.js'



const { t } = useI18n({
	useScope: 'global',
	messages: {
		es: {
			home: 'Inicio',
			organizer: {
				org: 'Organizador',
				scheduling: 'Planificación',
				timer: 'Temporizador',
				tasklist: 'Lista de Tareas',
				agenda: 'Agenda',
			}
		},
		en: {
			home: 'Home',
			organizer: {
				org: 'Organizer',
				scheduling: 'Scheduling',
				timer: 'Timer',
				tasklist: 'Task List',
				agenda: 'Agenda',
			}
		}
	}
})

const navdata = [
	{
		title: computed(() => t('home')),
		isActive: true,
		url: '/',
		icon: "mdi:home",
		items: null
	},
	{
		title: computed(() => t('organizer.org')),
		isActive: true,
		url: '/organizer',
		icon: "mdi:calendar",
		items: [
			{
				title: computed(() => t('organizer.timer')),
				url: '/organizer/timer'
			},
			{
				title: computed(() => t('organizer.tasklist')),
				url: '/organizer/tasklist'
			},
			{
				title: computed(() => t('organizer.agenda')),
				url: '/organizer/agenda'
			},
		]
	}
] as const


</script>

<template>
	<SidebarProvider>
		<Sidebar variant="floating">
			<SidebarHeader>
				<span class="mx-auto text-lg" style="font-family: Carter One;">
					SchoolOS
				</span>
			</SidebarHeader>
			<SidebarContent>
				<SidebarGroup>
					<!-- <SidebarGroupLabel>
						{{ t('home') }}
					</SidebarGroupLabel> -->
					<SidebarMenu>
						<template v-for="item in navdata" :key="item.url">
							<RouterLink :to="item.url" v-if="!item.items"
								class="flex p-2 text-primary font-bold text-center border-primary border-b hover:bg-primary hover:text-secondary justify-start gap-4">
								<Icon :icon="item.icon" class="h-6 w-6" />
								{{ item.title }}
							</RouterLink>
							<Collapsible as-child :default-open="item.isActive" class="group/collapsible" v-else>
								<SidebarMenuItem
									class="flex flex-col p-2 text-primary font-bold text-center border-primary border-b">
									<CollapsibleTrigger as-child>
										<SidebarMenuButton :tooltip="item.title.value">
											<Icon :icon="item.icon" class="h-6 w-6" />
											<span>{{ item.title }}</span>
											<Icon icon="mdi:chevron-right"
												class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
										</SidebarMenuButton>
									</CollapsibleTrigger>
									<CollapsibleContent>
										<SidebarMenuSub>
											<SidebarMenuSubItem v-for="subItem in item.items" :key="subItem.title.value">
												<SidebarMenuSubButton as-child
													class="flex p-2 text-primary font-bold text-center hover:bg-primary hover:text-secondary justify-start gap-4">
													<RouterLink :to="subItem.url">
														<span>{{ subItem.title }}</span>
													</RouterLink>
												</SidebarMenuSubButton>
											</SidebarMenuSubItem>
										</SidebarMenuSub>
									</CollapsibleContent>
								</SidebarMenuItem>
							</Collapsible>
						</template>
					</SidebarMenu>
				</SidebarGroup>
			</SidebarContent>
			<SidebarFooter>
				<div class="mt-auto gap-2 flex">
					<DarkMode />
					<LangSelect />
				</div>
			</SidebarFooter>
		</Sidebar>
		<SidebarInset>
			<div class="fixed flex items-center justify-start gap-2 p-2 h-14">
				<SidebarTrigger />
			</div>
			<!-- <SidebarRail /> -->
			<slot />
		</SidebarInset>


	</SidebarProvider>
	<!-- <Collapsible v-model:open="isOpen" class="absolute top-0 left-0 nav-menu">
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
	</Collapsible> -->
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