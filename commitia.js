#!/usr/bin/env bun

import {
	cancel,
	confirm,
	isCancel,
	multiselect,
	note,
	outro,
	spinner,
} from "@clack/prompts";
import { setTimeout as sleep } from "node:timers/promises";
import {
	checkIfGitRepo,
	addStagedFiles,
	commitStagedFiles,
	fileOptions,
	getDiffSummary,
	resetStagedFiles,
	addAllFiles
} from "~/components/gitStageManager";
import {
	readFirstLaunchFile,
	firstLaunchFile,
} from "~/components/readFirstLaunchFile";
import { firstLaunch, i18xs } from "~/utils/FIRST_LAUNCH";
import { DATA } from "~/utils/KEY";
import { generatePrompt, commitMessage } from "~/utils/PROMPT_GENERATOR";

const s = spinner();
const selectedFilesOptions = fileOptions;

async function main() {
	const firstLaunchData = await readFirstLaunchFile();

	if (firstLaunchData) {
		if (firstLaunchData.lang === "es") {
			i18xs.changeCurrentLocale("es");
		} else if (firstLaunchData.lang === "en") {
			i18xs.changeCurrentLocale("en");
		}
	}

	const isGitRepo = await checkIfGitRepo();

	if (!isGitRepo) {
		note(`${i18xs.t("common.is_git_repo")}`);
		outro(`${i18xs.t("common.goodbye")}`);
		return process.exit(0);
	}

	console.clear();

	const askGitCommitStatus = await confirm({
		message: `${i18xs.t("common.ask_select_files")}`,
		default: true,
	});

	if (isCancel(askGitCommitStatus)) {
		cancel(`${i18xs.t("common.operation_canceled")}`);
		return process.exit(0);
	}

	let selectedFiles;
	if (!askGitCommitStatus) {
		selectedFiles = await addAllFiles();
	} else {
		selectedFiles = await multiselect({
			message: `${i18xs.t("common.selecting_files")}`,
			options: selectedFilesOptions,
		});
	}

	if (isCancel(selectedFiles)) {
		cancel(`${i18xs.t("common.operation_canceled")}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	const stagedFiles = await addStagedFiles(selectedFiles);

	const { added, removed } = await getDiffSummary(stagedFiles);

	s.start(`${i18xs.t("common.generating_commit")}`);

	try {
		await generatePrompt(DATA, added, removed);
		s.stop(`${i18xs.t("common.prompt_generated")}`);
		note(`"${commitMessage}"`);
	} catch (error) {
		s.stop(`${i18xs.t("common.prompt_failed")}`);
		console.error(`${i18xs.t("common.prompt_error")}`, error);
		cancel(`${i18xs.t("common.operation_canceled")}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	const confirmCommit = await confirm({
		message: `${i18xs.t("common.ask_do_commit")}`,
		default: true,
	});

	if (isCancel(confirmCommit) || !confirmCommit) {
		cancel(`${i18xs.t("common.operation_canceled")}`);
		await resetStagedFiles(selectedFiles);
		return process.exit(0);
	}

	outro(`${i18xs.t("common.commit_deployed")}`);
	await commitStagedFiles(commitMessage);
	await sleep(1000);
}

async function checkAndRun() {
	const firstLaunchData = await readFirstLaunchFile();
	try {
		if (!(await firstLaunchFile.exists()) || !firstLaunchData) {
			await firstLaunch();
		}
		await main();
	} catch (error) {
		console.error(`${i18xs.t("common.error_verifying_launchfile")}`, error);
		await firstLaunch();
		await main();
	}
}

checkAndRun().catch(console.error);
