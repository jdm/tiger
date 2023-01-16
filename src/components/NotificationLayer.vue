<template>
	<div class="flex h-full w-full justify-end items-end px-14 py-16 pointer-events-none">
		<Notifications ref="notifications" />
	</div>
</template>

<script setup lang="ts">
import Notifications from "@/components/basic/Notifications.vue"
import { onMounted, Ref, ref } from "vue";

const notifications: Ref<InstanceType<typeof Notifications> | null> = ref(null);

onMounted(() => {
	if (!notifications.value) {
		return;
	}
	// notifications.value.push({
	// 	text: "Oink",
	// });
	// notifications.value.push({
	// 	text: "Gruik",
	// });
	// notifications.value.push({
	// 	text: "Miam",
	// });
});
</script>
