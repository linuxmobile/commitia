import { $ } from "bun";
import { RULES } from "./utils/PROMPT_CONTEXT_RULES.ts";

let globalFilesContent = "";

async function getCurrentDirectory(): Promise<string> {
  try {
    const result = await $`pwd`.text();
    return result.trim();
  } catch (error) {
    console.error("Error al obtener el directorio actual:", error);
    return "";
  }
}

async function readFileContent(fullPath: string): Promise<string | null> {
  try {
    const extension = fullPath.split('.').pop();
    const fileName = fullPath.split('/').pop();
    let extractFunction: (context: string) => Promise<{ functions?: string[], constants?: string[], keys?: string[] }>;

    switch (extension) {
      case 'js':
        extractFunction = RULES.JAVASCRIPT;
        break;
      case 'nix':
        extractFunction = RULES.NIX;
        break;
      case 'json':
        extractFunction = RULES.JSON_KEYS;
        break;
      default:
        console.log(`No specific extraction function for .${extension} files, skipping.`);
        return null;
    }

    const content = await Bun.file(fullPath).text();
    const result = await extractFunction(content);

    const output = {
      fileName,
      functions: result.functions || [],
      constants: result.constants || [],
      keys: result.keys || []
    };

    return JSON.stringify(output, null, 2);
  } catch (error) {
    console.error("Error al leer el archivo:", error);
    return null;
  }
}

async function updateGlobalFilesContent(selectedFiles: string[]): Promise<string> {
  globalFilesContent = "";
  const currentDirectory = await getCurrentDirectory();
  for (const file of selectedFiles) {
    const fullPath = `${currentDirectory}/${file}`;
    const content = await readFileContent(fullPath);
    if (content !== null) {
      globalFilesContent += content + '\n';
    }
  }
  return globalFilesContent;
}

export { updateGlobalFilesContent };