<template>
	<Select :options="options" :selected="currentPreset" @selected="onSelected" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import Select, { SelectOption } from '@/components/basic/Select.vue'
import { DirectionPreset } from '@/api/dto';
import { applyDirectionPreset } from '@/api/document';
import { useAppStore } from '@/stores/app';

const app = useAppStore();
const currentPreset = computed(() => app.currentAnimation?.directionPreset);

const options = computed((): SelectOption[] => {
	return [
		{ name: "4 Directions", value: DirectionPreset.FourDirections },
		{ name: "8 Directions", value: DirectionPreset.EightDirections },
		{ name: "Left / Right", value: DirectionPreset.LeftRight },
		{ name: "Up / Down", value: DirectionPreset.UpDown },
		{ name: "Isometric", value: DirectionPreset.Isometric },
		{ name: "Fixed Angle", value: DirectionPreset.FixedAngle },
	];
});

function onSelected(option: SelectOption) {
	applyDirectionPreset(option.value);
}

</script>