import { $ } from "bun";
import { updateGlobalFilesContent } from "stageScrapper";

const gitStatusOutputText = await $`git status --porcelain | awk '{print substr($0, 4)}'`.text();
const files = gitStatusOutputText.trim().split('\n');

const fileOptions = files.map(file => {
  return { value: file, label: file };
});

async function addStagedFiles(stagedFiles: string[]) {
  console.log("Agregando archivos seleccionados:", stagedFiles);
  for (const file of stagedFiles) {
    await $`git add ${file}`;
  }
}

async function resetStagedFiles(files: string[]) {
  console.log("Reseteando archivos seleccionados:", files);
  for (const file of files) {
    await $`git reset ${file}`;
  }
}

async function getFileContent(stagedFiles: string[]): Promise<string> {
  console.log("Obteniendo contenido de los archivos seleccionados:", stagedFiles);
  const content = await updateGlobalFilesContent(stagedFiles);
  console.log("Contenido de los archivos obtenidos:", content);
  return content;
}


export { fileOptions, addStagedFiles, resetStagedFiles, getFileContent };