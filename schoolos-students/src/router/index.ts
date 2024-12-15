import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '@/views/Home.vue'


const routes = [
	{ path: '/', component: HomeView },
	{ path: '/organizer', component: () => import('@/views/organizer/Agenda.vue') },
	{ path: '/organizer/agenda', component: () => import('@/views/organizer/Agenda.vue') },
	{ path: '/organizer/timer', component: () => import('@/views/organizer/Pomodoro.vue') },
	{ path: '/organizer/tasklist', component: () => import('@/views/organizer/TaskList.vue') },
	{ path: '/:pathMatch(.*)*', redirect: '/' }
]

const router = createRouter({
	
	history: createWebHistory(),
	routes,
})

export default router