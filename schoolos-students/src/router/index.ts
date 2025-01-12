import { createRouter, createWebHistory } from 'vue-router'

const routes = [
	{ path: '/', component: () => import('@/views/Home.vue') },
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