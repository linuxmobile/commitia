import { simpleGit, type SimpleGit, type SimpleGitOptions } from "simple-git";
import { tokenCount } from "~/utils/countTokens";

const options: Partial<SimpleGitOptions> = {
	baseDir: process.cwd(),
	binary: "git",
	maxConcurrentProcesses: 6,
	trimmed: false,
};

let totalTokenCount = 0;

const git: SimpleGit = simpleGit(options);
const status = await git.status();

async function checkIfGitRepo(): Promise<boolean> {
	return await git.checkIsRepo();
}

async function getFileOptions() {
	const fileOptions = status.files.map((file) => ({
		value: file.path,
		label: file.path,
	}));
	return fileOptions;
}

async function addStagedFiles(stagedFiles: string[]) {
	for (const file of stagedFiles) {
		await git.add(file);
	}
	return stagedFiles;
}

async function addAllFiles() {
    await git.add(".");
    const status = await git.status();
    return status.files.map(file => file.path);
}

async function resetStagedFiles(files: string[]) {
	for (const file of files) {
		await git.reset(["--", file]);
	}
}

function parseDiff(diff: string, fileName: string) {
	const lines = diff.split("\n");
	let added: string[] = [];
	let removed: string[] = [];
	let currentFile = "";

	for (const line of lines) {
		if (line.startsWith("+++ b/")) {
			currentFile = line.replace("+++ b/", "");
		} else if (line.startsWith("+") && !line.startsWith("+++")) {
			let cleanedLine = line
				.substring(1)
				.replace(/\\(.)/g, "$1")
				.replace(/\s+/g, "")
				.replace(/\\"/g, '"')
				.trim();

			if (cleanedLine) {
				added.push(cleanedLine);
			}
		} else if (line.startsWith("-") && !line.startsWith("---")) {
			let cleanedLine = line
				.substring(1)
				.replace(/\\(.)/g, "$1")
				.replace(/\s+/g, "")
				.replace(/\\"/g, '"')
				.trim();

			if (cleanedLine) {
				removed.push(cleanedLine);
			}
		}
	}

	return {
		file: currentFile || fileName,
		added: added.join(""),
		removed: removed.join(""),
	};
}

async function getDiffSummary(
	files: string[],
): Promise<{ added: string; removed: string }> {
	const diffCommand = ["--cached", "--", ...files];
	const diff = await git.diff(diffCommand);
	const parsedDiff = parseDiff(diff, files[0]);

	return {
		added: parsedDiff.added,
		removed: parsedDiff.removed,
	};
}

async function commitStagedFiles(commitMessage: string) {
	await git.commit(commitMessage);
}

const fileOptions = await getFileOptions();

export {
	addAllFiles,
	addStagedFiles,
	checkIfGitRepo,
	commitStagedFiles,
	fileOptions,
	getDiffSummary,
	resetStagedFiles,
	totalTokenCount,
};
