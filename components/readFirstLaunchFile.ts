import { file } from "bun";

const firstLaunchFile = file("/tmp/commitia/firstLaunch.json");

async function readFirstLaunchFile() {
	try {
		if (await firstLaunchFile.exists()) {
			const fileContent = await firstLaunchFile.text();
			if (fileContent.trim() === "") {
				return null;
			}
			try {
				return JSON.parse(fileContent);
			} catch (error) {
				console.error("Error parsing JSON:", error);
				return null;
			}
		}
	} catch (error) {
		console.error("Error parsing JSON:", error);
	}
	return null;
}

export { readFirstLaunchFile, firstLaunchFile };
