<template>
	<div id="mapview" style="height: 100%; width: 100%;" />
</template>
<style>
@import url("https://unpkg.com/leaflet@1.9.3/dist/leaflet.css");
</style>
<script lang="ts" setup>
import L, { LatLngTuple, map } from "leaflet"
import "leaflet-rotate"
import axios from "axios"

const emit = defineEmits([
	"mapClicked",
	"positionUpdated"
])
let map: L.Map | null = null

async function loadMap() {
	const res = await axios.get("/api/mapinfo/defaultlocation")
	if (!res) {
		console.error("Failed to fetch map info")
		return
	}
	const { data } = res
	console.log(data)
	if (!data) {
		console.error("Failed to load map")
		return
	}
	const { lat, lng } = data
	const location = [lat, lng] as LatLngTuple;
	map = L.map("mapview", {
		center: location,
		zoom: 18,
		zoomControl: false,
		attributionControl: false,
		// TODO: Fix ts-ignore
		/* @ts-ignore */
		rotate: true,
		touchRotate: true,
		/* @ts-ignore */
		rotateControl: {
			closeOnZeroBearing: false,
			position: 'bottomleft',
		},
	})
	// TODO: Fix ts-ignore
	/* @ts-ignore */
	//map.setBearing(160)
	map.setMaxZoom(22)
	map.setMinZoom(18)
	map.setMaxBounds(map.getBounds());


	L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
		attribution: "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors",
		maxNativeZoom: 19,
		maxZoom: 30,
		minZoom: 18,
	}).addTo(map)

	map.on("click", (e) => {
		console.log(e)
		emit("mapClicked", e)
	})
}
function setUserPosition(pos: LatLngTuple) {
	if (!map) {
		console.error("Map not loaded")
		return
	}
	// Find existing marker
	let marker : L.Marker | undefined;
	map?.eachLayer((layer) => {
		if (layer instanceof L.Marker) {
			if (layer.options.title === "user") {
				marker = layer
			}
		}
	})
	if (marker) {
		marker.setLatLng(pos)
	}
	else {
		L.marker(pos, {
			title: "user",
			icon: L.icon({
				iconUrl: "/img/user.png",
				iconSize: [32, 32],
				iconAnchor: [16, 16],
				popupAnchor: [0, -16],
			}),
		}).addTo(map)
	}
}
async function locateUser() {
	if (!navigator.geolocation) {
		console.error("Geolocation is not supported by your browser")
		return
	}
	function success(position: { coords: { latitude: number; longitude: number; }; }) {
		const latitude = position.coords.latitude
		const longitude = position.coords.longitude
		//console.log(`Latitude: ${latitude} °, Longitude: ${longitude} °`)
		// Center map on user
		if (!map) {
			console.error("Map not loaded")
			return
		}
		emit("positionUpdated", [latitude, longitude])

		// Get map bounds
		const bounds = map.getBounds()
		// If user is inside map bounds, center map on user
		if (bounds.contains([latitude, longitude])) {
			map.setView([latitude, longitude], 20)
			setUserPosition([latitude, longitude])
		}
		else {
			const { lat, lng } = bounds.getCenter()
			map.setView([lat, lng], 19)
		}
	}
	function error() {
		console.error("Unable to retrieve your location")
		if(map){
			const { lat, lng } = map.getBounds().getCenter()
			map.setView([lat, lng], 19)
		}
	}
	navigator.geolocation.getCurrentPosition(success, error)
}
let handles: any[] = []
onMounted(async () => {
	await loadMap()
	const handle = setInterval(locateUser, 5000)
	handles.push(handle)
})
onBeforeUnmount(() => {
	handles.forEach((handle) => {
		clearInterval(handle)
	})
})

</script>