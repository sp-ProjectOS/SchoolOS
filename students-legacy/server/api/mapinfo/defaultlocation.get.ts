export default defineEventHandler(async (event) => {
	const config = useRuntimeConfig()
	if (!config.location) {
		console.error('No location found in runtime config')
		return {
			lat: 0,
			lng: 0,
		}
	}
	const { center } = config.location
	if (!center) {
		console.error('No center found in location config')
		return {
			lat: 0,
			lng: 0,
		}
	}
	const { lat, lng } = center
	if (!lat || !lng) {
		console.error('No lat or lng found in center config')
		return {
			lat: 0,
			lng: 0,
		}
	}
	return {
		lat : lat,
		lng : lng,
	}
})