<template>
	<div class="disclaimer-div" v-if="!agreeDisclaimer">
		<h2 v-if="store.lang === 'en'">Disclaimer</h2>
		<h2 v-else>免责声明</h2>
		<h3 v-if="store.lang === 'en'">
			Do not use it for any illegal, harmful, or malicious purposes. The
			developer and provider are not liable for any consequences of your use or
			misuse of the software. You use the software at your own risk and
			responsibility. You must follow all laws and regulations in your area. If
			you disagree with these terms, do not use the software.
		</h3>
		<h3 v-else>
			请勿将其用于任何非法、有害或恶意的目的。开发人员和提供商不对您使用或滥用软件的任何后果负责。您自行承担使用本软件的风险和责任。你必须遵守你所在地区的所有法律法规。如果您不同意这些条款，请勿使用本软件。
		</h3>
		<div>
			<button @click="$emit('agreeToDisclaimer')">
				{{ store.lang === "en" ? "Agree" : "同意" }}
			</button>
			<button
				@click="
					async () => {
						store.add_log('not agree');
						await exit();
					}
				"
			>
				{{ store.lang === "en" ? "Disagree" : "不同意" }}
			</button>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { store } from "./store.js";
	import { exit } from "@tauri-apps/api/process";
	const props = defineProps<{
		agreeDisclaimer: boolean;
	}>();
</script>

<style scoped>
	.disclaimer-div {
		border: #a49898 1px solid;
		border-radius: 5px;
		padding: 5px 15px;
		margin-bottom: 20px;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
