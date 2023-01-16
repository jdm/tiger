<template>
	<div class="flex flex-col gap-1.5">
		<Notification v-for="notification in notifications" :text="notification.text" />
	</div>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import Notification, { NotificationData } from "@/components/basic/Notification.vue"

const notifications: Ref<NotificationData[]> = ref([]);

defineExpose({
	push: pushNotification,
});

function pushNotification(data: NotificationData) {
	notifications.value.push(data);
	while (notifications.value.length > 5) {
		notifications.value.shift();
	}
}
</script>
