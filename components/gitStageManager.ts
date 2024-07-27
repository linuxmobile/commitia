import { simpleGit, type SimpleGit, type SimpleGitOptions } from "simple-git";
import { updateGlobalFilesContent } from "stageScrapper";

const options: Partial<SimpleGitOptions> = {
  baseDir: process.cwd(),
  binary: 'git',
  maxConcurrentProcesses: 6,
  trimmed: false,
};
const git: SimpleGit = simpleGit(options);

async function getFileOptions() {
  const status = await git.status();
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

async function getFileContent(stagedFiles: string[]): Promise<string> {
  console.log("Obteniendo contenido de los archivos seleccionados:", stagedFiles);
  const content = await updateGlobalFilesContent(stagedFiles);
  return content;
}

const fileOptions = await getFileOptions();

export { fileOptions, addStagedFiles, resetStagedFiles, getFileContent };