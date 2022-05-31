<template>
	<Select :options="options" :selected="currentOption" @selected="onSelected" />
</template>

<script setup lang="ts">
import { computed, Ref, ref } from 'vue';
import Select, { SelectOption } from '@/components/basic/Select.vue'
import { DirectionPreset } from '@/api/dto';

const currentOption = ref(DirectionPreset.FixedAngle);

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
	currentOption.value = option.value;
}

</script>