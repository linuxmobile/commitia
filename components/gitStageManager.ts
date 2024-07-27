import { simpleGit, type SimpleGit, type SimpleGitOptions } from "simple-git";
import { RULES } from "~/utils/PROMPT_CONTEXT_RULES";

const options: Partial<SimpleGitOptions> = {
  baseDir: process.cwd(),
  binary: 'git',
  maxConcurrentProcesses: 6,
  trimmed: false,
};
const git: SimpleGit = simpleGit(options);

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
  let removed: string[] = [];
  let currentFile = '';

  for (const line of lines) {
    if (line.startsWith('+++ b/')) {
      currentFile = line.replace('+++ b/', '');
    } else if (line.startsWith('+') && !line.startsWith('+++')) {
      added.push(line.substring(1).replace(/\\t/g, ' ').replace(/\\/g, '').trim());
    } else if (line.startsWith('-') && !line.startsWith('---')) {
      removed.push(line.substring(1).replace(/\\t/g, ' ').replace(/\\/g, '').trim());
    }
  }

  return {
    file: currentFile || fileName,
    added,
    removed
  };
}

function formatJSONWithoutEscapes(obj: any): string {
  return JSON.stringify(obj, null, 2)
    .replace(/\\"/g, '"')
    .replace(/\\n/g, '\n')
    .replace(/\\t/g, '\t');
}

async function getDiffSummary(files: string[]): Promise<string> {
  const diffCommand = ['--cached', '--', ...files];
  const diff = await git.diff(diffCommand);
  const parsedDiff = parseDiff(diff, files[0]);
  return formatJSONWithoutEscapes({ parsed: parsedDiff });
}

async function applyRulesBasedOnExtension(file: string, diff: string): Promise<any> {
  const extension = file.split('.').pop();
  let ruleName: keyof typeof RULES;

  switch (extension) {
    case 'js':
    case 'ts':
      ruleName = 'JAVASCRIPT';
      break;
    case 'nix':
      ruleName = 'NIX';
      break;
    case 'json':
      ruleName = 'JSON_KEYS';
      break;
    default:
      throw new Error(`No rule defined for extension ${extension}`);
  }

  return await RULES[ruleName](diff);
}

async function getRuleResultsForFiles(files: string[]): Promise<string[]> {
  const results: string[] = [];

  for (const file of files) {
    const diff = await getDiffSummary([file]);
    const result = await applyRulesBasedOnExtension(file, diff);
    const formattedResult = JSON.stringify(result, null, 2);
    results.push(formattedResult);
  }

  return results.map(result => JSON.parse(result));
}

const fileOptions = await getFileOptions();

export { fileOptions, addStagedFiles, resetStagedFiles, getDiffSummary, getRuleResultsForFiles };