<template>
	<div class="default">
		<!-- Default View -->
		<slot />
	</div>
</template>
<style scoped>
.default {
	width: 100vw;
	height: 100svh;
}
</style>
<style>
/**
 * SchoolOS Default Theme
 */
:root {
	--color-primary: #daaeff;
	--color-secondary: #6A2C9E;
	--bg-primary: #500788;
	--bg-secondary: #ffffff;
	--bg-primary-dark: #1b1b1b;
	--bg-secondary-dark: #2b2b2b;
}

body {
	margin: 0;
}
</style>