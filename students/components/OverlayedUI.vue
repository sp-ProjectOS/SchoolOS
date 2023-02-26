<template>
	<div class="overlayed-ui">
		<slot />
	</div>
</template>
<style scoped>
.overlayed-ui {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
}
</style>