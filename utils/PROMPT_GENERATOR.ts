import { createOpenAI } from "@ai-sdk/openai";
import { z } from "zod";
import { generateObject } from "ai";
import { i18xs } from "~/utils/FIRST_LAUNCH";
import { readFirstLaunchFile } from "~/components/readFirstLaunchFile";

const DEFAULT_PROMPT: string = `${i18xs.t("common.default_prompt")}`;
let commitMessage: string = "";

async function generatePrompt(
	DATA: string,
	context: string[],
): Promise<string> {
	const firstLaunchData = await readFirstLaunchFile();

	if (firstLaunchData) {
		if (firstLaunchData.lang === "es") {
			i18xs.changeCurrentLocale("es");
		} else if (firstLaunchData.lang === "en") {
			i18xs.changeCurrentLocale("en");
		}
	}

	const groq = createOpenAI({
		baseURL: "https://api.groq.com/openai/v1",
		apiKey: DATA,
	});

	try {
		const { object } = await generateObject({
			model: groq("llama3-groq-70b-8192-tool-use-preview"),
			schema: z.object({
				commit: z
					.object({
						convention: z
							.string()
							.describe("Commit convention: feat, fix, etc."),
						message: z
							.string()
							.describe("Commit body message. Description of the changes"),
					})
					.describe("Commit message for the staged changes"),
			}),
			prompt: `${DEFAULT_PROMPT} ${context}`,
			maxTokens: 500,
			temperature: 0,
		});

		const { convention, message } = object.commit;
		commitMessage = `${convention}: ${message}`;
		return commitMessage;
	} catch (error) {
		console.error("Error generating text:", error);
		throw error;
	}
}

export { generatePrompt, commitMessage };
