<template>
	<input type="text" ref="renameInput" v-model="newName" @dblclick.stop @click.stop
		@keydown.enter="onRenameInputComplete" @keydown.escape="onRenameInputCancelled"
		@focusout="onRenameInputComplete"
		class="w-full rounded-sm bg-plastic-100 px-2 -ml-[0.55rem] -mt-0.5 mb-0.5 h-8 text-plastic-700">
</template>

<script setup lang="ts">
import { Ref, ref } from "@vue/reactivity"
import { nextTick, onMounted } from "vue"

type RenameState = "active" | "cancelled" | "complete";

const props = defineProps<{
	originalName: string
}>();

const emit = defineEmits<{
	(e: "completeRename", newName: string): void
	(e: "cancelRename"): void
}>();

const newName = ref("");
const renameInput: Ref<HTMLInputElement | null> = ref(null);
const state: Ref<RenameState> = ref("active");

onMounted(() => {
	newName.value = props.originalName;
	state.value = "active";
	nextTick(() => {
		if (renameInput.value) {
			renameInput.value.select();
		}
	});
});

function onRenameInputComplete() {
	if (state.value == "active") {
		state.value = "complete";
		emit("completeRename", newName.value);
	}
}

function onRenameInputCancelled() {
	state.value = "cancelled";
	emit("cancelRename");
}
</script>