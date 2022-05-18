<template>
	<Selectable @click="(event) => onAnimationClicked(event)" @dblclick="onAnimationDoubleClicked"
		:selected="animation.selected" :text="animation.name" left-icon="DocumentIcon"
		:actions="[{ icon: 'PencilAltIcon', callback: onRenameClicked }, { icon: 'XIcon', callback: onDeleteClicked }]">
	</Selectable>
</template>

<script setup lang="ts">
import { Animation as AnimationDTO } from '@/api/dto'
import { editAnimation, selectAnimation } from '@/api/document'
import Selectable from '@/components/Selectable.vue'

const props = defineProps<{
	animation: AnimationDTO
}>();

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey)
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}

function onRenameClicked() {

}

function onDeleteClicked() {

}
</script>
