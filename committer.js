#!/usr/bin/env bun

import { firstLaunch, i18xs } from "./FIRST_LAUNCH";
import { DATA } from "./utils/KEY";
import { outro, confirm, isCancel, cancel, multiselect } from "@clack/prompts";
import { readFirstLaunchFile } from "./components/readFirstLaunchFile";
import { fileOptions, addStagedFiles, resetStagedFiles, getDiffSummary } from "~/components/gitStageManager";

const selectedFilesOptions = fileOptions;

async function main() {
	const firstLaunchData = await readFirstLaunchFile();

	if (firstLaunchData) {
		if (firstLaunchData.lang === 'es') {
			i18xs.changeCurrentLocale('es');
		} else if (firstLaunchData.lang === 'en') {
			i18xs.changeCurrentLocale('en');
		}
	}

	console.clear()

	const askGitCommitStatus = await confirm({
		message: `${i18xs.t('common.ask_select_files')}`,
		default: true,
	});

	if (isCancel(askGitCommitStatus)) {
		cancel(`${i18xs.t('common.operation_canceled')}`);
		return process.exit(0);
	}

	let selectedFiles;
	if (!askGitCommitStatus) {
		selectedFiles = files;
	} else {
		selectedFiles = await multiselect({
			message: `${i18xs.t('common.selecting_files')}`,
			options: selectedFilesOptions,
		});
	}

	if (isCancel(selectedFiles)) {
		cancel(`${i18xs.t('common.operation_canceled')}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	const stagedFiles = await addStagedFiles(selectedFiles);

	const { added, totalTokenCount } = await getDiffSummary(stagedFiles);
	console.log("Resumen de cambios:", { added, totalTokenCount });
}

async function checkAndRun() {
	try {
		const firstLaunchData = await readFirstLaunchFile();

		if (!firstLaunchData || !firstLaunchData.exist) {
			await firstLaunch();
		}
		await main();
	} catch (error) {
		console.error(`${i18xs.t('common.error_verifying_launchfile')}`, error);
		await firstLaunch();
		await main();
	}
}

checkAndRun().catch(console.error);