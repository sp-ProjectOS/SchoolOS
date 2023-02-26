<template>
	<div>
		<MapView @mapClicked="mapClicked" @positionUpdated="positionUpdated" />
	</div>
</template>
<script setup lang="ts">
import { LatLngTuple, LeafletMouseEvent } from 'leaflet';
const userPosition = ref<LatLngTuple | null>([0, 0])
function mapClicked(e: LeafletMouseEvent) {
	console.log("Map clicked: ", e)
}
function positionUpdated(pos: LatLngTuple) {
	userPosition.value = pos
	console.log("Position updated: ", pos)
}
</script>