<template>
	<input type="text" ref="renameInput" v-model="value" @dblclick.stop @click.stop @change="onRenameInputComplete"
		@focusout="onRenameInputCancelled"
		class="w-full rounded-sm bg-plastic-100 px-2 -ml-[0.55rem] -mt-0.5 h-7 text-plastic-700">
</template>

<script setup lang="ts">
import { computed, Ref, ref } from '@vue/reactivity';
import { onMounted } from 'vue';

const props = defineProps(["modelValue"])
const emit = defineEmits(["update:modelValue", "completeRename", "cancelRename"])
const renameInput: Ref<HTMLInputElement | null> = ref(null);

const value = computed({
	get() {
		return props.modelValue;
	},
	set(value) {
		emit("update:modelValue", value);
	}
})

onMounted(() => {
	if (renameInput.value) {
		renameInput.value.select();
	}
});

function onRenameInputComplete() {
	emit("completeRename", { newName: props.modelValue });
}

function onRenameInputCancelled() {
	emit("cancelRename");
}
</script>