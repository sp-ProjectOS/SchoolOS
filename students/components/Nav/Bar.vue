<template>
	<div class="bottom-nav">
		<div class="bottom-nav__item">
			<NuxtLink :to="link.to" v-for="link in links" :key="link.name">
				<Icon :name="link.icon.name" :size="link.icon.size" :color="link.icon.color" />
			</NuxtLink>
		</div>
	</div>
</template>
<script lang="ts" setup>
const links = [
	{
		name: 'Home',
		icon: {
			name: 'home',
			size: 2,
			color: 'var(--color-secondary)',
		},
		to: '/',
	},
];
</script>
<style scoped>
.bottom-nav {
	position: fixed;
	bottom: 0;
	left: 0;
	width: 100%;
	height: 3rem;
	background-color: var(--bg-primary);
	z-index: 1;
}
.bottom-nav__item {
	display: flex;
	align-items: center;
	justify-content: space-around;
	width: 100%;
	height: 100%;
}
.bottom-nav__item a {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	width: 100%;
	height: 100%;
	color: var(--color-primary);
}
</style>