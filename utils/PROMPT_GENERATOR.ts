import { createOpenAI } from "@ai-sdk/openai";
import { generateText } from "ai";
import { i18xs } from "../FIRST_LAUNCH";
import { readFirstLaunchFile } from "~/components/readFirstLaunchFile";

const DEFAULT_PROMPT: string = `${i18xs.t('common.default_prompt')}`;

async function generatePrompt(DATA: string, context: string): Promise<string> {
  const firstLaunchData = await readFirstLaunchFile();

  if (firstLaunchData) {
    if (firstLaunchData.lang === 'es') {
      i18xs.changeCurrentLocale('es');
    } else if (firstLaunchData.lang === 'en') {
      i18xs.changeCurrentLocale('en');
    }
  }

  const groq = createOpenAI({
    baseURL: "https://api.groq.com/openai/v1",
    apiKey: DATA,
  });

  const { text } = await generateText({
    model: groq("llama-3.1-70b-versatile"),
    prompt: `${DEFAULT_PROMPT} ${context}`,
    maxTokens: 250,
  });

  return text;
}