import { z } from "zod";
import { generateObject } from "ai";
import { i18xs } from "~/utils/FIRST_LAUNCH";
import { readFirstLaunchFile } from "~/components/readFirstLaunchFile";
import { createGoogleGenerativeAI } from "@ai-sdk/google";

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

	const google = createGoogleGenerativeAI({
		apiKey: DATA,
	});

	try {
		const { object } = await generateObject({
			model: google("models/gemini-1.5-pro-latest"),
			schema: z.object({
				commit: z
					.object({
						convention: z
							.string()
							.describe(
								"Commit convention: feat, fix, chore, refactor, docs, style, test, etc.",
							),
						message: z
							.string()
							.describe(
								"Commit body message. Description of comprehensible changes, without convention.",
							),
					})
					.describe(
						"Commit message for the staged changes. Max characters: 74",
					),
			}),
			mode: "json",
			prompt: `${DEFAULT_PROMPT} ${context}`,
			maxTokens: 40,
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
