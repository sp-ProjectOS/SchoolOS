<script setup>
import { invoke } from "@tauri-apps/api";
import { reactive } from "vue";
import { useRouter } from "vue-router";
import { mdiLoading } from "@mdi/js";
import SectionFullScreen from "@/components/SectionFullScreen.vue";
import CardBox from "@/components/CardBox.vue";
import LayoutGuest from "@/layouts/LayoutGuest.vue";
import BaseIcon from "@/components/BaseIcon.vue";


invoke("logout").then(() => {
	router.push({ name: "login" });
});
</script>

<template>
	<LayoutGuest>
		<SectionFullScreen v-slot="{ cardClass }" bg="purplePink">
			<CardBox :class="cardClass">
				<BaseIcon :path="mdiLoading" class="spinner" />
				Logging out...
			</CardBox>
		</SectionFullScreen>
	</LayoutGuest>
</template>

<style scoped>
.spinner {
	animation: spin 1s linear infinite;
}

@keyframes spin {
	0% {
		transform: rotate(0deg);
	}

	100% {
		transform: rotate(360deg);
	}
}
</style>
