<template>
	<div ref="thisComponent">
		<Selectable @click="(event) => onAnimationClicked(event)" @dblclick="onAnimationDoubleClicked"
			:selected="animation.selected" :text="animation.name" left-icon="DocumentIcon"
			:actions="!renaming ? [{ icon: 'PencilAltIcon', callback: onRenameClicked }, { icon: 'XIcon', callback: onDeleteClicked }] : []">
			<template #content v-if="renaming">
				<input type="text" ref="renameInput" v-model.trim="newName" @dblclick.stop @click.stop
					@change="onRenameInputComplete" @focusout="onRenameInputCancelled"
					class="w-full rounded-sm bg-plastic-100 px-2 -ml-[0.55rem] -mt-0.5 h-7 text-plastic-700">
			</template>
		</Selectable>
	</div>
</template>

<script setup lang="ts">
import { Animation as AnimationDTO } from '@/api/dto'
import { editAnimation, renameAnimation, selectAnimation } from '@/api/document'
import Selectable from '@/components/Selectable.vue'
import { Ref, ref } from '@vue/reactivity';
import { nextTick } from 'vue';

const props = defineProps<{
	animation: AnimationDTO
}>();

const name = props.animation.name;
defineExpose({
	name,
	scrollIntoView() {
		if (thisComponent.value) {
			thisComponent.value.scrollIntoView({ behavior: "smooth", block: "center" });
		}
	}
});

const renaming = ref(false);
const newName = ref("");
const renameInput: Ref<HTMLInputElement | null> = ref(null);
const thisComponent: Ref<HTMLElement | null> = ref(null);

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey)
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}

function onRenameClicked() {
	newName.value = props.animation.name;
	renaming.value = true;

	// TODO extract to a separate component and use onMounted() for this
	nextTick(() => {
		if (renameInput.value) {
			renameInput.value.select();
		}
	});
}

async function onRenameInputComplete() {
	renameAnimation(props.animation.name, newName.value);
	renaming.value = false;
}

function onRenameInputCancelled() {
	renaming.value = false;
}

function onDeleteClicked() {

}
</script>
