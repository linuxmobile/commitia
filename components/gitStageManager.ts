import { simpleGit, type SimpleGit, type SimpleGitOptions } from "simple-git";
import { tokenCount } from "~/utils/countTokens";

const options: Partial<SimpleGitOptions> = {
  baseDir: process.cwd(),
  binary: 'git',
  maxConcurrentProcesses: 6,
  trimmed: false,
};
const git: SimpleGit = simpleGit(options);

let totalTokenCount = 0;

const status = await git.status();

async function getFileOptions() {
  const fileOptions = status.files.map(file => ({ value: file.path, label: file.path }));
  return fileOptions;
}

async function addStagedFiles(stagedFiles: string[]) {
  for (const file of stagedFiles) {
    await git.add(file);
  }
  return stagedFiles;
}

async function resetStagedFiles(files: string[]) {
  for (const file of files) {
    await git.reset(['--', file]);
  }
}

function parseDiff(diff: string, fileName: string) {
  const lines = diff.split('\n');
  let added: string[] = [];
  let currentFile = '';

  for (const line of lines) {
    if (line.startsWith('+++ b/')) {
      currentFile = line.replace('+++ b/', '');
    } else if (line.startsWith('+') && !line.startsWith('+++')) {
      let cleanedLine = line.substring(1)
        .replace(/\\(.)/g, '$1')
        .replace(/\s+/g, '')
        .replace(/\\"/g, '"')
        .trim()

      if (cleanedLine) {
        added.push(cleanedLine);
      }
    }
  }

  console.log("COMMILLAS DOBLES?", added)
  return {
    file: currentFile || fileName,
    added
  };
}

async function getDiffSummary(files: string[]): Promise<{ added: string[], totalTokenCount: number }> {
  const diffCommand = ['--cached', '--', ...files];
  const diff = await git.diff(diffCommand);
  const parsedDiff = parseDiff(diff, files[0]);

  const totalTokenCount = parsedDiff.added.reduce((sum, line) => sum + tokenCount(line), 0);

  return {
    added: parsedDiff.added,
    totalTokenCount
  };
}

const fileOptions = await getFileOptions();

export { fileOptions, addStagedFiles, resetStagedFiles, getDiffSummary, totalTokenCount };