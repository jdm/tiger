<template>
	<div ref="el">
		<Selectable @click="(event) => onAnimationClicked(event)" @dblclick="onAnimationDoubleClicked"
			:selected="animation.selected" :text="animation.name" left-icon="DocumentIcon" :actions="renaming ? [] : [
				{ icon: 'PencilAltIcon', callback: onRenameClicked },
				{ icon: 'XIcon', callback: onDeleteClicked }
			]">
			<template #content v-if="renaming">
				<InputRename v-model="newName" @complete-rename="onRenameInputComplete"
					@cancel-rename="onRenameInputCancelled" />
			</template>
		</Selectable>
	</div>
</template>

<script setup lang="ts">
import { Animation as AnimationDTO } from '@/api/dto'
import { deleteAnimation, editAnimation, renameAnimation, selectAnimation } from '@/api/document'
import { Ref, ref } from '@vue/reactivity';
import Selectable from '@/components/basic/Selectable.vue'
import InputRename from '@/components/basic/InputRename.vue';

const props = defineProps<{
	animation: AnimationDTO
}>();

const name = props.animation.name;
defineExpose({
	name,
	scrollIntoView() {
		if (el.value) {
			el.value.scrollIntoView({ behavior: "smooth", block: "center" });
		}
	}
});

const renaming = ref(false);
const newName = ref("");
const el: Ref<HTMLElement | null> = ref(null);

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey)
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}

function onRenameClicked() {
	newName.value = props.animation.name;
	renaming.value = true;
}

function onRenameInputComplete() {
	renameAnimation(props.animation.name, newName.value);
	renaming.value = false;
}

function onRenameInputCancelled() {
	renaming.value = false;
}

function onDeleteClicked() {
	deleteAnimation(props.animation.name);
}
</script>
