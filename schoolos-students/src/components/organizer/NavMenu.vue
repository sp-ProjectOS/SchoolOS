<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import {
	NavigationMenu,
	NavigationMenuContent,
	NavigationMenuItem,
	NavigationMenuLink,
	NavigationMenuList,
	NavigationMenuTrigger,
	navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu'

const { t } = useI18n({
	messages: {
		en: {
			agenda: 'Agenda',
			timer: 'Timer',
			tasklist: 'Task List',
		},
		es: {
			agenda: 'Agenda',
			timer: 'Temporizador',
			tasklist: 'Lista de Tareas',
		}
	}
});
</script>

<template>
	<NavigationMenu>
		<NavigationMenuList>
			<NavigationMenuItem>
				<!-- <NavigationMenuLink href="/organizer/timer" :class="navigationMenuTriggerStyle()">
					{{ t('timer') }}
				</NavigationMenuLink> -->
				<RouterLink to="/organizer/timer" :class="navigationMenuTriggerStyle()">
					{{ t('timer') }}
				</RouterLink>
			</NavigationMenuItem>
			<NavigationMenuItem>
				<!-- <NavigationMenuLink href="/organizer/tasklist" :class="navigationMenuTriggerStyle()">
					{{ t('tasklist') }}
				</NavigationMenuLink> -->
				<RouterLink to="/organizer/tasklist" :class="navigationMenuTriggerStyle()">
					{{ t('tasklist') }}
				</RouterLink>
			</NavigationMenuItem>
			<NavigationMenuItem>
				<!-- <NavigationMenuLink href="/organizer/agenda" :class="navigationMenuTriggerStyle()">
					{{ t('agenda') }}
				</NavigationMenuLink> -->
				<RouterLink to="/organizer/agenda" :class="navigationMenuTriggerStyle()">
					{{ t('agenda') }}
				</RouterLink>
			</NavigationMenuItem>
		</NavigationMenuList>
	</NavigationMenu>
</template>