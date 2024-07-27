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
  console.log("Agregando archivos seleccionados:", stagedFiles);
  for (const file of stagedFiles) {
    try {
      console.log(`Agregando archivo: ${file}`);
      await git.add(file);
      console.log(`Archivo agregado: ${file}`);
    } catch (error) {
      console.error(`Error al agregar el archivo ${file}:`, error);
      throw error;
    }
  }

  try {
    const status = await git.status();
    console.log("Estado de git después de agregar archivos:", status);
    return stagedFiles;
  } catch (error) {
    console.error("Error al obtener el estado de git:", error);
    throw error;
  }
}

async function resetStagedFiles(files: string[]) {
  console.log("Reseteando archivos seleccionados:", files);
  for (const file of files) {
    try {
      await git.reset(['--', file]);
      console.log(`Archivo reseteado: ${file}`);
    } catch (error) {
      console.error(`Error al resetear el archivo ${file}:`, error);
      throw error;
    }
  }
}

async function getDiffSummary(files: string[]): Promise<string> {
  console.log("Obteniendo cambios de git diff para los archivos seleccionados:", files);
  try {
    const diffCommand = ['--cached', '--', ...files];
    const diff = await git.diff(diffCommand);
    console.log("Contenido del diff obtenido:", diff); // Agregar log para verificar el contenido del diff
    return diff;
  } catch (error) {
    console.error("Error al obtener los cambios de git diff:", error);
    throw error;
  }
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
    results.push(JSON.stringify(result, null, 2))
  }

  return results;
}

const fileOptions = await getFileOptions();

export { fileOptions, addStagedFiles, resetStagedFiles, getDiffSummary, getRuleResultsForFiles };