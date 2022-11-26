<template>
	<input type="text" ref="renameInput" v-model="value" @dblclick.stop @click.stop
		@keydown.enter="onRenameInputComplete" @keydown.escape="onRenameInputCancelled"
		@focusout="onRenameInputComplete"
		class="w-full rounded-sm bg-plastic-100 px-2 -ml-[0.55rem] -mt-0.5 mb-0.5 h-8 text-plastic-700">
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "@vue/reactivity"
import { onMounted } from "vue"

type RenameState = "active" | "cancelled";

const props = defineProps(["modelValue"])
const emit = defineEmits(["update:modelValue", "completeRename", "cancelRename"])
const renameInput: Ref<HTMLInputElement | null> = ref(null);
const state: Ref<RenameState> = ref("active");

const value = computed({
	get() {
		return props.modelValue;
	},
	set(value) {
		emit("update:modelValue", value);
	}
})

onMounted(() => {
	state.value = "active";
	if (renameInput.value) {
		renameInput.value.select();
	}
});

function onRenameInputComplete() {
	if (state.value != "cancelled") {
		emit("completeRename", { newName: props.modelValue });
	}
}

function onRenameInputCancelled() {
	state.value = "cancelled";
	emit("cancelRename");
}
</script>