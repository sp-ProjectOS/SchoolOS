<script setup>
import { invoke } from "@tauri-apps/api";
import { reactive } from "vue";
import { useRouter } from "vue-router";
import { mdiLogin } from "@mdi/js";
import SectionFullScreen from "@/components/SectionFullScreen.vue";
import CardBox from "@/components/CardBox.vue";
import BaseButton from "@/components/BaseButton.vue";
import BaseButtons from "@/components/BaseButtons.vue";
import LayoutGuest from "@/layouts/LayoutGuest.vue";

const router = useRouter();

const submit = () => {
	invoke("login").then((response) => {
		if (response.error) {
			return;
		}
		router.push({ name: "dashboard" });
	});
};
</script>

<template>
	<LayoutGuest>
		<SectionFullScreen v-slot="{ cardClass }" bg="purplePink">
			<CardBox :class="cardClass" is-form @submit.prevent="submit">

				<h2 class="text-2xl font-bold">We use Oauth2 to login</h2>

				<template #footer>
					<BaseButtons>
						<BaseButton type="submit" color="info" label="Login with SchoolOS" :icon="mdiLogin" />
					</BaseButtons>
				</template>
			</CardBox>
		</SectionFullScreen>
	</LayoutGuest>
</template>
