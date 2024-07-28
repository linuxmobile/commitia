#!/usr/bin/env bun

import { firstLaunch, i18xs } from "~/utils/FIRST_LAUNCH";
import { setTimeout as sleep } from 'node:timers/promises';
import { DATA } from "~/utils/KEY";
import { outro, confirm, isCancel, cancel, multiselect, note, spinner } from "@clack/prompts";
import { readFirstLaunchFile } from "~/components/readFirstLaunchFile";
import { fileOptions, addStagedFiles, resetStagedFiles, getDiffSummary, commitStagedFiles } from "~/components/gitStageManager";
import { generatePrompt, cleanCommitMessage } from "~/utils/PROMPT_GENERATOR";

const s = spinner()
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

	const { added } = await getDiffSummary(stagedFiles);

	s.start('Generating commit message...');

	try {
		await generatePrompt(DATA, added);
		s.stop('Prompt generated successfully!');
		note(`"${cleanCommitMessage}"`)
	} catch (error) {
		s.stop('Failed to generate prompt.');
		console.error('Error generating prompt:', error);
		cancel(`${i18xs.t('common.operation_canceled')}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	const confirmCommit = await confirm({
		message: "¿Quieres hacer el commit?",
		default: true,
	});

	if (isCancel(confirmCommit) || !confirmCommit) {
		cancel(`${i18xs.t('common.operation_canceled')}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	outro("The commit was deployed!");
	await commitStagedFiles(cleanCommitMessage);
	await sleep(1000);

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