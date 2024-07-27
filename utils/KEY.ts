import { T, K, N, M } from "./TKNM";
import { IV1, IV2, IV3, IV4 } from "./IV";
import { secretKey } from "./SECRET";
import { file } from "bun";

const keyArray = Uint8Array.from(
  atob(secretKey),
  (c) => c.charCodeAt(0)
);

async function decryptData(encryptedData: string, ivBase64: string): Promise<string> {
  const iv = Uint8Array.from(atob(ivBase64), (c) => c.charCodeAt(0));

  const decrypted = await crypto.subtle.decrypt(
    {
      name: "AES-GCM",
      iv: iv,
    },
    await crypto.subtle.importKey(
      "raw",
      keyArray,
      "AES-GCM",
      false,
      ["decrypt"]
    ),
    Uint8Array.from(atob(encryptedData), (c) => c.charCodeAt(0))
  );

  return new TextDecoder().decode(decrypted);
}

async function processDataParts(): Promise<string> {
  const part1 = await decryptData(T, IV1);
  const part2 = await decryptData(K, IV2);
  const part3 = await decryptData(N, IV3);
  const part4 = await decryptData(M, IV4);

  const combinedData = part1 + part2 + part3 + part4;

  return combinedData;
}

export let DATA: string;

async function getDataFromFile(filePath: string): Promise<string | null> {
  try {
    const firstLaunchFile = file(filePath);
    const fileExists = await firstLaunchFile.exists();
    if (!fileExists) {
      return null;
    }

    const fileContent = await firstLaunchFile.text();
    const jsonData = JSON.parse(fileContent);
    if (!jsonData.token) {
      return null;
    }

    return jsonData.token;
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    return null;
  }
}

export async function initializeData() {
  const token = await getDataFromFile('/tmp/commitia/firstLaunch.json');
  if (token) {
    DATA = token;
  } else {
    DATA = await processDataParts();
  }
}

export function setData(newData: string) {
  DATA = newData;
}

// Initialize DATA when the module is loaded
(async () => {
  await initializeData();
})();