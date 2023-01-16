<template>
	<TransitionGroup tag="div" name="list" class="h-full w-96 justify-end relative flex flex-col gap-1.5"
		@before-leave="beforeLeave">
		<Notification v-for="[id, notification] in notifications" :key="id" :id="id" :flavor="notification.flavor"
			:title="notification.title" :icon="notification.icon" :description="notification.description"
			:actions="notification.actions" @close="closeNotification" />
	</TransitionGroup>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import Notification, { NotificationData } from "@/components/basic/Notification.vue"

const notifications: Ref<[number, NotificationData][]> = ref([]);
let nextId = 0;

defineExpose({
	push: pushNotification,
});

function pushNotification(data: NotificationData) {
	const id = nextId;
	nextId += 1;
	notifications.value.push([id, data]);
	while (notifications.value.length > 4) {
		notifications.value.shift();
	}
}

function closeNotification(idToClose: number) {
	const index = notifications.value.findIndex(([id, notification]) => id == idToClose);
	if (index >= 0) {
		notifications.value.splice(index, 1);
	}
}

// See https://stackoverflow.com/questions/59650480/vue-transition-group-item-with-flex-grid-wrapper-moves-to-top-left
function beforeLeave(element: Element) {
	const el = element as HTMLElement;
    const {marginLeft, marginTop, width, height} = window.getComputedStyle(el)
    el.style.left = `${el.offsetLeft - parseFloat(marginLeft)}px`;
    el.style.top = `${el.offsetTop - parseFloat(marginTop)}px`;
    el.style.width = width;
    el.style.height = height;
}
</script>

<style>
.list-move,
.list-enter-active,
.list-leave-active {
	transition: all 0.3s ease;
}

.list-enter-from,
.list-leave-to {
	opacity: 0;
	transform: translateX(30px);
}

.list-leave-active {
	position: absolute;
}
</style>
