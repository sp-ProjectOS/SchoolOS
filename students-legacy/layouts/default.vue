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
	--color-primary: #599cff;
	--color-secondary: #ff6b6b;
	--bg-primary: #f5f5f5;
	--bg-secondary: #ffffff;
	--bg-primary-dark: #1b1b1b;
	--bg-secondary-dark: #2b2b2b;
}
body {
	margin: 0;
}

</style>