<script setup lang="ts">
	import { ref } from "vue";
	import { invoke } from "@tauri-apps/api/tauri";
	import { store } from "./store.js";
	//varibles
	// const greetMsg = ref("");
	// const name = ref("");
	//functiongs
	// async function greet() {
	// 	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	// 	greetMsg.value = await invoke("greet", { name: name.value });
	// }
</script>

<template>
	<div id="Logs">
		<li v-for="(log, index) in store.logs" :title="log">
			<span v-if="index == 0">=></span>{{ log }}
		</li>
	</div>
</template>

<style scoped>
	#Logs {
		padding: 15px;
		min-width: 350px;
		text-align: left;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		/* position: sticky; */
		/* top: 0; */
		/* overflow: hidden; */
	}
	li {
		font-size: 1rem;
	}
</style>
