<template>
	<div class="search-container">
		<div class="search">
			<div class="select">
			</div>
		</div>
	</div>
</template>
<style scoped>
.search-container {
	display: flex;
	width: 100%;
	justify-content: center;
}

.search {
	width: 98%;
	height: 98%;
	position: absolute;
	background-color: var(--bg-primary);
	border-radius: 1rem 1rem 0 0;
}

.select {
	padding: 3rem;
	background-color: rgb(255, 255, 255);
	margin: 2rem;
	border-radius: 1rem;
}
</style>