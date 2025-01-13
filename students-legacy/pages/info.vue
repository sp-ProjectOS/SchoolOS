<template>
	<div class="default">
		<ClientOnly>
			<MapHandler class="map-handler" />
			<template #placeholder>
				<Loading />
			</template>
		</ClientOnly>
		<!-- <OverlayedUI class="overlayed-ui">
			<NavBar />
		</OverlayedUI> -->
		<NavBar />
	</div>
</template>
<style>
:root {
	--nav-height: 3rem;
}

.overlayed-ui {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
}

.map-handler {
	position: absolute;
	width: 100%;
	height: calc(100% - var(--nav-height));
	z-index: 0;
}
</style>